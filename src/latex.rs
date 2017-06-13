//! LaTeX parser

// use std::collections::HashSet;
// use std::collections::HashMap;
use std::convert::From;
use std::borrow::Cow;


pub struct Latex<'a> {
    src: Cow<'a, str>,
}



impl<'a> From<Cow<'a, str>> for Latex<'a> {
    fn from(src: Cow<'a, str>) -> Latex<'a> {
        Latex {
            src: src
        }
    }
}
