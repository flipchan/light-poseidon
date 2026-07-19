.PHONY: install build test test-onchain clean publish

install:
	python3 -m venv .venv
	.venv/bin/pip install maturin
	.venv/bin/maturin develop

build:
	.venv/bin/maturin build --release

test: install
	.venv/bin/python examples/basic.py

test-onchain: install
	.venv/bin/pip install requests
	.venv/bin/python examples/onchain_compare.py

clean:
	rm -rf .venv
	rm -rf target
	rm -rf dist

publish: build
	.venv/bin/maturin publish