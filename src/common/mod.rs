//! Common parsing functions that can be used elsewhere.

pub mod tree;
use std::collections::HashMap;
use std::str;
use std::str::from_utf8;
use nom::{eol, not_line_ending};


pub fn space_but_not_eol(chr:u8) -> bool {
    chr == ' ' as u8 || chr == '\t' as u8
}
pub fn any_space(chr:u8) -> bool {
    chr == ' ' as u8 || chr == '\t' as u8 || chr == '\r' as u8 || chr == '\n' as u8
}
pub fn char_is_space(chr:u8) -> bool {
    chr == ' ' as u8 || chr == '\t' as u8 || chr == '\r' as u8 || chr == '\n' as u8
}
pub fn not_space(chr:u8) -> bool {!any_space(chr)}

// named!(multi<&[u8], Vec<&[u8]> >,
//    fold_many0!( tag!( "abcd" ), Vec::new(), |mut acc: Vec<_>, item| {
//      acc.push(item);
//      acc
//  }));

named!(pub count_eols<usize>,
       do_parse!(
           eols: many0!(complete!(
               do_parse!(
                   not_line_ending >>
                   eol >>
                   ()
               )
           )) >>
           (eols.len())
       )
);


named!(pub space_not_eol, take_while!(space_but_not_eol));


named!(pub space_max1eol,
       recognize!(
           complete!(
               do_parse!(
                   opt!(space_not_eol) >>
                   opt!(eol) >>
                   opt!(space_not_eol) >>
                   ()
               )
           )
       )
);


named!(pub space_min2eol,
       complete!(
           recognize!(
               do_parse!(
                   opt!(take_while!(space_but_not_eol)) >>
                   eol >>
                   opt!(take_while!(space_but_not_eol)) >>
                   eol >>
                   spaces: opt!(take_while!(char_is_space)) >>
                   ()
               )
           )
       )
);



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
           do_parse!(
               is_not!( "./ \r\n\t" ) >>
               tag!(".")    >>
               is_not!( "./ \r\n\t" ) >>
               ()
           )
       )
);

named_attr!(
    #[doc = "1 pair of parameters from URL query (`a=1`)

 Examples:

 key=value
 key

 Value can contain \"=\". Value ends on space or \"&\" sign.

"], pub url_query_params1<(&str, &str)>,
    alt_complete!(
        do_parse!(
            key: map_res!(is_not!( " \r\n\t=" ), from_utf8) >>
            tag!("=") >>
            val: map_res!(is_not!( " \r\n\t&" ), from_utf8) >>
            // opt!(eol_or_eof) >>
            (key, val)
        ) |
        do_parse!(
            key: map_res!(is_not!( " \r\n\t=" ), from_utf8) >>
            // key: map_res!(take_until_either!( eol_or_eof ), from_utf8) >>
            // opt!(eol_or_eof) >>
            (key, "")
        )
    )
);

named_attr!(
    #[doc = "Url query parameters without first \"?\" sign.

 Returns: `Vec<tuple>`

 Example input:

```text
gfe_rd=cr&ei=zCZLWNPMHceAuAH2-oCYDw&gws_rd=ssl#newwindow=1&q=url+query+string
```
"], pub url_query_params<Vec<(&str, &str)> >,
    do_parse!(
        params: separated_list!(
            complete!(char!('&')),
            complete!(url_query_params1)
        ) >>
        (params)
    )
);


named!(pub url_query<HashMap<&str, &str> >,
       complete!(
           do_parse!(
               tag!("?") >>
               params: url_query_params >>
               (params.iter().fold(
                   HashMap::new(),
                   |mut total, tuple| {total.insert(tuple.0, tuple.1); total})
               )
           )
       )
);

