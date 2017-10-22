//! ```
//! let mut p = Article::from("[[page 1 | text]]");
//! ```

// #![cfg(feature = "stream")]

use std::fmt;
use std::fmt::{Display, Formatter};
use std::borrow::Cow;
use std::collections::HashSet;
use std::collections::HashMap;
use nom::{eol, IResult};
// use nom::not_line_ending;
use std::str::from_utf8;
use std::string::String;
use common::*;
use html;
use html::tag;
use std::convert::From;
use nom::{Consumer,ConsumerState,Move,Input,Producer,MemProducer, hex_digit, alphanumeric};
use nom::Offset;

#[cfg(feature = "python")]
use pyo3::{
    Py,
    Python, PyObject, ToPyObject, PyTuple, PyString,
    // PyResult,
    PyDict,
    // ObjectProtocol,
    IntoPyObject,
    // PyObjectRef,
    // FromPyObject
};
use pyo3::prelude::*;
// use pyo3::ToPyPointer;
// use pyo3::IntoPyDictPointer;
use pyo3::PyTryInto;
// use std::io::Beginning

// const youtube_format_str: &'static str = r#"<div style="position:relative;padding-bottom: 56.25%; /* 16:9 */ padding-top: 25px;height: 0;"><iframe style="position: absolute;top: 0;left: 0;width: 100%;height: 100%;border:0px;" src="https://www.youtube.com/embed/{}"></iframe></div>"#;
// fn format_youtube(&'str code) -> String {
//     format!(, code)
// }

const ALLOWED_HTML_TAGS: &'static [&'static str] = &[
    "b",
    "div",
    "hr",
    "p",
    "table",
    "td",
    "tr",
];

#[derive(PartialEq,Eq,Debug)]
enum State {
    Beginning,
    // Middle,
    // End,
    Done,
    Error
}

struct TestConsumer<'a> {
    tags:    Vec<Tag<'a>>,
    state:   State,
    c_state: ConsumerState<usize,(),Move>,
    counter: usize,
    line: usize,
    col: usize
}


