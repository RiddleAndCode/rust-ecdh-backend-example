# ecdh_backend

**more documentation to come**

## Running with Docker (recommended)

* Generate server keypair (needs to be done once): `make docker-genkey`
* `make docker-run`

## Running with Cargo

* Start a Redis Server (`make docker-db` if you don't have Redis locally installed)
* Edit the config file `config/settings.toml` with the Redis localtion
* Generate server keypair (needs to be done once): `make genkey`
* `make run`
