use std::ops::Index;
use std::path::Path;
use crate::http::stringops;
use std::fs::{self, File};
use std::io::{Write, Error};

pub fn delete(file: String) {
    if is_file_exists(file.to_string()) {
        let _ = fs::remove_file(file);
    }
}

pub fn get_filename_without_extension(file: String) -> String {
    let mut i = stringops::last_index_of(&file, "/".to_string(), true);
    let j = stringops::last_index_of(&file, "\\".to_string(), true);
    let c = stringops::count(&file) as i32;

    if j > i && j != -1 {
        i = j;
    }
    if i == -1 {
        let ii = stringops::last_index_of(&file, ".".to_string(), true);
        if ii > 0 {
            return stringops::substring(&file, 0, ii);
        }
        return file;
    }

    let result = stringops::substring(&file, i + 1, c - i - 1);

    let ii = stringops::last_index_of(&result, ".".to_string(), true);
    if ii > 0 {
        return stringops::substring(&result, 0, ii);
    }

    return result;
}

pub fn create_dir_all(dir: String) -> (bool, String) {
    let dirx = fs::create_dir_all(dir.to_string());
    match dirx {
        Ok(_) => { return (true, String::from("")); },
        Err(x) => {
            return (false, format!("{:?}", x));
        }
    }
}

pub fn create_file(file: &str) -> Result<File, Error> {
    let output;
    if let Ok(x) = Path::new(&file).try_exists() {
        let outputx;
        if x == false {
            outputx = File::create(&file);
        } else {
            _ = fs::remove_file(&file);
            outputx = File::create(&file);
        }
        match outputx {
            Ok(o) => {
                output = o;
            },
            Err(x) => { return Err(x); }
        }
    }
    else {
        _ = fs::remove_file(&file);
        let outputx = File::create(&file);
        
        match outputx {
            Ok(o) => {
                output = o;
            },
            Err(x) => { return Err(x); }
        }
    }

    return Ok(output);
}

pub fn write_all_text(filename: String, content: &String) {
    delete(filename.to_string());
    let file = create_file(filename.to_string().as_str());
    match file {
        Ok(mut x) => {
            let y = x.write_all(content.as_bytes());
            match y {
                Ok(_) => {},
                Err(x) => {
                    println!("Error, '{}' : {:?}", filename.to_string(), x);        
                }
            }
        },
        Err(x) => {
            println!("Error, create_file, '{}' : {:?}", filename.to_string(), x);
        }
    }
}

pub fn read_all_text(full_path: String) -> String {
    if is_file_exists(full_path.to_string()) == false { return String::from(""); }

    let p = fs::read(&full_path);
    match p {
        Ok(x) => {
            match String::from_utf8(x) {
                Ok(str) => { return  str; }
                Err(_) => { return String::from(""); }
            }
        },
        Err(_) => {
            return String::from("");
        }
    }
}

pub fn is_file_exists(full_path: String) -> bool {
    let sx = Path::new(full_path.as_str()).try_exists();
    let mut exist = false;
    match sx {
        Ok(s) => {
            if s {
                exist = true;
            }
        },
        Err(_s) => {}
    }
    return exist;
}