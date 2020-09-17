extern crate regex;
use regex::{Regex};
use path_objects;

pub fn get_complete(_path: String) -> Option<Vec<String>> {
    // ディレクトリ部の取得
    let re = Regex::new(r"^.+/").unwrap();
    let _target_dir = match re.find(&_path) {
        Some(dir) => dir.as_str(),
        None => "./"
    }.to_string();
    // 入力中のオブジェクトの取得
    let re = Regex::new(r"[^/]*$").unwrap();
    let _target_obj =  match re.find(&_path) {
        Some(dir) => dir.as_str(),
        None => ""
    }.to_string();

    match path_objects::new(_target_dir, _target_obj) {
        Some(_list) => {
            let _comletion_list:Vec<String> = _list.iter().map(
                |_entry| _entry.get_name().to_string()
            ).collect();
            Some(_comletion_list)
        },
        _ => None,
    }
}