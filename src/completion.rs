use std::fs;
use std::path::Path;

extern crate regex;
use regex::{Regex};

pub fn get_complete(_path: String) -> Vec<String> {
    let re = Regex::new(r"(^.*)(/|)").unwrap();
    let target_dir = match re.find(&_path) {
        Some(dir) => if Path::new(&dir.as_str().to_string()).exists() 
        {
            dir.as_str()
        }else{
            "./"
        },
        None => "./"
    };
    let paths = fs::read_dir(target_dir.to_string()).unwrap();
    let mut complete: Vec<String> = Vec::new();
    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
    complete
}