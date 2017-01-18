use std::string::String;
use std::string::ToString;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;
use nom::{IResult};
use common::{url_query};
use std::convert::Into;
use std::borrow::Cow;
// use std::borrow::ToOwned;

#[cfg(feature = "python")]
use cpython::{PyDict, Python, PyString, ToPyObject, PyObject, PyResult, PythonObject, PyTuple};

#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug)]
pub enum Tag<'a> {
    // block-level tags
    Paragraph,
    Rule,
    Header(i32),
    BlockQuote,
    CodeBlock(Cow<'a, str>),
    List(Option<usize>),  // TODO: add delim and tight for ast (not needed for html)
    Item,
    FootnoteDefinition(Cow<'a, str>),

    // tables
    Table(Vec<Alignment>),
    TableHead,
    TableRow,
    TableCell,

    // span-level tags
    Emphasis,
    Strong,
    Code,
    Link(Cow<'a, str>, Cow<'a, str>),
    Image(Cow<'a, str>, Cow<'a, str>),
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeClass{
    Root,
    Paragraph,
    ListUnnumbered,
    ListUnnumberedItem,
    Header,
    Code,
    URL,
    Text,
    Comment,
}

#[derive(PartialEq,Eq,Debug,Clone)]
pub struct Node {
    pub class: NodeClass,
    pub params: Option<HashMap<String, String>>,
    // Into<Cow<'a, str>>
    pub children: Option<Vec<Node>>,
}


impl Node{
    /// Form URL node from params
    pub fn new_url<S>(proto: S, hostname: S, path: S, query: S) -> Node
        where S: Into<String>
    {
        let mut x = HashMap::new();
        x.insert("proto".to_string(), proto.into());
        x.insert("hostname".to_string(), hostname.into());
        x.insert("path".to_string(), path.into());
        x.insert("query".to_string(), query.into());
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::URL,
        }
    }


    /// Root
    pub fn new_root(children: Vec<Node>) -> Node
    {
        Node{
            children: Some(children),
            params: None,
            class: NodeClass::Root,
        }
    }


    /// Paragraph
    pub fn new_paragraph(children: Vec<Node>) -> Node
    {
        Node{
            children: Some(children),
            params: None,
            class: NodeClass::Paragraph,
        }
    }


    /// Comment node
    pub fn new_comment<S>(txt: S) -> Node
        where S: Into<String>
    {
        let mut map = HashMap::new();
        map.insert("txt".to_string(), txt.into());
        Node{
            children: None,
            params: Some(map),
            class: NodeClass::Comment,
        }
    }


    /// List
    pub fn new_list_unnumbered_item<S>(txt: S) -> Node
        where S: Into<String>
    {
        let mut x = HashMap::new();
        x.insert("txt".to_string(), txt.into());
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::ListUnnumberedItem,
        }
    }
    pub fn new_list_unnumbered(items: Vec<Node>) -> Node
    {
        Node{
            children: Some(items),
            params: None,
            class: NodeClass::ListUnnumbered,
        }
    }


    /// H2 header
    pub fn new_h2<S>(txt: S) -> Node
        where S: Into<String>
    {
        let mut x = HashMap::new();
        x.insert("txt".to_string(), txt.into());
        x.insert("tag".to_string(), "h2".to_string());
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::Header,
        }
    }

    /// Code
    pub fn new_code<S>(txt: S, lng: S) -> Node
        where S: Into<String>
    {
        let mut x = HashMap::new();
        x.insert("txt".to_string(), txt.into());
        x.insert("lng".to_string(), lng.into());
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::Code,
        }
    }


    /// Text node
    pub fn new_text<S>(txt: S) -> Node
        where S: Into<String>
    {
        let mut x = HashMap::new();
        x.insert("txt".to_string(), txt.into());
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::Text,
        }
    }
}


impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.class {

            // Root
            NodeClass::Root => {
                match self.children {
                    Some(ref children) => {
                        let children_strings: Vec<String> = children.iter()
                            .map(|&ref x| x.to_string())
                            .collect();
                        let s = children_strings.join("");
                        write!(f, "{}", s)
                    }
                    _ => {
                        write!(f, "")
                    }
                }
            }

            // Paragraph
            NodeClass::Paragraph => {
                match self.children {
                    Some(ref children) => {
                        let children_strings: Vec<String> = children.iter()
                            .map(|&ref x| x.to_string())
                            .collect();
                        // let s = x.iter().fold(
                        //     "".to_string(),
                        //     |mut i,j| {i.push_str(&*j.to_string());i.push_str(" "); i});
                        let s = children_strings.join(" ");
                        write!(f, "<p>{}</p>", s)
                    }
                    _ => {
                        write!(f, "<p></p>")
                    }
                }
            }

            // URL
            NodeClass::URL => {
                match self.params {
                    Some(ref x) => {
                        let hostname = x.get("hostname").unwrap();
                        let url = format!(
                            "{}://{}{}{}",
                            x.get("proto").unwrap(),
                            x.get("hostname").unwrap(),
                            x.get("path").unwrap(),
                            x.get("query").unwrap(),
                        );
                        match hostname.as_ref() {
                            "www.youtube.com" => {
                                let q = x.get("query").unwrap();
                                let query_hm = url_query(q.as_bytes());
                                match query_hm {
                                    IResult::Done(_, query) => {
                                        let video_code = query.get("v");
                                        match video_code {
                                            // <iframe width="560" height="315" src="https://www.youtube.com/embed/g6ez7sbaiWc" frameborder="0" allowfullscreen></iframe>
                                            Some(code) => write!(f, r#"<iframe width="560" height="315" src="https://www.youtube.com/embed/{}" frameborder="0" allowfullscreen></iframe>"#, code),
                                            _ => write!(f, r#"<a href="{0}">{0}</a>"#, url)
                                        }
                                    },
                                    // IResult::Incomplete(x) => println!("incomplete: {:?}", x),
                                    // IResult::Error(e) => println!("error: {:?}", e)
                                    _ => write!(f, r#"<a href="{0}">{0}</a>"#, url)
                                }
                            }
                            _ => write!(f, r#"<a href="{0}">{0}</a>"#, url)
                        }
                    }
                    _ => {write!(f, "")}
                }
            }

            // Code
            NodeClass::Code => {
                match self.params {
                    Some(ref x) => {
                        write!(f, "<pre>{}</pre>", x.get("txt").unwrap())
                    }
                    _ => {write!(f, "")}
                }
            }

            // Headers
            NodeClass::Header => {
                match self.params {
                    Some(ref x) => {
                        write!(f, "<{0}>{1}</{0}>", x.get("tag").unwrap(), x.get("txt").unwrap())
                    }
                    _ => {write!(f, "")}
                }
            }

            // Comment
            NodeClass::Comment => {
                write!(f, "")
            }

            // new_list_unnumbered
            NodeClass::ListUnnumbered => {
                // write!(f, "LIST ")
                match self.children {
                    Some(ref children) => {
                        let children_strings: Vec<String> = children.iter()
                            .map(|&ref x| x.to_string())
                            .collect();
                        let s = children_strings.join("");
                        write!(f, "<ul>{}</ul>", s)
                    }
                    _ => {
                        write!(f, "<ul></ul>")
                    }
                }
            }
            // new_list_unnumbered
            NodeClass::ListUnnumberedItem => {
                // write!(f, "LISTITEM ")
                match self.params {
                    Some(ref x) => {
                        write!(f, "<li>{0}</li>", x.get("txt").unwrap())
                    }
                    _ => {write!(f, "")}
                }
            }

            // Text
            NodeClass::Text => {
                match self.params {
                    Some(ref x) => {
                        write!(f, "{}", x.get("txt").unwrap())
                    }
                    _ => {write!(f, "")}
                }
            }
        }
    }
}


// pub trait PythonObject: ToPyObject + Send + Sized + 'static {
//     fn as_object(&self) -> &PyObject;
//     fn into_object(self) -> PyObject;
//     unsafe fn unchecked_downcast_from(PyObject) -> Self;
//     unsafe fn unchecked_downcast_borrow_from(&PyObject) -> &Self;
// }
// impl<'a> PythonObject for Node<'a> {
//     #[inline]
//     fn as_object(self) -> &'a PyObject {
//         &self.to_py_object()
//     }

//     #[inline]
//     fn into_object(self) -> PyObject {
//         // py.None().into_object()
//         self
//     }
// }

// pub fn into_object(self) -> PyObject {
//         self.iter
// }



/// Convert Node to a python object (PyTuple, PyDict)
#[cfg(feature = "python")]
impl ToPyObject for Node{
    type ObjectType = PyObject;

    #[inline]
    fn to_py_object(&self, py: Python) -> PyObject {
        // py.None().into_object()
        match self.class {
            NodeClass::Root => {
                // PyTuple::new(py, &vec![]).into_object()
                match self.children {
                    Some(ref nodes) => {
                        let children: Vec<PyObject> = nodes.iter()
                            .map(|&ref x| x.to_py_object(py))
                            .collect();
                        PyTuple::new(py, &children.as_slice()).into_object()
                    }
                    None => {

                        PyTuple::new(py, &vec![]).into_object()
                    }
                }
            },
            NodeClass::Paragraph => {
                // PyTuple::new(py, &vec![]).into_object()
                let d = PyDict::new(py);
                d.set_item(py, "type", "paragraph").unwrap();
                // d.set_item(py, "items", PyTuple::new(py, &vec![]).into_object());
                d.set_item(
                    py,
                    "items",
                    match self.children {
                        Some(ref nodes) => {
                            let children: Vec<PyObject> = nodes.iter()
                                .map(|&ref x| x.to_py_object(py))
                                .collect();
                            PyTuple::new(py, &children.as_slice()).into_object()
                        }
                        None => {

                            PyTuple::new(py, &vec![]).into_object()
                        }
                    }
                ).unwrap();


                // PyString::new(py, &format!("{}", &self)).into_object()
                d.into_object()
            },
            NodeClass::ListUnnumbered => {
                PyTuple::new(py, &vec![]).into_object()
            },
            NodeClass::ListUnnumberedItem => {
                PyTuple::new(py, &vec![]).into_object()
            },
            NodeClass::Header => {
                PyTuple::new(py, &vec![]).into_object()
            },
            NodeClass::Code => {
                PyTuple::new(py, &vec![]).into_object()
            },
            NodeClass::URL => {
                PyTuple::new(py, &vec![]).into_object()
            },
            NodeClass::Comment => {
                // PyTuple::new(py, &vec![]).into_object()
                let d = PyDict::new(py);
                d.set_item(py, "type", "comment").unwrap();
                d.set_item(py, "text", match self.params {
                    Some(ref x) => {
                        x.get("txt").unwrap()
                    }
                    _ => ""
                }).unwrap();
                d.into_object()
            },
            NodeClass::Text => {
                let d = PyDict::new(py);
                d.set_item(py, "type", "text").unwrap();
                d.set_item(py, "text", match self.params {
                    Some(ref x) => {
                        x.get("txt").unwrap()
                    }
                    _ => ""
                }).unwrap();
                d.into_object()
            }
        }

    }
}


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
