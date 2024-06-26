REGISTRY ?= quay.io
REPOSITORY ?= duk4s
NAME ?= httpdumper
VERSION ?= $(shell awk -F'"' '/version =/{print $$2; exit}' Cargo.toml)
IMAGE = $(REGISTRY)/$(REPOSITORY)/$(NAME)

.PHONY: build-bin
build-bin:
	cargo build --release --target x86_64-unknown-linux-musl

.PHONY: build-image
build-image:
	DOCKER_BUILDKIT=1 docker image build --no-cache \
		-t ${NAME}:${VERSION} \
		.

.PHONY: push-image
push-image:
	docker image tag ${NAME}:${VERSION} $(IMAGE):${VERSION}; \
	docker push $(IMAGE):$(VERSION)