///
///
/// host1.example.org
/// sub.host1.example.org
named_attr!(
    #[doc = "Hostname (`host.example.org`).
"], pub hostname,
       recognize!(
           do_parse!(
               is_not!(". /\r\n\t") >>
               many1!(
                   recognize!(
                       do_parse!(
                           tag!(".") >>
                           is_not!(". /\r\n\t") >>
                           ()
                       )
                   )
               ) >>
               ()
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
    fn test_space_max1_eol() {
        let mut tests = HashMap::new();
        tests.insert(&b" \n \n "[..], Done(&b"\n "[..], &b" \n "[..]));
        for (input, expected) in &tests {
            assert_eq!(space_max1eol(input), *expected);
        }
    }

    #[test]
    fn test_space_min_2eol() {
        let mut tests = HashMap::new();
        tests.insert(&b" \n \n "[..], Done(&b""[..], &b" \n \n "[..]));
        tests.insert(&b"\n\n"[..], Done(&b""[..], &b"\n\n"[..]));
        tests.insert(&b"\r\n \r\n"[..], Done(&b""[..], &b"\r\n \r\n"[..]));
        tests.insert(&b"\r\n\r\n"[..], Done(&b""[..], &b"\r\n\r\n"[..]));
        for (input, expected) in &tests {
            assert_eq!(space_min2eol(input), *expected);
        }
    }


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

    #[test]
    fn test_hostname() {
        let mut tests = HashMap::new();
        tests.insert(
            "host.pashinin.com",
            Done(&b""[..], "host.pashinin.com".as_bytes())
        );
        tests.insert(
            "sub.www.youtube.com",
            Done(&b""[..], "sub.www.youtube.com".as_bytes())
        );
        tests.insert(
            "asd.тест.рф",
            Done(&b""[..], "asd.тест.рф".as_bytes())
        );
        for (input, expected) in &tests {
            assert_eq!(hostname(input.as_bytes()), *expected);
        }
    }

    // #[test]
    // fn test_url_query() {
    //     let mut tests = HashMap::new();
    //     // tests.insert(
    //     //     &b"?d=1"[..],
    //     //     Done(&b""[..], HashMap::new().insert("d", "1"))
    //     // );
    //     tests.insert(&b"d=1"[..], Done(&b""[..], vec![]));
    //     for (input, expected) in &tests {
    //         assert_eq!(url_query_params(input), *expected);
    //     }
    // }

    #[test]
    fn test_url_query_params() {
        let mut tests = HashMap::new();
        // key=value & key2=value2
        tests.insert(
            &b"a=1&b=2"[..],
            Done(&b""[..],
                 // ("a", "1")
                 vec![
                     ("a", "1"),
                     ("b", "2"),
                 ]
            )
        );
        // tests.insert(
        //     &b"gfe_rd=cr&ei=zCZLWNPMHceAuAH2-oCYDw&gws_rd=ssl#newwindow=1&q=url+query+string"[..],
        //     Done(&b""[..], vec![
        //         ("gfe_rd", "cr"),
        //         ("ei", "zCZLWNPMHceAuAH2-oCYDw"),
        //         ("gws_rd", "ssl#newwindow=1"),
        //         ("q", "url+query+string"),
        //     ])
        // );

        // test a key without a value:  /path?param
        // param   -  ("param", "")
        // tests.insert(
        //     &b"key"[..],
        //     Done(&b""[..], vec![
        //         ("key", ""),
        //     ])
        // );

        // tests.insert(&b""[..], Done(&b""[..], vec![]));
        for (input, expected) in &tests {
            assert_eq!(url_query_params(input), *expected);
        }
    }


    #[test]
    fn test_url_query_params1() {
        // key=value
        // key
        let mut tests = HashMap::new();
        tests.insert(&b"key=value"[..], Done(&b""[..], ("key", "value")));
        tests.insert(&b"b=2"[..], Done(&b""[..], ("b", "2")));
        tests.insert(&b"key"[..], Done(&b""[..], ("key", "")));
        // tests.insert(&b"key="[..], Incomplete(Needed::Size(4)));
        tests.insert(&b"key="[..], Done(&b""[..], ("key", "")));
        for (input, expected) in &tests {assert_eq!(url_query_params1(input), *expected);}
    }

}
