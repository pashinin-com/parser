//! Some parser functions

use nom::{eol};
use super::node::{Node};
use std::str::from_utf8;
use common::*;
use std::borrow::Cow;

/// Paragraphs are separated with 2 new lines with any number of spaces
/// between, before and after them.
named!(pub paragraph_separator,
       complete!(
           recognize!(
               chain!(
                   opt!(take_while!(space_but_not_eol)) ~
                       eol ~
                       opt!(take_while!(space_but_not_eol)) ~
                       eol ~
                       opt!(take_while!(any_space)),
                   || {}
               )
           )
       )
);


/// Comment
named!(comment<Node>,
       // recognize!(
           do_parse!(
               char!( '%' ) >>
                   opt!(take_while!(space_but_not_eol)) >>
                   txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
                   (Node::new_comment(txt.to_string()))
           )
       // )
);


/// List unnumbered
///
/// * item1
named!(pub list_unnumbered_item<Node>,
       do_parse!(
           opt!(eol) >>
               char!( '*' ) >>
               opt!(take_while!(space_but_not_eol)) >>
               txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
               opt!(take_while!(space_but_not_eol)) >>
               // opt!(eol) >>
               (Node::new_list_unnumbered_item(txt.to_string()))
       )
);

/// List
///
/// * item1
/// * item2
named!(pub list_unnumbered<Node>,
       do_parse!(
           // items: separated_list!(eol, list_unnumbered_item) >>
           items: many1!(list_unnumbered_item) >>
           (Node::new_list_unnumbered(items))
           //(items)     // Vec<&str>
       )
);


/// Header 2
named!(h2<Node>,
       do_parse!(
           tag!( "##" ) >>
               opt!(take_while!(space_but_not_eol)) >>
               txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
               (Node::new_h2(txt.to_string()))
       )
);


named!(code<Node>,
       do_parse!(
           tag!("```") >>
               language: map_res!(take_while!(not_space), from_utf8) >>
               txt: map_res!(take_until!("```"), from_utf8) >>
               tag!("```") >>
           // params: separated_list!(char!('&'), url_query_params1) >>
           (Node::new_code(txt.to_string(), language.to_string()))
               // (params.iter().fold(
               //         HashMap::new(),
               //         |mut T, tuple| {T.insert(tuple.0, tuple.1); T})
               // )
       )
);



/// URL parser
named!(pub url<Node>,
    do_parse!(
        proto: map_res!(uri_scheme, from_utf8)  >>
            tag!("://")   >>
            hostname: map_res!(hostname, from_utf8) >>
            // path: opt!(map_res!(is_not!( "? \t\r\n" ), from_utf8)) >>
            path: opt!(map_res!(is_not!( "? \t\r\n" ), from_utf8)) >>
            query: opt!(map_res!(recognize!(url_query), from_utf8)) >>
            (
                Node::new_url(
                    proto.to_string(), hostname.to_string(),
                    // path,
                    match path {
                        Some(x) => x.to_string(),
                        None => "".to_string(),
                    },
                    match query {
                        Some(x) => x.to_string(),
                        None => "".to_string(),
                    }
                    // query
                )
            )
       )
);

// pub fn parse(input: &[u8]) -> IResult<&[u8], Paragraph, u32> {
// named!(parse<&[u8], Paragraph>,


named!(root_element<Node>,
       do_parse!(
           block: alt_complete!(
               list_unnumbered |
               paragraph
               // list_unnumbered
           ) >>
               (block)
       )
);


/// Main parser function
named!(pub parse<Node>,
       do_parse!(
           opt!(take_while!(any_space)) >>
               pars: separated_list!(paragraph_separator, root_element) >>
               opt!(take_while!(any_space)) >>
           // (pars)
               (Node::new_root(pars))
       )
);

