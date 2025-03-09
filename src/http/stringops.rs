#![allow(unused)]
use rand_distr::Alphanumeric;
use unicode_segmentation::{UnicodeSegmentation, Graphemes};
use chrono::DateTime;
use rand::{rng, Rng};

pub fn enter() -> String {
    if cfg!(windows) {
        return String::from("\r\n");
    } else {
        return String::from("\n");
    }
}

pub fn random_string(len: i32) -> String {
    let result = rng()
        .sample_iter(&Alphanumeric)
        .take(len as usize)
        .map(char::from)
        .collect();
    return result;
}

pub fn count(str: &String) -> i32 {
    let result = str.graphemes(true).count() as i32;
    return result;
}

pub fn ltrim(str: String) -> String {
    let obj = str.clone();
    let mut gr = obj.graphemes(true);
    
    let count = (obj.graphemes(true).count()) as i32;
    let mut i = 0;
    let mut start = true;

    let mut output = String::from("");
    while i < count {
        let cc = gr.next();
        let c;
        match cc {
            Some(x) => c = x,
            None => { break; }
        }

        if start && (c == " " || c == "\t" || c == "\r" || c == "\n" || c == "\r\n") {
            i = i + 1;
            continue;
        } else {
            output.push_str(c);
            start = false;
        }
        i = i + 1;
    }
    return output;
}

pub fn rtrim(str: String) -> String {
    let obj = str.clone();
    let mut gr = obj.graphemes(true).rev();
    
    let count = (obj.graphemes(true).count()) as i32;
    let mut i = 0;
    let mut start = true;

    let mut output = String::from("");
    while i < count {
        let cc = gr.next();
        let c;
        match cc {
            Some(x) => c = x,
            None => { break; }
        }

        if start && (c == " " || c == "\t" || c == "\r" || c == "\n" || c == "\r\n") {
            i = i + 1;
            continue;
        } else {
            output.insert_str(0, c);
            start = false;
        }
        i = i + 1;
    }
    return output;
}

pub fn trim(str: String) -> String {
    let mut result = str.to_string();
    result = ltrim(result);
    result = rtrim(result);
    return result;
}

pub fn path_sep() -> String {
    if cfg!(windows) {
        return String::from("\\");
    } else {
        return String::from("/");
    }
}

pub fn distinct(vecs: Vec<String>) -> Vec<String> {
    let mut result = vec![];

    for i in 0..vecs.len() {
        if result.contains(&vecs[i]) == false {
            result.push(vecs[i].to_string());
        }
    }
    return result;
}

pub fn get_filename_only(file: String) -> String {
    let c = file.graphemes(true).count() as i32;
    let mut i = last_index_of(&file.to_string(), "/".to_string(), true);
    let j = last_index_of(&file.to_string(), "\\".to_string(), true);
    
    if j > i && j != -1 {
        i = j;
    }

    if i != -1 && i + 1 < c {
        return substring(&file.to_string(), i + 1, c - i - 1);
    }
    return file;
}

pub fn get_filename_without_extension(file: String) -> String {
    let i = last_index_of(&file.to_string(), ".".to_string(), true);
    
    if i != -1 {
        return substring(&file.to_string(), 0, i);
    }
    return file;
}

pub fn get_path(file: String) -> String {
    let mut i = last_index_of(&file.to_string(), "/".to_string(), true);
    let j = last_index_of(&file.to_string(), "\\".to_string(), true);
    
    if j > i && j != -1 {
        i = j;
    }

    if i != -1 {
        return substring(&file.to_string(), 0, i);
    }
    return file;
}

pub fn join(args: Vec<String>, sep: String) -> String {
    let mut result = String::from("");
    for i in 0..args.len() {
        result.push_str(args[i].to_string().as_str());
        if i != args.len() - 1 {
            result.push_str(sep.to_string().as_str());
        }
    }
    return result;
}

pub fn replace(str: &String, replace: String, with: String, ignore_case: bool) -> String {
    let mut result = str.to_string();
    let mut start = 0;
    let mut i = index_of(&result, replace.to_string(), start, ignore_case);
    let with_len = count(&with.to_string());
    let replace_len = count(&replace.to_string());

    while i >= 0 {
        let c = count(&result);
        let mut left = String::from("");
        if i > 0 { left = substring(&result, 0, i); }
        let middle = with.to_string();
        let mut right = String::from("");
        if i + replace_len < c { right = substring(&result, i + replace_len, c - (i + replace_len)); }

        result = format!("{}{}{}", left, middle, right);
        start = i + with_len;
        i = index_of(&result, replace.to_string(), start, ignore_case);
    }
    return result;
}