#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tag<'a> {
    Container {c: Vec<Tag<'a>>},
    Paragraph {c: Vec<Tag<'a>>},
    ListUnnumbered{c: Vec<Tag<'a>>},
    // ListUnnumbered{c: Vec<Vec<Tag<'a>>>},  // vector of list items, each item is a vector of Tags
    ListUnnumberedItem(
        // Cow<'a, str>
        Vec<Tag<'a>>  // contents of list item
    ),
    ListNumbered(Vec<Tag<'a>>),
    ListItemNumbered(Vec<Tag<'a>>),
    Header(usize, Cow<'a, str>),

    CodeTabs(Vec<Tag<'a>>),
    Code(
        Cow<'a, str>,   // language
        Cow<'a, str>    // source code
    ),
    URL{
        proto: Cow<'a, str>,
        hostname: Cow<'a, str>,
        path: Cow<'a, str>,
        query: Cow<'a, str>,
    },

    // Latex command
    Command{
        name: Cow<'a, str>,
        contents: Cow<'a, str>,
    },

    Text(Cow<'a, str>),
    HTMLTag(html::HTMLTag<'a>),
    MathInline(Cow<'a, str>),
    MathWholeLine(Cow<'a, str>),
    Comment,
    LinkInternal{page: Cow<'a, str>, text: Cow<'a, str>, link: Option<Cow<'a, str>>},

    /// `[http://pashinin.com Title]`
    ///
    /// Which will render as: [Title](http://pashinin.com)
    LinkExternal{
        url: Cow<'a, str>,
        text: Cow<'a, str>,
    },

    Space,
    // Cut
}

impl<'a> TestConsumer<'a> {

}


impl<'a> Consumer<&'a[u8], usize, (), Move> for TestConsumer<'a> {
    fn state(&self) -> &ConsumerState<usize,(),Move> {
        &self.c_state
    }

    fn handle(&mut self, input: Input<&'a [u8]>) -> &ConsumerState<usize,(),Move> {
        // println!("input: {:?}", input);
        match self.state {
            State::Beginning => {
                match input {
                    // if there is no more data
                    Input::Empty | Input::Eof(None) => {
                        self.state   = State::Error;
                        self.c_state = ConsumerState::Error(());
                    },
                    Input::Element(sl) | Input::Eof(Some(sl)) => {
                        let block;
                        if self.col == 0{
                            block = line_start_element(sl);
                        }else{
                            block = element(sl);
                        }

                        match block {
                            IResult::Error(_)      => {
                                // self.state   = State::Error;
                                // self.c_state = ConsumerState::Error(());
                                self.state   = State::Done;
                                self.c_state = ConsumerState::Done(Move::Consume(0), self.counter);
                            },
                            IResult::Incomplete(_n) => {
                                // println!("Middle got Incomplete({:?})", n);
                                // self.c_state = ConsumerState::Continue(Move::Await(n));
                                self.state   = State::Done;
                                self.c_state = ConsumerState::Done(Move::Consume(0), self.counter);
                            },
                            IResult::Done(i, tag)     => {
                                // println!("Got {:?}", tag);
                                // println!("moving: {:?}", sl.offset(i));
                                // println!("parsed: {:?}", &sl[0..sl.offset(i)]);
                                if tag != Tag::Space{
                                    self.tags.push(tag);
                                }

                                // println!("EOLS: {:?}", &sl[0..sl.offset(i)]);
                                // println!("EOLS: {:?}", count_eols(&sl[0..sl.offset(i)]));

                                // Count EOLs (end-of-lines) in a parsed block,
                                // detect current line position (column)
                                match count_eols(&sl[0..sl.offset(i)]) {
                                    IResult::Error(_)      => {},
                                    IResult::Incomplete(_) => {},
                                    IResult::Done(rest, eols) => {
                                        self.line += eols;
                                        if eols > 0 {
                                            self.col = rest.len()
                                        } else {
                                            self.col = rest.len() + sl.offset(i)
                                        }
                                    },
                                }

                                // self.state = State::Middle;
                                self.c_state = ConsumerState::Continue(Move::Consume(sl.offset(i)));
                            }
                        }
                    }
                }
            },
            // State::Middle    => {
            //     match input {
            //         // if there is no more data
            //         Input::Empty | Input::Eof(None) => {
            //             self.state   = State::Error;
            //             self.c_state = ConsumerState::Error(());
            //         },
            //         Input::Element(sl) | Input::Eof(Some(sl)) => {
            //             match root_element(sl) {
            //                 IResult::Error(_)      => {
            //                     // println!("Middle error, {:?} tags now", self.tags.len());
            //                     // self.state   = State::End;
            //                     // self.c_state = ConsumerState::Continue(Move::Consume(0));

            //                     self.state   = State::Done;
            //                     self.c_state = ConsumerState::Done(Move::Consume(0), self.counter);
            //                 },
            //                 IResult::Incomplete(n) => {
            //                     println!("Middle got Incomplete({:?})", n);
            //                     self.c_state = ConsumerState::Continue(Move::Await(n));

            //                     // self.state = State::Done;
            //                     // self.c_state = ConsumerState::Done(Move::Consume(0), ());

            //                     // self.state   = State::Done;
            //                     // self.c_state = ConsumerState::Done(Move::Consume(sl.offset(i)), self.counter);
            //                 },
            //                 IResult::Done(i, tag)     => {
            //                     println!("Got {:?}", tag);
            //                     self.tags.push(tag);
            //                     println!("EOLS: {:?}", &sl[0..sl.offset(i)]);
            //                     println!("EOLS: {:?}", count_eols(&sl[0..sl.offset(i)]));
            //                     // self.counter = self.counter + noms_vec.len();
            //                     self.state = State::Middle;
            //                     self.c_state = ConsumerState::Continue(Move::Consume(sl.offset(i)));
            //                 }
            //             }
            //         }
            //     }
            // },
            // State::End       => {
            //     match input {
            //         // if there is no more data
            //         Input::Empty | Input::Eof(None) => {
            //             self.state   = State::Error;
            //             self.c_state = ConsumerState::Error(());
            //         },
            //         Input::Element(sl) | Input::Eof(Some(sl)) => {
            //             match root_element(sl) {
            //                 IResult::Error(_)      => {
            //                     self.state   = State::Error;
            //                     self.c_state = ConsumerState::Error(());
            //                 },
            //                 IResult::Incomplete(n) => {
            //                     self.c_state = ConsumerState::Continue(Move::Await(n));
            //                 },
            //                 IResult::Done(i, tag)     => {
            //                     self.tags.push(tag);
            //                     self.state = State::Done;
            //                     self.c_state = ConsumerState::Done(Move::Consume(sl.offset(i)), self.counter);
            //                 }
            //             }
            //         }
            //     }
            // },
            State::Done | State::Error     => {
                // this should not be called
                self.state = State::Error;
                self.c_state = ConsumerState::Error(())
            }
        };
        &self.c_state
    }
}



// impl<'a> Display for N<Tag<'a>> {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         match self.data {
//             // Tag::Root => write!(f, "Root {}", self.),
//             ref d => write!(f, "{}", d),
//         }
//     }
// }


impl<'a> Display for Tag<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Tag::Container{..} => write!(f, "{}", "<Root>"),
            Tag::Paragraph{..} => write!(f, "{}", "<Paragraph>"),
            Tag::LinkExternal{ref url, ..} => write!(f, "{}", url),
            Tag::URL{ref proto, ref hostname, ref path, ref query} =>
                write!(f, "{}://{}{}{}", proto, hostname, path, query),
            // Tag::Text(ref txt) => write!(f, txt),
            _ => write!(f, "{}", "unknown node"),
        }
        // write!(f, "{}", "unknown node")
    }
}



// #[derive(PartialEq,Eq,Debug,Clone)]
// pub struct Node<'a> {
//     pub class: Tag<'a>,
//     pub params: Option<HashMap<&'a str, Cow<'a, str>>>,
//     // pub params: Option<HashMap<&'a str, &'a str>>,
//     // Into<Cow<'a, str>>
//     // pub parent: Option<Box<Node<'a>>>,
//     pub parent: Option<Rc<Node<'a>>>,
//     pub children: Option<Vec<Node<'a>>>,
// }


/// Build AST in Python
/// Convert Tag to a python object (PyTuple, PyDict)
#[cfg(feature = "python")]
impl<'a> ToPyObject for Tag<'a> {
    // type ObjectType = PyObject;

    #[inline]
    fn to_object(&self, py: Python) -> PyObject {
        match *self {
            // Tag::Root{c} => {
            //     match self.children {
            //         Some(ref nodes) => {
            //             let children: Vec<PyObject> = nodes.iter()
            //                 .map(|&ref x| x.to_py_object(py))
            //                 .collect();
            //             PyTuple::new(py, &children.as_slice()).into_object()
            //         }
            //         None => PyTuple::new(py, &vec![]).into_object()
            //     }
            // },
            // Tag::Paragraph(_) => {
            //     let d = PyDict::new(py);
            //     d.set_item(py, "type", "paragraph");
            //     d.set_item(
            //         py,
            //         "items",
            //         match self.children {
            //             Some(ref nodes) => {
            //                 let children: Vec<PyObject> = nodes.iter()
            //                     .map(|&ref x| x.to_py_object(py))
            //                     .collect();
            //                 PyTuple::new(py, &children.as_slice()).into_object()
            //             }
            //             None => PyTuple::new(py, &vec![]).into_object()
            //         }
            //     );
            //     d.into_object()
            // },
            // Tag::ListUnnumbered => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
            // Tag::ListUnnumberedItem => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
            // Tag::Header => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
            // Tag::Code => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
            // Tag::URL => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
        // Tag::Comment => {
        //         // PyTuple::new(py, &vec![]).into_object()
        //         let d = PyDict::new(py);
        //         d.set_item(py, "type", "comment").unwrap();
        //         d.set_item(py, "text", match self.params {
        //             Some(ref x) => {
        //                 x.get("txt").unwrap()
        //             }
        //             _ => ""
        //         }).unwrap();
        //         d.into_object()
        //     },
            // Tag::Text => {
            //     let d = PyDict::new(py);
            //     d.set_item(py, "type", "text");
            //     d.set_item(py, "text", match self.params {
            //         Some(ref x) => {
            //             x.get("txt").unwrap()
            //         }
            //         _ => ""
            //     });
            //     d.into_object()
            // },
            // Tag::LinkInternal => {
            //     let d = PyDict::new(py);
            //     d.set_item(py, "type", "link_internal");
            //     d.set_item(py, "url", match self.params {
            //         Some(ref x) => x.get("url").unwrap(),
            //         _ => ""
            //     });
            //     d.set_item(py, "text", match self.params {
            //         Some(ref x) => x.get("text").unwrap(),
            //         _ => ""
            //     });
            //     d.into_object()
            // }
            _ => PyDict::new(py).into_object(py)
        }
    }
}


// impl<'a> IntoIterator for &'a Node<'a> {
//     type Item = &'a Node<'a>;
//     type IntoIter = PixelIntoIterator<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         PixelIntoIterator { node: self, root: true, todo:vec![] }
//     }
// }
// /// Current iterator
// pub struct PixelIntoIterator<'a> {
//     node: &'a Node<'a>,
//     // parent: Option<&'a Node<'a>>,
//     root: bool,
//     // level: usize,
//     // index: usize,
//     todo: Vec<&'a Node<'a>>,
// }
// impl<'a> Iterator for PixelIntoIterator<'a> {
//     type Item = &'a Node<'a>;
//     fn next(&mut self) -> Option<&'a Node<'a>> {

//         if self.root {
//             self.root = false;
//             return Some(self.node)
//         }

//         // if node has children - push them in "todo" and return node
//         if let Some(ref x) = self.node.children {
//             // add all children
//             for item in x {
//                 self.todo.push(&item);
//             }
//             if let Some(top) = self.todo.pop() {
//                 self.node = top;
//                 return Some(self.node)
//             }
//         }

//         while let Some(top) = self.todo.pop() {
//             self.node = top;
//             return Some(self.node);
//         }
//         None
//     }
// }


// Example of implementing Cow:
// https://github.com/tbu-/rust/commit/7a37b00045f44c637cda1617fbb06f2c62808cad
// impl<'a> From<Node<'a>> for Cow<'a, Node<'a>>{
//     // fn from(n: T) -> Cow<'a, [T]> {
//     #[inline]
//     fn from(n: Node<'a>) -> Cow<'a, Node<'a>> {
//         Cow::Owned(n)
//     }
// }

// impl<'a> From<&'a Node<'a>> for Cow<'a, Node<'a>>{
//     fn from(n: &'a Node<'a>) -> Cow<'a, Node<'a>> {
//         Cow:Borrowed(n)
//     }
// }


// pub trait ToOwned {
//     type Owned: Borrow<Self>;
//     fn to_owned(&self) -> Self::Owned;
// }

// impl<'a> ToOwned for Node<'a> {
//     // type Owned: Borrow<Self>;
//     // type ObjectType = PyObject;
//     type Owned = Borrow<Self>;

//     fn to_owned(&self) -> Self::Owned{

//     }
// }



/// Render article to HTML
#[cfg(feature = "python")]
pub fn article_render<'a>(args: &PyTuple, kwargs: Option<&PyDict>) -> Py<PyTuple> {
    let py = args.py();
    let source = args.get_item(0).to_string();

    // let mut article = Article::from(source.as_bytes());
    let mut article = Article::new(py);
    article.src = source.as_bytes();
    article.render();
    if let Some(kwargs) = kwargs {
        article.set_info(kwargs);
    }


    PyTuple::new(py, &[
        PyString::new(py, &article.html).into_object(py),
        article.py_info(py).into_object(py)
    ])
}

pub struct File<'a> {
    pub sha1: Cow<'a, str>,
    pub contenttype: Cow<'a, str>,
}

/// Article parser
pub struct Article<'a> {
    pub src: &'a [u8],
    // root: Tag<'a>,
    pub html: Cow<'a, str>,

    py: Python<'a>,

    //
    pub links_internal: HashMap<Cow<'a, str>, Cow<'a, str>>,
    // pub links_internal: HashMap<&'a PyString, &'a PyString>,

    // #[cfg(feature = "python")]
    // pub context: HashMap<Cow<'a, str>, ToPyObject + 'a>,
    // pub context: HashMap<Cow<'a, str>, PyObject>,
    pub context: &'a PyDict,

    pub files: HashMap<Cow<'a, str>, File<'a>>,
    pub files_missing: HashSet<String>,
    pub files_used: HashSet<String>,
    links_internal_missing: HashSet<String>,
}

impl<'a> Article<'a> {
    /// Get some information about current article.
    /// Return a PyDict like:
    /// {
    ///   "missing_links": ("page name 1", "page name 2", ...)
    /// }
    ///
    /// What information is returned:
    ///
    pub fn new(py: Python<'a>) -> Self {
        let a = Article {
            src: "".as_bytes(),
            html: Cow::from(""),
            py: py,
            links_internal: HashMap::new(),
            // context: HashMap::new(),
            context: PyDict::new(py),
            files: HashMap::new(),
            files_missing: HashSet::new(),
            files_used: HashSet::new(),
            links_internal_missing: HashSet::new(),
        };
        // a.context.set_item("os", a.py.import("os").unwrap()).unwrap();
        a.context.set_item("datetime", a.py.import("datetime").unwrap()).unwrap();
        a
    }

    #[cfg(feature = "python")]
    pub fn py_info(&self, py: Python) -> PyObject {
    // pub fn py_info(&self, py: Python) -> PyObjectRef {
        let info = PyDict::new(py);

        if !self.links_internal_missing.is_empty() {
            let v: Vec<PyObject> = self.links_internal_missing.iter()
                .map(|x| PyString::new(py, &x).into_object(py))
                .collect();
            let missing_links = PyTuple::new(py, v.as_slice());
            info.set_item("missing_links", missing_links).unwrap();
        }

        if !self.files_missing.is_empty() {
            let v: Vec<PyObject> = self.files_missing.iter()
                .map(|x| PyString::new(py, &x).into_object(py))
                .collect();
            let files_missing = PyTuple::new(py, v.as_slice());
            info.set_item("files_missing", files_missing).unwrap();
        }

        if !self.files_used.is_empty() {
            let v: Vec<PyObject> = self.files_used.iter()
                .map(|x| PyString::new(py, &x).into_object(py))
                .collect();
            let files_used = PyTuple::new(py, v.as_slice());
            info.set_item("files_used", files_used).unwrap();
        }
        // info
        // let mut map = HashMap::new();
        // map.insert(Cow::from("page 1 "), Cow::from(" text"));
        info.into_object(py)
    }

    /// Set some internal variables for rendering from PyDict
    /// Used as kwargs.
    ///
    /// article_parse("src", set_links={"page name 1": "/articles/1"})
    #[cfg(feature = "python")]
    pub fn set_info(&mut self, dict: &PyDict) {
        self.files_missing = HashSet::new();
        self.files_used = HashSet::new();
        self.links_internal_missing = HashSet::new();

        // set internal links
        if let Some(set_links) = dict.get_item("set_links") {  // PyObjectRef
            // let links = set_links.extract::<PyDict>().unwrap();
            let links: &PyDict = set_links.try_into_exact().unwrap();
            // let links = set_links.to_object(py).extract::<PyDict>(py).unwrap();
            // let links = links.extract::<PyDict>().unwrap();
            // let links = set_links.extract().unwrap();
            // let links = set_links;
            for (page, _url) in links.iter() {
                // let p: String = page.extract::<PyString>(py).unwrap().to_string(py).unwrap().into_owned();
                // let u: String = url.extract::<PyString>(py).unwrap().to_string(py).unwrap().into_owned();
                // self.links_internal.insert(Cow::from(p), Cow::from(u));
                let p: &PyString = page.try_into_exact().unwrap();
                let key: String = p.to_string().unwrap().into_owned();
                let value: String = p.to_string().unwrap().into_owned();
                self.links_internal.insert(Cow::from(key), Cow::from(value));
            }
        }
        // if let Some(files_obj) = dict.get_item("files") {
        //     // let files = files_obj.extract::<PyDict>().unwrap();
        //     let files = files_obj.extract().unwrap();
        //     for (k, _) in files.items() {
        //         let hash: String = k.extract::<PyString>(py).unwrap().to_string(py).unwrap().into_owned();
        //         // let u: String = url.extract::<PyString>(py).unwrap().to_string(py).unwrap().into_owned();
        //         self.files.insert(Cow::from(hash.clone()), File{
        //             sha1: Cow::from(hash),
        //             contenttype: Cow::from(""),
        //         });
        //     }
        // }
        self.render();
    }

    // fn add_context_variable(&mut self, py: Python) {
        // let p: String = page.extract::<PyString>(py).unwrap().to_string(py).unwrap().into_owned();
        // let u: String = url.extract::<PyString>(py).unwrap().to_string(py).unwrap().into_owned();
        // let locals = PyDict::new(py);
        // let user: String = py.eval("os.getenv('USER') or os.getenv('USERNAME')", None, Some(&locals))?.extract(py)?;
        // let user: String = py.eval("os.getenv('USER')", None, Some(&locals))?.extract(py)?;

        // self.context.insert(Cow::from("asd"), PyString::new(py, user).into_object());
    // }

    pub fn render(&mut self) {
        let mut p = MemProducer::new(&self.src, self.src.len());
        let mut c = TestConsumer{
            state: State::Beginning,
            counter: 0,
            c_state: ConsumerState::Continue(Move::Consume(0)),
            tags: vec![],
            line: 1,
            col: 0
        };
        while let &ConsumerState::Continue(Move::Consume(_)) = p.apply(&mut c) {
            // println!("move: {:?}", mv);
        }
        // println!("last consumer state: {:?} | last state: {:?}", c.c_state, c.state);
        // println!("Total lines: {:?}", c.line);
        // println!("Current column: {:?}", c.col);

        // if let ConsumerState::Done(Move::Consume(0), 0) = c.c_state {
        //     println!("consumer state ok");
        // } else {
        //     assert!(false, "consumer should have reached Done state");
        // }

        self.html = Cow::from(format!("{}", c.tags.to_html(self)));
        // self.html = match parse(self.src) {
        //     IResult::Done(_, parsed_data) => Cow::from(format!("{}", parsed_data.to_html(self))),
        //     _ => Cow::from("")
        // };
    }
}


// impl<'a> From<&'a str> for Article<'a> {
//     fn from(src: &'a str) -> Article<'a> {
//         let gil = Python::acquire_gil();
//         let py = gil.python();
//         let mut a = Article {
//             src: src.as_bytes(),
//             html: Cow::from(""),
//             py: py,
//             links_internal: HashMap::new(),
//             context: HashMap::new(),
//             files: HashMap::new(),
//             files_missing: HashSet::new(),
//             files_used: HashSet::new(),
//             links_internal_missing: HashSet::new(),
//         };
//         // a.render();
//         a
//     }
// }
// impl<'a> From<&'a [u8]> for Article<'a> {
//     fn from(src: &'a [u8]) -> Article<'a> {
//         let mut a = Article {
//             src: src,
//             html: Cow::from(""),
//             py: None,
//             files: HashMap::new(),
//             context: HashMap::new(),
//             files_missing: HashSet::new(),
//             links_internal: HashMap::new(),
//             files_used: HashSet::new(),
//             links_internal_missing: HashSet::new(),
//         };
//         a.render();
//         a
//     }
// }


pub trait ToHtml {
    fn to_html(&self, tree: &mut Article) -> Cow<str>;
}

impl<'a> ToHtml for Vec<Tag<'a>> {
    fn to_html(&self, article: &mut Article) -> Cow<str>
    {
        let parts: Vec<Cow<str>> = self.iter()
            .map(|&ref x| x.to_html(article))
            .collect();
        Cow::from(parts.join(""))
    }
}

impl<'a> ToHtml for Tag<'a> {
    fn to_html(&self, article: &mut Article) -> Cow<str>
    {
        match *self {
            Tag::Container{ref c} => c.to_html(article),
            // {
            //     let children_strings: Vec<Cow<str>> = c.iter()
            //         .map(|&ref x| x.to_html(parser))
            //         .collect();
            //     let s = children_strings.join("");
            //     Cow::from(s)
            // }

            Tag::Command{ref name, ref contents} => {
                match &*name.to_string() {
                    "youtube" => {
                        // <div style="position: relative;padding-bottom: 56.25%; /* 16:9 */ padding-top: 25px;height: 0;"><iframe style="position: absolute;top: 0;left: 0;width: 100%;height: 100%;border:0px;" src="https://www.youtube.com/embed/HL-75xTzn6A"></iframe></div>
                        Cow::from(format!(r#"<div style="position: relative;padding-bottom: 56.25%; /* 16:9 */ height: 0;"><iframe style="position: absolute;top: 0;left: 0;width: 100%;height: 100%;border:0px;" src="https://www.youtube.com/embed/{}"></iframe></div>"#, contents))
                        // Cow::from(format!(youtube_format_str, contents))
                    }
                    "file" => {
                        let sha1 = contents.trim();
                        match article.files.get(sha1) {
                            Some(f) => {
                                // Cow::from(format!("<a href=\"{}\">{}</a>", l, text))
                                article.files_used.insert(String::from(sha1));

                                match &*f.contenttype.to_string() {
                                    "image" => {
                                        // Cow::from(format!("<a href=\"{}\">{}</a>", l, text))
                                        article.files_used.insert(String::from(sha1));
                                        Cow::from(format!("<img src=\"{}\"/>", sha1))
                                    },
                                    _ => {
                                        // parser.files_missing.insert(String::from(sha1));
                                        // Cow::from(format!("file-{} (unknown type - download)", sha1))
                                        // parser.links_internal_missing.insert(String::from(page));
                                        Cow::from(format!("<a class=\"download unknowntype\" href=\"{}\">Download</a>", f.contenttype))
                                    }
                                }
                                // Cow::from(format!("file-{}", sha1))
                            },
                            None => {
                                article.files_used.insert(String::from(sha1));
                                article.files_missing.insert(String::from(sha1));
                                // Cow::from(format!("no-file-{}", sha1))
                                Cow::from(format!(""))
                                // parser.links_internal_missing.insert(String::from(page));
                                // Cow::from(format!("<a class=\"redlink\" href=\"/articles/{}\">{}</a>", page, text))
                            }
                        }
                    },
                    "expression" => {  // Jinja expression:  {{ var1 + var2 }}
                        // <div style="position: relative;padding-bottom: 56.25%; /* 16:9 */ padding-top: 25px;height: 0;"><iframe style="position: absolute;top: 0;left: 0;width: 100%;height: 100%;border:0px;" src="https://www.youtube.com/embed/HL-75xTzn6A"></iframe></div>
                        // Cow::from(format!(r#"<div style="position: relative;padding-bottom: 56.25%; /* 16:9 */ padding-top: 25px;height: 0;"><iframe style="position: absolute;top: 0;left: 0;width: 100%;height: 100%;border:0px;" src="https://www.youtube.com/embed/{}"></iframe></div>"#, contents))

                        // let locals = PyDict::new(article.py);
                        let res: String = match article.py.eval(
                            &format!("str({})", contents),
                            None,
                            Some(&article.context)
                        ) {
                            Ok(s) => format!("{}", s),
                            Err(err) => format!("Ошибка eval"),
                        };
                        // let res: String = article.py.eval(
                        //     &format!("str({})", contents),
                        //     None,
                        //     Some(&article.context)
                        // ).unwrap().extract().unwrap();
                        Cow::from(res)

                        // if let Some(py) = parser.py {
                        //     let res: String = py.eval("1+2", None, None).unwrap().extract().unwrap();
                        //     Cow::from(res)
                        // } else {
                        //     Cow::from("!No article.py!")
                        // }

                    }
                    _ => Cow::from(format!("Unknown command: \\{}", name)),
                }
            },
            Tag::Paragraph{ref c} => {
                let children_strings: Vec<Cow<str>> = c.iter()
                    .map(|&ref x| x.to_html(article))
                    .collect();
                let s = children_strings.join(" ");
                Cow::from(format!("<p>{}</p>", &s))
            }
            Tag::LinkExternal{ref url, ref text} => {
                Cow::from(format!("<a class=\"external\" target=\"_blank\" href=\"{}\">{}</a>", url, text))
            }
            Tag::LinkInternal{ref page, ref text, ref link} => {
                match *link {
                    Some(ref x) => Cow::from(format!("<a href=\"/articles/{}\">{}</a>", x, text)),
                    None => {
                        let page = page.trim();
                        match article.links_internal.get(page) {
                            Some(l) => Cow::from(format!("<a href=\"{}\">{}</a>", l, text)),
                            None => {
                                article.links_internal_missing.insert(String::from(page));
                                // parser.links_internal_missing.insert(Cow::from(page).to_owned());
                                Cow::from(format!("<a class=\"redlink\" href=\"/articles/{}\">{}</a>", page, text))
                            }
                        }
                    }
                }

            //                    x.get("text").unwrap())
                // Cow::from(format!("<p>{}</p>", &s))
            },
            // Tag::Text(ref txt) => Cow::from(format!("{}", txt)),
            Tag::URL{ref proto, ref hostname, ref path, ref query} => {
                let url = format!(
                    "{}://{}{}{}",
                    proto,
                    hostname,
                    path,
                    query,
                );
                match hostname.as_ref() {
                    "www.youtube.com" => {
                        let query_hm = url_query(query.as_bytes());
                        match query_hm {
                            IResult::Done(_, query) => {
                                let video_code = query.get("v");
                                match video_code {
                                    // <iframe width="560" height="315" src="https://www.youtube.com/embed/g6ez7sbaiWc" frameborder="0" allowfullscreen></iframe>

                                    Some(code) => Cow::from(format!(r#"<div style="position:relative;padding-bottom: 56.25%; /* 16:9 */ height: 0;"><iframe style="position: absolute;top: 0;left: 0;width: 100%;height: 100%;border:0px;" src="https://www.youtube.com/embed/{}"></iframe></div>"#, code)),
                                    // Some(code) => Cow::from(format!(Cow::from(format!(youtube_format_str, contents)), code)),
                                    _ => Cow::from(format!(r#"<a href="{0}">{0}</a>"#, url))
                                }
                            },
                            // IResult::Incomplete(x) => println!("incomplete: {:?}", x),
                            // IResult::Error(e) => println!("error: {:?}", e)
                            _ => Cow::from(format!(r#"<a href="{0}">{0}</a>"#, url))
                        }
                    }
                    _ => Cow::from(format!(r#"<a target="_blank" class="external" href="{0}">{0}</a>"#, url))
                }
            },
            Tag::Text(ref txt) => txt.clone(),
            Tag::MathInline(ref text) => Cow::from(format!("\\({0}\\)", text)),
            Tag::MathWholeLine(ref text) => Cow::from(format!("\\[{0}\\]", text)),

            Tag::Header(ref level, ref text) => Cow::from(format!("<h{0}>{1}</h{}>", level, text)),

            // Tag::ListNumbered{ref c} => {
            //     Cow::from(format!("List NUMBERED:<br><ul>{}</ul>", c.to_html(parser)))
            // },
            Tag::HTMLTag(ref tag) => {
                // let parts: Vec<Cow<str>> = items.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();
                if ALLOWED_HTML_TAGS.iter().any(|v| v == &tag.name) {
                    Cow::from(format!("{}", tag))
                } else {
                    Cow::from(format!("&lt;{}&gt;", tag.name))
                }
            },

            Tag::ListNumbered(ref items) => {
                let parts: Vec<Cow<str>> = items.iter()
                    .map(|&ref x| x.to_html(article))
                    .collect();
                Cow::from(format!("<ol>{}</ol>", parts.join("")))
            },

            Tag::ListUnnumbered{ref c} => {
                // let items: Vec<Cow<str>> = c.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();
                // let items: Vec<Tag> = c.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();
                // Cow::from(format!("<li>{}</li>", items.join("</li><li>")))
                // Cow::from(format!("<ul>{}</ul>", items.join("")))
                Cow::from(format!("<ul>{}</ul>", c.to_html(article)))

                    // let parts: Vec<Cow<str>> = c.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();

                // Cow::from(format!("<ul>{}</ul>", s))
            },
            // Tag::ListUnnumberedItem(ref txt) => Cow::from(format!("<li>{}</li>", txt)),
            Tag::ListUnnumberedItem(ref words) => {
                let parts: Vec<Cow<str>> = words.iter()
                    .map(|&ref x| x.to_html(article))
                    .collect();
                Cow::from(format!("<li>{}</li>", parts.join(" ")))
            },
            Tag::ListItemNumbered(ref words) => {
                let parts: Vec<Cow<str>> = words.iter()
                    .map(|&ref x| x.to_html(article))
                    .collect();
                Cow::from(format!("<li>{}</li>", parts.join(" ")))
            },

            Tag::Code(ref lng, ref code) => Cow::from(format!("<pre><code class=\"{}\">{}</code></pre>", lng, code)),

            // css tabs
            // https://codepen.io/wallaceerick/pen/ojtal
            Tag::CodeTabs(ref code_blocks) => {
                if code_blocks.len() == 1 {
                    code_blocks[0].to_html(article)
                } else {
                    let parts: Vec<Cow<str>> = code_blocks.iter()
                        .map(|&ref x| x.to_html(article))
                        .collect();

                    Cow::from(format!(
                        "<ul class=\"tabs\" role=\"tablist\"><li>{}</li></ul>",
                        parts.join("</li><li>")
                    ))
                }
                // let parts: Vec<Cow<str>> = code_blocks.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();
                // Cow::from(format!("<li>{}</li>", parts.join(" ")))
                // Cow::from(format!("<pre><code class=\"{}\">{}</code></pre>", lng, code)),
            }
            Tag::Comment => Cow::from(""),
            Tag::Space => Cow::from(" "),
        }
    }
}


/// Comment
named!(comment<Tag>,
       do_parse!(
           char!( '%' ) >>
           opt!(take_while!(space_but_not_eol)) >>
           map_res!(is_not!( "\r\n" ), from_utf8) >>
           (Tag::Comment)
       )
);


named!(list_item_word<Tag>,
       alt_complete!(
           internal_link |
           external_link |
           url |
           comment |
           symbols
       )
);

named!(pub list_item_words<Tag>,
       do_parse!(
           w: list_item_word >>
           words: many1!(list_item_word) >>
           ({
               let mut v = words;
               v.insert(0, w);
               Tag::Container{c: v}
           })
       )
);

named!(pub cut,
       tag!(">---")
);

named!(pub sha1,
       recognize!(many_m_n!(32, 32, hex_digit ))
);

named!(
    list_item_content<Vec<Tag>>,
    do_parse!(
        // opt!(space_not_eol) >>
        content: separated_nonempty_list!(
            complete!(space_not_eol),
            complete!(
                alt_complete!(
                    list_item_words |
                    list_item_word
                )
            )
        ) >>
        opt!(space_not_eol) >>
        (content)
    )
);


named_attr!(
    #[doc = "A list item like one of these these:

* item 1
* item 2"],
    pub list_unnumbered_item<Tag>,
    // pub list_unnumbered_item<Vec<Tag>>,
    do_parse!(
        opt!(eol) >>
        char!( '*' ) >>
        opt!(space_not_eol) >>

        // txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        words: list_item_content >>
        // opt!(map_res!(is_not!( "\r\n" ), from_utf8)) >>

        // opt!(space_not_eol) >>
        // (Tag::ListUnnumberedItem(Cow::from(txt)))
        // (Tag::Text(Cow::from(txt)))
        ({
            // if (words.len() = 1 )
            Tag::ListUnnumberedItem(words)
        })
        // (words)
    )
);

named_attr!(
    #[doc = "`\\youtube{...}` command"],
    pub youtube<Tag>,
    do_parse!(
        char!( '\\' ) >>
        tag!("youtube") >>
        tag!("{") >>
        sha1: map_res!(alphanumeric, from_utf8) >>
        tag!("}") >>
        ({
            Tag::Command{
                name: Cow::from("youtube"),
                contents: Cow::from(sha1),
            }
        })
    )
);

named_attr!(
    #[doc = "`LaTeX-style command: \\command{...}`"],
    pub command<Tag>,
    do_parse!(
        char!( '\\' ) >>
        name: map_res!(alphanumeric, from_utf8) >>
        tag!("{") >>
        // sha1: map_res!(alphanumeric, from_utf8) >>
        contents: map_res!(take_until!("}"), from_utf8) >>
        tag!("}") >>
        ({
            Tag::Command{
                name: Cow::from(name),
                contents: Cow::from(contents),
            }
        })
    )
);

named_attr!(
    #[doc = "Jinja set expression: `{% set variable1 = variable2 %}`"],
    pub expression<Tag>,
    do_parse!(
        tag!("{{") >>
        contents: map_res!(take_until!("}}"), from_utf8) >>
        tag!("}}") >>
        ({
            Tag::Command {
                name: Cow::from("expression"),
                contents: Cow::from(contents),
            }
        })
    )
);

named_attr!(
    #[doc = "Jinja set expression: `{% set variable1 = variable2 %}`"],
    pub jinjatag<Tag>,
    do_parse!(
        tag!("{%") >>
        contents: map_res!(take_until!("%}"), from_utf8) >>
        tag!("%}") >>
        ({
            Tag::Command {
                name: Cow::from("jinjatag"),
                contents: Cow::from(contents),
            }
        })
    )
);

named_attr!(
    #[doc = "Numbered list item: `#. Item 1`

An item from such list:

```text
#. item 1
#.#. item 1.1
#. item 2
```
"],
    pub list_numbered_item<Tag>,
    // pub list_unnumbered_item<Vec<Tag>>,

    do_parse!(
        many1!(tag!( "#." )) >>
        tag!( " " ) >>
        // opt!(space_not_eol) >>
        // txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        // words: map_res!(not_line_ending, from_utf8) >>
        words: list_item_content >>
        // opt!(map_res!(is_not!( "\r\n" ), from_utf8)) >>
        // opt!(space_not_eol) >>
        // opt!(not_line_ending) >>
        alt_complete!(
            eol |
            eof!()
        ) >>

        // (Tag::ListUnnumberedItem(Cow::from(txt)))
        // (Tag::Text(Cow::from(txt)))
        ({
            // if (words.len() = 1 )
            Tag::ListItemNumbered(
                // vec![
                //     Tag::Text(Cow::from(words))
                // ]
                words
            )
        })
        // (words)
    )
);


named_attr!(
    #[doc = "Numbered list

```text
#. item 1
#.#. item 1.1
#. item 2
```
"],
    pub list_numbered<Tag>,
    do_parse!(
        // items: separated_nonempty_list!(complete!(tag!("")), complete!(list_numbered_item)) >>
        // items: separated_nonempty_list!(complete!(eol), complete!(list_numbered_item)) >>
        // items: separated_list!(complete!(space_max1eol), list_unnumbered_item) >>
        // items: separated_list!(space_max1eol, list_unnumbered_item) >>
        // take_while!(any_space) >>
        // items: separated_list!(eol, map_res!(is_not!( "\r\n" ), from_utf8)) >>
        items: many1!(list_numbered_item) >>
        (Tag::ListNumbered(items))
    )
);


named_attr!(
    #[doc = "Unnumbered list

```
* item1
* item2
```"],
    pub list_unnumbered<Tag>,
    do_parse!(
        // items: separated_list!(space_max1eol, list_unnumbered_item) >>
        // take_while!(any_space) >>
        // items: separated_list!(eol, map_res!(is_not!( "\r\n" ), from_utf8)) >>
        items: many1!(list_unnumbered_item) >>
        (Tag::ListUnnumbered{c: items})
    )
);


named_attr!(
    #[doc = "Parse headers (Markdown style, ex.: `## Header 2`)

```text
# Header 1
## Header 2
### Header 3
#### Header 4
##### Header 5
###### Header 6
```"],
    pub header<Tag>,
    do_parse!(
        level: many1!(tag!( "#" )) >>
        opt!(space_not_eol) >>
        txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        (Tag::Header(level.len(), Cow::from(txt)))
    )
);


named_attr!(
    #[doc = "Parse inline math formula (Latex style, ex.: `\\( ... \\)`)

Not using Tex-style `$ ... $`. Easier to work with and parse Latex style formulas.

```text
\\(a+b\\)    inline formulas
\\[a+b\\]    separate line formulas
```
"],
    pub inline_formula<Tag>,
    do_parse!(
        tag!( "\\(" ) >>
        // opt!(space_not_eol) >>
        txt: map_res!(take_until!("\\)"), from_utf8) >>
        tag!( "\\)" ) >>
        (Tag::MathInline(Cow::from(txt)))
    )
);


named_attr!(
    #[doc = "Parse separate math formula (Latex style, ex.: `\\[ ... \\]`)

Not using Tex-style `$ ... $`. Easier to work with and parse Latex style formulas.

```text
\\(a+b\\)    inline formulas
\\[a+b\\]    separate line formulas
```
"],
    pub separate_formula<Tag>,
    do_parse!(
        tag!( "\\[" ) >>
        opt!(space_not_eol) >>
        // txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        txt: map_res!(take_until!("\\]"), from_utf8) >>
        tag!( "\\]" ) >>
        (Tag::MathWholeLine(Cow::from(txt)))
    )
);

named_attr!(
    #[doc = "Parse math formula (Latex style)

Not using Tex-style `$ ... $`. Easier to work with and parse Latex style formulas.

```text
\\(a+b\\)    inline formulas
\\[a+b\\]    separate line formulas
```
"],
    pub math_formula<Tag>,
    alt_complete!(
        inline_formula |
        separate_formula
    )
);


named_attr!(
    #[doc = "Code block defined between 3 backticks (```)

```no_run

```
"],
    pub code<Tag>,
    do_parse!(
        tag!("```") >>
        language: map_res!(take_while!(not_space), from_utf8) >>
        take_while!(any_space) >>
        code: map_res!(take_until!("```"), from_utf8) >>
        tag!("```") >>
        (Tag::Code(
            match language {
                "" => Cow::from("bash"),
                _ => Cow::from(language)
            },
            Cow::from(code)))
        // (params.iter().fold(
        //         HashMap::new(),
        //         |mut T, tuple| {T.insert(tuple.0, tuple.1); T})
        // )
    )
);

named_attr!(
    #[doc = "Several code blocks (as tabs)

Defined just as several code blocks without any blank lines between
them.

```

```
"],
    pub code_tabs<Tag>,
    do_parse!(
        codes: separated_nonempty_list!(
            complete!(eol),
            complete!(
                code
            )
        ) >>
        (Tag::CodeTabs(codes))
        // (params.iter().fold(
        //         HashMap::new(),
        //         |mut T, tuple| {T.insert(tuple.0, tuple.1); T})
        // )
    )
);


/// internal link
named!(internal_link1<Tag>,
    do_parse!(
        tag!("[[") >>
        page: map_res!(take_until!("|"), from_utf8) >>
        tag!("|") >>
        text: map_res!(take_until!("]]"), from_utf8) >>
        tag!("]]") >>
        (Tag::LinkInternal{page: Cow::from(page), text: Cow::from(text), link: None})
    )
);
named!(internal_link2<Tag>,
    do_parse!(
        tag!("[[") >>
        page: map_res!(take_until!("]]"), from_utf8) >>
        tag!("]]") >>
        (Tag::LinkInternal{page: Cow::from(page), text: Cow::from(page), link: None})
    )
);

named_attr!(
    #[doc = "Internal link like in Wikipedia: `[[Page name | Link title]]`"],
    pub internal_link<Tag>,
    alt_complete!(
        internal_link1 |
        internal_link2
    )
);



named!(pub external_link<Tag>,
    do_parse!(
        tag!("[") >>
        url: url >>
        tag!(" ") >>
        opt!(take_while!(any_space)) >>
        text: map_res!(take_until!("]"), from_utf8) >>
        tag!("]") >>
        (Tag::LinkExternal {
            url: Cow::from(url.to_string()),
            text: Cow::from(text),
            // tag_after: tag_after,
        })
    )
);


/// URL parser
named!(pub url<Tag>,
    do_parse!(
        proto: map_res!(uri_scheme, from_utf8)  >>
        tag!("://")   >>
        hostname: map_res!(hostname, from_utf8) >>
        path: opt!(map_res!(is_not!( "? \t\r\n" ), from_utf8)) >>
        query: opt!(map_res!(recognize!(url_query), from_utf8)) >>
        (
            (Tag::URL{
                proto: Cow::from(proto),
                hostname: Cow::from(hostname),
                path: Cow::from(path.unwrap_or("")),
                query: Cow::from(query.unwrap_or("")),
            })
        )
    )
);

named_attr!(
    #[doc = "Main parser function"],
    pub parse<Vec<Tag>>,
    do_parse!(
        opt!(take_while!(any_space)) >>
        pars: separated_list!(complete!(space_min2eol), complete!(root_element)) >>
        opt!(take_while!(any_space)) >>
        // (Tag::Container{c:pars})
        (pars)
    )
);


named_attr!(
    #[doc = "Line-start elements (line position == 0)"],
    line_start_element<Tag>,
    alt_complete!(
        code_tabs |
        complete!(list_numbered) |
        complete!(list_unnumbered) |
        html_tag |
        header |
        expression |
        command |
        complete!(paragraph) |
        space_tag
    )
);


named_attr!(
    #[doc = "Middle-line elements (line position > 0)"],
    element<Tag>,
    alt_complete!(
        code_tabs |
        html_tag |
        expression |
        command |
        paragraph |
        space_tag
    )
);


named!(root_element<Tag>,
       alt_complete!(
           // list_unnumbered |
           complete!(list_numbered) |
           // paragraphs |
           command |
           complete!(paragraph) |
           space_tag
       )
);

named!(space_tag<Tag>,
       do_parse!(
           txt: take_while1!(any_space) >>
           (Tag::Space)
       )
);

named_attr!(
    #[doc = "Anything but spaces and new lines"],
    symbols<Tag>,
    do_parse!(
        txt: map_res!(take_while1!(not_space), from_utf8) >>
        (Tag::Text(Cow::from(txt)))
    )
);

named!(html_tag<Tag>,
       do_parse!(
           // t:tag!("asd") >>
           t: tag >>
           (Tag::HTMLTag(t))
               // (Tag::Space)
       )
);

named!(
    word<Tag>,
    alt_complete!(
        // many1!(word) => {|x| Tag::Comment} |
        internal_link |
        external_link |
        expression |
        url |
        comment |
        html_tag |
        symbols               // any text
    )
);

named!(words<Tag>,
       do_parse!(
           w: word >>
           words: many1!(word) >>
           ({
               let mut v = words;
               v.insert(0, w);
               Tag::Container{c: v}
           })
       )
);

named_attr!(
    #[doc = "Paragraph

 # Examples

 ```
 use rparser::article::parser::paragraph;
 let res = paragraph(&b\"1 2\"[..]);
 assert_eq!(res, Done(&b\"\"[..], Tag::Space));
 ```
"],
    pub paragraph<Tag>,
    do_parse!(
        words: separated_nonempty_list_complete!(
            space_max1eol,
            // word
            alt_complete!(
                words |     // 2 "words" with no space between them
                word        // single "word"
            )
        ) >>
        // opt!(space_min2eol) >>
           // words: many1!(word) >>
           // , Vec::new(), |mut acc: Vec<_>, item| {
           // ), Vec::new(), |mut acc: Vec<_>, item| {
           //     // match item {
           //     //     Tag::Container{ref c} => {}
           //     //     _ => {}
           //     // }
           //     // println!("{}", item);
           //     for c in item.c {

           //     }
           //     acc.push(item);
           //     acc
           // )) >>
        (Tag::Paragraph{c: words})
       )
);

named_attr!(
    #[doc = "Paragraphs"],
    pub paragraphs<Tag>,
    do_parse!(
        pars: separated_nonempty_list!(
            complete!(space_min2eol),
            complete!(paragraph)
            // word
        ) >>
        (Tag::Container{c: pars})
    )
);



#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::{Done};
    use std::collections::HashMap;
    // use super::super::node::Tag;
    // use std::str::from_utf8;
    // use common::*;
    use std::borrow::Cow;

    // #[test]
    // fn test_list_unnumbered_item() {
    //     let mut tests = HashMap::new();
    //     // let mut x = HashMap::new();
    //     // x.insert("txt".to_string(), "asd".to_string());
    //     tests.insert(
    //         &b"* asd"[..],
    //         Done(&b""[..], Tag::ListUnnumberedItem(vec![Tag::Text(Cow::from("asd"))]))
    //     );
    //     for (input, expected) in tests {
    //         assert_eq!(list_unnumbered_item(&input), expected);
    //     }
    // }

    #[test]
    #[ignore]
    fn test_list_unnumbered() {
        let mut tests = HashMap::new();
        tests.insert(
            // &b"*asd\n*123"[..],
            &b"*1 txt \n* 2"[..],
            Done(&b""[..],
                 // Tag::ListUnnumbered{c: vec![
                 //     vec![Tag::Text(Cow::from("asd"))],
                 //     vec![Tag::Text(Cow::from("123"))]
                 // ]}
                 Tag::ListUnnumbered{c: vec![
                     Tag::ListUnnumberedItem(vec![Tag::Text(Cow::from("asd"))]),
                     Tag::ListUnnumberedItem(vec![Tag::Text(Cow::from("123"))])
                 ]}
            )
        );
        for (input, expected) in &tests {
            assert_eq!(list_unnumbered(input), *expected);
        }
    }

    #[test]
    fn test_listitem_numbered() {
        let mut tests = HashMap::new();
        // tests.insert(
        //     &b"#. txt"[..],
        //     Done(&b""[..],
        //          Tag::ListItemNumbered(
        //              vec![
        //                  Tag::Container{
        //                      c: vec![Tag::Text(Cow::from("txt"))]
        //                  }

        //              ]
        //          )
        //     )
        // );
        tests.insert(
            &b"#.#. txt\n"[..],
            Done(&b""[..],
                 Tag::ListItemNumbered(
                     vec![
                         Tag::Text(Cow::from("txt"))
                     ]
                 )
            )
        );
        for (input, expected) in &tests {
            assert_eq!(list_numbered_item(input), *expected);
        }
    }

    #[test]
    fn test_list_numbered() {
        let mut tests = HashMap::new();

        // List from 1 item
        tests.insert(
            &b"#. item1"[..],
            Done(&b""[..],
                 Tag::ListNumbered(vec![
                     Tag::ListItemNumbered(vec![Tag::Text(Cow::from("item1"))]),
                 ])
            )
        );

        // List from 2 items
        tests.insert(
            &b"#. item1\n#. item2"[..],
            Done(&b""[..],
                 Tag::ListNumbered(vec![
                     Tag::ListItemNumbered(vec![Tag::Text(Cow::from("item1"))]),
                     Tag::ListItemNumbered(vec![Tag::Text(Cow::from("item2"))])
                 ])
            )
        );
        for (input, expected) in &tests {
            assert_eq!(list_numbered(input), *expected);
        }
    }

    #[test]
    fn test_formula() {
        let mut tests = HashMap::new();
        tests.insert(
            &b"\\( a + b \\)"[..],
            Done(&b""[..], Tag::MathInline(Cow::from(" a + b ")))
        );
        tests.insert(
            &b"\\[ a + b \\]"[..],
            Done(&b""[..], Tag::MathWholeLine(Cow::from("a + b ")))
        );
        for (input, expected) in &tests {
            assert_eq!(math_formula(input), *expected);
        }
    }

    #[test]
    fn test_url() {
        let mut tests = HashMap::new();
        tests.insert(
            &b"https://www.youtube.com/watch?v=g6ez7sbaiWc"[..],
            Done(&b""[..], Tag::URL {
                proto: Cow::from("https"),
                hostname: Cow::from("www.youtube.com"),
                path: Cow::from("/watch"),
                query: Cow::from("?v=g6ez7sbaiWc"),
            })
        );
        for (input, expected) in &tests {assert_eq!(url(input), *expected);}
    }

    #[test]
    fn test_paragraph() {
        // "1 2"
        assert_eq!(
            paragraph(&b"1 2"[..]),
            Done(&b""[..], Tag::Paragraph{c: vec![
                Tag::Text(Cow::from("1")),
                Tag::Text(Cow::from("2"))
            ]})
        );

        // 2 elements without space - it is 1 paragraph:
        // assert_eq!(
        //     paragraph(&b"https://www.youtube.com/watch?v=g6ez7sbaiWc"[..]),
        //     Done(&b""[..], Tag::Paragraph{c: vec![
        //         Tag::Text(Cow::from("1")),
        //         Tag::Text(Cow::from("2"))
        //     ]})
        // );

        // assert_eq!(
        //     paragraph(Cow::from("Так даже лучше. Если Вы находитесь в другом городе, тогда это вообще единственный вариант. Но и в Москве Вы сэкономите деньги и время, потомуasdasd что не надо будет их тратить на поездку по городу.").as_bytes()),
        //     // paragraph(Cow::from("abc\n").as_bytes()),
        //     Done(&b""[..], Tag::Paragraph{c: vec![
        //         Tag::Text(Cow::from("1")),
        //         // Tag::Text(Cow::from("2"))
        //     ]})
        // );

        // no spaces between paragraph "words" - still 1 paragraph
        // assert_eq!(
        //     paragraph(&b"[[page1 | title1]][[page2 | title2]]"[..]),
        //     Done(&b""[..], Tag::Paragraph{c: vec![
        //         Tag::Text(Cow::from("1")),
        //         Tag::Text(Cow::from("2"))
        //     ]})
        // );

        // tests.insert(
        //     &b" 1 2  "[..],  // dot after link
        //     Done(&b""[..], Tag::Paragraph{
        //         c: vec![
        //             Tag::Text(Cow::from("."))
        //         ]}
        //     ));
        // tests.insert(
        //     &b"[http://pashinin.com Text]."[..],  // dot after link
        //     Done(&b""[..], Tag::Paragraph{
        //         c: vec![
        //             Tag::LinkExternal{
        //                 url: Cow::from("http://pashinin.com"),
        //                 text: Cow::from("Text"),
        //             },
        //             Tag::Text(Cow::from("."))
        //         ]}
        //     ));
        // tests.insert(
        //     &b"https://host.pashinin.com 2"[..],
        //     Done(&b""[..], Tag::Paragraph{
        //         c: vec![
        //             Tag::URL{
        //                 proto: Cow::from("https"),
        //                 hostname: Cow::from("host.pashinin.com"),
        //                 path: Cow::from(""),
        //                 query: Cow::from(""),
        //             },
        //             Tag::Text(Cow::from("2"))
        //         ]},
        //     )
        // );

        // for (input, expected) in tests {
        //     assert_eq!(paragraph(&input), expected);
        // }
    }

    #[test]
    fn test_list_item_content(){
        let mut tests = HashMap::new();
        tests.insert("123 ", Done(&b""[..], vec![
            Tag::Text(Cow::from("123"))
        ]));

        for (input, expected) in tests {
            assert_eq!(
                list_item_content(input.as_bytes()),
                expected);
        }
    }

    #[test]
    fn test_youtube(){
        let mut tests = HashMap::new();
        tests.insert("\\youtube{123abc}", Done(
            &b""[..],
            Tag::Command{
                name: Cow::from("youtube"),
                contents: Cow::from("123abc")
            }
        ));

        for (input, expected) in tests {
            assert_eq!(
                youtube(input.as_bytes()),
                expected);
        }
    }

    #[test]
    fn test_expression(){
        let mut tests = HashMap::new();
        let s = "{{exp.total_seconds() / 60 / 60 / 24 / 365.25}}";
        tests.insert(s, Done(
            &b""[..],
            Tag::Command{
                name: Cow::from("expression"),
                contents: Cow::from("exp.total_seconds() / 60 / 60 / 24 / 365.25")
            }
        ));
        for (input, expected) in tests {
            assert_eq!(expression(input.as_bytes()), expected);
        }


        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut a = Article::new(py);
        // a.py = Some(py);
        a.src = "{{ 'asd'+'111' }}".as_bytes();
        a.render();
        assert_eq!(a.html, "asd111");

        a.src = "{{ datetime.datetime.now() - datetime.datetime(2013, 1, 1) }}".as_bytes();
        a.render();
        // assert_eq!(a.html, "asd111");

        a.src = "{{ no_such_variable }}".as_bytes();
        a.render();
        assert_eq!(a.html, "Ошибка eval");
    }

    #[test]
    fn test_code_tabs(){
        let mut tests = HashMap::new();
        tests.insert(
            "```pascal\nvar x: integer;\n```\n```cpp\nint x=1;\n``` ",
            Done(&b" "[..],
                 Tag::CodeTabs(vec![
                     Tag::Code(Cow::from("pascal"), Cow::from("var x: integer;\n")),
                     Tag::Code(Cow::from("cpp"), Cow::from("int x=1;\n"))
                 ])
        ));

        for (input, expected) in tests {
            assert_eq!(
                code_tabs(input.as_bytes()),
                expected);
        }
    }

    #[test]
    fn test_render() {
        let mut tests = HashMap::new();
        // tests.insert("* 123 \n* asd ", "<ul><li>123</li><li>asd</li></ul>");
        tests.insert("1", "<p>1</p>");
        tests.insert("1 2", "<p>1 2</p>");

        tests.insert("  1  \n\n 2  ", "<p>1</p><p>2</p>");

        // This is not a header (a line starts from a space symbol)
        tests.insert(" # Header (no)", "<p># Header (no)</p>",);

        // Numbered list, 1 item:
        tests.insert("#. 123", "<ol><li>123</li></ol>");
        // Numbered list, 2 items:
        tests.insert("#. 123\n#. asd", "<ol><li>123</li><li>asd</li></ol>");
        // Numbered list, 2 items, space in the end:
        tests.insert("#. 123\n#. asd ", "<ol><li>123</li><li>asd</li></ol>");

        // for (input, expected) in &tests {assert_eq!(parse(input), *expected);}

        //
        // HTML tags
        //
        // Forbidden tags: <script> <iframe> and everything except
        // allowed tags
        tests.insert("<script >", "&lt;script&gt;");
        tests.insert("<table><tr><td>", "<table><tr><td>");
        // tests.insert("<i>italics</i>", "<table><tr><td>");


        // tests.insert("\\youtube{abc}", "command");


        // code tabs
        // 1 code block
        tests.insert(
            "```pascal\nvar x: integer;\n```",
            "<pre><code class=\"pascal\">var x: integer;\n</code></pre>"
        );

        // 2 tabs
        // tests.insert(
        //     "```pascal\nvar x: integer;\n```\n```cpp\nint x=1;\n```",
        //     "<table><tr><td>"
        // );


        for (input, expected) in tests {
            // let a = Article::from(input);
            // assert_eq!(a.html, *expected);
        }
    }

    #[test]
    // #[ignore]
    fn test_parse() {
        let mut tests = HashMap::new();

        tests.insert(
            &b"1"[..],
            Done(&b""[..], vec![
                    Tag::Paragraph {
                        c: vec![Tag::Text(Cow::from("1"))]
                    }
                ]
            )
        );

        // Numbered list with space after it
        tests.insert(
            &b"#. item1\n#. item2"[..],
            Done(&b""[..], vec![
                Tag::ListNumbered(
                    vec![
                        Tag::ListItemNumbered(vec![Tag::Text(Cow::from("item1"))]),
                        Tag::ListItemNumbered(vec![Tag::Text(Cow::from("item2"))]),
                    ]
                )
            ])
        );


        for (input, expected) in &tests {assert_eq!(parse(input), *expected);}
    }
}



#[cfg(test)]
mod test {
    use super::*;
    // use nom::IResult::{Done, Incomplete, Error};
    use std::collections::HashMap;
    // use std::str::from_utf8;
    // use common::*;
    use std::borrow::Cow;

    #[test]
    fn parser() {
        let mut map = HashMap::new();
        map.insert(Cow::from("page 1 "), Cow::from(" text"));
        // Article::from("[[page 1 | text]]");
        // assert_eq!(p.links, map!(Cow::from("page 1 ") => Cow::from(" text")));
        // assert_eq!(p.links, map);
    }
}
