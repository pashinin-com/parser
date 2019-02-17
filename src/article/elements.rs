// use std::fmt;
// use std::fmt::{Display, Formatter};
use nom::IResult::Done;
use std::borrow::Cow;
// use std::collections::HashSet;
use std::collections::HashMap;
use nom::{eol};
use nom::{IResult};
// use nom::not_line_ending;
use std::str::from_utf8;
// use std::string::String;
use common::*;
// use super::elements::*;
use html;
use html::tag;
use std::convert::From;
// use nom::{Consumer,ConsumerState,Move,Input,Producer,MemProducer, hex_digit, alphanumeric};
use nom::{hex_digit, alphanumeric};
// use nom::Offset;


pub const ALLOWED_HTML_TAGS: &'static [&'static str] = &[
    "b",
    "div",
    "hr",
    "p",
    "table",
    "td",
    "tr",
];

#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token<'a> {
    Container {c: Vec<Token<'a>>},
    Paragraph {c: Vec<Token<'a>>},
    ListUnnumbered{c: Vec<Token<'a>>},
    // ListUnnumbered{c: Vec<Vec<Token<'a>>>},  // vector of list items, each item is a vector of Tokens
    ListUnnumberedItem(
        // Cow<'a, str>
        Vec<Token<'a>>  // contents of list item
    ),
    ListNumbered(Vec<Token<'a>>),
    ListItemNumbered(Vec<Token<'a>>),
    Header(usize, Cow<'a, str>),

    CodeTabs(Vec<Token<'a>>),
    Code(
        Cow<'a, str>,   // language
        Cow<'a, str>    // source code
    ),
    SafeCode(Cow<'a, str>),
    HTMLtable(Cow<'a, str>),
    URL{
        proto: Cow<'a, str>,
        hostname: Cow<'a, str>,
        path: Cow<'a, str>,
        query: Cow<'a, str>,
    },

    // Latex command
    Command{
        name: Cow<'a, str>,
        contents: Cow<'a, str>,
    },

    Text(Cow<'a, str>),
    HTMLTag(html::HTMLTag<'a>),
    MathInline(Cow<'a, str>),
    MathWholeLine(Cow<'a, str>),
    Comment,
    LinkInternal{page: Cow<'a, str>, text: Cow<'a, str>, link: Option<Cow<'a, str>>},

    /// `[http://pashinin.com Title]`
    ///
    /// Which will render as: [Title](http://pashinin.com)
    LinkExternal{
        url: Cow<'a, str>,
        text: Cow<'a, str>,
    },

    Space,
    // Cut
}






/// Comment
named!(comment<Token>,
       do_parse!(
           char!( '%' ) >>
           opt!(take_while!(space_but_not_eol)) >>
           map_res!(is_not!( "\r\n" ), from_utf8) >>
           (Token::Comment)
       )
);


named!(list_item_word<Token>,
       alt_complete!(
           internal_link |
           external_link |
           url |
           comment |
           symbols
       )
);

named!(pub list_item_words<Token>,
       do_parse!(
           w: list_item_word >>
           words: many1!(list_item_word) >>
           ({
               let mut v = words;
               v.insert(0, w);
               Token::Container{c: v}
           })
       )
);

// named!(pub cut,
//        tag!(">---")
// );

named!(pub sha1, recognize!(many_m_n!(32, 32, hex_digit )));

named!(pub file_sha1,
       recognize!(many_m_n!(32, 32, hex_digit ))
);

named!(
    pub list_item_content<Vec<Token>>,
    do_parse!(
        // opt!(space_not_eol) >>
        content: separated_nonempty_list!(
            complete!(space_not_eol),
            complete!(
                alt_complete!(
                    list_item_words |
                    list_item_word
                )
            )
        ) >>
        opt!(space_not_eol) >>
        (content)
    )
);


named_attr!(
    #[doc = "A list item like one of these these:

* item 1
* item 2"],
    pub list_unnumbered_item<Token>,
    // pub list_unnumbered_item<Vec<Token>>,
    do_parse!(
        opt!(eol) >>
        char!( '*' ) >>
        opt!(space_not_eol) >>

        // txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        words: list_item_content >>
        // opt!(map_res!(is_not!( "\r\n" ), from_utf8)) >>

        // opt!(space_not_eol) >>
        // (Token::ListUnnumberedItem(Cow::from(txt)))
        // (Token::Text(Cow::from(txt)))
        ({
            // if (words.len() = 1 )
            Token::ListUnnumberedItem(words)
        })
        // (words)
    )
);

named_attr!(
    #[doc = "`\\youtube{...}` command"],
    pub youtube<Token>,
    do_parse!(
        char!( '\\' ) >>
        tag!("youtube") >>
        tag!("{") >>
        sha1: map_res!(alphanumeric, from_utf8) >>
        tag!("}") >>
        ({
            Token::Command{
                name: Cow::from("youtube"),
                contents: Cow::from(sha1),
            }
        })
    )
);

named_attr!(
    #[doc = "`LaTeX-style command: \\command{...}`"],
    pub command<Token>,
    do_parse!(
        char!( '\\' ) >>
        name: map_res!(alphanumeric, from_utf8) >>
        tag!("{") >>
        // sha1: map_res!(alphanumeric, from_utf8) >>
        contents: map_res!(take_until!("}"), from_utf8) >>
        tag!("}") >>
        ({
            Token::Command{
                name: Cow::from(name),
                contents: Cow::from(contents),
            }
        })
    )
);

named_attr!(
    #[doc = "Jinja set expression: `{% set variable1 = variable2 %}`"],
    pub expression<Token>,
    do_parse!(
        tag!("{{") >>
        contents: map_res!(take_until!("}}"), from_utf8) >>
        tag!("}}") >>
        ({
            Token::Command {
                name: Cow::from("expression"),
                contents: Cow::from(contents),
            }
        })
    )
);

named_attr!(
    #[doc = "Jinja set expression: `{% set variable1 = variable2 %}`"],
    pub jinjatag<Token>,
    do_parse!(
        tag!("{%") >>
        contents: map_res!(take_until!("%}"), from_utf8) >>
        tag!("%}") >>
        ({
            Token::Command {
                name: Cow::from("jinjatag"),
                contents: Cow::from(contents),
            }
        })
    )
);

named_attr!(
    #[doc = "Numbered list item: `#. Item 1`

An item from such list:

```text
#. item 1
#.#. item 1.1
#. item 2
```
"],
    pub list_numbered_item<Token>,
    // pub list_unnumbered_item<Vec<Token>>,

    do_parse!(
        many1!(tag!( "#." )) >>
        tag!( " " ) >>
        // opt!(space_not_eol) >>
        // txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        // words: map_res!(not_line_ending, from_utf8) >>
        words: list_item_content >>
        // opt!(map_res!(is_not!( "\r\n" ), from_utf8)) >>
        // opt!(space_not_eol) >>
        // opt!(not_line_ending) >>
        alt_complete!(
            eol |
            eof!()
        ) >>

        // (Token::ListUnnumberedItem(Cow::from(txt)))
        // (Token::Text(Cow::from(txt)))
        ({
            // if (words.len() = 1 )
            Token::ListItemNumbered(
                // vec![
                //     Token::Text(Cow::from(words))
                // ]
                words
            )
        })
        // (words)
    )
);


named_attr!(
    #[doc = "Numbered list

```text
#. item 1
#.#. item 1.1
#. item 2
```
"],
    pub list_numbered<Token>,
    do_parse!(
        // items: separated_nonempty_list!(complete!(tag!("")), complete!(list_numbered_item)) >>
        // items: separated_nonempty_list!(complete!(eol), complete!(list_numbered_item)) >>
        // items: separated_list!(complete!(space_max1eol), list_unnumbered_item) >>
        // items: separated_list!(space_max1eol, list_unnumbered_item) >>
        // take_while!(any_space) >>
        // items: separated_list!(eol, map_res!(is_not!( "\r\n" ), from_utf8)) >>
        items: many1!(list_numbered_item) >>
        (Token::ListNumbered(items))
    )
);


named_attr!(
    #[doc = "Unnumbered list

```
* item1
* item2
```"],
    pub list_unnumbered<Token>,
    do_parse!(
        // items: separated_list!(space_max1eol, list_unnumbered_item) >>
        // take_while!(any_space) >>
        // items: separated_list!(eol, map_res!(is_not!( "\r\n" ), from_utf8)) >>
        items: many1!(list_unnumbered_item) >>
        (Token::ListUnnumbered{c: items})
    )
);


named_attr!(
    #[doc = "Parse headers (Markdown style, ex.: `## Header 2`)

```text
# Header 1
## Header 2
### Header 3
#### Header 4
##### Header 5
###### Header 6
```"],
    pub header<Token>,
    do_parse!(
        level: many1!(tag!( "#" )) >>
        opt!(space_not_eol) >>
        txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        (Token::Header(level.len(), Cow::from(txt)))
    )
);


named_attr!(
    #[doc = "Parse inline math formula (Latex style, ex.: `\\( ... \\)`)

Not using Tex-style `$ ... $`. Easier to work with and parse Latex style formulas.

```text
\\(a+b\\)    inline formulas
\\[a+b\\]    separate line formulas
```
"],
    pub inline_formula<Token>,
    do_parse!(
        tag!( "\\(" ) >>
        // opt!(space_not_eol) >>
        txt: map_res!(take_until!("\\)"), from_utf8) >>
        tag!( "\\)" ) >>
        (Token::MathInline(Cow::from(txt)))
    )
);


named_attr!(
    #[doc = "Parse separate math formula (Latex style, ex.: `\\[ ... \\]`)

Not using Tex-style `$ ... $`. Easier to work with and parse Latex style formulas.

```text
\\(a+b\\)    inline formulas
\\[a+b\\]    separate line formulas
```
"],
    pub separate_formula<Token>,
    do_parse!(
        tag!( "\\[" ) >>
        opt!(space_not_eol) >>
        // txt: map_res!(is_not!( "\r\n" ), from_utf8) >>
        txt: map_res!(take_until!("\\]"), from_utf8) >>
        tag!( "\\]" ) >>
        (Token::MathWholeLine(Cow::from(txt)))
    )
);

named_attr!(
    #[doc = "Parse math formula (Latex style)

Not using Tex-style `$ ... $`. Easier to work with and parse Latex style formulas.

```text
\\(a+b\\)    inline formulas
\\[a+b\\]    separate line formulas
```
"],
    pub math_formula<Token>,
    alt_complete!(
        inline_formula |
        separate_formula
    )
);


named_attr!(
    #[doc = "Code block defined between 3 backticks (```)

```no_run

```
"],
    pub code<Token>,
    do_parse!(
        tag!("```") >>
        language: map_res!(take_while!(not_space), from_utf8) >>
        take_while!(any_space) >>
        code: map_res!(take_until!("```"), from_utf8) >>
        tag!("```") >>
        (Token::Code(
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


named_attr!(
    #[doc = "Safe code HTML inside `safe`"],
    pub html_safe<Token>,
    do_parse!(
        tag!("`safe`") >>
        // language: map_res!(take_while!(not_space), from_utf8) >>
        take_while!(any_space) >>
        code: map_res!(take_until!("`/safe`"), from_utf8) >>
        tag!("`/safe`") >>
        (Token::SafeCode(
            // match language {
            //     "" => Cow::from("bash"),
            //     _ => Cow::from(language)
            // },
            Cow::from(code))
        )
    )
);


named_attr!(
    #[doc = "HTML table"],
    pub html_table<Token>,
    do_parse!(
        tag!("<table>") >>
        // language: map_res!(take_while!(not_space), from_utf8) >>
        take_while!(any_space) >>
        code: map_res!(take_until!("</table>"), from_utf8) >>
        tag!("</table>") >>
        (Token::HTMLtable(
            // match language {
            //     "" => Cow::from("bash"),
            //     _ => Cow::from(language)
            // },
            Cow::from(code))
        )
    )
);


named_attr!(
    #[doc = "Several code blocks (as tabs)

Defined just as several code blocks without any blank lines between
them.

```

```
"],
    pub code_tabs<Token>,
    do_parse!(
        codes: separated_nonempty_list!(
            complete!(eol),
            complete!(
                code
            )
        ) >>
        (Token::CodeTabs(codes))
        // (params.iter().fold(
        //         HashMap::new(),
        //         |mut T, tuple| {T.insert(tuple.0, tuple.1); T})
        // )
    )
);


/// internal link
named!(internal_link1<Token>,
    do_parse!(
        tag!("[[") >>
        page: map_res!(take_until!("|"), from_utf8) >>
        tag!("|") >>
        text: map_res!(take_until!("]]"), from_utf8) >>
        tag!("]]") >>
        (Token::LinkInternal{page: Cow::from(page), text: Cow::from(text), link: None})
    )
);
named!(internal_link2<Token>,
    do_parse!(
        tag!("[[") >>
        page: map_res!(take_until!("]]"), from_utf8) >>
        tag!("]]") >>
        (Token::LinkInternal{page: Cow::from(page), text: Cow::from(page), link: None})
    )
);

named_attr!(
    #[doc = "Internal link like in Wikipedia: `[[Page name | Link title]]`"],
    pub internal_link<Token>,
    alt_complete!(
        internal_link1 |
        internal_link2
    )
);



named!(pub external_link<Token>,
    do_parse!(
        tag!("[") >>
        url: url >>
        tag!(" ") >>
        opt!(take_while!(any_space)) >>
        text: map_res!(take_until!("]"), from_utf8) >>
        tag!("]") >>
        (Token::LinkExternal {
            url: Cow::from(url.to_string()),
            text: Cow::from(text),
            // tag_after: tag_after,
        })
    )
);


/// URL parser
named!(pub url<Token>,
    do_parse!(
        proto: map_res!(uri_scheme, from_utf8)  >>
        tag!("://")   >>
        hostname: map_res!(hostname, from_utf8) >>
        path: opt!(map_res!(is_not!( "? \t\r\n" ), from_utf8)) >>
        query: opt!(map_res!(recognize!(url_query), from_utf8)) >>
        (
            (Token::URL{
                proto: Cow::from(proto),
                hostname: Cow::from(hostname),
                path: Cow::from(path.unwrap_or("")),
                query: Cow::from(query.unwrap_or("")),
            })
        )
    )
);

named_attr!(
    #[doc = "Main parser function"],
    pub parse<Vec<Token>>,
    do_parse!(
        opt!(take_while!(any_space)) >>
        pars: separated_list!(complete!(space_min2eol), complete!(root_element)) >>
        opt!(take_while!(any_space)) >>
        // (Token::Container{c:pars})
        (pars)
    )
);


named_attr!(
    #[doc = "Line-start elements (line position == 0)"],
    pub line_start_element<Token>,
    alt_complete!(
        // html_safe |
        html_table |
        code_tabs |
        complete!(list_numbered) |
        complete!(list_unnumbered) |
        html_tag |
        header |
        expression |
        command |
        complete!(paragraph) |
        space_tag
    )
);


named_attr!(
    #[doc = "Middle-line elements (line position > 0)"],
    pub element<Token>,
    alt_complete!(
        code_tabs |
        html_tag |
        expression |
        command |
        paragraph |
        space_tag
    )
);


named!(root_element<Token>,
       alt_complete!(
           // list_unnumbered |
           complete!(list_numbered) |
           // paragraphs |
           command |
           complete!(paragraph) |
           space_tag
       )
);

named!(space_tag<Token>,
       do_parse!(
           txt: take_while1!(any_space) >>
           (Token::Space)
       )
);

named_attr!(
    #[doc = "Anything but spaces and new lines"],
    symbols<Token>,
    do_parse!(
        txt: map_res!(take_while1!(not_space), from_utf8) >>
        (Token::Text(Cow::from(txt)))
    )
);

named!(html_tag<Token>,
       do_parse!(
           // t:tag!("asd") >>
           t: tag >>
           (Token::HTMLTag(t))
               // (Token::Space)
       )
);

named!(
    word<Token>,
    alt_complete!(
        // many1!(word) => {|x| Token::Comment} |
        internal_link |
        external_link |
        expression |
        url |
        comment |
        html_tag |
        symbols               // any text
    )
);

named!(words<Token>,
       do_parse!(
           w: word >>
           words: many1!(word) >>
           ({
               let mut v = words;
               v.insert(0, w);
               Token::Container{c: v}
           })
       )
);

named_attr!(
    #[doc = "Paragraph

 # Examples

 ```
 use rparser::article::parser::paragraph;
 let res = paragraph(&b\"1 2\"[..]);
 assert_eq!(res, Done(&b\"\"[..], Token::Space));
 ```
"],
    pub paragraph<Token>,
    do_parse!(
        words: separated_nonempty_list_complete!(
            space_max1eol,
            // word
            alt_complete!(
                words |     // 2 "words" with no space between them
                word        // single "word"
            )
        ) >>
        // opt!(space_min2eol) >>
           // words: many1!(word) >>
           // , Vec::new(), |mut acc: Vec<_>, item| {
           // ), Vec::new(), |mut acc: Vec<_>, item| {
           //     // match item {
           //     //     Token::Container{ref c} => {}
           //     //     _ => {}
           //     // }
           //     // println!("{}", item);
           //     for c in item.c {

           //     }
           //     acc.push(item);
           //     acc
           // )) >>
        (Token::Paragraph{c: words})
       )
);

named_attr!(
    #[doc = "Paragraphs"],
    pub paragraphs<Token>,
    do_parse!(
        pars: separated_nonempty_list!(
            complete!(space_min2eol),
            complete!(paragraph)
            // word
        ) >>
        (Token::Container{c: pars})
    )
);


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    #[ignore]
    fn test_list_unnumbered() {
        let mut tests = HashMap::new();
        tests.insert(
            // &b"*asd\n*123"[..],
            &b"*1 txt \n* 2"[..],
            Done(&b""[..],
                 // Token::ListUnnumbered{c: vec![
                 //     vec![Token::Text(Cow::from("asd"))],
                 //     vec![Token::Text(Cow::from("123"))]
                 // ]}
                 Token::ListUnnumbered{c: vec![
                     Token::ListUnnumberedItem(vec![Token::Text(Cow::from("asd"))]),
                     Token::ListUnnumberedItem(vec![Token::Text(Cow::from("123"))])
                 ]}
            )
        );
        for (input, expected) in &tests {
            assert_eq!(list_unnumbered(input), *expected);
        }
    }

    #[test]
    fn test_listitem_numbered() {
        let mut tests = HashMap::new();
        // tests.insert(
        //     &b"#. txt"[..],
        //     Done(&b""[..],
        //          Token::ListItemNumbered(
        //              vec![
        //                  Token::Container{
        //                      c: vec![Token::Text(Cow::from("txt"))]
        //                  }

        //              ]
        //          )
        //     )
        // );
        tests.insert(
            &b"#.#. txt\n"[..],
            Done(&b""[..],
                 Token::ListItemNumbered(
                     vec![
                         Token::Text(Cow::from("txt"))
                     ]
                 )
            )
        );
        for (input, expected) in &tests {
            assert_eq!(list_numbered_item(input), *expected);
        }
    }

    #[test]
    fn test_list_numbered() {
        let mut tests = HashMap::new();

        // List from 1 item
        tests.insert(
            &b"#. item1"[..],
            Done(&b""[..],
                 Token::ListNumbered(vec![
                     Token::ListItemNumbered(vec![Token::Text(Cow::from("item1"))]),
                 ])
            )
        );

        // List from 2 items
        tests.insert(
            &b"#. item1\n#. item2"[..],
            Done(&b""[..],
                 Token::ListNumbered(vec![
                     Token::ListItemNumbered(vec![Token::Text(Cow::from("item1"))]),
                     Token::ListItemNumbered(vec![Token::Text(Cow::from("item2"))])
                 ])
            )
        );
        for (input, expected) in &tests {
            assert_eq!(list_numbered(input), *expected);
        }
    }

    #[test]
    fn test_formula() {
        let mut tests = HashMap::new();
        tests.insert(
            &b"\\( a + b \\)"[..],
            Done(&b""[..], Token::MathInline(Cow::from(" a + b ")))
        );
        tests.insert(
            &b"\\[ a + b \\]"[..],
            Done(&b""[..], Token::MathWholeLine(Cow::from("a + b ")))
        );
        for (input, expected) in &tests {
            assert_eq!(math_formula(input), *expected);
        }
    }

    #[test]
    fn test_url() {
        let mut tests = HashMap::new();
        tests.insert(
            &b"https://www.youtube.com/watch?v=g6ez7sbaiWc"[..],
            Done(&b""[..], Token::URL {
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
            Done(&b""[..], Token::Paragraph{c: vec![
                Token::Text(Cow::from("1")),
                Token::Text(Cow::from("2"))
            ]})
        );

        // 2 elements without space - it is 1 paragraph:
        // assert_eq!(
        //     paragraph(&b"https://www.youtube.com/watch?v=g6ez7sbaiWc"[..]),
        //     Done(&b""[..], Token::Paragraph{c: vec![
        //         Token::Text(Cow::from("1")),
        //         Token::Text(Cow::from("2"))
        //     ]})
        // );

        // assert_eq!(
        //     paragraph(Cow::from("Так даже лучше. Если Вы находитесь в другом городе, тогда это вообще единственный вариант. Но и в Москве Вы сэкономите деньги и время, потомуasdasd что не надо будет их тратить на поездку по городу.").as_bytes()),
        //     // paragraph(Cow::from("abc\n").as_bytes()),
        //     Done(&b""[..], Token::Paragraph{c: vec![
        //         Token::Text(Cow::from("1")),
        //         // Token::Text(Cow::from("2"))
        //     ]})
        // );

        // no spaces between paragraph "words" - still 1 paragraph
        // assert_eq!(
        //     paragraph(&b"[[page1 | title1]][[page2 | title2]]"[..]),
        //     Done(&b""[..], Token::Paragraph{c: vec![
        //         Token::Text(Cow::from("1")),
        //         Token::Text(Cow::from("2"))
        //     ]})
        // );

        // tests.insert(
        //     &b" 1 2  "[..],  // dot after link
        //     Done(&b""[..], Token::Paragraph{
        //         c: vec![
        //             Token::Text(Cow::from("."))
        //         ]}
        //     ));
        // tests.insert(
        //     &b"[http://pashinin.com Text]."[..],  // dot after link
        //     Done(&b""[..], Token::Paragraph{
        //         c: vec![
        //             Token::LinkExternal{
        //                 url: Cow::from("http://pashinin.com"),
        //                 text: Cow::from("Text"),
        //             },
        //             Token::Text(Cow::from("."))
        //         ]}
        //     ));
        // tests.insert(
        //     &b"https://host.pashinin.com 2"[..],
        //     Done(&b""[..], Token::Paragraph{
        //         c: vec![
        //             Token::URL{
        //                 proto: Cow::from("https"),
        //                 hostname: Cow::from("host.pashinin.com"),
        //                 path: Cow::from(""),
        //                 query: Cow::from(""),
        //             },
        //             Token::Text(Cow::from("2"))
        //         ]},
        //     )
        // );

        // for (input, expected) in tests {
        //     assert_eq!(paragraph(&input), expected);
        // }
    }

    #[test]
    fn test_list_item_content(){
        let mut tests = HashMap::new();
        tests.insert("123 ", Done(&b""[..], vec![
            Token::Text(Cow::from("123"))
        ]));

        for (input, expected) in tests {
            assert_eq!(
                list_item_content(input.as_bytes()),
                expected);
        }
    }

    #[test]
    fn test_youtube(){
        let mut tests = HashMap::new();
        tests.insert("\\youtube{123abc}", Done(
            &b""[..],
            Token::Command{
                name: Cow::from("youtube"),
                contents: Cow::from("123abc")
            }
        ));

        for (input, expected) in tests {
            assert_eq!(
                youtube(input.as_bytes()),
                expected);
        }
    }

    #[test]
    fn test_expression(){
        let mut tests = HashMap::new();
        let s = "{{exp.total_seconds() / 60 / 60 / 24 / 365.25}}";
        tests.insert(s, Done(
            &b""[..],
            Token::Command{
                name: Cow::from("expression"),
                contents: Cow::from("exp.total_seconds() / 60 / 60 / 24 / 365.25")
            }
        ));
        for (input, expected) in tests {
            assert_eq!(expression(input.as_bytes()), expected);
        }


        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut a = Article::new(py);
        // a.py = Some(py);
        a.src = "{{ 'asd'+'111' }}".as_bytes();
        a.render();
        assert_eq!(a.html, "asd111");

        a.src = "{{ datetime.datetime.now() - datetime.datetime(2013, 1, 1) }}".as_bytes();
        a.render();
        // assert_eq!(a.html, "asd111");

        a.src = "{{ no_such_variable }}".as_bytes();
        a.render();
        assert_eq!(a.html, "Ошибка eval");
    }

    #[test]
    fn test_code_tabs(){
        let mut tests = HashMap::new();
        tests.insert(
            "```pascal\nvar x: integer;\n```\n```cpp\nint x=1;\n``` ",
            Done(&b" "[..],
                 Token::CodeTabs(vec![
                     Token::Code(Cow::from("pascal"), Cow::from("var x: integer;\n")),
                     Token::Code(Cow::from("cpp"), Cow::from("int x=1;\n"))
                 ])
        ));

        for (input, expected) in tests {
            assert_eq!(
                code_tabs(input.as_bytes()),
                expected);
        }
    }

    #[test]
    fn test_safe_code(){
        let mut tests = HashMap::new();
        tests.insert(
            "`safe`\nasd\n`/safe` ",
            Done(&b" "[..], Token::SafeCode(Cow::from("asd\n"))
        ));

        for (input, expected) in tests {
            assert_eq!(
                html_safe(input.as_bytes()),
                expected);
        }
    }


    #[test]
    fn test_htmltable(){
        let mut tests = HashMap::new();
        tests.insert(
            "<table><tr>\nasd\n</tr></table>",
            Done(&b""[..], Token::HTMLtable(Cow::from("<tr>\nasd\n</tr>"))
        ));

        for (input, expected) in tests {
            assert_eq!(
                html_table(input.as_bytes()),
                expected);
        }
    }
}
