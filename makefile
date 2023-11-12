.PHONY: help

compile-dev: env lint build-dev

compile-prod: env lint build-prod

build-dev:
	cargo build

build-prod:
	cargo build --release

start-dev: compile-dev run-dev

start-prod: compile-prod run-prod

run-dev: 
	cargo run

run-prod:
	./target/release/lottery-number-maker

watch:
	cargo watch -q -c -w src/ -x run

watch-tests:
	cargo watch -q -c -w . -x "test -- --nocapture"

lint:
	cargo clippy --fix --allow-dirty

dcr:
	docker-compose up -d

dcs:
	docker-compose stop

dcd:
	docker-compose down -v

env:
	cp -n .env.sample .env || true

fix-docker-issue:
	echo 'export COMPOSE_DOCKER_CLI_BUILD=0' >> ~/.zshrc
	echo 'export DOCKER_BUILDKIT=0' >> ~/.zshrc
	source ~/.zshrc

image-mac:
	docker build -t lottery-number-maker .

image-amd64:
	docker buildx build --platform linux/amd64 -t lottery-number-maker .