//! `rparser` is just an experiment to parse some stuff.
//!
//! [PyPI](https://pypi.python.org/pypi/rparser)

#![feature(proc_macro, specialization)]  // used in pyo3


const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// TODO
// #![deny(missing_docs)]
// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]
// #[macro_use]
// extern crate itertools;

#[cfg(feature = "python")]
extern crate pyo3;
use pyo3::prelude::*;
use pyo3::{PyTuple, PyResult, PyDict, Python};
#[macro_use]
extern crate nom;

pub mod common;
pub mod article;
pub mod html;

#[cfg(feature = "python")]
use self::article::{Article};


///
/// rparser - just for fun
///
#[cfg(feature = "python")]
#[py::modinit(rparser)]
fn init_module(py: Python, m: &PyModule) -> PyResult<()> {
    try!(m.add("__title__", "rparser"));
    try!(m.add("__doc__", "Module documentation string"));
    try!(m.add("__version__", VERSION));
    try!(m.add("__author__", "Sergey Pashinin"));
    try!(m.add("__license__", "GPL 3.0"));
    try!(m.add("__copyright__", "Copyright 2018 Sergey Pashinin"));

    #[pyfn(m, "article_render", args="*", kwargs="**")]
    fn article_render(args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<Py<PyTuple>> {
        let py = args.py();
        let source = args.get_item(0).to_string();
        let mut article = Article::new(py);
        article.src = source.as_bytes();
        article.render();
        if let Some(kwargs) = kwargs {
            article.set_context(kwargs);
        }

        Ok(PyTuple::new(py, &[
            PyString::new(py, &article.html).into_object(py),
            article.get_article_info().into_object(py)
        ]))
    }

    Ok(())
}
