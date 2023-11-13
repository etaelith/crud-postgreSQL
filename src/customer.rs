use crate::{DB_URL, INTERNAL_ERROR, NOT_FOUND, OK_RESPONSE};
use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
#[derive(Serialize, Deserialize)]
struct Customer {
    first_name: String,
    last_name: String,
    email: String,
    active: bool,
    address: Address,
}
#[derive(Serialize, Deserialize)]
struct Address {
    address: String,
    address2: String,
    province: String,
    postal_code: String,
    phone: String,
    city: City,
}
#[derive(Serialize, Deserialize)]
struct City {
    city: String,
    country: Country,
}
#[derive(Serialize, Deserialize)]
struct Country {
    country: String,
}
fn get_request_body(request: &str) -> Result<Customer, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
fn insert_country(client: &mut Client, country: &Country) -> Result<(), PostgresError> {
    client.execute(
        "INSERT INTO country (country) VALUES ($1)",
        &[&country.country],
    )?;
    Ok(())
}

fn insert_city(client: &mut Client, city: &City) -> Result<(), PostgresError> {
    client.execute(
        "INSERT INTO city (city, country_id) VALUES ($1, (SELECT country_id FROM country WHERE country = $2))",
        &[&city.city, &city.country.country],
    )?;
    Ok(())
}

fn insert_address(client: &mut Client, address: &Address) -> Result<i32, PostgresError> {
    client.query_one(
        "INSERT INTO address (address, address2, province, postal_code, phone, city_id) VALUES ($1, $2, $3, $4, $5, (SELECT city_id FROM city WHERE city = $6)) RETURNING address_id",
        &[&address.address, &address.address2, &address.province, &address.postal_code, &address.phone, &address.city.city],
    ).map(|row| row.get(0))
}

fn insert_customer(
    client: &mut Client,
    user: &Customer,
    address_id: i32,
) -> Result<(), PostgresError> {
    client.execute(
        "INSERT INTO customer (first_name, last_name, email, address_id, active) VALUES ($1, $2, $3, $4, $5, $6)",
        &[ &user.first_name, &user.last_name, &user.email, &address_id, &user.active],
    )?;
    Ok(())
}
pub fn handle_post_request(request: &str) -> (String, String) {
    match (get_request_body(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            insert_country(&mut client, &user.address.city.country).unwrap();
            insert_city(&mut client, &user.address.city).unwrap();
            let address_id = insert_address(&mut client, &user.address).unwrap();
            insert_customer(&mut client, &user, address_id).unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
