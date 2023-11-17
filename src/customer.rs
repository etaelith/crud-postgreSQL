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
#[derive(Serialize, Deserialize)]
struct CustomerRes {
    customer_id: i32,
    first_name: String,
    last_name: String,
    email: String,
    active: bool,
    address_id: i32,
    created: std::time::SystemTime,
    updated: std::time::SystemTime,
}
fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}
fn get_request_body(request: &str) -> Result<Customer, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
fn insert_country(client: &mut Client, country: &Country) -> Result<(), PostgresError> {
    client.execute(
        "INSERT INTO country (country) VALUES ($1) ON CONFLICT (country) DO NOTHING",
        &[&country.country],
    )?;
    Ok(())
}

fn insert_city(client: &mut Client, city: &City) -> Result<(), PostgresError> {
    client.execute(
        "INSERT INTO city (city, country_id) VALUES ($1, (SELECT country_id FROM country WHERE country = $2)) ON CONFLICT (city, country_id) DO NOTHING",
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
    user: Customer,
    address_id: i32,
) -> Result<(), PostgresError> {
    client.execute(
        "INSERT INTO customer (first_name, last_name, email, address_id, active) VALUES ($1, $2, $3, $4, $5)",
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
            insert_customer(&mut client, user, address_id).unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
pub fn handle_get_request(request: &str) -> (String, String) {
    println!("hola");
    match (
        get_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            match client.query_one("SELECT * FROM customer WHERE customer_id = $1", &[&id]) {
                Ok(row) => {
                    let customer = CustomerRes {
                        customer_id: row.get(0),
                        first_name: row.get(1),
                        last_name: row.get(2),
                        email: row.get(3),
                        address_id: row.get(4),
                        active: row.get(5),
                        created: row.get(6),
                        updated: row.get(7),
                    };
                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string(&customer).unwrap(),
                    )
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
pub fn handle_get_all_request(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut customers = Vec::new();
            for row in client
                .query(
                    "SELECT customer_id, first_name, last_name, email, address_id, active, create_date,last_update FROM customer",
                    &[],
                )
                .unwrap()
            {
                customers.push(CustomerRes {
                    customer_id: row.get(0),
                    first_name: row.get(1),
                    last_name: row.get(2),
                    email: row.get(3),
                    address_id: row.get(4),
                    active: row.get(5),
                    created: row.get(6),
                    updated: row.get(7),
                })
            }
            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&customers).unwrap(),
            )
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_put_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        get_request_body(&request),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET first_name = $1, email = $2 WHERE customer_id = $3",
                    &[&user.first_name, &user.email, &id],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
pub fn handle_delete_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client
                .execute("DELETE FROM customer WHERE customer_id = $1", &[&id])
                .unwrap();

            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
