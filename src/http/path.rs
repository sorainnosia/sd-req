#![allow(unused)]
use std::error::Error;
use std::path::Path;
use crate::http::stringops;
use std::fs::{self, File};
use std::io::{Write, Read};
use urlencoding;

pub fn file_len(file_path: &String) -> u64 {
    let m = fs::metadata(file_path);
    match m {
        Ok(mm) => {
            if mm.is_file() {
                return mm.len();
            }
        },
        Err(_) => {}
    }
    return 0;
}

pub fn read_first_n_bytes(file_path: &str, num_bytes: usize) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file_path);
    match file {
        Ok(mut f) => {
            let mut buffer = vec![0; num_bytes];
            let bytes_read = f.read(&mut buffer);
            match bytes_read{
                Ok (b) => {
                    buffer.truncate(b);
                    return Ok(buffer);
                },
                Err(x) => {
                    return Err(x);
                }
            }
        }, 
        Err (x) => {
            return Err(x);
        }
    }
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "File open error"));
}

pub fn url_between(prefix: String, str: String) -> Vec<String> {
    let urls = stringops::between(&str, prefix.to_string(), "\"".to_string(), 0, true);

    let mut result = vec![];
    for i in 0..urls.len() {
        let mut u = urls[i].to_string();
        if u.contains("\"") {
            let j = stringops::index_of(&u, "\"".to_string(), 0, true);
            u = stringops::substring(&u, 0, j);
        }
        if u.contains("?") {
            let j = stringops::index_of(&u, "?".to_string(), 0, true);
            u = stringops::substring(&u, 0, j);
        }
        if u.contains("\\") {
            let j = stringops::last_index_of(&u, "\\".to_string(), true);
            u = stringops::substring(&u, 0, j);
        }
        result.push(format!("{}{}", prefix, u));
    }
    return result;
}

pub fn url_between_all(prefix: String, str: String) -> (Vec<String>, i32) {
    let urls = stringops::between(&str, prefix.to_string(), "\"".to_string(), 0, true);

    //println!("{:?}", urls.clone());
    let mut result = vec![];
    for i in 0..urls.len() {
        let mut uu =  urlencoding::decode(urls[i].as_str());
        let mut u = urls[i].to_string();
        match uu {
            Ok (x) => {
                u = x.to_string();
            }, 
            Err (x) => {}
        }
        if u.contains("\"") {
            let j = stringops::index_of(&u, "\"".to_string(), 0, true);
            u = stringops::substring(&u, 0, j);
        }
        if u.contains("&quot;") {
            let j = stringops::index_of(&u, "&quot;".to_string(), 0, true);
            u = stringops::substring(&u, 0, j);
        }
        match urlencoding::decode(u.as_str()) {
            Ok(x) => {
                // u = x.to_string();
            },
            Err(x) => {}
        }
        //println!("{}", u.to_string());
        result.push(format!("{}{}", prefix, u));
    }

    let mut last_index = -1;
    if result.len() > 0 {
        let find = result[result.len() - 1].to_string();
        last_index = stringops::last_index_of(&str, find.to_string(), true);
    }
    return (result, last_index);
}

pub fn url_inside_all(prefix: String, str: String) -> (Vec<String>, i32) {
    let mut i = 0;
    let c = stringops::count(&str);
    let pc = stringops::count(&prefix);

    let mut last_index = -1;
    let mut urls = vec![];
    while i < c && i != -1 {
        let s = stringops::index_of(&str, prefix.to_string(), i, true);
        if s != -1 {
            let end = stringops::index_of(&str, "\"".to_string(), s, true);
            if end != -1 {
                let str2 = stringops::substring(&str, i, end - i);
                let c2 = stringops::count(&str2);

                let start = stringops::last_index_of(&str2, "\"".to_string(), true);
                if start != -1 {
                    let url = stringops::substring(&str2, start + 1, c2 - (start + 1));
                    last_index = end + 1;

                    urls.push(url);
                }
            }
        } else {
            break;
        }
        i = s + pc;
    }
    
    return (urls, last_index);
}

pub fn delete(file: String) {
    if is_file_exists(file.to_string()) {
        let _ = fs::remove_file(file);
    }
}

