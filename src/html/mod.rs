//! See https://docs.rs/url/1.2.4/url/

use std::borrow::Cow;
use std::str::from_utf8;
use nom::{alpha, space};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HTMLTag<'a> {
    pub name: Cow<'a, str>,
    pub closing: bool
    // tags:    Vec<Tag<'a>>,
    // state:   State,
    // c_state: ConsumerState<usize,(),Move>,
    // counter: usize,
    // line: usize,
    // col: usize
}

impl<'a> Display for HTMLTag<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // match *self {
        //     Tag::Container{..} => write!(f, "{}", "<Root>"),
        //     Tag::Paragraph{..} => write!(f, "{}", "<Paragraph>"),
        //     Tag::LinkExternal{ref url, ..} => write!(f, "{}", url),
        //     Tag::URL{ref proto, ref hostname, ref path, ref query} =>
        //         write!(f, "{}://{}{}{}", proto, hostname, path, query),
        //     // Tag::Text(ref txt) => write!(f, txt),
        //     _ => write!(f, "{}", "unknown node"),
        // }
        write!(f, "<{}>", self.name)
    }
}

/// HTML tag
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tag<'a> {
    /// `<!-- comment -->`
    Comment(Cow<'a, str>),
}


// macro_rules! html_tag {
//     // () => (fn x() { });
//     ($x:expr) => { fn html_tag_$x() {
//         tag_no_case!(stringify!($x))
//         // recognize!(
//         // do_parse!(
//         //     tag!( "<" ) >>
//         //     txt: tag_no_case!(stringify!($x)) >>
//         //     tag!( ">" ) >>
//         //     (1)
//         // );
//     // )
//     }};
// }


// named!(ff,
//        html_tag!(Tag)
// );

named_attr!(
    #[doc = "HTML tag `<tagname[ attributes][/]>`"],
    pub tag<HTMLTag>,
    alt_complete!(

        // simple tags like: <br> <p> <html> <body>
        do_parse!(
            tag!( "<" ) >>
            // closing: opt!(tag!( "/" )) >>
                name: map_res!(alpha, from_utf8) >>
                opt!(space) >>
                tag!( ">" ) >>
                (HTMLTag{
                    name: Cow::from(name),
                    closing: false
                })
        ) |

        // closing tag: </script> </p>
        do_parse!(
            tag!( "</" ) >>
                name: map_res!(alpha, from_utf8) >>
                tag!( ">" ) >>
                (HTMLTag{
                    name: Cow::from(name),
                    closing: true
                })
        ) |

        // self-closing tag: <br />
        do_parse!(
            tag!( "<" ) >>
                name: map_res!(alpha, from_utf8) >>
                opt!(space) >>
                tag!("/>") >>
                (HTMLTag{
                    name: Cow::from(name),
                    closing: false
                })
        )

            )
);

named_attr!(
    #[doc = "HTML comment

Consists of `<!--` + `text` + `-->`, where text does not start with `>`
or `->`, does not end with `-`, and does not contain `--`. See the
[HTML5 spec](https://www.w3.org/TR/html5/syntax.html#comments)

"],
    pub comment<Tag>,
    do_parse!(
        tag!( "<!--" ) >>
        txt: map_res!(take_until!("-->"), from_utf8) >>
        tag!( "-->" ) >>
        (Tag::Comment(Cow::from(txt)))
    )
);


named_attr!(
    #[doc = "HTML `tr` tag

"],
    pub tr<Tag>,
    do_parse!(
        tag!( "<" ) >>
        txt: map_res!(take_until!("-->"), from_utf8) >>
        tag!( ">" ) >>
        (Tag::Comment(Cow::from(txt)))
    )
);

named_attr!(
    #[doc = "HTML table

"],
    pub table<Tag>,
    do_parse!(
        tag!( "<!--" ) >>
        txt: map_res!(take_until!("-->"), from_utf8) >>
        tag!( "-->" ) >>
        (Tag::Comment(Cow::from(txt)))
    )
);

// named!(tag,
//        // delimited!(char!('<'), alpha, char!('>'))
//        recognize!(
//        do_parse!(
//            char!('<') >>
//                opt!(take_while!(any_space)) >>
//                name: map_res!(take_while!(not_space), from_utf8) >>
//                params: map_res!(is_not!( ">" ), from_utf8) >>
//                opt!(char!('/')) >>
//                char!('>') >>
//                ()
//        ))
// );

// named!(closing_tag, delimited!(tag!("</"), alpha, char!('>')));


#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::{Done};
    use std::collections::HashMap;
    use std::borrow::Cow;

    #[test]
    fn html_comment() {
        let mut x = HashMap::new();
        x.insert(
            &b"<!-- html comment -->"[..],
            Done(&b""[..], Tag::Comment(Cow::from(" html comment ")))
        );
        for (input, expected) in &x {
            assert_eq!(comment(input), *expected);
        }
    }

    #[test]
    fn test_html_tag() {
        let mut x = HashMap::new();
        x.insert(
            &b"<html>"[..],
            Done(&b""[..], HTMLTag{
                name: Cow::from("html"),
                closing: false
            })
        );
        for (input, expected) in &x {
            assert_eq!(tag(input), *expected);
        }
    }

    // #[test]
    // #[ignore]
    // fn test_html_tag() {
    //     let mut x = HashMap::new();
    //     x.insert(
    //         &b"<html>"[..],
    //         Done(&b""[..], &b"html"[..])
    //     );
    //     for (input, expected) in &x {
    //         assert_eq!(html_tag!(input), *expected);
    //     }
    // }

}
