#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(stable_features)]

#![feature(os)]

use std::iter;
use std::os;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;

trait SummaryParser {
    fn parse_summary<'a>(&self, _: &'a mut dyn Read, _: &'a Path) -> impl core::fmt::Debug + 'a;
}

impl SummaryParser for () {
    fn parse_summary<'a>(&self, _: &'a mut dyn Read, path: &'a Path) -> impl core::fmt::Debug + 'a {
        let components_count = path.components().count();
        let binding = "../".repeat(components_count - 1);
        let parent_path = Path::new(&binding);
        parent_path.to_owned()
    }
}

fn foo() {
    let cwd = env::current_dir().unwrap();
    let src = cwd.clone();
    let mut summary = File::open(&src.join("SUMMARY.md")).unwrap();
    let parser: () = ();
    let result = parser.parse_summary(&mut summary, &src);
    println!("{:?}", result);
}

fn main() {}