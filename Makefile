.PHONY: contract-check contract-test app-install app-start app-build

contract-check:
	cargo check

contract-test:
	cargo test

app-install:
	cd spreddit-app && npm install

app-start:
	cd spreddit-app && npm start

app-build:
	cd spreddit-app && npm run build
