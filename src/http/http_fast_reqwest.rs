use std::borrow::BorrowMut;
use std::collections::hash_map::RandomState;
use std::{io::Read, collections::hash_map::DefaultHasher};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::{Hash, Hasher};
use reqwest::{Client, multipart, header::HeaderMap, header::{SET_COOKIE}, header::HeaderName, header::HeaderValue};
use std::thread;
use std::time;
use std::time::Duration;

lazy_static! {
    pub static ref TIMEOUT: u64 = 20000;
    pub static ref CLIENTS: Arc<Mutex<HashMap<MyClient, bool>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone)]
pub struct MyClient {
    pub client: Client
}

impl Eq for MyClient {}

impl PartialEq for MyClient {
    fn eq(&self, other: &MyClient) -> bool {
        return equal(self, other);
    }
}

impl Hash for MyClient {
    fn hash<H: Hasher>(&self, state: &mut H) {
        &self.client;
    }
}

pub fn get_client(parallel: i32) -> MyClient
{
    let clients = &mut *CLIENTS.lock().unwrap();
    let mut para = parallel;
    if para <= 0 { para = 1; }

    if (clients.len() as i32) < para {
        let c = reqwest::Client::builder().cookie_store(true).timeout(Duration::from_secs(10 * 60)).build().unwrap();
        let cc = MyClient { client: c };
        
        clients.insert(cc.clone(), true);
        return cc.clone();
    }
    
    for (c, run) in clients {
        if run == &false {
            *run = true;
            return c.clone();
        }
    }

    // if continuously enter below, will cause handle leaks
    let c = reqwest::Client::builder().cookie_store(true).build().unwrap();
    let cc = MyClient { client: c };
    return cc.to_owned();
}

fn equal(s: &MyClient, other: &MyClient) -> bool {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let mut hasher2 = DefaultHasher::new();
    other.hash(&mut hasher2);

    let s1 = format!("{:x}", hasher.finish());
    let s2 = format!("{:x}", hasher2.finish());

    if s1 == s2 {
        return true;
    }
    return false;
}

pub fn post_json(parallel: i32, url: String, method: String, value2: HashMap<&str, &str, RandomState>) -> Result<String, String> {
    let mut formx = multipart::Form::new();

    let mut headersx = HeaderMap::new();
    headersx.append("Content-Type", HeaderValue::from_static("application/json"));

    let init_c = get_client(parallel);

    let form = formx;

    let resp: Result<reqwest::Response, reqwest::Error>;
    if method.to_uppercase() == "POST" {
        resp = init_c.client.post(url.as_str()).headers(headersx).json(&value2).send();
    }
    else if method.to_uppercase() == "PUT" {
        resp = init_c.client.put(url.as_str()).headers(headersx).json(&value2).send();
    }
    else if method.to_uppercase() == "DELETE" {
        resp = init_c.client.delete(url.as_str()).headers(headersx).json(&value2).send();
    } else {
        resp = init_c.client.get(url.as_str()).headers(headersx).send();
    }
    
    {
        let dd = &mut *CLIENTS.lock().unwrap();
        _ = dd.remove(&init_c);
        dd.insert(init_c.to_owned(), false);
    }

    match resp {
        Ok(mut rep) => {
            let cls = rep.headers().get(SET_COOKIE);

            match cls {
                Some(cl) => {
                    let _ = cl.to_str();
                },
                None => {}
            }

              let mut repbody = String::from("");
            _ = rep.read_to_string(&mut repbody);

            return Ok((repbody));
        }
        Err(err) => {
            let mut st = String::from("");
            match err.status() {
                Some(s) => st = s.to_string(),
                None => {}
            }
            let s = format!("Error. Status {}, message {}", st, err.to_string());
            return Err(s);
        }
    }
}
