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

## Running The RESTful Actix Web API

First, open the terminal and clone the project using `git` (or you can simply download the project) and then change the directory:

```Rust
git clone https://github.com/lastemp/mpesa-system.git 
cd mpesa-system
```

```MySQL
Open folder "mpesa-system\sql" that contains the SQL files to create the tables in your local MySQL database server.
Please ensure you execute the SQL statements found in the files in your MySQL database server.
```

```Config file
Open file "xxx" located in path "mpesa-system\xxx" and then 
change the configurations to match the credentials of your MySQL database server.
```

Once you are inside the project directory, run the application:

```Rust
cargo run
```