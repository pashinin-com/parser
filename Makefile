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

# test:
# 	cargo test

wheel:
	mkdir -p tmp/wheels
	python3 -m pip wheel . -w tmp/wheels/
# python -m pip wheel . -w tmp/wheels/


# py2-release:
# 	cargo rustc --release --features "python27-sys" -- -C prefer-dynamic

py2:
	python2.7 setup.py build_rust
	python2.7 setup.py build
	(cd rparser; ln -sf ../build/lib.linux-x86_64-2.7/rparser/librparser.so librparser.so)

# .PHONY: build
py3:
	python3.6 setup.py build_rust
	python3.6 setup.py build
	(cd rparser; ln -sf ../build/lib/rparser/librparser.so librparser.so)

install:
	python setup.py build_rust
	python setup.py build
	python setup.py bdist_wheel
	sudo python setup.py install

test:
	python setup.py test
# python setup.py -q nosetests
# py.test tests

bench:
	python tests/bench.py

# py:
# 	python27-sys
# cargo build --release --features "shumway pdf"

# ipython3 -c 'from rparser import Article;a=Article(" 123  \cmd\n \n asd").render()'
