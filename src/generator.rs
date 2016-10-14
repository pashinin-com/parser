// use nom::{IResult, space, alpha, alphanumeric, digit};
// use itertools::Itertools;
// PyTuple, PyDict, ToPyObject, PythonObject
// use cpython::{PyObject, PyResult, Python, PyString};
use std::string::String;
use std::fmt;

// #[derive(Debug)]
pub trait ToHtml {
    fn html(&self) -> String;
    // fn fmt(&self, &mut fmt::Formatter) -> Result<(), fmt::Error>;
    // fn fmt(&self, &mut fmt::Formatter) -> Result<(), fmt::Error>;
}

impl fmt::Display for Box<ToHtml> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.html())
    }
}
// impl PartialEq for Box<ToHtml> {
//     // fn eq(&self, other: &Box<ToHtml>) -> bool {
//     //     self.html() == other.html()
//     // }
//     fn eq(&self, other: &str) -> bool {
//         self.html() == other
//     }
// }
// impl Eq for Box<ToHtml> {
//     fn eq(&self, other: &Box<ToHtml>) -> bool {
//         self.html() == other.html()
//     }
//     // fn eq(&self, other: &str) -> bool {
//     //     self.html() == other
//     // }
// }



#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub contents: String,

    // parent: Box<Node>,
    // params: Vec<String>,
    // contents: Vec<Node>,
}


impl Command{
    pub fn finalize(&self) -> Box<ToHtml> {
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

impl ToHtml for Command {
    fn html(&self) -> String {
        self.finalize().html()
    }
}

impl ToHtml for Youtube {
    fn html(&self) -> String {
        // Box::new(
        let greetings = format!("https://youtube.com/{}", self.video_code);
        greetings.to_string()
        // )
    }
}

impl ToHtml for String {
    fn html(&self) -> String {
        // "Asd".to_string()
        (**self).to_string()
    }
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //     //write!(f, "({}, {})", self.x, self.y)
    //     write!(f, "123")
    // }
}

// impl ToHtml for Command {
//     fn html(&self) -> &String {
//         //&"Asd".to_string()
//         &self.text
//     }
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         //write!(f, "({}, {})", self.x, self.y)
//         write!(f, "123")
//     }
// }

// impl fmt::Display for Node {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }


impl ToHtml for Vec<Box<ToHtml> > {
    fn html(&self) -> String {
        self.iter().fold("".to_string(),
                      |mut i,j| {i.push_str(&*j.html()); i})     // &*j.html()
    }
}
impl ToHtml for Vec<Command> {
    fn html(&self) -> String {
        self.iter().fold("".to_string(),
                      |mut i,j| {i.push_str(&*j.html()); i})     // &*j.html()
    }
}
