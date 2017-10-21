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
# for musl set -crt-static
# RUSTFLAGS='-C target-feature=-crt-static' python3.6 setup.py build_rust
	CARGO_INCREMENTAL=1 python3.6 setup.py build_rust
	python3.6 setup.py build
	cp -f target/release/librparser.so rparser.so
#	ln -sf build/lib/rparser/rparser.so rparser.so
#	(cd rparser; ln -sf ../build/lib/rparser/librparser.so librparser.so)
# (cd rparser; ln -sf ../target/x86_64-unknown-linux-musl/release/librparser.so librparser.so)

py3.5:
	python3.5 setup.py build_rust
	python3.5 setup.py build
	(cd rparser; ln -sf ../build/lib/rparser/librparser.so librparser.so)

pypy3:
	# pypy3 -m ensurepip --user
	# pypy3 -m pip install --user --upgrade pip
	# pypy3 -m pip install --user setuptools
	pypy3 setup.py build_rust
	pypy3 setup.py build
	(cd rparser; ln -sf ../build/lib/rparser/librparser.so librparser.so)
	pypy3 -c "import rparser"

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
