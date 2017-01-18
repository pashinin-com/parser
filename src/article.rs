//! ```
//! let mut p = Article::from("[[page 1 | text]]");
//! ```

use std::fmt;
use std::fmt::{Display, Formatter};
use std::borrow::Cow;
use std::collections::HashSet;
use std::collections::HashMap;
use nom::{eol, IResult};
use std::str::from_utf8;
use common::*;
use std::convert::From;

#[cfg(feature = "python")]
use cpython::{Python, PythonObject, PyObject, ToPyObject, PyTuple, PyString, PyResult, PyDict};


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
    Header(usize, Cow<'a, str>),
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
    Text(Cow<'a, str>),
    Comment,
    LinkInternal{page: Cow<'a, str>, text: Cow<'a, str>, link: Option<Cow<'a, str>>},

    /// `[[http://pashinin.com Title]]`
    ///
    /// Which will render as: [Title](http://pashinin.com)
    LinkExternal{
        url: Cow<'a, str>,
        text: Cow<'a, str>,
    },

    Space,
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
    type ObjectType = PyObject;

    #[inline]
    fn to_py_object(&self, py: Python) -> PyObject {
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
            _ => PyDict::new(py).into_object()
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
pub fn article_render<'a>(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyTuple> {
    let source = args.get_item(py, 0).to_string();
    let mut p = Article::from(source.as_bytes());
    if let Some(kwargs) = kwargs {
        p.set_info_from_pydict(py, kwargs);
    }

    Ok(PyTuple::new(py, &[
        PyString::new(py, &p.html).into_object(),
        p.py_info(py).into_object()
    ]))
}


/// Article parser
pub struct Article<'a> {
    src: &'a [u8],
    // root: Tag<'a>,
    pub html: Cow<'a, str>,
    pub links_internal: HashMap<Cow<'a, str>, Cow<'a, str>>,
    links_internal_missing: HashSet<String>,
}

impl<'a> Article<'a> {
    /// Get some information about current article.
    /// Return a PyDict like:
    /// {
    ///   "missing_links": ("page name 1", "page name 2", ...)
    /// }
    #[cfg(feature = "python")]
    pub fn py_info(&self, py: Python) -> PyDict {
        let info = PyDict::new(py);
        if !self.links_internal_missing.is_empty() {
            let v: Vec<PyObject> = self.links_internal_missing.iter()
                .map(|x| PyString::new(py, &x).into_object())
                .collect();
            let missing_links = PyTuple::new(py, v.as_slice());
            info.set_item(py, "missing_links", missing_links).unwrap();
        }
        info
    }

    /// Set some internal variables for rendering from PyDict
    /// Used as kwargs.
    ///
    /// article_parse("src", set_links={"page name 1": "/articles/1"})
    #[cfg(feature = "python")]
    pub fn set_info_from_pydict(&mut self, py: Python, dict: &PyDict) {
        // set internal links
        if let Some(set_links) = dict.get_item(py, "set_links") {
            let links = set_links.extract::<PyDict>(py).unwrap();
            for (page, url) in links.items(py) {
                let p: String = page.extract::<PyString>(py).unwrap().to_string(py).unwrap().into_owned();
                let u: String = url.extract::<PyString>(py).unwrap().to_string(py).unwrap().into_owned();
                self.links_internal.insert(Cow::from(p), Cow::from(u));
            }
            self.render();
        }
    }

    fn render(&mut self) {
        self.html = match parse(self.src) {
            IResult::Done(_, tag) => Cow::from(format!("{}", tag.to_html(self))),
            _ => Cow::from("")
        };
    }
}


impl<'a> From<&'a str> for Article<'a> {
    fn from(src: &'a str) -> Article<'a> {
        let mut a = Article {
            src: src.as_bytes(),
            html: Cow::from(""),
            links_internal: HashMap::new(),
            links_internal_missing: HashSet::new(),
        };
        a.render();
        a
    }
}
impl<'a> From<&'a [u8]> for Article<'a> {
    fn from(src: &'a [u8]) -> Article<'a> {
        let mut a = Article {
            src: src,
            html: Cow::from(""),
            links_internal: HashMap::new(),
            links_internal_missing: HashSet::new(),
        };
        a.render();
        a
    }
}




// #[derive(PartialEq,Eq,Debug)]
// enum State {
//   Beginning,
//   Middle,
//   End,
//   Done,
//   Error
// }


pub trait ToHtml {
    fn to_html(&self, tree: &mut Article) -> Cow<str>;
}

impl<'a> ToHtml for Vec<Tag<'a>> {
    fn to_html(&self, parser: &mut Article) -> Cow<str>
    {
        let parts: Vec<Cow<str>> = self.iter()
            .map(|&ref x| x.to_html(parser))
            .collect();
        Cow::from(parts.join(""))
    }
}

impl<'a> ToHtml for Tag<'a> {
    fn to_html(&self, parser: &mut Article) -> Cow<str>
    {
        match *self {
            Tag::Container{ref c} => c.to_html(parser),
            // {
            //     let children_strings: Vec<Cow<str>> = c.iter()
            //         .map(|&ref x| x.to_html(parser))
            //         .collect();
            //     let s = children_strings.join("");
            //     Cow::from(s)
            // }
            Tag::Paragraph{ref c} => {
                let children_strings: Vec<Cow<str>> = c.iter()
                    .map(|&ref x| x.to_html(parser))
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
                        match parser.links_internal.get(page) {
                            Some(l) => Cow::from(format!("<a href=\"{}\">{}</a>", l, text)),
                            None => {
                                parser.links_internal_missing.insert(String::from(page));
                                // parser.links_internal_missing.insert(Cow::from(page).to_owned());
                                Cow::from(format!("<a class=\"redlink\" href=\"/articles/{}\">{}</a>", page, text))
                            }
                        }
                        // link = &Some(Cow::from(page));
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
                                    Some(code) => Cow::from(format!(r#"<iframe width="560" height="315" src="https://www.youtube.com/embed/{}" frameborder="0" allowfullscreen></iframe>"#, code)),
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
            Tag::Header(ref level, ref text) => Cow::from(format!("<h{0}>{1}</h{}>", level, text)),


            Tag::ListUnnumbered{ref c} => {
                // let items: Vec<Cow<str>> = c.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();
                // let items: Vec<Tag> = c.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();
                // Cow::from(format!("<li>{}</li>", items.join("</li><li>")))
                // Cow::from(format!("<ul>{}</ul>", items.join("")))
                Cow::from(format!("<ul>{}</ul>", c.to_html(parser)))

                    // let parts: Vec<Cow<str>> = c.iter()
                //     .map(|&ref x| x.to_html(parser))
                //     .collect();

                // Cow::from(format!("<ul>{}</ul>", s))
            },
            // Tag::ListUnnumberedItem(ref txt) => Cow::from(format!("<li>{}</li>", txt)),
            Tag::ListUnnumberedItem(ref words) => {
                let parts: Vec<Cow<str>> = words.iter()
                    .map(|&ref x| x.to_html(parser))
                    .collect();
                Cow::from(format!("<li>{}</li>", parts.join(" ")))
            },

            Tag::Code(ref lng, ref code) => Cow::from(format!("<pre><code class=\"{}\">{}</code></pre>", lng, code)),
            Tag::Comment => Cow::from(""),
            Tag::Space => Cow::from(" "),
        }
    }
}





/// Paragraphs are separated with 2 new lines with any number of spaces
/// between, before and after them.
// named!(pub paragraph_separator,
//        complete!(
//            recognize!(
//                chain!(
//                    opt!(take_while!(space_but_not_eol)) ~
//                        eol ~
//                        opt!(take_while!(space_but_not_eol)) ~
//                        eol ~
//                        opt!(take_while!(any_space)),
//                    || {}
//                )
//            )
//        )
// );


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
       do_parse!(
           block: alt_complete!(
               internal_link |
               external_link |
               url |
               comment |
               symbols
           ) >>
           (block)
       )
);

named!(pub list_item_words<Tag>,
       do_parse!(
           words: many1!(list_item_word) >>
           (Tag::Container{c: words})
       )
);

named!(
    list_item_content<Vec<Tag>>,
    separated_list!(
        // word_separator,
        // take_while!(space_but_not_eol),
        space_not_eol,
        alt_complete!(
            list_item_words |
            list_item_word
        )
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

        opt!(space_not_eol) >>
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
    #[doc = "Unnumbered list: * item1 * item2

```
* item1
* item2
```"],
    pub list_unnumbered<Tag>,
    do_parse!(
        // items: separated_list!(space_max1_eol, list_unnumbered_item) >>
        // take_while!(any_space) >>
        // items: separated_list!(eol, map_res!(is_not!( "\r\n" ), from_utf8)) >>
        items: many1!(list_unnumbered_item) >>
        (Tag::ListUnnumbered{c: items})
    )
);


named_attr!(
    #[doc = "Header
`# h1`
`## h2`
`### h3`"],
    header<Tag>,
    do_parse!(
        level: many1!(tag!( "#" )) >>
        opt!(space_not_eol) >>
        txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        (Tag::Header(level.len(), Cow::from(txt)))
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

/// Main parser function
named!(pub parse<Tag>,
    do_parse!(
        opt!(take_while!(any_space)) >>
        pars: separated_list!(space_min_2eol, root_element) >>
        opt!(take_while!(any_space)) >>
        (Tag::Container{c:pars})
    )
);

named!(root_element<Tag>,
       do_parse!(
           block: alt_complete!(
               list_unnumbered |
               paragraph
           ) >>
           (block)
       )
);


named!(word_separator<Tag>,
       // recognize!(
           complete!(
               do_parse!(
                   opt!(take_while!(space_but_not_eol)) >>
                   opt!(eol) >>
                   opt!(take_while!(space_but_not_eol)) >>
                   (Tag::Space)
               )
           )
       // )
);


named!(symbols<Tag>,
       do_parse!(
           txt: map_res!(take_while1!(not_space), from_utf8) >>
           (Tag::Text(Cow::from(txt)))
       )
);

named!(
    word<Tag>,
    do_parse!(
        block: alt_complete!(
            // words |
            // many1!(word) => {|x| Tag::Comment} |
            code |
            header |
            internal_link |
            external_link |
            url |
            comment |
            // list_unnumbered |
            symbols
            // word_separator
        ) >>
        (block)
    )
);

// named!(words<Vec<Tag>>,
named!(words<Tag>,
       do_parse!(
           w: word >>
           words: many1!(word) >>
           // vec.extend([1, 2, 3].iter().cloned());
           ({
               let mut v = words;
               v.insert(0, w);
               // v
               Tag::Container{c: v}
               // Tag::Container{c: words}
           })
       )
);

// named!(p1<Tag>,
//        do_parse!(
//            words: many1!(word) >>
//            (Tag::Container{c: words})
//        )
// );
// named!(p2<Tag>,
//        do_parse!(
//            words: separated_list!(space_max1_eol, word) >>
//            (Tag::Container{c: words})
//        )
// );

// fold_many1!( tag!( "abcd" ), Vec::new(), |mut acc: Vec<_>, item| {
//      acc.push(item);
//      acc
//  })
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
        words: separated_list!(
            space_max1_eol,
            alt_complete!(
                words |     // if no space between for example link and text
                word
            )
            // word
        ) >>
           // words: many1!(word) >>

           // words: many1!(alt_complete!(
           //     p1 |
           //     p2
           // )) >>

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



#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::{Done, Incomplete, Error};
    use std::collections::HashMap;
    // use super::super::node::Tag;
    use std::str::from_utf8;
    use common::*;
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
    fn test_hostname() {
        let mut tests = HashMap::new();
        tests.insert(
            &b"host.pashinin.com"[..],
            Done(&b""[..], "host.pashinin.com".as_bytes())
        );
        tests.insert(
            &b"sub.www.youtube.com"[..],
            Done(&b""[..], "sub.www.youtube.com".as_bytes())
        );
        for (input, expected) in &tests {
            assert_eq!(hostname(input), *expected);
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

        // "link + text"
        // assert_eq!(
        //     paragraph(&b"[[page | title]]txt"[..]),
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

    // #[test]
    // fn test_par_separator() {
    //     let mut tests = HashMap::new();
    //     // tests.insert(&b"\r\n"[..], Done(&b""[..], &b"\r\n\r\n"[..]));
    //     for (input, expected) in &tests {assert_eq!(paragraph_separator(input), *expected);}
    // }

    // #[test]
    // fn test_word_separator() {
    //     let mut tests = HashMap::new();
    //     // tests.insert(&b"\n\n"[..], Error(error_position!(ErrorKind::ManyTill, &b"\n"[..])));
    //     tests.insert(&b"\n\n"[..], Done(&b"\n"[..], Tag::Space));
    //     tests.insert(&b"     \n "[..], Done(&b""[..], Tag::Space));
    //     // assert_eq!(multi(&c[..]), Error(error_position!(ErrorKind::ManyTill,&c[..])));
    //     // for (input, expected) in &tests {assert_eq!(word_separator(input), *expected);}
    //     for (input, expected) in &tests {assert_eq!(word_separator(input), *expected);}
    // }

    #[test]
    fn test_url_query_params() {
        let mut tests = HashMap::new();
        // key=value & key2=value2
        tests.insert(
            &b"gfe_rd=cr&ei=zCZLWNPMHceAuAH2-oCYDw&gws_rd=ssl#newwindow=1&q=url+query+string"[..],
            Done(&b""[..], vec![
                ("gfe_rd", "cr"),
                ("ei", "zCZLWNPMHceAuAH2-oCYDw"),
                ("gws_rd", "ssl#newwindow=1"),
                ("q", "url+query+string"),
            ])
        );

        // test a key without a value:  /path?param
        // param   -  ("param", "")
        // tests.insert(
        //     &b"key"[..],
        //     Done(&b""[..], vec![
        //         ("key", ""),
        //     ])
        // );

        // tests.insert(&b""[..], Done(&b""[..], vec![]));
        for (input, expected) in &tests {assert_eq!(url_query_params(input), *expected);}
    }


    #[test]
    fn test_url_query_params1() {
        // key=value
        // key
        let mut tests = HashMap::new();
        tests.insert(&b"key=value"[..], Done(&b""[..], ("key", "value")));
        tests.insert(&b"key"[..], Done(&b""[..], ("key", "")));
        // tests.insert(&b"key="[..], Incomplete(Needed::Size(4)));
        tests.insert(&b"key="[..], Done(&b""[..], ("key", "")));
        for (input, expected) in &tests {assert_eq!(url_query_params1(input), *expected);}
    }

    #[test]
    fn test_url_query() {
        // let mut tests = HashMap::new();
        // tests.insert(
        //     &b"?d=1"[..],
        //     Done(&b""[..], HashMap::new().insert("d", "1"))
        // );
        // tests.insert(&b""[..], Done(&b""[..], vec![]));
        // for (input, expected) in &tests {assert_eq!(url_query(input), *expected);}
    }

    #[test]
    fn test_parse() {
        let mut tests = HashMap::new();
        tests.insert("* 123 \n* asd ", "<ul><li>123</li><li>asd</li></ul>");
        tests.insert("1", "<p>1</p>",);
        tests.insert("1 2", "<p>1 2</p>",);
        tests.insert("1 \n\n 2", "<p>1</p><p>2</p>",);
        // for (input, expected) in &tests {assert_eq!(parse(input), *expected);}


        for (input, expected) in tests {
            let mut p = Article::from(input);
            // let r = match parse(input) {
            //     Done(_, tag) => {tag.to_html(&)},
            //     _ => "error".to_string()
            // };
            // assert_eq!(parse(input.as_bytes()), Done(&b""[..], Tag::Space));
            assert_eq!(p.html, *expected);
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::{Done, Incomplete, Error};
    use std::collections::HashMap;
    use std::str::from_utf8;
    use common::*;
    use std::borrow::Cow;

    #[test]
    fn parser() {
        let mut map = HashMap::new();
        map.insert(Cow::from("page 1 "), Cow::from(" text"));
        let p = Article::from("[[page 1 | text]]");
        // assert_eq!(p.links, map!(Cow::from("page 1 ") => Cow::from(" text")));
        // assert_eq!(p.links, map);
    }
}
