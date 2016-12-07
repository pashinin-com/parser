use generator::*;
use std::borrow::Cow;

// <'a>
// pub struct URL<'a> {
// #[derive(PartialEq)]
// #[derive(Debug)]
#[derive(PartialEq,Eq,Debug)]
pub struct Paragraph<'a> {
// pub struct Paragraph {
    // pub elements: Vec<Box<Node<'a>+'a>>,
    // pub elements: Option<Vec<Box<Node>>>,
    // pub elements: Box<Vec<Box<Node>>>,
    // pub children: PyTuple,
    // pub name: &'a str,
    pub src: Cow<'a, str>,
}

// #[derive(PartialEq,Eq,Debug)]
// struct FileType<'a> {
//   major_brand:         &'a str,
//   major_brand_version: &'a [u8],
//   compatible_brands:   Vec<&'a str>
// }

impl<'a> Html for Paragraph<'a>{
    fn html(&self) -> String {
        "<p>".to_string()
        // self.src.to_string()
        // PyString::new(&self.src)
        // PyString::new(py, "art")
    }
}


// impl<'a> Node<'a> for Paragraph<'a> {
// impl<'a> Node for Paragraph<'a> {
//     fn children(&self) -> Option<Vec<Box<Node>>>{
//         None
//     }
// }
// impl<'a> PartialEq for Paragraph {
//     // fn eq(&self, other: &Box<Node>) -> bool {
//     fn eq(&self, other: &Paragraph) -> bool {
//         // self.children == other.children
//         // self == other
//         true
//     }
// }
// impl<'a> PartialEq for Paragraph<'a> {
// // impl PartialEq for Paragraph {
//     fn eq(&self, other: &Paragraph) -> bool {
//         if self.children.len() != other.children.len(){
//             return false
//         }
//         else{
//             for i in 0..self.children.len() {
//                 if &self.children[i] != &other.children[i] {
//                     return false
//                 }
//             }
//         }
//         true
//     }
// }
// struct Node<K,V> {
//     left: Option<Box<Node<K,V>>>,
//     right: Option<Box<Node<K,V>>>,
//     key: K,
//     value: V,
// }
// Vec<Box<Html> >
// impl ToPyObject for Paragraph{

// }
