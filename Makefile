SHELL := /bin/bash

.PHONY: help ssh-tools

help:
	@echo "Makefile targets:"
	@echo "  make ssh-tools    # set executable bits for scripts and show next steps"

ssh-tools:
	@./scripts/install-ssh-tools
