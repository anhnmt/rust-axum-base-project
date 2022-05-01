APP_NAME=rust-axum-base-project
APP_VERSION=latest
DOCKER_REGISTRY=registry.gitlab.com/xdorro/registry
MAIN_DIR=./cmd

docker.build:
	docker build -f ./Dockerfile -t $(DOCKER_REGISTRY)/$(APP_NAME):$(APP_VERSION) .

docker.push:
	docker push $(DOCKER_REGISTRY)/$(APP_NAME):$(APP_VERSION)

docker.run:
	docker rm -f $(APP_NAME)
	docker-compose -f ./docker-compose.yml up -d --force-recreate

docker.dev: docker.build docker.push

docker.test: docker.build docker.run

cargo.install:
	cargo install cargo-watch --locked
	cargo install cargo-udeps --locked

cargo.watch:
	cargo watch -x check

cargo.udeps:
	cargo +nightly udeps
