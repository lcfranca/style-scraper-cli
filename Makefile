.PHONY: help build compile run test install clean

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in debug mode
	cargo build

compile: ## Build the project in release mode
	cargo build --release

run: ## Run the CLI tool (e.g. make run ARGS="extract --url https://example.com")
	cargo run -p style-scraper-cli -- $(ARGS)

test: ## Run the test suite
	cargo test

install: ## Compile and install the binary globally to ~/.cargo/bin
	cargo install --path crates/style-scraper-cli --force

clean: ## Clean the target directory and artifacts
	cargo clean
	rm -rf .style-scraper/artifacts