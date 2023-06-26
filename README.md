# mpesa-system

This is a RESTful Actix Web API that connects to MySQL database. 
It integrates with Safaricom M-Pesa Mobile Money Payment Gateway (i.e exposed API endpoints for accessing M-Pesa services by Kenyan Telco called "Safaricom")
and enables customers to transfer money and pay for utilities like water, PayTv, electricity from their phone wallets. 
The Kenyan Telco "Safaricom" has provided M-Pesa API endpoints for B2C, C2B and B2B (https://developer.safaricom.co.ke/Documentation). 

Currently this RESTful API supports: 
- Customer to Business
- Generate an OAuth Access Token
- Register C2B Confirmation and Validation URLs

The RESTful Actix Web API has below listed dependencies:
- [Actix Web](https://github.com/actix/actix-web) web framework for Rust
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [MySQL](https://github.com/mysql/mysql-server) MySQL database server
- [mysql](https://github.com/blackbeam/rust-mysql-simple) MySql database driver

## Instructions

### NOTE:

You may need to ensure that you are running the commands with the correct MySQL user/password.

1. Access MySQL Server

   Log in to the MySQL Server using a user account that has the CREATE DATABASE privilege.

2. Create database

   ```sql
   CREATE DATABASE my_mpesa;
   ```

3. Create tables in the database

   Directory "mysql\sql" contains below listed ".sql" files:
   - *.sql

   Copy the contents of each of the ".sql" and execute them separately on MySQL Server. This will create tables/stored procedures in the database.

4. Create `.env` file:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   MYSQL_USER=XXX
   MYSQL_PASSWORD=XXX
   MYSQL_HOST=127.0.0.1
   MYSQL_PORT=3306
   MYSQL_DBNAME=my_mpesa
   ```
   
   Update "MYSQL_USER" and "MYSQL_PASSWORD" values with the correct MySQL user/password.
   If your password contains dollar sign "$", then remember to escape it eg "123$abc" will need to be changed to "123\\$abc"

5. Run the server:

   ```shell
   cargo run
   ```

6. Using a different terminal send an HTTP GET/POST requests to the running server:

   Directory "mysql\apis" contains below listed api's files:
   - generateauth.txt
   - registerclienturls.txt
   - validationc2b.txt
   - confirmationc2b.txt

   Copy the curl request on each of the ".txt" and execute them on separate terminals. Each ".txt" contains curl request and expected json reponse data.