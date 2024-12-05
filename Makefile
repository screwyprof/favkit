.PHONY: help test coverage coverage-text clean

# Variables for common settings
CARGO_TEST_FLAGS := --quiet
COVERAGE_ENV := CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/coverage/coverage-%p-%m.profraw'
GRCOV_FLAGS := --binary-path ./target/debug/ -s . --branch --ignore-not-existing --ignore "/*" --ignore "tests/*" --ignore "src/main.rs"

# Default target
all: test

## Show this help message
help:
	@echo 'Usage:'
	@echo '  make <target>'
	@echo
	@echo 'Targets:'
	@awk '/^[a-zA-Z0-9-]+:/ { \
		helpMessage = match(lastLine, /^## (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
			printf "  %-20s %s\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

## Run all tests
test:
	@cargo test $(CARGO_TEST_FLAGS)

## Run tests and generate HTML coverage report
coverage: clean-coverage
	@$(COVERAGE_ENV) cargo test $(CARGO_TEST_FLAGS)
	@grcov . $(GRCOV_FLAGS) -t html -o target/coverage/html > /dev/null 2>&1
	@open target/coverage/html/index.html

## Run tests and show coverage report in terminal
coverage-text: clean-coverage
	@$(COVERAGE_ENV) cargo test $(CARGO_TEST_FLAGS)
	@grcov . $(GRCOV_FLAGS) -t markdown -o target/coverage/report.md > /dev/null 2>&1
	@echo "Coverage Report:"
	@echo
	@cat target/coverage/report.md | column -t -s'|'
	@echo

## Clean build artifacts and coverage data
clean: clean-coverage
	@cargo clean

clean-coverage:
	@rm -rf target/coverage
	@mkdir -p target/coverage
