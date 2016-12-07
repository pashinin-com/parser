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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeClass{
    Root,
    Paragraph,
    URL,
    Text,
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
                        let url = format!(
                            "{}://{}{}{}",
                            x.get("proto").unwrap(),
                            x.get("hostname").unwrap(),
                            x.get("path").unwrap(),
                            x.get("query").unwrap(),
                        );
                        write!(f, r#"<a href="{0}">{0}</a>"#, url)
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
