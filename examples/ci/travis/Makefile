#!/usr/bin/make -f

LAMBDA_FUNCTION_NAME:=YOUR_LAMBDA_NAME
CACHE_DIR:=$(HOME)/.cache/cargo
ZIP_FILE:=$(CACHE_DIR)/target/deploy/lambda.zip
# change this to your own container based on lambci/lambda:build-python3.6
DOCKER_IMAGE:=naftulikay/circleci-lambda-rust:latest

WORKDIR:=$(shell pwd)

USER_UID:=$(shell id -u)
USER_GID:=$(shell id -g)

IS_TTY:=$(shell test -t && echo '-t')

DOCKER_INVOKE:=docker run -i $(IS_TTY) --rm \
	-e USER_UID=$(USER_UID) \
	-e USER_GID=$(USER_GID) \
	-v $(WORKDIR):/home/circleci/project \
	-v $(CACHE_DIR)/cargo/registry:/home/circleci/.cargo/registry \
	-v $(CACHE_DIR)/target:/home/circleci/project/target \
	-v $(WORKDIR)/bin:/home/circleci/.local/bin \
	$(DOCKER_IMAGE)

init:
	@mkdir -p target/deploy

pull:
	@docker pull $(DOCKER_IMAGE)

build: init
	@$(DOCKER_INVOKE) .local/bin/build
	@cp $(ZIP_FILE) target/deploy

test: init
	@$(DOCKER_INVOKE) .local/bin/test

clean: init
	@$(DOCKER_INVOKE).local/bin/clean

shell: init
	@$(DOCKER_INVOKE)

deploy: build
	@# update the function
	aws --region us-east-1 lambda update-function-code --function-name $(LAMBDA_FUNCTION_NAME) \
		--zip-file fileb://$(ZIP_FILE)