pub fn get_url(prefix: String, str: String) -> String {
    let mut i = stringops::index_of(&str, prefix.to_string(), 0, true);
    let mut c = stringops::count(&str);
    if i == -1 {
        return String::from("");
    }

    let mut start = stringops::substring(&str, i, c - i);
    // c = stringops::count(&start);

    i = stringops::index_of(&start, "\"".to_string(), 0, true);
    if i != -1 {
        start = stringops::substring(&start, 0, i);
    }
    let _ = stringops::count(&start);

    i = stringops::index_of(&start, "?".to_string(), 0, true);
    if i != -1 {
        start = stringops::substring(&start, 0, i);
    }
    let _ = stringops::count(&start);

    return start;
}

pub fn get_extension(file: String) -> String {
    let mut i = stringops::last_index_of(&file, ".".to_string(), true);
    if i == -1 {
        return String::from("");
    }

    let ext = stringops::substring(&file, i, stringops::count(&file) - i);
    return ext;
}

pub fn get_filename(file: String) -> String {
    let mut i = stringops::last_index_of(&file, "/".to_string(), true);
    let j = stringops::last_index_of(&file, "\\".to_string(), true);
    let c = stringops::count(&file) as i32;

    if (j > i || i == -1) && j != -1 {
        i = j;
    }
    if i == c - 1 { return String::from(""); }
    if i == -1 {
        return file;
    }
    
    let result = stringops::substring(&file, i + 1, c - i - 1);
    return result;
}

pub fn remove_last_slash(mut url: String) -> String {
    while stringops::ends_with(&url, "/".to_string(), false) == true {
        let c = stringops::count(&url);
        url = stringops::substring(&url, 0, c - 1);
    }
    return url;
}

pub fn get_url_filename(mut file: String) -> String {
    file = remove_last_slash(file.to_string());
    if file.to_string() == String::from("") { return file.to_string(); }
    
    let mut i = stringops::last_index_of(&file, "/".to_string(), true);
    let j = stringops::last_index_of(&file, "\\".to_string(), true);
    let c = stringops::count(&file) as i32;

    if j > i && (j != -1 || i == -1) {
        i = j;
    }
    
    let mut result = stringops::substring(&file, i + 1, c - i - 1);
    let mut resultx = result.to_string();
    
    let mut ext = String::from("");
    
    let k2 = stringops::index_of(&result, "?".to_string(), 0, true);
    let mut k = -1;
    if k2 > 0 {
        let s = stringops::substring(&result, 0, k2);
        k = stringops::last_index_of(&s, ".".to_string(), true);
    }
    if k > 0 {
        if k2 != -1 && k2 > k {
            ext = stringops::substring(&result, k, k2 - k);
        } else  {
            let c2 = stringops::count(&result) as i32;
            ext = stringops::substring(&result, k, c2 - k);
        }

        resultx = stringops::substring(&result, 0, k);
        
        let mut tail  = String::from("");
        if k2 >= 0 {
            let cc = stringops::count(&result);
            tail = stringops::substring(&result, k2 + 1, cc - k2 - 1);
        }
        result = format!("{}&{}{}", resultx, tail, ext);
    }
    return result;
}

pub fn get_url_filename_compare(file: String, up: String, down: String) -> String {
    let file1 = get_url_filename(file.to_string());
    let up = get_url_filename(up.to_string());
    let down = get_url_filename(down.to_string());

    if up == String::from("") && down == String::from("") {
        return get_url_filename(file1);
    }
    
    let sps1: Vec<&str> = file1.split("&").collect();
    let sps2: Vec<&str> = up.split("&").collect();
    let sps3: Vec<&str> = down.split("&").collect();
    let mut up_success = false;
    let mut diff = vec![];
    if sps1.len() == sps2.len() {
        if sps1[0].to_string() == sps2[0].to_string() {
            if sps1.len() > 0 {
                let mut i = 1;
                while i < sps1.len() {
                    if sps1[i].to_string() != sps2[i].to_string() {
                        diff.push(sps1[i]);
                    }
                    i += 1;
                }    
                up_success = true;
            }
        }
    }
    if sps1.len() == sps3.len() && up_success == false {
        if sps1[0].to_string() == sps3[0].to_string() {
            if sps1.len() > 0 {
                let mut i = 1;
                while i < sps1.len() {
                    if sps1[i].to_string() != sps3[i].to_string() {
                        diff.push(sps1[i]);
                    }
                    i += 1;
                }    
            }
        }
    }
    let mut result = String::from(sps1[0].to_string());
    for j in 0..diff.len() {
        result = format!("{}&{}", result, diff[j]);
    }
    return get_url_filename(result);
}