pub fn get_args(args: &Vec<String>, sep: &String) -> Vec<Vec<String>> {
    let mut result : Vec<Vec<String>> = vec![];

    let mut slash = false;
    for arg in args {
        let narg: String = arg.to_string();
        if starts_with(&narg, sep.to_string(), true) {
            let mut cur = vec![];
            cur.push(narg);
            result.push(cur);
            slash = true;
        }
        else {
            if result.len() == 0 || slash == false{
                result.push(vec![]);
            }
            let i = result.len();
            let cur = &mut result[i - 1];
            cur.push(narg);
        }
    }
    return result;
}

pub fn pad_left(str:&String, char: &String, count: i32) -> String {
    let c = str.graphemes(true).count();

    let mut result = String::from("");

    if (c as i32) >= count { return str.to_string(); }
    let mut amt = count - (c as i32);
    while amt > 0 {
        result.push_str(char.as_str());
        amt = amt - 1;
    }

    result.push_str(str.as_str());
    return result.to_string();
}

pub fn pad_right(str:&String, char: &String, count: i32) -> String {
    let c = str.graphemes(true).count();

    let mut result = String::from("");
    result.push_str(str.as_str());

    if (c as i32) >= count { return str.to_string(); }
    let mut amt = count - (c as i32);
    while amt > 0 {
        result.push_str(char.as_str());
        amt = amt - 1;
    }

    return result.to_string();
}

pub fn is_whitespace(str: &String) -> bool {
    if str == " " || str == "\r" || str == "\n" || str == "\t" || str == "\r\n" { return true; }
    return false;
}

pub fn is_numeric(str: &String) -> bool {
    let mut gs = str.graphemes(true);
    
    while let Some(c) = gs.next() {
        let mut cr = c.chars();
        
        while let Some(cc) = cr.next() {
            if cc.is_numeric() == false {
                return false;
            }
        }
    }
    return true;
}

pub fn is_alphabetic(str: &String) -> bool {
    let mut gs = str.graphemes(true);
    
    while let Some(c) = gs.next() {
        let mut cr = c.chars();
        
        while let Some(cc) = cr.next() {
            if cc.is_alphabetic() == false {
                return false;
            }
        }
    }
    return true;
}

pub fn is_alphanumeric(str: &String) -> bool {
    let mut gs = str.graphemes(true);
    
    while let Some(c) = gs.next() {
        let mut cr = c.chars();
        
        while let Some(cc) = cr.next() {
            if cc.is_alphanumeric() == false {
                return false;
            }
        }
    }
    return true;
}

pub fn is_datetime(str: &String) -> bool {
    let datetime = DateTime::parse_from_rfc3339(str);//DateTime::parse_from_rfc2822(str);
    let x = match datetime {
        Ok(_x) => true,
        Err(_e) => false
    };
    return x;
}

pub fn starts_with(str: &String, sub: String, ignore_case: bool) -> bool {
    let sc = sub.graphemes(true).count();
    let stc = str.graphemes(true).count();
    if sc > stc { return false; }

    let mut strs = str.graphemes(true);
    let mut i = 0;

    for c in sub.graphemes(true) {
        let opt = strs.next();
        match opt {
            Some(d) => { 
                if (ignore_case && c.to_uppercase().cmp(&d.to_uppercase()) == std::cmp::Ordering::Equal) || c == d {
                    i += 1;
                } else {
                    break;
                }
            },
            None => { break; }
        }
    }
    
    if i == sc { return true; }
    return false;
}

pub fn ends_with(str: &String, sub: String, ignore_case: bool) -> bool {
    let sc = sub.graphemes(true).count();
    let stc = str.graphemes(true).count();
    if sc > stc { return false; }

    let mut strs = str.graphemes(true).rev();
    let mut i: usize = 0;

    for c in sub.graphemes(true).rev() {
        let opt = strs.next();
        match opt {
            Some(d) => { 
                if (ignore_case && c.to_uppercase().cmp(&d.to_uppercase()) == std::cmp::Ordering::Equal) || c == d {
                    i += 1;
                } else {
                    break;
                }
            },
            None => { break; }
        }
    }
    if i == sc { return true; }
    return false;
}

