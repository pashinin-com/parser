mod parser;
mod node;
use std::collections::HashMap;
use std::str;
use std::borrow::Cow;
use std::cell;


#[cfg(feature = "python")]
use cpython::{PyResult, PyString, PyObject, PythonObject};

/// Header
// named!(header<Node>,
//        do_parse!(
//            tag!( "##" ) >>
//                opt!(take_while!(space_but_not_eol)) >>
//                txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
//                (Node::new_h2(txt.to_string()))
//        )
// );

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parser<'a> {
    pub src: Cow<'a, str>,
    // off: usize,

    // // opts: Options,
    // active_tab: [u8; 256],

    // // state: State,
    // // stack: Vec<(Tag<'a>, usize, usize)>,
    // leading_space: usize,

    // // containers: Vec<Container>,
    // last_line_was_empty: bool,

    // // state for code fences
    // fence_char: u8,
    // fence_count: usize,
    // fence_indent: usize,

    // info, used in second pass
    // loose_lists: HashSet<usize>,  // offset is at list marker
    // links: HashMap<String, (Cow<'a, str>, Cow<'a, str>)>,
    links: HashMap<String, (Cow<'a, str>, Cow<'a, str>)>,
}


impl<'a> Parser<'a> {
    pub fn new<S>(text: S) -> Parser<'a>
        where S: Into<Cow<'a, str>>
    {
        Parser{
            src: text.into(),
            links: HashMap::new(),
        }
    }

    /// Load a new text as a Markdown source code
    /// TODO: Parse it
    pub fn load<S>(&mut self, text: S)
        where S: Into<Cow<'a, str>>
    {
        self.src = text.into();
    }

    pub fn test(&self){
        println!("asd");
    }

    // pub fn py(&self, py: Python) -> PyObject{
    //     // let s = self.src.into_owned();
    //     // PyString::new(py, &*self.clone().src.into_owned()).into_object()
    //     // PyString::new(py, &s).into_object()
    //     // PyString::new(py, Cow::Borrowed(self.src)).into_object()
    //     //
    // }
}


impl<'a> From<Parser<'a>> for Cow<'a, Parser<'a>>{
    fn from(n: Parser<'a>) -> Cow<'a, Parser<'a>> {
        Cow::Owned(n)
        // Cow::Borrowed(n)
    }
}


// impl<'a> From<PyString> for Cow<'a, Parser<'a>>{
//     fn from(s: PyString) -> Cow<'a, Parser<'a>> {
//         let parser : Parser = Parser::new("asd");
//         Cow::Owned(parser)
//         // Cow::Borrowed(n)
//     }
// }

#[cfg(feature = "python")]
py_class!(class Markdown |py| {
    data parser: cell::RefCell<Parser<'static>>;

    def __new__(_cls, src: PyString) -> PyResult<Markdown> {
        // let ref s = &*src.str(py).unwrap();
        let m = Markdown::create_instance(
            py,
            cell::RefCell::new(Parser::new(src.into_object().extract::<String>(py).unwrap())),
        );
        m
    }
    def load(&self, src: PyString) -> PyResult<PyObject> {
        let mut p = self.parser(py).borrow_mut();
        p.load(src.into_object().extract::<String>(py).unwrap());
        Ok(py.None().into_object())
    }

    def source(&self) -> PyResult<PyObject> {
        Ok(PyString::new(py, &self.parser(py).borrow().src).into_object())
    }

    // def render(&self) -> PyResult<PyString> {
    //     // Ok(PyString::new(py, &*self.tree(py).borrow().to_string()))
    //     Ok(py.None().into_object())
    // }
});
