//! This crate contains parser functions that can be common with other
//! parsers.

use std::collections::HashMap;
use std::str;
use std::str::from_utf8;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

// pub mod node;

// fn not_eol(chr:u8) -> bool {
//     chr != '\r' as u8 && chr == '\n' as u8
// }
pub fn space_but_not_eol(chr:u8) -> bool {
    chr == ' ' as u8 || chr == '\t' as u8
}
pub fn any_space(chr:u8) -> bool {
    chr == ' ' as u8 || chr == '\t' as u8 || chr == '\r' as u8 || chr == '\n' as u8
}
pub fn not_space(chr:u8) -> bool {!any_space(chr)}

// fn is_line_ending_or_comment(chr:char) -> bool {
//   chr == ';' || chr == '\n'
// }

// named!(alphanumeric<&str,&str>,         take_while_s!(is_alphanumeric));
// named!(not_line_ending<&str,&str>,      is_not_s!("\r\n"));
// named!(space_or_eol<&str,&str>, is_a_s!(" \t\r\n"));
// named!(space<&str,&str>, is_a_s!(" \t\r\n"));
// named!(space<&str,&str>,                take_while_s!(is_space));



/// URI scheme
///
/// Taken from here:
/// https://github.com/google/pulldown-cmark/blob/master/src/scanners.rs
named!(uri_scheme1, alt_complete!( tag!("aaas") | tag!("aaa") |
       tag!("about") | tag!("acap") | tag!("adiumxtra") | tag!("afp") |
       tag!("afs") | tag!("aim") | tag!("apt") | tag!("attachment") |
       tag!("aw") | tag!("beshare") | tag!("bitcoin") | tag!("bolo") |
       tag!("callto") | tag!("cap") | tag!("chrome") |
       tag!("chrome-extension") | tag!("cid") | tag!("coap") |
       tag!("com-eventbrite-attendee") | tag!("content") | tag!("crid")
       | tag!("cvs") | tag!("data") | tag!("dav") | tag!("dict") |
       tag!("dlna-playcontainer") | tag!("dlna-playsingle") |
       tag!("dns") | tag!("doi") | tag!("dtn") | tag!("dvb") |
       tag!("ed2k") | tag!("facetime") | tag!("feed") | tag!("file") |
       tag!("finger") | tag!("fish") | tag!("ftp") | tag!("geo") |
       tag!("gg") | tag!("git") | tag!("gizmoproject") ) );


named!(uri_scheme2, alt_complete!( tag!("go") | tag!("gopher") |
       tag!("gtalk") | tag!("h323") | tag!("hcp") | tag!("https") |
       tag!("http") | tag!("iax") | tag!("icap") | tag!("icon") |
       tag!("im") | tag!("imap") | tag!("info") | tag!("ipn") |
       tag!("ipp") | tag!("irc") | tag!("irc6") | tag!("ircs") |
       tag!("iris") | tag!("iris.beep") | tag!("iris.lwz") |
       tag!("iris.xpc") | tag!("iris.xpcs") | tag!("itms") | tag!("jar")
       | tag!("javascript") | tag!("jms") | tag!("keyparc") |
       tag!("lastfm") | tag!("ldap") | tag!("ldaps") | tag!("magnet") |
       tag!("mailto") | tag!("maps") | tag!("market") | tag!("message")
       | tag!("mid") | tag!("mms") | tag!("ms-help") | tag!("msnim") |
       tag!("msrp") | tag!("msrps") | tag!("mtqp") | tag!("mumble") |
       tag!("mupdate") | tag!("mvn") | tag!("news") | tag!("nfs") ) );


named!(uri_scheme3, alt_complete!( tag!("ni") | tag!("nih") |
       tag!("nntp") | tag!("notes") | tag!("oid") |
       tag!("opaquelocktoken") | tag!("palm") | tag!("paparazzi") |
       tag!("platform") | tag!("pop") | tag!("pres") | tag!("proxy") |
       tag!("psyc") | tag!("query") | tag!("res") | tag!("resource") |
       tag!("rmi") | tag!("rsync") | tag!("rtmp") | tag!("rtsp") |
       tag!("secondlife") | tag!("service") | tag!("session") |
       tag!("sftp") | tag!("sgn") | tag!("shttp") | tag!("sieve") |
       tag!("sip") | tag!("sips") | tag!("skype") | tag!("smb") |
       tag!("sms") | tag!("snmp") ) );

named!(uri_scheme4, alt_complete!( tag!("soap.beeps") |
       tag!("soap.beep") | tag!("soldat") | tag!("spotify") |
       tag!("ssh") | tag!("steam") | tag!("svn") | tag!("tag") |
       tag!("teamspeak") | tag!("telnet") | tag!("tel") | tag!("tftp") |
       tag!("things") | tag!("thismessage") | tag!("tip") |
       tag!("tn3270") | tag!("tv") | tag!("udp") | tag!("unreal") |
       tag!("urn") | tag!("ut2004") | tag!("vemmi") | tag!("ventrilo") |
       tag!("view-source") | tag!("webcal") | tag!("ws") | tag!("wss") |
       tag!("wtai") | tag!("wyciwyg") | tag!("xcon") |
       tag!("xcon-userid") | tag!("xfire") | tag!("xmlrpc.beep") |
       tag!("xmlrpc.beeps") | tag!("xmpp") | tag!("xri") | tag!("ymsgr")
       | tag!("z39.50r") | tag!("z39.50s") ) );

named!(pub uri_scheme,
       alt_complete!(
           uri_scheme1 |
           uri_scheme2 |
           uri_scheme3 |
           uri_scheme4
       )
);

/// Domain name
///
/// example.org
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
           do_parse!(
               key: map_res!(is_not!( " \r\n\t=" ), from_utf8) >>
                   tag!("=") >>
                   val: map_res!(is_not!( " \r\n\t&" ), from_utf8) >>
                   (key, val)
           )
               |
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


#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::{Done, Incomplete, Error};
    use std::collections::HashMap;
    use std::str::from_utf8;


    #[test]
    fn test_scheme() {
        let mut tests = HashMap::new();
        tests.insert("https", "https");
        for (input, expected) in &tests {
            match uri_scheme(input.as_bytes()) {
                Done(_, output) => {
                    assert_eq!(&from_utf8(&output).unwrap(), expected);
                },
                Incomplete(x) => panic!("incomplete: {:?}", x),
                Error(e) => panic!("error: {:?}", e),
            }
        }
    }


    #[test]
    fn test_domain() {
        let mut tests = HashMap::new();
        tests.insert("pashinin.com", "pashinin.com");
        tests.insert("тест.рф", "тест.рф");
        for (input, expected) in &tests {
            match domain_name(input.as_bytes()) {
                Done(_, output) => {
                    assert_eq!(&from_utf8(&output).unwrap(), expected);
                },
                Incomplete(x) => panic!("incomplete: {:?}", x),
                Error(e) => panic!("error: {:?}", e),
            }
        }
    }

}
