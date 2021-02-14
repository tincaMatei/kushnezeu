# BacDB

This is a small website in which you can have groups that can edit different pages. You can manage the userrs, the groups and the privilleges for each group.

# Components

* an Nginx server that returns the static content from the frontend
* a PostgresQL database
* a Rust backend that manages the information from the database
* a rust binary crate that can be used for administrator management

# Dependencies

* docker
* docker-compose
* cargo

# Installation

Run `docker-compose up`, and the site should be running.
To run the administrator commands, you should compile the project by running:

```
cd backend
cargo build --release --bin bacdb-admin
```

To do admin commands, run:

```
cargo run --release --bin bacdb-admin [COMMANDS] [ARGUMENTS]
```
