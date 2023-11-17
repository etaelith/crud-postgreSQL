# Rust CRUD App with PostgreSQL and Docker

This repository contains a simple CRUD (Create, Read, Update, Delete) application written in Rust, using PostgreSQL as the database and Docker for containerization.

## Getting Started

### Prerequisites

Make sure you have Docker and Docker Compose installed on your machine.

### Running the Application

1. Clone the repository:

```bash
   git clone https://github.com/etaelith/crud-postgreSQL.git
   cd crud-postgreSQL
```

2. Build and run the Docker containers:

```bash
docker-compose up --build
```

This command will build the Rust application, set up the PostgreSQL database, and start the application.

3. Access the application:
   The Rust application will be accessible at http://localhost:8080.

## Docker Compose Configuration

The docker-compose.yml file defines two services: rustapp (Rust application) and db (PostgreSQL database).

```
version: "3.9"

services:
  rustapp:
    # ... (details about the Rust application container)

  db:
    # ... (details about the PostgreSQL container)

volumes:
  pgdata: {}
```

## Application Structure

Main.rs
The main.rs file is the entry point of the Rust application. It sets up a TCP listener and handles incoming requests.

Tables.rs
The tables.rs file contains functions for setting up the PostgreSQL database tables.

Customer.rs
The customer.rs file defines data structures and functions for handling CRUD operations on customer data.

## API Endpoints

1. Create a new customer

```
POST /users
```

### Body

```bash
{
  "first_name": "John",
  "last_name": "Doe",
  "email": "john.doe@example.com",
  "active": true,
  "address": {
    "address": "123 Main St",
    "address2": "",
    "province": "Sample Province",
    "postal_code": "12345",
    "phone": "555-1234",
    "city": {
      "city": "Sample City",
      "country": {
        "country": "Sample Country"
      }
    }
  }
}
```

2. Get a customer by ID

```
GET /users/{customer_id}
```

### Response

```bash
{
    "customer_id": 1,
    "first_name": "Sancho",
    "last_name": "Perez",
    "email": "sancho.perez@example.com",
    "active": true,
    "address_id": 1,
    "created": {
        "secs_since_epoch": 1700249932,
        "nanos_since_epoch": 215310000
    },
    "updated": {
        "secs_since_epoch": 1700249932,
        "nanos_since_epoch": 215310000
    }
}
```

3. Get all customers

```
GET /users
```

### Response

```bash
[
    {
        "customer_id": 1,
        "first_name": "Sancho",
        "last_name": "Perez",
        "email": "sancho.perez@example.com",
        "active": true,
        "address_id": 1,
        "created": {
            "secs_since_epoch": 1700249932,
            "nanos_since_epoch": 215310000
        },
        "updated": {
            "secs_since_epoch": 1700249932,
            "nanos_since_epoch": 215310000
        }
    },
    {
        "customer_id": 2,
        "first_name": "Sancho",
        "last_name": "Perez",
        "email": "sancho.perez@example.com",
        "active": true,
        "address_id": 3,
        "created": {
            "secs_since_epoch": 1700250013,
            "nanos_since_epoch": 698417000
        },
        "updated": {
            "secs_since_epoch": 1700250013,
            "nanos_since_epoch": 698417000
        }
    },
    // ... (other customer entries)
]
```

4. Update a customer by ID

```
PUT /users/{customer_id}
```

### Body

```bash
{
  "first_name": "UpdatedFirstName",
  "email": "updated.email@example.com"
}
```

4. Delete a customer by ID

```
DELETE /users/{customer_id}
```

# Contributing

Feel free to contribute to this project by opening issues or submitting pull requests.

MIT License

Copyright (c) [2023] [Etaelithtest@gmail.com]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
