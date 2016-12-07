// use nom::{IResult, space, alpha, alphanumeric, digit};
// use itertools::Itertools;
// PyTuple, PyDict, ToPyObject, PythonObject
// use cpython::{PyObject, PyResult, Python, PyString};
use std::string::String;
use std::fmt;
// use cpython::{ToPyObject, PyTuple};
use std::fmt::Debug;
use std::cmp::PartialEq;

// #[derive(Debug)]
pub trait Html {
    fn html(&self) -> String;
    // fn fmt(&self, &mut fmt::Formatter) -> Result<(), fmt::Error>;
    // fn fmt(&self, &mut fmt::Formatter) -> Result<(), fmt::Error>;
}


impl fmt::Display for Box<Html> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.html())
    }
}



#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub contents: String,

    // parent: Box<Node>,
    // params: Vec<String>,
    // contents: Vec<Node>,
}

impl Command{
    pub fn finalize(&self) -> Box<Html> {
        Box::new(Youtube{ video_code: self.contents.to_string() })
    }
}



// impl fmt::Debug for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
//     }
// }


#[derive(Debug)]
pub struct Youtube {
    pub video_code: String,
}

// struct Text {
//     text: String,
//     // parent: ToHTML,
// }

impl Html for Command {
    fn html(&self) -> String {
        self.finalize().html()
    }
}

impl Html for Youtube {
    fn html(&self) -> String {
        // Box::new(
        let greetings = format!("https://youtube.com/{}", self.video_code);
        greetings.to_string()
        // )
    }
}

impl Html for String {
    fn html(&self) -> String {
        (**self).to_string()
    }
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //     //write!(f, "({}, {})", self.x, self.y)
    //     write!(f, "123")
    // }
}


#[derive(PartialEq)]
#[derive(Debug)]
pub struct URL<'a> {
    pub proto: &'a str,
    pub hostname: &'a str,
    pub path: &'a str,
    pub query: Option<&'a str>,
    // pub proto: String,
    // pub contents: String,
}
impl<'a> Html for URL<'a> {
    fn html(&self) -> String {
        let a = format!(
            "<a href=\"{}://{}{}\">asd</a>",
            self.proto,
            self.hostname,
            self.path
        );
        a.to_string()
    }
}

impl Html for Vec<Box<Html> > {
    fn html(&self) -> String {
        self.iter().fold("".to_string(),
                      |mut i,j| {i.push_str(&*j.html()); i})     // &*j.html()
    }
}
impl Html for Vec<Command> {
    fn html(&self) -> String {
        self.iter().fold("".to_string(),
                      |mut i,j| {i.push_str(&*j.html()); i})     // &*j.html()
    }
}
