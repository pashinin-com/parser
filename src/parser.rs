// use nom::{IResult, space, alpha, alphanumeric, digit};
use nom::{IResult, eof};

// PyTuple, PyDict, ToPyObject, PythonObject
use cpython::{PyObject, PyResult, Python, PyString};
use std::string::String;
use generator::{ToHtml, Command, URL};
use std::collections::HashMap;

// http://stackoverflow.com/questions/40070972/how-to-get-the-output-for-several-sequential-nom-parsers-when-the-input-is-a-st
// This will be in Nom 2.0
// use nom::Offset;
//
// But not now yet.
// Need this to work with &str input data, not &[u8]
trait Offset {
    fn offset(&self, second: &Self) -> usize;
}
impl Offset for str {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.as_ptr();
        let snd = second.as_ptr();

        snd as usize - fst as usize
    }
}

// fn is_alphabetic(chr:char) -> bool {
//   (chr as u8 >= 0x41 && chr as u8 <= 0x5A) || (chr as u8 >= 0x61 && chr as u8 <= 0x7A)
// }

fn not_space(chr:char) -> bool {
  chr != ' ' && chr != '\t'
}

// fn is_line_ending_or_comment(chr:char) -> bool {
//   chr == ';' || chr == '\n'
// }

// named!(alphanumeric<&str,&str>,         take_while_s!(is_alphanumeric));
// named!(not_line_ending<&str,&str>,      is_not_s!("\r\n"));
// named!(space<&str,&str>,                take_while_s!(is_space));
named!(space_or_line_ending<&str,&str>, is_a_s!(" \r\n"));
// named!(space_or_end,
//        alt!(
//            eof     |
//            is_a_s!(" \r\n")
//        )
// );

fn right_bracket(c:char) -> bool {
  c == ']'
}

fn right_bracket_curly(c:char) -> bool {
    c == '}'
}
fn left_bracket_curly(c:char) -> bool {
    c == '{'
}
fn dot(c:char) -> bool {
    c == '.'
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



// Full URL
// named!(url <&str, Box<ToHtml> >,
//        chain!(proto: take_until_s!("://") ~
//               tag_s!("://") ~
//               rest: take_while_s!( not_space ) ~
//               eof?,
//            // ~
//                // name: take_till_s!(right_bracket) ~
//                // tag_s!("]")                 ~
//                // space_or_line_ending?       ,
//               ||
//               {
//                   // Box::new(Command{
//                   //     name: proto.to_string(),
//                   //     contents: proto.to_string(),
//                   // })
//                   Box::new(
//                       URL{
//                           proto: "proto", //.to_string(),
//                       // contents: rest.to_string(),
//                       }
//                   )
//               }
//        )
// );

/// http, https, ftp, ...
named!(url_proto <&str, &str>,
       alt_complete!(
           tag_s!("https") |
           tag_s!("http")  |
           tag_s!("ftp")
       )
);

/// Domain (example.org)
named!(domain_name <&str, &str>,
       recognize!(
           chain!(
               is_not_s!(".") ~
                   tag_s!(".")    ~
                   is_not_s!( "./ \r\n\t" ),
               || {}
           )
       )
);

// /// Host (host1.example.org)
// named!(domain_name <&str, &str>,
//        chain!(
//            many1!(not!(char!('.'))) ~
//            domain_name
//                ,
//            || {}
//        )
// );

fn url(input: &str) -> IResult<&str, URL> {
  // if input.len() < size {
  //   return IResult::Incomplete(Needed::Size(size));
  // }

    let (i1, res)     = try_parse!(
        input,
        chain!(
            // proto: take_until_s!("://") ~
            proto: url_proto        ~
                tag_s!("://")       ~
                domain: domain_name ~
                rest: is_not_s!( " \t\r\n" )
            // eof
                ,
            ||
            {
                URL{

                    proto: proto, //.to_string(),   &input[1..5]
                    rest: rest,
                    // contents: rest.to_string(),
                }
            }
        )
    );

    // let (i2, length) = try_parse!(i1, expr_opt!(size.checked_add(sz)));
    // let (i2, data)   = try_parse!(i1, take!(10));
    // return Done(i3, data);

    IResult::Done(
        &i1,
        res
    )
}


// named!(command <&str, &str>,
named!(command <&str, Box<ToHtml> >,
       chain!(
           tag_s!("\\")                 ~
               name: take_till_s!(left_bracket_curly) ~
               tag_s!("{")                 ~
               contents: take_till_s!(right_bracket_curly) ~
               tag_s!("}")       ,
           ||{
               // name
               Box::new(Command{
                   name: name.to_string(),
                   contents: contents.to_string(),
               })
               //.finalize()
               // Box::new(
               //     Command{text: name.to_string()}
               // )
           }

       )
);


// named!(
//     parse <&str, Vec<URL> >,
//     many0!(
//         alt_complete!(
//             // command |
//             url
//         )
//     )
//        // chain!(
//            // ||{
//            //     let v: Vec<Box<ToHtml>> = vec![];
//            //     v
//            //     // -> IResult<&[u8], Vec<Sequence>>
//            //     // name
//            //     // Command{
//            //     //     name: name.to_string(),
//            //     //     contents: contents.to_string(),
//            //     // }.finalize()
//            //     // Box::new(
//            //     //     Command{text: name.to_string()}
//            //     // )
//            // }
//        // )
// );




// pub fn parse(input: &str) -> Box<Vec<Box<ToHtml>>>{
//     let v: Vec<Box<ToHtml>> = vec![];
//     Box::new(v)
// }

#[test]
#[ignore]
fn check_complex() {
    let mut tests = HashMap::new();
    tests.insert("\\youtube{1}\\youtube{2}",
                 "My favorite book.");

    for (src, html) in &tests {
        // println!("{}: \"{}\"", book, review);
        match url(&src.to_string()) {
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
fn check_link() {
    let mut tests = HashMap::new();
    tests.insert("https://www.youtube.com/watch?v=g6ez7sbaiWc ",
                 "https://www.youtube.com/watch?v=g6ez7sbaiWc ");

    for (src, html) in &tests {
        // println!("{}: \"{}\"", book, review);
        match url(&src.to_string()) {
            IResult::Done(_, out) => {
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
                 "https://youtube.com/1");

    for (src, html) in &tests {
        // println!("{}: \"{}\"", book, review);
        match url(&src.to_string()) {
            IResult::Done(_, out) => {
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
