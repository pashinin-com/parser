// use nom::{IResult, space, alpha, alphanumeric, digit};
use nom::{IResult, eol, anychar, is_space, alpha};

// PyTuple, PyDict, ToPyObject, PythonObject
// use cpython::{PyObject, PyResult, Python, PyString, PyTuple};
// use std::string::String;
use generator::{URL};
use node::{Node, NodeClass};
use paragraph::{Paragraph};
// use generator::{ToHtml, Command, ParagraphElement, Node};
use std::collections::HashMap;
use nom::IResult::{Done, Incomplete, Error};
use std::str;
use std::str::from_utf8;
// use nom::{alpha, alphanumeric};
use nom::{ErrorKind, Needed};
// use nom::Err::Position;

fn not_eol(chr:u8) -> bool {
    chr != '\r' as u8 && chr == '\n' as u8
}
fn space_but_not_eol(chr:u8) -> bool {
    chr == ' ' as u8 || chr == '\t' as u8
}
fn any_space(chr:u8) -> bool {
    chr == ' ' as u8 || chr == '\t' as u8 || chr == '\r' as u8 || chr == '\n' as u8
}
fn not_space(chr:u8) -> bool {!any_space(chr)}

// fn is_line_ending_or_comment(chr:char) -> bool {
//   chr == ';' || chr == '\n'
// }

// named!(alphanumeric<&str,&str>,         take_while_s!(is_alphanumeric));
// named!(not_line_ending<&str,&str>,      is_not_s!("\r\n"));
// named!(space_or_eol<&str,&str>, is_a_s!(" \t\r\n"));
// named!(space<&str,&str>, is_a_s!(" \t\r\n"));
// named!(space<&str,&str>,                take_while_s!(is_space));


// ( &[u8] ) -> &[u8]

// fn EOL_min2(input: &[u8]) -> IResult<&[u8], &[u8]> {
//     if input.len() < 2 {
//         return Incomplete(Needed::Size(size));
//     }
//     IResult::Done(input, input)
// }


named!(get_any_space, take_while!(any_space));


named!(paragraph_separator,
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
                   (Node::new_comment(txt))
           )
       // )
);


/// Header 2
named!(h2<Node>,
       do_parse!(
           tag!( "##" ) >>
               opt!(take_while!(space_but_not_eol)) >>
               txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
               (Node::new_h2(txt))
       )
);


// named!(space_or_end,
//        alt!(
//            eof     |
//            is_a_s!(" \r\n")
//        )
// );

// fn right_bracket(c:char) -> bool {
//   c == ']'
// }

// fn right_bracket_curly(c:char) -> bool {
//     c == '}'
// }
// fn left_bracket_curly(c:char) -> bool {
//     c == '{'
// }
// fn dot(c:char) -> bool {
//     c == '.'
// }

/// Protocol (http, https, ftp, ...)
named!(url_proto,
       alt_complete!(
           tag!("https") |
           tag!("http")  |
           tag!("ftp")
       )
);


/// Domain name
///
/// example.org
// named!(domain_name <&str, &str>,
named!(domain_name,
       recognize!(
           chain!(
               is_not!( "./ \r\n\t" ) ~
                   tag!(".")    ~
                   is_not!( "./ \r\n\t" ),
               || {}
           )
       )
);

/// Url query part:
///
/// Examples:
///
/// key=value
/// key
///
/// Value can contain "=". Value ends on space or "&" sign
named!(url_query_params1<(&str, &str)>,
       alt_complete!(
           // key=value
           // complete!(
           do_parse!(
               key: map_res!(is_not!( " \r\n\t=" ), from_utf8) >>
                   tag!("=") >>
                   val: map_res!(is_not!( " \r\n\t&" ), from_utf8) >>
                   (key, val)
           )
       // )
               |
           // // key
           // complete!(
           do_parse!(
               key: map_res!(is_not!( " \r\n\t=" ), from_utf8) >>
                   (key, "")
           )
       )
);


named!(code<Node>,
       do_parse!(
           tag!("```") >>
               language: map_res!(take_while!(not_space), from_utf8) >>
               txt: map_res!(take_until!("```"), from_utf8) >>
               tag!("```") >>
           // params: separated_list!(char!('&'), url_query_params1) >>
           (Node::new_code(txt, language))
               // (params.iter().fold(
               //         HashMap::new(),
               //         |mut T, tuple| {T.insert(tuple.0, tuple.1); T})
               // )
       )
);


/// Url query params without first "?" sign
///
/// Returns: Vec<tuple>
///
/// Example input:
///
/// gfe_rd=cr&ei=zCZLWNPMHceAuAH2-oCYDw&gws_rd=ssl#newwindow=1&q=url+query+string
///
// HashMap<&'a str, &'a str>
named!(url_query_params<Vec<(&str, &str)> >,
// named!(url_query_params<HashMap<&str, &str> >,
       do_parse!(
           params: separated_list!(char!('&'), url_query_params1) >>
           (params)
               // (params.iter().fold(
               //         HashMap::new(),
               //         |mut T, tuple| {T.insert(tuple.0, tuple.1); T})
               // )
       )
);