pub fn index_of(str: &String, sub: String, start_index: i32, ignore_case: bool) -> i32 {
    let sc = sub.graphemes(true).count();
    let stc = str.graphemes(true).count();
    if sc > stc { return -1; }
    if sc <= 0 { return 0; }
    if start_index + (sc as i32) > (stc as i32) { return -1; }

    let mut subs= sub.graphemes(true);
    let mut i:i32 = 0;
    let mut cnt:i32 = 0;

    let mut cc:i32 = start_index;
    let mut opt = subs.next();
    
    for c in str.graphemes(true) {
        if cc > 0 {
            cc -= 1;
            continue;
        }
        
        match opt {
            Some(d) => { 
                if (ignore_case && c.to_uppercase().cmp(&d.to_uppercase()) == std::cmp::Ordering::Equal) || c == d {
                    i += 1;
                    cnt += 1;
                    if cnt == sc as i32 {
                        break;
                    }
                    opt = subs.next();
                } else {
                    i += 1;
                    cnt = 0;
                    subs = sub.graphemes(true); 
                    opt = subs.next();
                    match opt {
                        Some(e) => {
                            if (ignore_case && c.to_uppercase().cmp(&e.to_uppercase()) == std::cmp::Ordering::Equal) || c == e {
                                cnt += 1;
                                if cnt == sc as i32 {
                                    break;
                                }
                                opt = subs.next();
                            }
                        },
                        None => { }
                    }
                }
            },
            None => { break; }
        }
    }
    if cnt == sc as i32 {
        return i - cnt + start_index;
    }
    return -1;
}

pub fn last_index_of(str: &String, sub: String, ignore_case: bool) -> i32 {
    let sc = sub.graphemes(true).count();
    let stc = str.graphemes(true).count();
    if sc > stc { return -1; }
    if sc <= 0 { return 0; }

    let mut subs= sub.graphemes(true).rev();
    let mut i:i32 = stc as i32;
    let mut cnt:i32 = 0;

    let mut opt = subs.next();
    for c in str.graphemes(true).rev() {
        match opt {
            Some(d) => { 
                if (ignore_case && c.to_uppercase().cmp(&d.to_uppercase()) == std::cmp::Ordering::Equal) || c == d {
                    i -= 1;
                    cnt += 1;
                    if cnt == sc as i32 {
                        break;
                    }
                    opt = subs.next();
                } else {
                    i -= 1;
                    cnt = 0;
                    subs = sub.graphemes(true).rev(); 
                    opt = subs.next();
                    match opt {
                        Some(e) => {
                            if (ignore_case && c.to_uppercase().cmp(&e.to_uppercase()) == std::cmp::Ordering::Equal) || c == e {
                                cnt += 1;
                                if cnt == sc as i32 {
                                    break;
                                }
                                opt = subs.next();
                            }
                        },
                        None => { }
                    }
                }
            },
            None => { break; }
        }
    }
    if cnt == sc as i32 {
        return i;
    }
    return -1;
}

pub fn substring(str: &String, i: i32, mut count: i32) -> String {
    let mut result = String::new();

    let stc = str.graphemes(true).count();
    if i + count > stc as i32 { panic!("Substring index or count is longer than string length count") }

    let mut strs = str.graphemes(true);
    let mut j = i;
    while j > 0 {
        strs.next();
        j -= 1;
    }

    while count > 0 {
        let opt= strs.next();
        match opt {
            Some(c) => result.push_str(&c.to_string()),
            None => { }
        }
        count -= 1;
    }
    return result;
} 

fn same_char_m(ignore_case: bool, opt: Option<&str>, c: &str, m: &mut i32, sc: usize, started: &mut bool) -> bool {
    let mut reset = false;
    match opt {
        Some(e) => {
            if (ignore_case && c.to_uppercase().cmp(&e.to_uppercase()) == std::cmp::Ordering::Equal) || c == e {
                *m += 1;
                if *m == sc as i32 {
                    *started = true;
                    reset = true;
                }
            }
        }, 
        None => {}
    }
    return reset;
}

fn between_inner(ignore_case: bool, c: &str, d: &str, m: &mut i32, sc: usize, starts: &mut Graphemes, started: &mut bool) -> bool {
    let mut reset = false;
    if (ignore_case && c.to_uppercase().cmp(&d.to_uppercase()) == std::cmp::Ordering::Equal) || c == d  {
        *m += 1;
        if *m == sc as i32 {
            *started = true;
            reset = true;
        } 
    } else {
        *m = 0;
        reset = true;
        let opt = starts.next();
        if same_char_m(ignore_case, opt, c, m, sc, started) {
            reset = true;
        }
    }
    return reset;
}

