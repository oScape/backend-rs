# backend-rs

Multithreaded Web Server in rust-lang with a model managed by Diesel as ORM for a MySQL database.

## MySQL
- Follow [this tutorial](https://steemit.com/programming/@mrblueberry/installing-rust-and-diesel-for-rocket-on-windows-10) or:
- Download the `mysql-installer-community-8.X`
- Install MySQL Server (mandatory) and Workbench (optional)
- Add Connector/C 6.1 (mandatory)
- Run `setx MYSQLCLIENT_LIB_DIR "C:\Program Files\MySQL\MySQL Connector C 6.1\lib\vs14"`
- Create a database
- Add the env. path `DATABASE_URL` `mysql://username:password@localhost/db_name`
- If some auth issues look [here](https://stackoverflow.com/questions/49194719/authentication-plugin-caching-sha2-password-cannot-be-loaded).

## diesel_cli
- Run `cargo install diesel_cli --no-default-features --features mysql`