named!(symbols<Node>,
       do_parse!(
           txt: map_res!(take_while!(not_space), from_utf8) >>
               (Node::new_text(txt.to_string()))
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

/// Anything between spaces in 1 paragraph
named!(word<Node>,
       do_parse!(
           block: alt_complete!(
               code |
               h2 |
               url |
               comment |
               list_unnumbered |
               symbols
           ) >>
               (block)
       )
);


named!(pub word_separator,
       recognize!(
           complete!(
               do_parse!(
                   opt!(take_while!(space_but_not_eol)) >>
                   opt!(eol) >>
                   opt!(take_while!(space_but_not_eol)) >>
                       ()
               )
           )
       )
);


named!(pub paragraph<Node>,
       do_parse!(
           words: separated_list!(word_separator, word) >>
           // nl: opt!(paragraph_separator) >>
           // nl: opt!(take_while!(space_but_not_eol)) >>
               (
                   Node::new_paragraph(words)
               )
       )
);


#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::{Done, Incomplete, Error};
    use std::collections::HashMap;
    use super::super::node::{Node, NodeClass};
    use std::str::from_utf8;
    use common::*;
    // use std::str::from_utf8;



    #[test]
    fn test_list_unnumbered_item() {
        let mut tests = HashMap::new();
        let mut x = HashMap::new();
        x.insert("txt".to_string(), "asd".to_string());
        tests.insert(
            &b"* asd"[..],
            Done(&b""[..], Node{
                children: None,
                params: Some(x),
                class: NodeClass::ListUnnumberedItem,
            })
        );
        for (input, expected) in &tests {
            assert_eq!(list_unnumbered_item(input), *expected);
        }
    }

    #[test]
    fn test_list_unnumbered() {
        let mut tests = HashMap::new();
        let mut x = HashMap::new();
        x.insert("txt".to_string(), "asd".to_string());
        let mut x2 = HashMap::new();
        x2.insert("txt".to_string(), "123".to_string());
        tests.insert(
            &b"*asd\n*123"[..],
            Done(&b""[..], Node{
                children: Some(vec![
                    Node{
                        children: None,
                        class: NodeClass::ListUnnumberedItem,
                        params: Some(x),
                    },
                    Node{
                        children: None,
                        class: NodeClass::ListUnnumberedItem,
                        params: Some(x2),
                    }
                ]),
                params: None,
                class: NodeClass::ListUnnumbered,
            })

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
        // tests.insert(
        //     &b"host .pashinin.com"[..],
        //     Error(Position(ErrorKind::Many1, &b" .pashinin.com"[..]))
        // );
        for (input, expected) in &tests {
            assert_eq!(hostname(input), *expected);
        }
    }

    #[test]
    fn test_url() {
        let mut tests = HashMap::new();
        let mut x = HashMap::new();
        x.insert("proto".to_string(), "https".to_string());
        x.insert("hostname".to_string(), "www.youtube.com".to_string());
        x.insert("path".to_string(), "/watch".to_string());
        x.insert("query".to_string(), "?v=g6ez7sbaiWc".to_string());
        tests.insert(
            &b"https://www.youtube.com/watch?v=g6ez7sbaiWc"[..],
            // Done(&b""[..], URL{proto:"https", hostname:"host.pashinin.com", path:"",
            //                    query:None})
            Done(&b""[..], Node{
                class: NodeClass::URL,
                children: None,
                params: Some(x),
            })
        );
        // tests.insert(
        //     &b"ftp://pashinin.com"[..],
        //     Done(&b""[..], URL{proto:"ftp", hostname:"pashinin.com", path:"",
        //                        query:None})
        // );
        // tests.insert(
        //     &b"https://www.youtube.com/watch?v=g6ez7sbaiWc"[..],
        //     Done(&b""[..], URL{proto:"https", hostname:"www.youtube.com", path:"/watch",
        //                        query: Some("?v=g6ez7sbaiWc")})
        // );

        for (input, expected) in &tests {assert_eq!(url(input), *expected);}
    }

    #[test]
    fn test_paragraph() {
        let mut tests = HashMap::new();
        let mut x = HashMap::new();
        x.insert("proto".to_string(), "https".to_string());
        x.insert("hostname".to_string(), "host.pashinin.com".to_string());
        x.insert("path".to_string(), "".to_string());
        x.insert("query".to_string(), "".to_string());
        tests.insert(
            &b"https://host.pashinin.com"[..],
            Done(&b""[..], Node{
                children: Some(vec![
                    Node{
                        children: None,
                        class: NodeClass::URL,
                        params: Some(x),
                    }
                ]),
                class: NodeClass::Paragraph,
                params: None,
            })
        );

        // for (input, expected) in &tests {assert_eq!((input), *expected);}
        for (input, expected) in &tests {
            let res = paragraph(input);
            // let (x1, x2) = res.unwrap();
            assert_eq!(
                res,
                *expected
            );
        }
    }

    #[test]
    fn test_tags() {
        // let mut tests = HashMap::new();
        // tests.insert(&b"<img>"[..], Done(&b""[..], &b"img"[..]));
        // tests.insert(&b"\r\n\r\n"[..], Done(&b""[..], &b"\r\n\r\n"[..]));
        // tests.insert(&b"\r\n"[..], Done(&b""[..], &b"\r\n\r\n"[..]));
        // for (input, expected) in &tests {assert_eq!(tag(input), *expected);}
    }

    #[test]
    fn test_par_separator() {
        let mut tests = HashMap::new();
        tests.insert(&b"\n\n"[..], Done(&b""[..], &b"\n\n"[..]));
        tests.insert(&b"\r\n \r\n"[..], Done(&b""[..], &b"\r\n \r\n"[..]));
        tests.insert(&b"\r\n\r\n"[..], Done(&b""[..], &b"\r\n\r\n"[..]));
        // tests.insert(&b"\r\n"[..], Done(&b""[..], &b"\r\n\r\n"[..]));
        for (input, expected) in &tests {assert_eq!(paragraph_separator(input), *expected);}
    }

    #[test]
    fn test_word_separator() {
        let mut tests = HashMap::new();
        // tests.insert(&b"\n\n"[..], Error(error_position!(ErrorKind::ManyTill, &b"\n"[..])));
        tests.insert(&b"\n\n"[..], Done(&b"\n"[..], &b"\n"[..]));
        tests.insert(&b"     \n "[..], Done(&b""[..], &b"     \n "[..]));
        // assert_eq!(multi(&c[..]), Error(error_position!(ErrorKind::ManyTill,&c[..])));
        // for (input, expected) in &tests {assert_eq!(word_separator(input), *expected);}
        for (input, expected) in &tests {assert_eq!(word_separator(input), *expected);}
    }

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
        tests.insert(&b"* 123 \n* asd "[..], "<ul><li>123 </li><li>asd </li></ul>");
        // tests.insert(
        //     // &b"qwerty\n\nhttps://host.pashinin.com\n\n"[..],
        //     // &b" 123 \n\n asd "[..],
        //     // "<p>123</p><p>asd</p>",

        //     &b"* 123 \n* asd "[..],
        //     // Done(&b""[..], Node{
        //     //     children: None,
        //     //     params: None,
        //     //     class: NodeClass::ListUnnumberedItem,
        //     // })
        //     "* 123 \n* asd ",
        // );
        tests.insert(&b" 123 \n\n asd "[..], "<p>123</p><p>asd</p>",);
        // for (input, expected) in &tests {assert_eq!(parse(input), *expected);}

        for (input, expected) in &tests {
            let r = match parse(input) {
                Done(_, node) => {node.to_string()},
                _ => "error".to_string()
            };
            assert_eq!(r, *expected);
        }
    }
}
