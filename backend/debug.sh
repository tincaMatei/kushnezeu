cargo build --bin bacdb-server
cp target/debug/bacdb-server .
docker-compose -f "../docker-compose-dev.yml" up --build
