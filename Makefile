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

test:
	cargo test

wheel:
	mkdir -p tmp/wheels
	python -m pip wheel . -w tmp/wheels/

install:
	sudo python3 setup.py install

# ipython3 -c 'import articleparser;articleparser.html(" 123  \cmd\n \n asd")'
