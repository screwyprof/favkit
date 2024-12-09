# Configuration
SHELL := /bin/bash
.DEFAULT_GOAL := help

# Mark all targets as PHONY
.PHONY: help run test fmt lint check coverage coverage-text coverage-detailed clean clean-coverage all build build-release watch

# Cargo settings
CARGO := cargo
CARGO_TEST_FLAGS := --quiet
CARGO_LLVM_COV_FLAGS := --no-cfg-coverage-nightly

# Coverage settings
COVERAGE_DIR := target/coverage
LLVM_COV_DIR := target/llvm-cov
COVERAGE_ENV := CARGO_INCREMENTAL=0 \
                RUSTFLAGS='-Cinstrument-coverage' \
                LLVM_PROFILE_FILE='$(COVERAGE_DIR)/coverage-%p-%m.profraw'

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
	$(CARGO) run

test: ## Run all tests
	$(CARGO) test $(CARGO_TEST_FLAGS)

watch: ## Watch for changes and run tests and clippy
	$(CARGO) watch \
		-w src \
		-w tests \
		-x "test --features test-utils" \
		-x "clippy --all-targets --all-features -- -D warnings" \
		-c

build: ## Build debug version
	$(CARGO) build --all-features

build-release: ## Build optimized release version
	$(CARGO) build --all-features --release

##@ Code Quality
fmt: ## Check code formatting
	$(CARGO) fmt --all -- --check

lint: ## Run clippy linter
	$(CARGO) clippy --all-features -- -D warnings

check: ## Perform compile checks without producing binaries
	$(CARGO) check --all-features

##@ Coverage
coverage: clean-coverage ## Generate HTML coverage report and open it
	@printf "\033[1;34mGenerating coverage report...\033[0m\n"
	@$(COVERAGE_ENV) $(CARGO) test $(CARGO_TEST_FLAGS)
	@$(CARGO) llvm-cov $(CARGO_LLVM_COV_FLAGS) --html
	@open $(LLVM_COV_DIR)/html/index.html

coverage-text: clean-coverage ## Show brief coverage report in terminal
	@printf "\033[1;34mGenerating coverage report...\033[0m\n"
	@$(COVERAGE_ENV) $(CARGO) test $(CARGO_TEST_FLAGS)
	@$(CARGO) llvm-cov $(CARGO_LLVM_COV_FLAGS) --summary-only

coverage-detailed: clean-coverage ## Show detailed coverage report in terminal
	@printf "\033[1;34mGenerating detailed coverage report...\033[0m\n"
	@$(COVERAGE_ENV) $(CARGO) test $(CARGO_TEST_FLAGS)
	@$(CARGO) llvm-cov $(CARGO_LLVM_COV_FLAGS)

##@ Cleaning
clean: clean-coverage ## Clean all build artifacts
	$(CARGO) clean

clean-coverage: ## Clean coverage data
	@rm -rf $(COVERAGE_DIR) $(LLVM_COV_DIR)
	@mkdir -p $(COVERAGE_DIR) $(LLVM_COV_DIR)

##@ CI/CD
all: fmt lint test build-release ## Run all checks and build release version
