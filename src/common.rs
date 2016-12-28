//! This crate contains parser functions that can be common with other
//! parsers.

use std::collections::HashMap;
use std::str;
use std::str::from_utf8;


/// Protocol (http, https, ftp, ...)
named!(pub url_proto,
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
named!(pub domain_name,
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
named!(pub url_query_params1<(&str, &str)>,
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

/// Url query params without first "?" sign
///
/// Returns: Vec<tuple>
///
/// Example input:
///
/// gfe_rd=cr&ei=zCZLWNPMHceAuAH2-oCYDw&gws_rd=ssl#newwindow=1&q=url+query+string
///
// HashMap<&'a str, &'a str>
// named!(url_query_params<HashMap<&str, &str> >,
named!(pub url_query_params<Vec<(&str, &str)> >,
       do_parse!(
           params: separated_list!(char!('&'), url_query_params1) >>
           (params)
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
                       |mut total, tuple| {total.insert(tuple.0, tuple.1); total})
                   )
           )
       )
);

/// Host name
///
/// host1.example.org
/// sub.host1.example.org
named!(pub hostname,
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
