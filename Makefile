# Configuration
SHELL := /bin/bash
.DEFAULT_GOAL := help

# Declare all phony targets (all and clean first as required)
.PHONY: all clean test \
        help run test-debug test-failures test-live \
        fmt lint \
        coverage coverage-text coverage-summary coverage-lcov coverage-html \
        watch build build-release build-nix

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

##@ Main Commands
help: ## Show available commands
	@awk 'BEGIN {FS = ":.*##"; printf "\033[1;34mUsage:\033[0m\n  make \033[36m<target>\033[0m\n\n\033[1;34mAvailable targets:\033[0m\n"} \
		/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0,5) } \
		/^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' \
		$(MAKEFILE_LIST)

all: fmt lint test build-release ## Format, lint, test, and build everything

##@ Development
run: ## Run the project
	@$(CARGO) $(CARGO_FLAGS) run

watch: ## Watch for changes and run tests and clippy
	@bacon

##@ Testing
test: ## Run all tests
	@cargo $(CARGO_FLAGS) nextest run

test-debug: ## Show paths to test binaries for debugging
	@$(CARGO) $(CARGO_FLAGS) test --no-run --message-format=json | jq -r 'select(.profile.test == true) | .executable'

test-failures: ## Run tests and show only failures
	@cargo $(CARGO_FLAGS) nextest run --status-level=fail --failure-output immediate --success-output never

test-live: ## Run tests with live output (no capture)
	@cargo $(CARGO_FLAGS) nextest run --no-capture

##@ Building
build: ## Build debug version
	@$(CARGO) $(CARGO_FLAGS) build --all-features

build-release: ## Build optimized release version
	@$(CARGO) $(CARGO_FLAGS) build --all-features --release

build-nix: ## Build using Nix (with caching)
	@nix build

##@ Code Quality
fmt: ## Format code
	@$(CARGO) fmt --all

lint: ## Run clippy and security audit
	@$(CARGO) $(CARGO_FLAGS) clippy --all-targets --all-features -- -D warnings
	@$(CARGO) $(CARGO_FLAGS) audit

##@ Coverage
coverage: ## Generate code coverage report and open it in browser
	@cargo $(CARGO_FLAGS) llvm-cov nextest $(LLVM_COV_FLAGS) --html --open

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