pub fn get_filename_without_extension(file: String) -> String {
    let file2 = get_filename(file);
    let ii = stringops::last_index_of(&file2, ".".to_string(), true);
    if ii > 0 {
        return stringops::substring(&file2, 0, ii);
    }
    return file2;
}

pub fn get_files(full_path: String) -> Vec<String> {
    let mut files = vec![];
    let paths = fs::read_dir(full_path);
    match paths
    {
        Ok(x) => {
            for i in x {
                _ = i.map(|entry| entry.path()).map(|path| {
                    let s = path.to_str();
                    match s {
                        Some(x) => {
                            let s2 = x.to_string();
                            files.push(s2);
                        },
                        None => {}
                    }
                });
            }
        },
        Err(_) => { return files; }
    }
    return files;
}

pub fn get_directory_name(file: String) -> String {
    let mut i = stringops::last_index_of(&file, "/".to_string(), true);
    let j = stringops::last_index_of(&file, "\\".to_string(), true);
    let c = stringops::count(&file) as i32;

    if (j > i || i == -1) && j != -1 {
        i = j;
    }
    if i != -1 {
        return stringops::substring(&file, 0, i);
    }

    return ".".to_string();
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

pub fn create_file_or_append(file: &str) -> Result<File, std::io::Error> {
    let output;
    if let Ok(x) = Path::new(&file).try_exists() {
        let outputx;
        if x == false {
            outputx = File::create(&file);
        } else {
            outputx = fs::OpenOptions::new().write(true).append(true).open(&file);
        }
        match outputx {
            Ok(o) => {
                output = o;
            },
            Err(x) => { return Err(x); }
        }
    }
    else {
        let outputx = fs::OpenOptions::new().write(true).append(true).open(&file);
        
        match outputx {
            Ok(o) => {
                output = o;
            },
            Err(x) => { return Err(x); }
        }
    }

    return Ok(output);
}

pub fn create_file(file: &str) -> Result<File, std::io::Error> {
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

pub fn get_domain(url: String) -> String {
    let mut i = 8;
    if stringops::starts_with(&url, "https://".to_string(), true) { i = 8; }
    else if stringops::starts_with(&url, "http://".to_string(),  true) { i = 7; }
    let mut x = stringops::index_of(&url, "/".to_string(), i, true);
    if x == -1 {
        x = stringops::count(&url) as i32;
    }
    let result = stringops::substring(&url, i, x - i);
    return result;
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

pub fn append_all_text(filename: String, content: &String) {
    let file = create_file_or_append(filename.to_string().as_str());
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
            println!("Error, create_file_or_append, '{}' : {:?}", filename.to_string(), x);
        }
    }
}

pub fn read_all_lines(full_path: String) -> Vec<String> {
    let mut result = vec![];

    let str = read_all_text(full_path);
    let list = str.split("\n").collect::<Vec<&str>>();
    for i in 0..list.len() {
        let s = list[i].to_string();
        let c = stringops::count(&s);

        if stringops::ends_with(&s, "\r".to_string(), false) {
            result.push(stringops::substring(&s, 0, c - 1));
        } else {
            result.push(stringops::substring(&s, 0, c));
        }
    }
    return result;
}

pub fn read_all_bytes(full_path: String) -> Vec<u8> {
    let result = vec![];

    let p = fs::read(&full_path);
    match p {
        Ok(x) => {
            return x;
        },
        Err(_) => {
            return result;
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
    // let mut file = BufferedReader::new(File::open(&path));
    // for i in 0..file.lines.len() {
    //     let str = file.lines[i].to_string();
    //     result.push_str(result.as_str());
    //     if i != file.lines.len() - 1 {
    //         result.push_str(stringops::enter().as_str());
    //     }
    // }
    // return result;
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

pub fn is_directory_exists(full_path: String) -> bool {
    let sx = fs::metadata(full_path.to_string());

    match sx {
        Ok(s) => {
            if s.is_dir() {
                return true;
            }
        },
        Err(_s) => {}
    }
    return false;
}