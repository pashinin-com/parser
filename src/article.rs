

use cpython::{Python, PyString, ToPyObject};
use std::borrow::Cow;
// use generator::{Html};
use parser::{parse};
use nom::{IResult};
use std::fmt;


#[derive(PartialEq,Eq,Debug)]
pub struct Article<'a> {
    src: Cow<'a, str>,
}
impl<'a> Article<'a> {
    pub fn new<S>(src: S) -> Article<'a>
        where S: Into<Cow<'a, str>>
    {
        Article {
            src: src.into(),
            // paragraphs: vec![],
            // html: "".into(),
        }
    }

    // fn ast(py: Python, input_str: PyString) -> PyResult<PyTuple> {
    //     match input_str.to_string(py) {
    //         Ok(_) => {
    //             Ok(PyTuple::new(py, &Vec::new()))
    //             // Ok(PyTuple::new(py, &PyString::new(py, "asd")))
    //         }
    //         Err(e) => Err(e)
    //     }
    //     // Ok(PyTuple::new(py, &[input_str]))
    // }

}

impl<'a> fmt::Display for Article<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.src.to_string();
        let parsed = parse(s.as_bytes());
        match parsed {
            IResult::Done(_, node) => {
                // prev:
                // v.iter().fold("".to_string(),
                //               |mut i,j| {i.push_str(&*j.to_string()); i})     // &*j.html()
                write!(f, "{}", node.to_string())
            },
            IResult::Incomplete(x) => write!(f, "incomplete, rest: {:?}!", x),
            IResult::Error(e) => write!(f, "Parsing error: {:?}", e),
        }
    }
}

/// Convert article to a python string (PyString)
impl<'a> ToPyObject for Article<'a>{
    type ObjectType = PyString;
    // type ObjectType = Article<'a>;

    #[inline]
    fn to_py_object(&self, py: Python) -> PyString {
        // PyString::new(py, &self.html())
        PyString::new(py, &format!("{}", &self))
    }
}
