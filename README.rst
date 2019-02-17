rparser - Python parsers (in Rust)
==================================

.. image:: https://travis-ci.org/pashinin-com/rparser.png?branch=master
    :target: https://travis-ci.org/pashinin-com/rparser

.. code-block:: bash

   pip install rparser


http://rparser.readthedocs.io/en/latest/


Install `rustup`:

.. code-block:: bash

   curl https://sh.rustup.rs -sSf | sh
   rustup default nightly


Enable Python 3.6 by default in Ubuntu:





.. code-block:: bash

   ImportError: libpython3.5m.so.1.0: cannot open shared object file: No
   such file or directory


Added to cargo.toml (maybe I needed to recompile pyo3 after I installed
python 3.6 by default):

.. code-block:: text

   [build-dependencies]
   cbindgen = "0.1"


TODO:

https://github.com/getsentry/milksnake