fn between_inner2(ignore_case: bool, c: &str, e: &str, end: &String, temp: &mut String, temp2: &mut String, m: &mut i32, ec: i32, started: &mut bool) -> bool {
    let mut reset = false;
    if (ignore_case && c.to_uppercase().cmp(&e.to_string().to_uppercase()) == std::cmp::Ordering::Equal) || c == e  {
        *m += 1;
        (*temp2).push_str(e);
    }
    else {
        *m = 0;
        (*temp).push_str(&temp2.to_string());
        *temp2 = String::from("") ;

        let mut ends = end.graphemes(true);
        let opt = ends.next();
        match opt {
            Some(f) => {
                if (ignore_case && c.to_uppercase().cmp(&f.to_string().to_uppercase()) == std::cmp::Ordering::Equal) || c == f  {
                    *m += 1;
                    temp2.push_str(f);
                } else {
                    temp.push_str(c);
                    reset = true;
                }
            },
            None => {}
        }
    }
    return reset;
}

pub fn between(str: &String, start: String, end: String, limit: i32, ignore_case: bool) -> Vec<String> {
    let mut result:Vec<String> = vec![];
    let strs = str.graphemes(true);
    let mut starts = start.graphemes(true);
    let mut ends = end.graphemes(true);
    
    let sc = start.graphemes(true).count();
    let ec = end.graphemes(true).count();
   
    let mut m = 0;
    let mut started:bool = false;
   
    let mut temp = String::from("");
    let mut temp2 = String::from("");
    for c in strs {
        if started == false {
            
            let opt = starts.next();
            match opt {
                Some(d) => {
                    if between_inner(ignore_case, c, d, &mut m, sc, &mut starts, &mut started) {
                        starts = start.graphemes(true);
                        m = 0;
                    }
                },
                None => {
                    starts = start.graphemes(true);
                    let opt = starts.next();
                    if same_char_m(ignore_case, opt, c, &mut m, sc, &mut started) {
                        starts = start.graphemes(true);
                        m = 0;
                    }
                } 
            }
        }
        else if started == true {
            let opt = ends.next();
            match opt {
                Some(e) => {
                    if between_inner2(ignore_case, c, e, &end, &mut temp, &mut temp2, &mut m, ec as i32, &mut started) {
                        ends = end.graphemes(true);
                    }
                },
                None => {
                    ends = end.graphemes(true);
                    let opt = ends.next();
                    match opt {
                        Some(e) => {
                            if between_inner2(ignore_case, c, e, &end, &mut temp, &mut temp2, &mut m, ec as i32, &mut started) {
                                ends = end.graphemes(true);
                            }
                        },
                        None => { }
                    }
                }
            }
            
            if m == ec as i32 {
                ends = end.graphemes(true);
                m = 0;
                temp2 = String::from("");
                result.push(temp);
                if limit > 0 && (result.len() as i32) >= limit { return result; } 
                started = false;
                temp = String::from("") ;
            }
        }
    }
    return result;
}

pub fn mul_string(dirs: &str, sep: &str, opstart: &str, opend: &str, opsep: &str) -> Vec<String> {
    let dirs2 = dirs.split(sep).collect::<Vec<&str>>();
    let c = dirs2.len();

    let vs = mul_string_inner(&dirs2, "", sep, opstart, opend, opsep, 0);
    let mut result:Vec<String> = vec![];
    
    for x in 0..vs.len() {
        let cc = vs[x].to_string().split(sep).collect::<Vec<&str>>().len();
        if cc == c {
            result.push(vs[x].to_string());
        }
    }
    return result;
}

fn mul_string_inner(dirs2: &Vec<&str>, str1: &str, sep: &str, opstart:&str, opend:&str, opsep:&str, i: usize) -> Vec<String> {
    let dr = dirs2[i].to_string();
    let mut chars = dr.graphemes(true);
    let c = dr.graphemes(true).count();

    let mut vecs:Vec<String> = vec![];
    let sc = chars.nth(0);
    let ec = if c >= 2 { chars.nth(c-2) } else { Some("") };
    if sc == Some(opstart) && ec == Some(opend) {
        let t = substring(&dr, 1, (c as i32) - 2);
        let sps = t.split(opsep).collect::<Vec<&str>>();

        for j in 0..sps.len() {
            let mut x = String::from("");
            x.push_str(str1);
            x.push_str(sep);
            x.push_str(sps[j]);
            vecs.push(x);
        }
    }
    else {
        let mut x = String::from("");
        x.push_str(str1);
        if str1 != "" { x.push_str("\\"); }
        x.push_str(dr.as_str());
        vecs.push(x);
    }

    if i < dirs2.len() - 1 {
        for x in 0..vecs.len() {
           let vecs2 = mul_string_inner(dirs2, vecs[x].as_str(), sep, opstart, opend, opsep, i + 1);

           for y in 0..vecs2.len() {
                vecs.push(vecs2[y].to_string());
           }
        }
    }
    
    return vecs;
}
