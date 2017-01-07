use std::collections::HashMap;
use std::borrow::Cow;
pub mod parser;
pub mod node;
use self::parser::parse;
use nom::IResult;
use std::fmt;
use self::node::{Node, NodeClass};
use std::cell;

#[cfg(feature = "python")]
use cpython::{Python, ToPyObject, PyObject, PythonObject, PyTuple, PyString, PyResult, PyErr};


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parser<'a> {
    pub src: Cow<'a, str>,
    links: HashMap<String, (Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> Parser<'a> {
    pub fn new<S>(text: S) -> Parser<'a>
        where S: Into<Cow<'a, str>>
    {
        let p = Parser{
            src: text.into(),
            links: HashMap::new(),
        };
        // let res = parse()
        let x = match parse(p.src.as_bytes()) {
            IResult::Done(_, root) => {root}
            _ => Node{
                children: None,
                params: None,
                class: NodeClass::Root,
            }
        };
        p
    }

    /// Load a new text as a Markdown source code
    /// TODO: Parse it
    pub fn load<S>(&mut self, text: S)
        where S: Into<Cow<'a, str>>
    {
        self.src = text.into();
    }
}


#[cfg(feature = "python")]
py_class!(class Article |py| {
    data src: cell::RefCell<PyString>;
    data tree: cell::RefCell<Node>;

    def __new__(_cls, src: PyString) -> PyResult<Article> {
        let n = match parse(src.to_string_lossy(py).as_bytes()) {
            IResult::Done(_, root) => root,
            _ => Node{
                children: None,
                params: None,
                class: NodeClass::Root,
            }
        };
        Article::create_instance(py, cell::RefCell::new(src), cell::RefCell::new(n))
    }
    def load(&self, src: PyString) -> PyResult<PyObject> {
        *self.src(py).borrow_mut() = src;
        *self.tree(py).borrow_mut() =
            match parse(self.src(py).borrow().to_string_lossy(py).clone().as_bytes()) {
                IResult::Done(_, root) => {root}
                _ => Node{
                    children: None,
                    params: None,
                    class: NodeClass::Root,
                }
            };
        Ok(py.None().into_object())
    }

    def source(&self) -> PyResult<PyObject> {
        let ref s = *self.src(py).borrow();
        Ok(PyString::new(py, s.to_string(py).unwrap().as_ref()).into_object())
    }

    def render(&self) -> PyResult<PyString> {
        Ok(PyString::new(py, &*self.tree(py).borrow().to_string()))
    }
});


#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::{Done, Incomplete, Error};
    use std::collections::HashMap;
    use std::str::from_utf8;
    use common::*;
    // use std::str::from_utf8;

    #[test]
    fn parser() {
        let p = Parser::new("[[page 1 | text]]");
        assert_eq!(p.links, HashMap::new());
    }
}
