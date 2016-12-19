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
	python3 -m pip wheel . -w tmp/wheels/
# python -m pip wheel . -w tmp/wheels/


# py2-release:
# 	cargo rustc --release --features "python27-sys" -- -C prefer-dynamic


install:
	sudo python3 setup.py install

# py:
# 	python27-sys
# cargo build --release --features "shumway pdf"

# ipython3 -c 'import articleparser;articleparser.html(" 123  \cmd\n \n asd")'
