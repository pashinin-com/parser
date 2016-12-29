use std::string::String;
use std::string::ToString;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;
use nom::{IResult};
use cpython::{PyDict, Python, PyString, ToPyObject, PyObject, PyResult, PythonObject, PyTuple};
use common::{url_query};

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

#[derive(PartialEq,Eq,Debug)]
pub struct Node<'a> {
    pub class: NodeClass,

    // pub params: Option<HashMap<&'a str, &'a str> >,
    pub params: Option<HashMap<&'a str, &'a str> >,
    // Into<Cow<'a, str>>

    // pub hostname: &'a str,
    // pub path: &'a str,
    // pub query: Option<&'a str>,
    pub children: Option<Vec<Node<'a>>>,
    // pub contents: String,
}


impl<'a> Node<'a>{

    /// Form URL node from params
    pub fn new_url(
        proto: &'a str,
        hostname: &'a str,
        path: &'a str,
        query: &'a str,
    ) -> Node<'a>
    // pub fn new_url<S>(proto: S) -> Node<'a>
        // where S: Into<Cow<'a, str>>
    {
        let mut x = HashMap::new();
        x.insert("proto", proto);
        x.insert("hostname", hostname);
        x.insert("path", path);
        x.insert("query", query);
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::URL,
        }
    }


    /// Root
    pub fn new_root(
        children: Vec<Node<'a>>,
    ) -> Node<'a>
    {
        Node{
            children: Some(children),
            params: None,
            class: NodeClass::Root,
        }
    }


    /// Paragraph
    pub fn new_paragraph(
        children: Vec<Node<'a>>,
    ) -> Node<'a>
    {
        Node{
            children: Some(children),
            params: None,
            class: NodeClass::Paragraph,
        }
    }


    /// Comment node
    pub fn new_comment(txt: &'a str) -> Node<'a>
    {
        let mut x = HashMap::new();
        x.insert("txt", txt);
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::Comment,
        }
    }


    /// List
    pub fn new_list_unnumbered_item(txt: &'a str) -> Node<'a>
    {
        let mut x = HashMap::new();
        x.insert("txt", txt);
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::ListUnnumberedItem,
        }
    }
    pub fn new_list_unnumbered(items: Vec<Node<'a> >) -> Node<'a>
    {
        Node{
            children: Some(items),
            params: None,
            class: NodeClass::ListUnnumbered,
        }
    }


    /// H2 header
    pub fn new_h2(txt: &'a str) -> Node<'a>
    {
        let mut x = HashMap::new();
        x.insert("txt", txt);
        x.insert("tag", "h2");
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::Header,
        }
    }

    /// Code
    pub fn new_code(txt: &'a str, lng: &'a str) -> Node<'a>
    {
        let mut x = HashMap::new();
        x.insert("txt", txt);
        x.insert("lng", lng);
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::Code,
        }
    }


    /// Text node
    pub fn new_text(
        txt: &'a str,
    ) -> Node<'a>
    {
        let mut x = HashMap::new();
        x.insert("txt", txt);
        Node{
            children: None,
            params: Some(x),
            class: NodeClass::Text,
        }
    }
}


impl<'a> Display for Node<'a> {
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
                        match hostname {
                            &"www.youtube.com" => {
                                let q = x.get("query").unwrap();
                                let query_hm = url_query(q.as_bytes());
                                match query_hm {
                                    IResult::Done(_, query) => {
                                        let video_code = query.get("v");
                                        match video_code {
                                            // <iframe width="560" height="315" src="https://www.youtube.com/embed/g6ez7sbaiWc" frameborder="0" allowfullscreen></iframe>
                                            Some(code) => write!(f, r#"<iframe width="560" height="315" src="https://www.youtube.com/embed/{}" frameborder="0" allowfullscreen></iframe>"#, code),
                                            // Some(code) => write!(f, "code: {}", code),
                                            _ => write!(f, r#"<a href="{0}">{0}</a>"#, url)
                                        }
                                        // println!("i: {} | o: {:?}", i, o);
                                        // return Ok(PyString::new(py, &o));
                                        // write!(f, r#"get video"#)
                                    },
                                    // IResult::Incomplete(x) => println!("incomplete: {:?}", x),
                                    // IResult::Error(e) => println!("error: {:?}", e)
                                    _ => write!(f, r#"<a href="{0}">{0}</a>"#, url)
                                }

                                // write!(f, r#"<iframe width="560" height="315" src="https://www.youtube.com/embed/g6ez7sbaiWc" frameborder="0" allowfullscreen></iframe>"#)
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



/// Convert Node to a python object
impl<'a> ToPyObject for Node<'a>{
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
                d.set_item(py, "type", "paragraph");
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
                );


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
                d.set_item(py, "type", "comment");
                d.set_item(py, "text", match self.params {
                    Some(ref x) => {
                        x.get("txt").unwrap()
                    }
                    _ => ""
                });
                d.into_object()
            },
            NodeClass::Text => {
                let d = PyDict::new(py);
                d.set_item(py, "type", "text");
                d.set_item(py, "text", match self.params {
                    Some(ref x) => {
                        x.get("txt").unwrap()
                    }
                    _ => ""
                });
                d.into_object()
            }
        }

    }
}
