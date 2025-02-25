git pull origin main

touch data.db
cargo sqlx mig run

docker build -t APP_NAME .
docker kill APP_NAME_container
docker container prune -f
docker run -d \
	-p 8080:8080 \
    -v $(pwd)/data.db:/app/data.db \
	--name APP_NAME_container APP_NAME

