# Configuration
SHELL := /bin/bash
.DEFAULT_GOAL := help

.PHONY: help run test test-debug fmt lint check coverage coverage-text coverage-summary coverage-lcov coverage-html clean watch build build-release nix-build checks all

# Cargo settings
CARGO := cargo
CARGO_FLAGS := --quiet
LLVM_COV_FLAGS := --all-features --workspace --show-missing-lines \
                  --ignore-filename-regex=".cargo|test.rs" \
				  --branch

# Coverage settings
COVERAGE_DIR := target/coverage

# Environment variables for coverage
export CARGO_INCREMENTAL=0
export RUSTFLAGS=-C instrument-coverage -C codegen-units=1 -C opt-level=0 -C link-dead-code --cfg coverage_nightly
export LLVM_PROFILE_FILE=$(COVERAGE_DIR)/coverage-%p-%m.profraw

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
	@cargo nextest run

test-debug: ## Show paths to test binaries for debugging
	@$(CARGO) test --no-run --message-format=json | jq -r 'select(.profile.test == true) | .executable'

watch: ## Watch for changes and run tests and clippy
	@$(CARGO) watch \
		-w src \
		-w tests \
		-x "nextest run --features test-utils" \
		-x "clippy --all-targets --all-features -- -D warnings" \
		-c

build: ## Build debug version
	@$(CARGO) $(CARGO_FLAGS) build --all-features

build-release: ## Build optimized release version
	@$(CARGO) $(CARGO_FLAGS) build --all-features --release

nix-build: ## Build using Nix (with caching)
	@nix build

##@ Code Quality
fmt: ## Format code
	@$(CARGO) fmt --all

lint: ## Run clippy
	@$(CARGO) $(CARGO_FLAGS) clippy --all-targets --all-features -- -D warnings

check: fmt lint ## Run all checks
	@$(CARGO) $(CARGO_FLAGS) check --all-features

##@ Coverage
coverage: ## Generate code coverage report and open it in browser
	@cargo $(CARGO_FLAGS) llvm-cov nextest $(LLVM_COV_FLAGS) --html
	@open target/llvm-cov/html/index.html

coverage-text: ## Generate code coverage report in text format
	@cargo $(CARGO_FLAGS) llvm-cov nextest $(LLVM_COV_FLAGS)

coverage-summary: ## Generate code coverage summary
	@cargo $(CARGO_FLAGS) llvm-cov nextest $(LLVM_COV_FLAGS) --summary-only

coverage-lcov: ## Generate code coverage report in lcov format
	@mkdir -p $(COVERAGE_DIR)
	@cargo $(CARGO_FLAGS) llvm-cov nextest $(LLVM_COV_FLAGS) --lcov \
			| rustfilt | sed '/^Uncovered/,$$d' > $(COVERAGE_DIR)/lcov.info

coverage-html: coverage-lcov ## Generate detailed HTML coverage report with all metrics
	@genhtml $(COVERAGE_DIR)/lcov.info \
		--output-directory $(COVERAGE_DIR)/html \
		--prefix $(shell pwd) \
		--title "FavKit Coverage Report" \
		--legend \
		--show-details \
		--missed \
		--dark-mode \
		--sort \
		--branch-coverage
	@echo "Opening coverage report..."
	@open $(COVERAGE_DIR)/html/index.html

##@ Cleanup
clean: ## Clean build artifacts
	@$(CARGO) $(CARGO_FLAGS) clean
	@rm -rf $(COVERAGE_DIR)

checks: check test coverage ## Run all checks, tests and coverage
