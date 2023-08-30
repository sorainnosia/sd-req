pub fn i32_to_u32(i: i32) -> u32 {
    let result = i.to_string().parse::<u32>();
    match result {
        Ok(u) => {
            return u;
        },
        Err(x) => {
            println!("{}", format!("{:?}",x));
            return 0;
        }
    }
}

pub fn str_to_i32(s: String) -> (bool, i32) {
    let result = s.parse::<i32>();
    match result {
        Ok(s) => {
            return (true, s);
        },
        Err(x) => {
            return (false, 0);
        }
    }
}

pub fn u64_to_f64(i: u64) -> f64 {
    let result = i.to_string().parse::<f64>();
    match result {
        Ok(u) => {
            return u;
        },
        Err(x) => {
            println!("{}", format!("{:?}",x));
            return 0f64;
        }
    }
}