# Hashstorage port, modify it to change
export HASHSTORAGE_PORT=8080

# Hashstorage database location, modify it to change
export HASHSTORATE_DB_DIR=$(pwd)/db

# Build and run a docker container
docker build -t hashstorage .
docker run -it \
    -p $HASHSTORAGE_PORT:8080 \
    --restart=always \
    --volume $HASHSTORATE_DB_DIR:/app/db \
    --name hashstorage-app \
    -d hashstorage
