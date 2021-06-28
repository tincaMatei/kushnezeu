# [Kushnezeu.go.ro](http://www.kushnezeu.go.ro)

This is a small web server used to host my blog. It can also be used to share and edit content privately by having accounts. The accounts can only be made by using some commands.

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
