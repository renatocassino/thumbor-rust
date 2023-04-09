.PHONY: help

##@ Info
help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[30m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[32m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Installation
install: ## Install core dependencies to run the project
	echo "Installing rustup llvm-tools-preview"
	rustup component add llvm-tools-preview
	echo "Installing grcov"
	curl -L https://github.com/mozilla/grcov/releases/latest/download/grcov-x86_64-unknown-linux-gnu.tar.bz2 | tar jxf -
	sudo mv grcov /usr/local/bin/

	echo "Install http-serve to see coverage"
	npm i -g http-serve

##@ Code quality
test: ## Run tests
	cargo test

test-coverage: ## Run tests with coverage
	make clean
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

coverage: ## Serve the coverage html report
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
	http-serve ./target/debug/coverage/

lint: ## Run clippy lint
	cargo clippy

lint-fix: ## Run clippy lint and fix (must be committed)
	cargo clippy --fix

lint-fix-force: ## Run clippy lint forced in staging
	cargo clippy --fix --allow-dirty

##@ Clean
clean: ## Remove useless files
	rm -r cargo-test-*.profraw
	rm -r target/debug/coverage
