// use nom::{IResult, space, alpha, alphanumeric, digit};
// use itertools::Itertools;
// PyTuple, PyDict, ToPyObject, PythonObject
// use cpython::{PyObject, PyResult, Python, PyString};
// use std::concat;
use std::string::String;
use std::string::ToString;
use std::fmt;
// use cpython::{ToPyObject, PyTuple};
use std::fmt::{Debug, Display, Formatter};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::borrow::Cow;
// use parser::url_query;
use nom::{IResult};
use parser::{url_query};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeClass{
    Root,
    Paragraph,
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
            // html: "".into(),
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

            // Headers
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
