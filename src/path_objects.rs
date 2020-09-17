use std::fs;
extern crate regex;
use regex::{Regex};

#[derive(Clone)]
pub struct PathObject {
    pub _name: String,
    pub _is_file: bool,
}

impl PathObject {
    pub fn get_name(&self) -> &String {
        &self._name
    }
    pub fn is_file(&self) -> &bool {
        &self._is_file
    }
}

pub fn new(_target_dir: String, _target_name: String) -> Option<Vec<PathObject>> {
    match fs::read_dir(_target_dir) {
        Ok(objs) => {
            let mut _vec: Vec<PathObject> = Vec::new();
            for _entry in objs {
                if let Ok(__entry) = _entry {
                    if let Ok(_obj_name) = __entry.file_name().into_string() {
                        // ファイル候補のフィルタリング
                        let re = Regex::new(r"^\..*$").unwrap();
                        if let None = re.find(&_target_name) {
                            if let Some(_check) = re.find(&_obj_name) {
                                // ファイル名が"."で開始していない場合に
                                // 隠しファイルを対象外とする
                                continue;
                            }
                        }
                        // ファイル名が空白以外の場合のみ実行する
                        if &_target_name != "" {
                            // 表示候補のフィルタリング
                            let _reg_filter = format!(r"^{}.*$", &_target_name);
                            let re = Regex::new(&_reg_filter).unwrap();
                            if let None = re.find(&_obj_name) {
                                // 入力とファイル名が先頭一致していなければ
                                // continueする
                                continue;
                            }
                        }
                        if let Ok(_obj_type) = __entry.file_type() {
                            _vec.push(
                                PathObject{
                                    _name: if _obj_type.is_file() {
                                            _obj_name
                                        }else{
                                            format!("{}/", _obj_name).to_string()
                                        },
                                    _is_file: _obj_type.is_file(),
                                }
                            );
                        }
                    }
                }
            }
            if _vec.len() < 1 {
                return None;
            }
            if _vec.len() == 1 && !_vec[0].is_file() {
                let mut _clone = _vec[0].clone();
                _clone._name = format!("{}-", _vec[0].get_name()).to_string();
                _vec.push(_clone);
            }
            Some(_vec)
        },
        _ => None,
    }
}