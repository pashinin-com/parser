debug:
	cargo build

release:
	cargo build --release

# debug dynamic
dd:
	cargo rustc --debug -- -C prefer-dynamic

ds:
	cargo build

rd:
	cargo rustc --release -- -C prefer-dynamic
