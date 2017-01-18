//! `rparser` is just an experiment to parse some stuff.
//!
//! [PyPI](https://pypi.python.org/pypi/rparser)

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
#[macro_use]
extern crate cpython;

#[macro_use]
extern crate nom;

pub mod common;
pub mod article;
pub mod latex;
// pub mod markdown;
pub mod html;

#[cfg(feature = "python")]
use self::article::{article_render};
// #[cfg(feature = "python")]
// use self::markdown::Markdown;


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
    // To add a class named "Article":
    // try!(m.add_class::<Article>(py));
    // try!(m.add(py, "run", py_fn!(py, run(*args, **kwargs))));
    // try!(m.add(py, "article_render", py_fn!(py, article_render(*args, **kwargs))));
    try!(m.add(py, "article_render", py_fn!(py, article_render(*args, **kwargs))));
    // try!(m.add_class::<Markdown>(py));
    Ok(())
});
