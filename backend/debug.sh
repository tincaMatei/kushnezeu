cargo build
cp target/debug/backend .
sudo docker-compose -f "../docker-compose-dev.yml" up --build
