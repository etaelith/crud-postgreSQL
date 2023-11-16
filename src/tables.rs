use postgres::Error as PostgresError;
use postgres::{Client, NoTls};

use crate::DB_URL;

pub fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS country (
            country_id SERIAL PRIMARY KEY,
            country VARCHAR(50) NOT NULL,
            last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS city (
            city_id SERIAL PRIMARY KEY,
            city VARCHAR(50) NOT NULL,
            country_id SMALLINT REFERENCES country (country_id) ON DELETE RESTRICT ON UPDATE CASCADE,
            last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS address (
            address_id SERIAL PRIMARY KEY,
            address VARCHAR(50) NOT NULL,
            address2 VARCHAR(50),
            province VARCHAR(20) NOT NULL,
            postal_code VARCHAR(10),
            phone VARCHAR(20) NOT NULL,
            city_id INTEGER REFERENCES city (city_id) ON DELETE RESTRICT ON UPDATE CASCADE,
            last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS customer (
            customer_id SERIAL PRIMARY KEY,
            first_name VARCHAR(45) NOT NULL,
            last_name VARCHAR(45) NOT NULL,
            email VARCHAR(50),
            address_id INTEGER REFERENCES address (address_id) ON DELETE RESTRICT ON UPDATE CASCADE,
            active BOOLEAN NOT NULL DEFAULT TRUE,
            create_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_update TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
        
        ",
    )?;
    Ok(())
}
