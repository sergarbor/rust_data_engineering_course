SHELL := /bin/bash
.PHONY: help

help: # print this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: # build the project
	cargo build

clean: # clean the project
	cargo clean

lint: # lint the project
	cargo clippy

format: # format the project
	cargo fmt

version: # print the current version
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"

all: format lint test run