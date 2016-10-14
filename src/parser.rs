// use nom::{IResult, space, alpha, alphanumeric, digit};
use nom::{IResult};

// PyTuple, PyDict, ToPyObject, PythonObject
use cpython::{PyObject, PyResult, Python, PyString};
use std::string::String;
use generator::{ToHtml, Command};
use std::collections::HashMap;

// named!(name_parser<&[u8]>,
// // named!(name_parser<std::string::String>,
//        chain!(
//            tag!("hello") ~
//                space? ~
//                // name: map_res!(
//                name: map_res!(
//                    alpha
//                //     std::str::from_utf8
//                )
//                ,
//            || name
//        )
// );

// fn is_alphabetic(chr:char) -> bool {
//   (chr as u8 >= 0x41 && chr as u8 <= 0x5A) || (chr as u8 >= 0x61 && chr as u8 <= 0x7A)
// }

// fn is_digit(chr: char) -> bool {
//   chr as u8 >= 0x30 && chr as u8 <= 0x39
// }

// fn is_alphanumeric(chr: char) -> bool {
//   is_alphabetic(chr) || is_digit(chr)
// }

// fn is_space(chr:char) -> bool {
//   chr == ' ' || chr == '\t'
// }

// fn is_line_ending_or_comment(chr:char) -> bool {
//   chr == ';' || chr == '\n'
// }

// named!(alphanumeric<&str,&str>,         take_while_s!(is_alphanumeric));
// named!(not_line_ending<&str,&str>,      is_not_s!("\r\n"));
// named!(space<&str,&str>,                take_while_s!(is_space));
named!(space_or_line_ending<&str,&str>, is_a_s!(" \r\n"));

fn right_bracket(c:char) -> bool {
  c == ']'
}

fn right_bracket_curly(c:char) -> bool {
    c == '}'
}
fn left_bracket_curly(c:char) -> bool {
    c == '{'
}

named!(category <&str, &str>,
       chain!(
           tag_s!("[")                 ~
               name: take_till_s!(right_bracket) ~
               tag_s!("]")                 ~
               space_or_line_ending?       ,
           ||{ name }
       )
);


// named!(command <&str, &str>,
named!(command <&str, Command>,
       chain!(
           tag_s!("\\")                 ~
               name: take_till_s!(left_bracket_curly) ~
               tag_s!("{")                 ~
               contents: take_till_s!(right_bracket_curly) ~
               tag_s!("}")       ,
           ||{
               // name
               Command{
                   name: name.to_string(),
                   contents: contents.to_string(),
               }
               //.finalize()
               // Box::new(
               //     Command{text: name.to_string()}
               // )
           }

       )
);


named!(parse <&str, Vec<Command> >,
       // chain!(
           many0!(command)
           // ||{
           //     let v: Vec<Box<ToHtml>> = vec![];
           //     v
           //     // -> IResult<&[u8], Vec<Sequence>>
           //     // name
           //     // Command{
           //     //     name: name.to_string(),
           //     //     contents: contents.to_string(),
           //     // }.finalize()
           //     // Box::new(
           //     //     Command{text: name.to_string()}
           //     // )
           // }
       // )
);




// pub fn parse(input: &str) -> Box<Vec<Box<ToHtml>>>{
//     let v: Vec<Box<ToHtml>> = vec![];
//     Box::new(v)
// }

#[test]
fn check_complex() {
    let mut tests = HashMap::new();
    tests.insert("\\youtube{1}\\youtube{2}",
                 "My favorite book.");

    for (src, html) in &tests {
        // println!("{}: \"{}\"", book, review);
        match parse(&src.to_string()) {
            IResult::Done(_, out) => {
                // assert_eq!(out, Command{
                //     name: "youtube".to_string(),
                //     contents: "code123".to_string(),
                // }.finalize());
                assert_eq!(out.html(), html.to_string());
                // assert_eq!(in_, "");
            },
            IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
            IResult::Error(e) => panic!("error: {:?}", e),
        }
    }
}



#[test]
#[ignore]
fn check_command() {
    let mut tests = HashMap::new();
    tests.insert("\\youtube{1}",
                 "My favorite book.");

    for (src, html) in &tests {
        // println!("{}: \"{}\"", book, review);
        match parse(&src.to_string()) {
            IResult::Done(_, out) => {
                // assert_eq!(out, Command{
                //     name: "youtube".to_string(),
                //     contents: "code123".to_string(),
                // }.finalize());
                assert_eq!(out.html(), html.to_string());
                // assert_eq!(in_, "");
            },
            IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
            IResult::Error(e) => panic!("error: {:?}", e),
        }
    }
}

#[test]
#[ignore]
fn check_get_cell() {
    // let f = "hello age\ncarles,30\n";  // .to_string()
    // let g = b"age2,carles,30\n";

    let ini_file = "[category]
parameter=value
key = value2";

      let ini_without_category = "parameter=value
key = value2";

    let res = category(ini_file);
    println!("{:?}", res);
    match res {
        IResult::Done(i, o) => println!("i: {} | o: {:?}", i, o),
        _ => println!("error")
    }

    assert_eq!(res, IResult::Done(ini_without_category, "category"));

    // match name_parser(f.as_bytes()) {
    //     // name_parser
    //     // IResult::Done(_, out) => assert_eq!(out, b"age"),
    //     // IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
    //     // IResult::Error(e) => panic!("error: {:?}", e),
    //     IResult::Done(_, out) => {
    //         assert_eq!(out, b"age");
    //     },
    //     IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
    //     IResult::Error(e) => panic!("error: {:?}", e),
    // }

    // let f = b"age\ncarles,30\n";
    // let g = b"age2,carles,30\n";

    // match get_cell(f) {
    //     IResult::Done(_, out) => assert_eq!(out, b"age"),
    //     IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
    //     IResult::Error(e) => panic!("error: {:?}", e),
    // }
    // match get_cell(g) {
    //     IResult::Done(_, out) => assert_eq!(out, b"age2"),
    //     IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
    //     IResult::Error(e) => panic!("error: {:?}", e),
    // }
}
