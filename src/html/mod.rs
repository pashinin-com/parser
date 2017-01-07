use std::borrow::Cow;
use nom::IResult;
use std::fmt;
// use super::article::node::{Node, NodeClass};
use common::*;
use std::str::from_utf8;
use std::cell;

// #[cfg(feature = "python")]
// use cpython::{Python, ToPyObject, PyObject, PythonObject, PyTuple, PyString, PyResult, PyErr};


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tag<'a> {
    Comment(Cow<'a, str>),
}


// An HTML comment consists of "<!--" + text + "-->", where text does
// not start with ">" or "->", does not end with "-", and does not
// contain "--". See the HTML5 spec:
// https://www.w3.org/TR/html5/syntax.html#comments
/// Comment
named!(pub comment<Tag>,
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
    use nom::IResult::{Done, Incomplete, Error};
    use std::collections::HashMap;
    use std::str::from_utf8;
    use common::*;
    use std::borrow::Cow;

    #[test]
    fn html_comment() {
        let mut x = HashMap::new();
        x.insert(&b"<!-- html comment -->"[..],
                 Done(&b""[..], Tag::Comment(Cow::from(" html comment "))));
        for (input, expected) in &x {
            assert_eq!(comment(input), *expected);
        }
    }

}