named!(pub url_query<HashMap<&str, &str> >,
       complete!(
           do_parse!(
               tag!("?") >>
                   params: separated_list!(tag!("&"), url_query_params1) >>
               // (params)
                   (params.iter().fold(
                       HashMap::new(),
                       |mut T, tuple| {T.insert(tuple.0, tuple.1); T})
                   )
           )
       )
);


#[test]
fn test_domain() {
    let mut tests = HashMap::new();
    tests.insert("pashinin.com", "pashinin.com");
    tests.insert("тест.рф", "тест.рф");
    // .as_bytes()
    for (input, expected) in &tests {
        // let i = input.as_bytes();
        match domain_name(input.as_bytes()) {
            Done(_, output) => {
                // assert_eq!(from_utf8(&output).unwrap(), from_utf8(expected).unwrap());
                assert_eq!(&from_utf8(&output).unwrap(), expected);
            },
            Incomplete(x) => panic!("incomplete: {:?}", x),
            Error(e) => panic!("error: {:?}", e),
        }
    }
}

/// Host name
///
/// host1.example.org
/// sub.host1.example.org
named!(hostname,
       recognize!(
           chain!(
               // many1!(
               // not!(char!('.'))
               // is_not_s!( "./ \r\n\t" ) ~
               // recognize!(
               is_not!(". /\r\n\t") ~
                   many1!(
                       recognize!(
                       chain!(
                           tag!(".") ~
                               is_not!(". /\r\n\t"),
                           || {}
                       )
                       )
                   )
                   // domain_name
                   ,
               || {}
           )
       )
);


/// URL parser
named!(url<Node>,
    do_parse!(
        proto: map_res!(url_proto, from_utf8)  >>
            tag!("://")   >>
            hostname: map_res!(hostname, from_utf8) >>
            // path: opt!(map_res!(is_not!( "? \t\r\n" ), from_utf8)) >>
            path: opt!(map_res!(is_not!( "? \t\r\n" ), from_utf8)) >>
            query: opt!(map_res!(recognize!(url_query), from_utf8)) >>
            (
                Node::new_url(
                    proto, hostname,
                    // path,
                    match path {
                        Some(x) => x,
                        None => "",
                    },
                    match query {
                        Some(x) => x,
                        None => "",
                    }
                    // query
                )
            )
       )
);

// pub fn parse(input: &[u8]) -> IResult<&[u8], Paragraph, u32> {
// named!(parse<&[u8], Paragraph>,


/// Main parser function
named!(pub parse<Node>,
       do_parse!(
           opt!(take_while!(any_space)) >>
               pars: separated_list!(paragraph_separator, paragraph) >>
               opt!(take_while!(any_space)) >>
           // (pars)
               (Node::new_root(pars))
       )
);

named!(symbols<Node>,
       do_parse!(
           txt: map_res!(take_while!(not_space), from_utf8) >>
               (Node::new_text(txt))
       )
);

named!(tag,
       // delimited!(char!('<'), alpha, char!('>'))
       recognize!(
       do_parse!(
           char!('<') >>
               opt!(take_while!(any_space)) >>
               name: map_res!(take_while!(not_space), from_utf8) >>
               params: map_res!(is_not!( ">" ), from_utf8) >>
               opt!(char!('/')) >>
               char!('>') >>
               ()
       ))
);

named!(closing_tag, delimited!(tag!("</"), alpha, char!('>')));

/// Anything between spaces in 1 paragraph
named!(word<Node>,
       do_parse!(
           block: alt_complete!(
               code |
               h2 |
               url |
               comment |
               symbols
           ) >>
               (block)
       )
);


named!(word_separator,
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


named!(paragraph<Node>,
       do_parse!(
           words: separated_list!(word_separator, word) >>
           // nl: opt!(paragraph_separator) >>
           // nl: opt!(take_while!(space_but_not_eol)) >>
               (
                   Node::new_paragraph(words)
               )
       )
);



//
// Tests
//

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
    x.insert("proto", "https");
    x.insert("hostname", "www.youtube.com");
    x.insert("path", "/watch");
    x.insert("query", "?v=g6ez7sbaiWc");
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
    x.insert("proto", "https");
    x.insert("hostname", "host.pashinin.com");
    x.insert("path", "");
    x.insert("query", "");
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

// url_query


#[test]
fn test_parse() {
    let mut tests = HashMap::new();
    tests.insert(
        // &b"qwerty\n\nhttps://host.pashinin.com\n\n"[..],
        &b" 123 \n\n asd "[..],
        "<p>123</p><p>asd</p>",
    );
    for (input, expected) in &tests {
        let r = match parse(input) {
            Done(_, node) => {
                // println!("i: {} | o: {:?}", i, o);
                // return Ok(PyString::new(py, &o));
                node.to_string()
            },
            _ => "".to_string()
        };
        assert_eq!(r, *expected);
    }
}
