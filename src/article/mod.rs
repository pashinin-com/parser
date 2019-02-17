//! ```
//! let mut p = Article::from("[[page 1 | text]]");
//! ```
//!
//! asd111
//!

// #![cfg(feature = "stream")]
// extern crate elements;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::borrow::Cow;
use std::collections::HashSet;
use std::collections::HashMap;
use nom::{IResult};
// use nom::not_line_ending;
// use std::str::from_utf8;
use std::string::String;
use common::*;
// use super::elements::*;
// use html;
// use html::tag;
use std::convert::From;
use nom::{Consumer,ConsumerState,Move,Input,Producer,MemProducer};
use nom::Offset;

mod elements;
// use elements::*;
use self::elements::*;

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

#[derive(PartialEq,Eq,Debug)]
enum State {
    Beginning,
    // Middle,
    // End,
    Done,
    Error
}

struct TestConsumer<'a> {
    tags:    Vec<Token<'a>>,
    state:   State,
    c_state: ConsumerState<usize,(),Move>,
    counter: usize,
    line: usize,
    col: usize
}



// impl<'a> TestConsumer<'a> {

// }


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
                                if tag != Token::Space{
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



// impl<'a> Display for N<Token<'a>> {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         match self.data {
//             // Token::Root => write!(f, "Root {}", self.),
//             ref d => write!(f, "{}", d),
//         }
//     }
// }


impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Token::Container{..} => write!(f, "{}", "<Root>"),
            Token::Paragraph{..} => write!(f, "{}", "<Paragraph>"),
            Token::LinkExternal{ref url, ..} => write!(f, "{}", url),
            Token::URL{ref proto, ref hostname, ref path, ref query} =>
                write!(f, "{}://{}{}{}", proto, hostname, path, query),
            // Token::Text(ref txt) => write!(f, txt),
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
impl<'a> ToPyObject for Token<'a> {
    // type ObjectType = PyObject;

    #[inline]
    fn to_object(&self, py: Python) -> PyObject {
        match *self {
            // Token::Root{c} => {
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
            // Token::Paragraph(_) => {
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
            // Token::ListUnnumbered => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
            // Token::ListUnnumberedItem => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
            // Token::Header => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
            // Token::Code => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
            // Token::URL => {
            //     PyTuple::new(py, &vec![]).into_object()
            // },
        // Token::Comment => {
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
            // Token::Text => {
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
            // Token::LinkInternal => {
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



pub trait ToHtml {
    fn to_html(&self, tree: &mut Article) -> Cow<str>;
}

impl<'a> ToHtml for Vec<Token<'a>> {
    fn to_html(&self, article: &mut Article) -> Cow<str>
    {
        let parts: Vec<Cow<str>> = self.iter()
            .map(|&ref x| x.to_html(article))
            .collect();
        Cow::from(parts.join(""))
    }
}

impl<'a> ToHtml for Token<'a> {
    fn to_html(&self, article: &mut Article) -> Cow<str>
    {
        match *self {
            Token::Container{ref c} => c.to_html(article),
            // {
            //     let children_strings: Vec<Cow<str>> = c.iter()
            //         .map(|&ref x| x.to_html(parser))
            //         .collect();
            //     let s = children_strings.join("");
            //     Cow::from(s)
            // }

            Token::Command{ref name, ref contents} => {
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
                        // Examples:
                        //
                        //
                        // <div style="position: relative;padding-bottom: 56.25%; /* 16:9 */ padding-top: 25px;height: 0;"><iframe style="position: absolute;top: 0;left: 0;width: 100%;height: 100%;border:0px;" src="https://www.youtube.com/embed/HL-75xTzn6A"></iframe></div>
                        // Cow::from(format!(r#"<div style="position: relative;padding-bottom: 56.25%; /* 16:9 */ padding-top: 25px;height: 0;"><iframe style="position: absolute;top: 0;left: 0;width: 100%;height: 100%;border:0px;" src="https://www.youtube.com/embed/{}"></iframe></div>"#, contents))

                        // let locals = PyDict::new(article.py);
                        let res: String = match article.py.eval(
                            &format!("str({})", contents),
                            None,
                            Some(&article.context)
                        ) {
                            Ok(s) => format!("{}", s),

                            Err(_) => format!("Ошибка eval"),
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
            Token::Paragraph{ref c} => {
                let children_strings: Vec<Cow<str>> = c.iter()
                    .map(|&ref x| x.to_html(article))
                    .collect();
                let s = children_strings.join(" ");
                Cow::from(format!("<p>{}</p>", &s))
            }
            Token::LinkExternal{ref url, ref text} => {
                Cow::from(format!("<a class=\"external\" target=\"_blank\" href=\"{}\">{}</a>", url, text))
            }
            Token::LinkInternal{ref page, ref text, ref link} => {
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

            // render URLS
            Token::URL{ref proto, ref hostname, ref path, ref query} => {
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
                    "pashinin.com" => {
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
            Token::Text(ref txt) => txt.clone(),
            Token::MathInline(ref text) => Cow::from(format!("\\({0}\\)", text)),
            Token::MathWholeLine(ref text) => Cow::from(format!("\\[{0}\\]", text)),

            Token::Header(ref level, ref text) => Cow::from(format!("<h{0}>{1}</h{}>", level, text)),

            // Token::ListNumbered{ref c} => {
            //     Cow::from(format!("List NUMBERED:<br><ul>{}</ul>", c.to_html(parser)))
            // },
            Token::HTMLTag(ref tag) => {
                // let parts: Vec<Cow<str>> = items.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();
                if ALLOWED_HTML_TAGS.iter().any(|v| v == &tag.name) {
                    Cow::from(format!("{}", tag))
                } else {
                    Cow::from(format!("&lt;{}&gt;", tag.name))
                }
            },

            Token::ListNumbered(ref items) => {
                let parts: Vec<Cow<str>> = items.iter()
                    .map(|&ref x| x.to_html(article))
                    .collect();
                Cow::from(format!("<ol>{}</ol>", parts.join("")))
            },

            Token::ListUnnumbered{ref c} => {
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
            // Token::ListUnnumberedItem(ref txt) => Cow::from(format!("<li>{}</li>", txt)),
            Token::ListUnnumberedItem(ref words) => {
                let parts: Vec<Cow<str>> = words.iter()
                    .map(|&ref x| x.to_html(article))
                    .collect();
                Cow::from(format!("<li>{}</li>", parts.join(" ")))
            },
            Token::ListItemNumbered(ref words) => {
                let parts: Vec<Cow<str>> = words.iter()
                    .map(|&ref x| x.to_html(article))
                    .collect();
                Cow::from(format!("<li>{}</li>", parts.join(" ")))
            },

            Token::Code(ref lng, ref code) => Cow::from(format!("<pre><code class=\"{}\">{}</code></pre>", lng, code)),
            Token::SafeCode(ref code) => Cow::from(format!("{}", code)),
            Token::HTMLtable(ref code) => Cow::from(format!("<table>{}</table>", code)),

            // Token::SafeCode(ref code) => Cow::from(code),

            // css tabs
            // https://codepen.io/wallaceerick/pen/ojtal
            Token::CodeTabs(ref code_blocks) => {
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
            Token::Comment => Cow::from(""),
            Token::Space => Cow::from(" "),
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
    let mut article = Article::new(py);
    article.src = source.as_bytes();
    article.render();
    if let Some(kwargs) = kwargs {
        article.set_context(kwargs);
    }

    PyTuple::new(py, &[
        PyString::new(py, &article.html).into_object(py),
        article.get_article_info().into_object(py)
    ])
}

pub struct File<'a> {
    pub sha1: Cow<'a, str>,
    pub contenttype: Cow<'a, str>,
}


/// Article parser
///
/// some docs
pub struct Article<'a> {
    pub src: &'a [u8],
    // root: Tag<'a>,
    pub html: Cow<'a, str>,

    py: Python<'a>,

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
    pub fn new(py: Python<'a>) -> Self {
    // pub fn new(args: &PyTuple) -> Self {
        // let gil = Python::acquire_gil();
        // let py = gil.python();
        // let py = args.py();
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


    /// Returns PyDict containing some information about current article.
    ///
    /// Return example:
    ///
    /// ```
    /// {
    ///   "missing_links": ("page name 1", "page name 2", ...),
    ///   ...
    /// }
    /// ```
    ///
    /// # What is returned
    ///
    /// `missing_articles` - a tuple of articles that were not in
    /// context while rendering. Can be article title or an id. Ex.:
    ///
    /// `("Title 1", "Title 2", 42, ...)`
    ///
    #[cfg(feature = "python")]
    pub fn get_article_info(&self) -> PyObject {
    // pub fn get_article_info(&self, py: Python) -> PyObject {
        // pub fn get_article_info(&self, py: Python) -> PyObjectRef {
        let py = self.py;
        let info = PyDict::new(self.py);

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
        info.into_object(self.py)
    }

    /// Set some internal variables for rendering from PyDict
    /// Used as kwargs.
    ///
    /// article_parse("src", set_links={"page name 1": "/articles/1"})
    #[cfg(feature = "python")]
    pub fn set_context(&mut self, dict: &PyDict) {
        self.files_missing = HashSet::new();
        self.files_used = HashSet::new();
        self.links_internal_missing = HashSet::new();

        // set internal links
        if let Some(set_links) = dict.get_item("set_links") {
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

//             // context: HashMap::new(),
//             context: PyDict::new(py),
//                 // .into_object(py),

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






#[cfg(test)]
mod tests {
    use super::*;
    use super::elements::*;
    // use article::elements::list_item_content;
    // list_item_content
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
    //         Done(&b""[..], Token::ListUnnumberedItem(vec![Token::Text(Cow::from("asd"))]))
    //     );
    //     for (input, expected) in tests {
    //         assert_eq!(list_unnumbered_item(&input), expected);
    //     }
    // }



    #[test]
    fn test_render() {
        let mut tests = HashMap::new();
        // tests.insert("* 123 \n* asd ", "<ul><li>123</li><li>asd</li></ul>");
        tests.insert("1", "<p>1</p>");
        tests.insert("1 2", "<p>1 2</p>");

        tests.insert("  1  \n\n 2  ", "<p>1</p><p>2</p>");

        // This is not a header (a line starts from a space symbol)
        tests.insert(" # Header (no)", "<p># Header (no)</p>",);

        // Numbered lists
        tests.insert("#. 123", "<ol><li>123</li></ol>");
        tests.insert("#. 123\n#. asd", "<ol><li>123</li><li>asd</li></ol>");
        tests.insert("#. 123\n#. asd ", "<ol><li>123</li><li>asd</li></ol>");

        // for (input, expected) in &tests {assert_eq!(parse(input), *expected);}

        //
        // HTML tags
        //
        // Forbidden tags: <script> <iframe> and everything except
        // allowed tags
        tests.insert("<script >", "&lt;script&gt;");
        tests.insert("<table><tr><td>", "<table><tr><td>");
        // tests.insert("</td>", "</td>");
        // tests.insert("<td>123\n\n</td>", "<td><p>123</p></td>");
        // tests.insert("<i>italics</i>", "<table><tr><td>");


        // tests.insert("\\youtube{abc}", "command");


        // tests.insert("(http://domain.org/path)", "(http://domain.org/path)");


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
            let gil = Python::acquire_gil();
            let py = gil.python();
            let mut article = Article::new(py);
            article.src = input.as_bytes();
            article.render();
            // let a = Article::from(input);
            assert_eq!(article.html, *expected);
        }
    }

    #[test]
    // #[ignore]
    fn test_parse() {
        let mut tests = HashMap::new();

        tests.insert(
            &b"1"[..],
            Done(&b""[..], vec![
                    Token::Paragraph {
                        c: vec![Token::Text(Cow::from("1"))]
                    }
                ]
            )
        );

        // Numbered list with space after it
        tests.insert(
            &b"#. item1\n#. item2"[..],
            Done(&b""[..], vec![
                Token::ListNumbered(
                    vec![
                        Token::ListItemNumbered(vec![Token::Text(Cow::from("item1"))]),
                        Token::ListItemNumbered(vec![Token::Text(Cow::from("item2"))]),
                    ]
                )
            ])
        );


        for (input, expected) in &tests {assert_eq!(parse(input), *expected);}
    }
}



// #[cfg(test)]
// mod test {
//     use super::*;
//     // use nom::IResult::{Done, Incomplete, Error};
//     use std::collections::HashMap;
//     // use std::str::from_utf8;
//     // use common::*;
//     use std::borrow::Cow;

//     #[test]
//     fn parser() {
//         let mut map = HashMap::new();
//         map.insert(Cow::from("page 1 "), Cow::from(" text"));
//         // Article::from("[[page 1 | text]]");
//         // assert_eq!(p.links, map!(Cow::from("page 1 ") => Cow::from(" text")));
//         // assert_eq!(p.links, map);
//     }
// }
