use unicode_segmentation::{UnicodeSegmentation, Graphemes};
use chrono::DateTime;

pub fn enter() -> String {
    if cfg!(windows) {
        return String::from("\r\n");
    } else {
        return String::from("\n");
    }
}

pub fn count(str: &String) -> i32 {
    return str.graphemes(true).count() as i32;
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

