//! lib.rs is the main file which has `py_module_initializer!` macro.
//!
//! It defines an entry point in .SO library with exported python
//! functions.

// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]
// #[macro_use]
// extern crate itertools;

#[cfg(feature = "python")]
#[macro_use]
extern crate cpython;

#[macro_use]
extern crate nom;

pub mod common;
pub mod article;
pub mod markdown;
pub mod html;

use nom::{IResult};
use self::article::node::{Node, NodeClass};
use self::article::parser::{parse};
use std::cell;
use std::borrow::Cow;

#[cfg(feature = "python")]
use cpython::{PyResult, PyString, PyObject, PythonObject, ToPyObject};
#[cfg(feature = "python")]
use self::article::Article;
#[cfg(feature = "python")]
use self::markdown::Markdown;


///
/// Main Python lib init function
///
/// Docs about this macros:
/// http://dgrunwald.github.io/rust-cpython/doc/cpython/macro.py_module_initializer.html
///
/// Arguments:
///
/// 1. name: The module name as a Rust identifier.
/// 2. py2_init: "init" + $name. Necessary because macros can't use concat_idents!().
/// 3. py3_init: "PyInit_" + $name. Necessary because macros can't use concat_idents!().
///
#[cfg(feature = "python")]
py_module_initializer!(librparser, initlibrparser, PyInit_librparser, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add_class::<Article>(py));
    try!(m.add_class::<Markdown>(py));
    Ok(())
});
