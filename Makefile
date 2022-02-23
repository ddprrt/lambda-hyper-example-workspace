.PHONY: lambda-aarch64

lambda-aarch64: 
	cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.17 --bin lambda
	mv ./target/aarch64-unknown-linux-gnu/release/lambda ./bootstrap
	zip lambda-aarch64.zip bootstrap
	rm bootstrap


lambda-x86_64: 
	cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17 --bin lambda
	mv ./target/aarch64-unknown-linux-gnu/release/lambda ./bootstrap
	zip lambda-aarch64.zip bootstrap
	rm bootstrap