# secure-messaging-server
This project demonstrates a basic Rust application that connects to a MySQL database for managing users in a secure messaging system.

Prerequisites

Before you begin, ensure you have the following installed:

Rust (stable version recommended)
MySQL server
Cargo (Rust's package manager)
Setup

Clone the repository:

bash
Copy code
git clone https://github.com/cayanide/secure-messaging-server.git
cd secure-messaging-server
Install dependencies:

Ensure dependencies are installed using Cargo:

bash
Copy code
cargo build
Configure MySQL Database:

Create a MySQL database for the project (e.g., secure_messaging_db).

Create a user (e.g., secure_user) with appropriate privileges on this database:

sql
Copy code
CREATE DATABASE secure_messaging_db;
CREATE USER 'secure_user'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON secure_messaging_db.* TO 'secure_user'@'localhost';
FLUSH PRIVILEGES;
Modify Connection URL:

Update main.rs with your MySQL connection URL:

rust
Copy code
let url = "mysql://secure_user:your_password@localhost:3306/secure_messaging_db";
Usage

To run the application:

bash
Copy code
cargo run
This will compile and execute the Rust application. If successful, it will establish a connection to your MySQL database (secure_messaging_db) using the provided credentials (secure_user) and print Connection successful! upon successful connection.

Troubleshooting

Connection Issues:
If you encounter connection errors, ensure that your MySQL server is running (sudo lsof -iTCP -sTCP:LISTEN -n -P | grep 3306) and that the user credentials and database name in your connection URL are correct.

Dependencies:
Ensure all dependencies specified in Cargo.toml are correctly installed using cargo build.

MySQL Privileges:
Verify that the MySQL user (secure_user) has sufficient privileges (ALL PRIVILEGES) on the database (secure_messaging_db).

Contributing

Contributions are welcome! Fork the repository and submit a pull request with your enhancements.

License

This project is licensed under the MIT License - see the LICENSE file for details.
