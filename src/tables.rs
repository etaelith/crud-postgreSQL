use postgres::Error as PostgresError;
use postgres::{Client, NoTls};

use crate::DB_URL;

pub fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS country (
            country_id SMALLSERIAL PRIMARY KEY,
            country VARCHAR(50) NOT NULL,
            last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS city (
            city_id SMALLSERIAL PRIMARY KEY,
            city VARCHAR(50) NOT NULL,
            country_id SMALLINT REFERENCES country (country_id) ON DELETE RESTRICT ON UPDATE CASCADE,
            last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS address (
            address_id SMALLSERIAL PRIMARY KEY,
            address VARCHAR(50) NOT NULL,
            address2 VARCHAR(50),
            province VARCHAR(20) NOT NULL,
            postal_code VARCHAR(10),
            phone VARCHAR(20) NOT NULL,
            city_id SMALLINT REFERENCES city (city_id) ON DELETE RESTRICT ON UPDATE CASCADE,
            last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS customer (
            customer_id SMALLSERIAL PRIMARY KEY,
            store_id SMALLINT NOT NULL,
            first_name VARCHAR(45) NOT NULL,
            last_name VARCHAR(45) NOT NULL,
            email VARCHAR(50),
            address_id SMALLINT REFERENCES address (address_id) ON DELETE RESTRICT ON UPDATE CASCADE,
            active BOOLEAN NOT NULL DEFAULT TRUE,
            create_date TIMESTAMP NOT NULL,
            last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
        
        ",
    )?;
    Ok(())
}
