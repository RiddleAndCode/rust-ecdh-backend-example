SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.ONESHELL:
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

DOCKER_TAG := ecdh_backend
DOCKER_SENTINAL := target/.docker-$(DOCKER_TAG)

run:
> cargo run
.PHONY: run

watch:
> cargo watch -x run
.PHONY: watch

build:
> cargo build
.PHONY: build

test:
> cargo test
.PHONY: test

clean:
> cargo clean
.PHONY: clean

genkey:
> cargo run --bin genkey
.PHONY: genkey

docker-build: $(DOCKER_SENTINAL)

docker-run: docker-build
> docker-compose up
.PHONY: docker-run

docker-clean:
> -docker image rm $(DOCKER_TAG)
> -rm $(DOCKER_SENTINAL)
.PHONY: docker-clean

docker-db:
> docker-compose run -p 6379:6379 db
.PHONY: docker-db

docker-genkey: docker-build
> docker-compose run server /bin/genkey
.PHONY: docker-genkey

$(DOCKER_SENTINAL):
> mkdir -p $(@D)
> DOCKER_BUILDKIT=1 docker build -f Cargo.toml -t $(DOCKER_TAG) .
> touch $@
