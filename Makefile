# Configuration
SHELL := /bin/bash
.DEFAULT_GOAL := help

.PHONY: help run test fmt lint check coverage coverage-text coverage-detailed clean clean-coverage watch

# Cargo settings
CARGO := cargo
CARGO_FLAGS := --quiet
CARGO_TEST_FLAGS :=
CARGO_LLVM_COV_FLAGS := --no-cfg-coverage-nightly

# Coverage settings
COVERAGE_DIR := target/coverage
LLVM_COV_DIR := target/llvm-cov
COVERAGE_ENV := CARGO_INCREMENTAL=0 \
                RUSTFLAGS='-Cinstrument-coverage' \
                LLVM_PROFILE_FILE='$(COVERAGE_DIR)/coverage-%p-%m.profraw'

all: Cargo.toml Cargo.lock $(shell find src -name '*.rs') fmt lint test build-release ## Format, lint, test, and build everything

help: ## Show available commands
	@printf "\033[1;34mUsage:\033[0m\n"
	@printf "  make \033[36m<target>\033[0m\n\n"
	@printf "\033[1;34mAvailable targets:\033[0m\n\n"
	@awk 'BEGIN {FS = ":.*##"; printf ""} \
		/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0,5) } \
		/^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' \
		$(MAKEFILE_LIST)

##@ Development Commands
run: ## Run the project
	@$(CARGO) $(CARGO_FLAGS) run

test: ## Run all tests
	@$(CARGO) $(CARGO_FLAGS) test $(CARGO_TEST_FLAGS)

watch: ## Watch for changes and run tests and clippy
	@$(CARGO) watch \
		-w src \
		-w tests \
		-x "test --features test-utils" \
		-x "clippy --all-targets --all-features -- -D warnings" \
		-c

build: ## Build debug version
	@$(CARGO) $(CARGO_FLAGS) build --all-features

build-release: ## Build optimized release version
	@$(CARGO) $(CARGO_FLAGS) build --all-features --release

##@ Code Quality
fmt: ## Format code
	@$(CARGO) fmt --all

lint: ## Run clippy
	@$(CARGO) $(CARGO_FLAGS) clippy --all-targets --all-features -- -D warnings

check: fmt lint ## Run all checks
	@$(CARGO) $(CARGO_FLAGS) check --all-features

##@ Coverage
coverage: ## Generate code coverage report and open it in browser
	@$(COVERAGE_ENV) $(CARGO) $(CARGO_FLAGS) llvm-cov $(CARGO_LLVM_COV_FLAGS) --html
	@echo "Opening coverage report..."
	@open $(LLVM_COV_DIR)/html/index.html

coverage-text: ## Generate code coverage report in text format
	@$(COVERAGE_ENV) $(CARGO) $(CARGO_FLAGS) llvm-cov $(CARGO_LLVM_COV_FLAGS) --text

##@ Cleanup
clean: ## Clean build artifacts
	@$(CARGO) $(CARGO_FLAGS) clean

clean-coverage: ## Clean coverage artifacts
	@rm -rf $(COVERAGE_DIR) $(LLVM_COV_DIR)

checks: check test coverage ## Run all checks, tests and coverage
