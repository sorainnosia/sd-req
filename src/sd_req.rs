use std::collections::{HashMap};
use std::sync::{Mutex, Arc};
use std::io;
use std::io::Write;
use std::thread;
use std::time;
use serde_json::{Value, Map, json};
use base64::{*, engine::general_purpose};
use close_file::Closable;
use sanitize_filename;
use crate::http::{http_fast_reqwest, path, stringops, types, json as json_comlib};

#[derive(Clone, Debug)]
pub struct sd_reqo {
    pub is_running: Arc<Mutex<bool>>,
    pub json: Arc<Mutex<Option<Value>>>
}

impl sd_reqo {
    pub fn new() -> sd_reqo {
        let obj = sd_reqo { is_running: Arc::new(Mutex::new(false)), json : Arc::new(Mutex::new(None)) };
        return obj;
    }

    fn set_config(&self) {
        let config = r#"
        {
            "url" : "http://127.0.0.1:7860",
            "output_path" : "output",
			"model" : "",
            "seed_start" : -1,
            "seed_end" : -1,
            "negative_prompt" : "string",
            "steps" : 20,
            "width" : 512,
            "height" : 512,
            "sampler_index" : "Euler",
            "cfg_scale" : 7,
            "tiling" : false,
            "n_iter" : 1,
            "batch_size" : 1,
            "restore_faces" : false,
            "denoising_strength" : 0,
            "firstphase_width" : 0,
            "firstphase_height" : 0,
            "seed" : -1,
            "subseed" : -1,
            "subseed_strength" : 0,
            "seed_resize_from_h" : -1,
            "seed_resize_from_w" : -1,
            "eta" : 0,
            "s_churn" : 0,
            "s_tmax" : 0,
            "s_tmin" : 0,
            "s_noise" : 1,
            "enable_hr" : false,
            "styles" : ["string"]
        }"#;

        let prog_names: Vec<String> = std::env::args().collect();
        let mut prog_name = "config".to_string();
        if prog_names.len() > 0 {
            prog_name = path::get_filename_without_extension(prog_names[0].to_string());
        }

        let str = format!("{}.json", prog_name);
        if path::is_file_exists(str.to_string()) == false {
            path::write_all_text(str.to_string(), &config.to_string());
        }

        let read = path::read_all_text(str.to_string());
        let x = serde_json::from_str(&read);
        match x {
            Ok(y) => {
                *self.json.lock().unwrap() = y;
            },
            Err(x) => { println!("{:?}", x); }
        }
    }

    fn save_image(&self, output_path: &String, prompt: &String, mut base64: String) -> bool {
        let mut i = 1;
        
        let prompt2 = sanitize_filename::sanitize(prompt);
        let mut fname = format!("{}/{} - {}.png", output_path.to_string(), prompt2.to_string(), i);
        while path::is_file_exists(fname.to_string()) {
            i = i + 1;
            fname = format!("{}/{} - {}.png", output_path.to_string(), prompt2.to_string(), i);
        }

        let i = stringops::index_of(&base64, ",".to_string(), 0, false);
        if i >= 0 {
            let c = stringops::count(&base64);
            base64 = stringops::substring(&base64, i + 1, c - i - 1);
        }

        path::create_dir_all(output_path.to_string());
        let val = general_purpose::STANDARD.decode(base64);
        match val {
            Ok(x) => {
                let nf = path::create_file(&fname.to_string());

                match nf {
                    Ok(mut y) => {
                        let _a = y.write_all(&x);
                        let _b = y.close();

                        return true;
                    },
                    Err(x) => {
                        println!("Invalid during file creation: {:?}", x);
                    }
                }
            },
            Err(x) => {
                println!("Fail to decode base64: {:?}", x);
            }
        }
        return false;
    }

    fn print_header(&self) {
        println!("sd-req 0.3.0");
        println!("Stable Diffusion WebUI API Requestor");
        println!("");
    }

    fn print_help(&self) {
        self.print_header();
        println!("Arguments");
        println!("   [repeat/norepeat] [prompt] [amount] [CONFIGS...]");
        println!("Example 1");
        println!("   repeat \"rock in a river\" 5");
        println!("Example 2");
        println!("   repeat \"rock in a river\" 1 seed 5 negative_prompt \"sand\" steps 50");
        println!("Example 3");
        println!("   norepeat \"rock in a river\" 1");
        println!("CONFIGS");
        println!("   <key> <value>...");
        println!("   List of key value pair of txt2img json property to override from default config file");
        println!("CONFIGS also possible to contain following:");
        println!("   seed_start <value> seed_end <value>");
        println!("   to start generating image from starting seed_start to ending seed_end");
    }

    fn change_model(&self, model: String) -> bool {
        let mut url = String::from("");
        {
            let j = &*self.json.lock().unwrap();
            match j {
                Some(k) => {
                    url = json_comlib::get_string(k, "url".to_string(), url.to_string());
                },
                None => {}
            }
        }

        let url2 = format!("{}/sdapi/v1/options", url.to_string());
        let json_str = http_fast_reqwest::post_json(1, url2.to_string(), "GET".to_string(), HashMap::new());
        match json_str {
            Ok(o) => {
                let val = serde_json::from_str::<Value>(o.as_str());
                match val {
                    Ok(mut v) => {
                        v["sd_model_checkpoint"] = Value::String(model.to_string());

                        let vstr = serde_json::to_string(&v);
                        match vstr {
                            Ok(v) => {
                                let json2 = http_fast_reqwest::post_body(1, url2.to_string(), "POST".to_string(), "application/json".to_string(), v.to_string());
                                match json2 {
                                    Ok(xx) => {
                                        return true;
                                    },
                                    Err(xy) => {
                                        println!("Invalid result from change_model: {:?}", xy);
                                    }
                                }
                            },
                            Err(xz) => {
                                println!("Invalid during conversion json to string: {:?}", xz);
                            }
                        }
                    },
                    Err(xu) => {
                        println!("Invalid during conversion of options API response to json: {:?}", xu);
                    }
                }
            },
            Err(xr) => { 
                println!("Invalid Response of options API: {}", xr);
            }
        }
        return false;
    }

    pub fn sdcall(&mut self) {
        let mut args:Vec<String> = std::env::args().collect();
        args.remove(0);

        let mut repeat = false;
        let mut arg_prompt = String::new();
        let mut arg_count = 0;
        
        self.set_config();
        if args.len() > 0 && args[0].to_string().to_lowercase() == "-h" {
            self.print_help();
            return;
        }

        if args.len() == 0 {
            self.print_header();
        }    
        if args.len() > 0 {
            if args[0].to_lowercase() == "repeat" {
                repeat = true;
            } else if args[0].to_lowercase() == "norepeat" {
                repeat = false;
            } else {
                self.print_help();
                println!("Argument 1 '{}': should be \"repeat\" or \"norepeat\"", args[0].to_string());
                return;
            }
        }
        if args.len() > 1 {
            arg_prompt = args[1].to_string();
        }
        if args.len() > 2 {
            let x = args[2].to_string();
            let (b, r) = types::str_to_i32(x);
            
            if b == false || (b && r <= 0) {
                self.print_help();
                println!("Argument 3 '{}': should be positive integer count", args[2].to_string());
                return;
            } else {
                arg_count = r;
            }
        }

        {
            *self.is_running.lock().unwrap() = true;
        }
        let mut first = true;
        while first || repeat {
            if arg_prompt == String::new() {
                print!("{}", "Input prompt : ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut arg_prompt)
                    .expect("failed to read from stdin");    
            }
            if arg_prompt.to_string() == stringops::enter() {
                println!("End");
                break;
            }
            
            if arg_count <= 0 {
                print!("{}", "Input amount : ");
                io::stdout().flush().unwrap();
                let mut input_text = String::new();
                io::stdin().read_line(&mut input_text)
                    .expect("failed to read from stdin");
            
                let (b, r) = types::str_to_i32(input_text.trim().to_string());
                if b == false || (b && r <= 0) {
                    println!("[amount] need to be greater than 0");
                    break;
                } else {
                    arg_count = r;
                }
            }

            if arg_count == 0 { arg_count = 1; }
            
			let mut model = "".to_string();
            let mut url = String::from("http://127.0.0.1:7860");
            let mut output_path = String::from("output");
            let mut seed_start = -1;
            let mut seed_end = -1;
            let mut k;
            {
                let j = &*self.json.lock().unwrap();
                match j {
                    Some(o) => {
                        k = o.clone();
                        url = json_comlib::get_string(&k, "url".to_string(), url.to_string());
                        output_path = json_comlib::get_string(&k, "output_path".to_string(), output_path.to_string());
						model = json_comlib::get_string(&k, "model".to_string(), model.to_string());
                        let seed_start_str = json_comlib::get_string(&k, "seed_start".to_string(), model.to_string());
                        let seed_end_str = json_comlib::get_string(&k, "seed_end".to_string(), model.to_string());
                        let (b, r) = types::str_to_i64(seed_start_str);
                        if (b && r >= 0) || b == false {
                            seed_start = r;
                        }
                        let (b2, r2) = types::str_to_i64(seed_end_str);
                        if (b2 && r2 >= 0) || b2 == false {
                            seed_end = r;
                        }
                    },
                    None => {
                        k = Value::Object(Map::new());
                    }
                }
            }

            let mut args2:Vec<String> = std::env::args().collect();
            if args2.len() > 0 { args2.remove(0); }
            if args2.len() > 0 { args2.remove(0); }
            if args2.len() > 0 { args2.remove(0); }
            if args2.len() > 0 { args2.remove(0); }

            let mut x = 0;
            while x < args2.len() {
                let key = args2[x].to_string();
                if key.to_string().to_lowercase() == "model".to_string() && x + 1 < args2.len() {
                    model = args2[x + 1].to_string();
                    x = x + 2;
                    continue;
                } else if key.to_string().to_lowercase() == "url".to_string() && x + 1 < args2.len() {
                    url = args2[x + 1].to_string();
                    x = x + 2;
                    continue;
                } else if key.to_string().to_lowercase() == "output_path".to_string() && x + 1 < args2.len() {
                    output_path = args2[x + 1].to_string();
                    x = x + 2;
                    continue;
                } else if key.to_string().to_lowercase() == "seed_start".to_string() && x + 1 < args2.len() {
                    let seed_start_str = args2[x + 1].to_string();
                    let (b, r) = types::str_to_i64(seed_start_str);
                    if (b && r < 0) || b == false {
                        println!("seed_start need to be greater or equal 0");
                    }
                    else {
                        seed_start = r;
                    }
                    x = x + 2;
                    continue;
                } else if key.to_string().to_lowercase() == "seed_end".to_string() && x + 1 < args2.len() {
                    let seed_end_str = args2[x + 1].to_string();
                    let (b, r) = types::str_to_i64(seed_end_str);
                    if (b && r < 0) || b == false {
                        println!("seed_end need to be greater or equal 0");
                    } else {
                        seed_end = r;
                    }
                    x = x + 2;
                    continue;
                } else {
                    if x + 1 < args2.len() {
                        let v = args2[x + 1].to_string();
                        let opt = k.get(key.as_str());
                        match opt {
                            Some(o) => {
                                k[key.as_str()] = json_comlib::get_value(key.to_string(), o, v);
                            },
                            None => {
                                self.print_help();
                                println!("API does not contain configuration '{}'", key.as_str());
                                break;
                            }
                        }
                    }
                }
                x = x + 2;
            }

            if seed_start != -1 && seed_end != -1 {
                if seed_start > seed_end {
                    println!("seed_start {} need to be lesser than seed_end {}", seed_start, seed_end);
                    break;
                }
            }
			
            let mut json = Value::Object(Map::new());
            match k.as_object() {
                Some(ao) => {
                    for (key, value) in ao {
                        if key.to_string() != "output_path".to_string() &&
                            key.to_string() != "model".to_string() &&
                            key.to_string() != "url".to_string() {
                                json[key.as_str()] = value.clone();
                            }
                    }
                },
                None => {}
            }
            
            json["prompt"] = json!(arg_prompt.as_str());
			if model.to_string() != String::from("") {
				let b = self.change_model(model.to_string());
				if b {
					println!("Successfully change model to '{}'", model.to_string());
				} else {
					println!("Fail change model to '{}'", model.to_string());
				}
				thread::sleep(time::Duration::from_secs(5));
			}

            let mut c:i32 = 1;
            println!("Requesting '{}'", format!("{}/sdapi/v1/txt2img", url.to_string()));
            let mut jsonc = json.clone();
            while c <= arg_count {
                if seed_start >= 0 && seed_start <= seed_end {
                    for ss in seed_start..=seed_end {
                        jsonc["seed"] = json!(ss);
                        println!("Seed {}", ss);
                        let url3 = format!("{}/sdapi/v1/txt2img", url.to_string());
                        self.call_api(url3.to_string(), &output_path, &arg_prompt, &jsonc);
                    }
                    break;
                } else {
                    println!("Request {}", c.to_string());
                    let url3 = format!("{}/sdapi/v1/txt2img", url.to_string());
                    self.call_api(url3.to_string(), &output_path, &arg_prompt, &jsonc);
                }
                
                c = c + 1;
            }

            arg_prompt = String::from("");
            arg_count = 0;
            first = false;
        }
        {
            *self.is_running.lock().unwrap() = false;
        }
    }

    fn between(&self, str: &String, start: String) -> String{
        let mut result = String::from("");
        let c = stringops::count(&start);

        let mut i = stringops::index_of(str, start.to_string(), 0, false);
        if i != -1 {
            i = i + c;
        }
        let mut j = stringops::index_of(str, ",".to_string(), i + 1, false);
        let k = stringops::index_of(str, ")".to_string(), i + 1, false);
        let l = stringops::index_of(str, "\n".to_string(), i + 1, false);

        if (k < j && k != -1) || j == -1 { j = k; }
        if (l < j && l != -1) || j == -1 { j = l; }

        if j != -1 {
            result = stringops::substring(&str, i, j - i);
        }
        return result;
    }

    fn call_api(&self, url3: String, output_path: &String, arg_prompt: &String, json: &Value) -> bool {
        let body_r = serde_json::to_string(json);
        let mut body = String::from("");
        match body_r {
            Ok(b) => {
                body = b;
            },
            Err(x) => {
                println!("Invalid during conversion from json to string for call_api : {:?}", x);
                return false;
            }
        }

        let str = http_fast_reqwest::post_body(1, url3.to_string(), "POST".to_string(), "application/json".to_string(), body);
        match str {
            Ok(x) => {
                let jsonx = serde_json::from_str::<Value>(x.as_str());
                match jsonx {
                    Ok(j) => {
                        let mut info = "".to_string();
                        let clx = j.get("info");
                        match clx
                        {
                            Some(cl) => {
                                match cl.as_str() {
                                    Some(cc) => {
                                        info = cc.to_string();
                                    },
                                    None => {}
                                }
                            },
                            None => {}
                        }
                        
                        let mut infotext = "".to_string();
                        let vs = stringops::between(&info, "\"infotexts\": [\"".to_string(), "\"],".to_string(), 1, false);
                        if vs.len() > 0 { 
                            infotext = format!(" ({})", vs[0].to_string()); 
                        }

                        let arrs = j.get("images");
                        match arrs {
                            Some(ar) => {
                                let arrs2 = ar.as_array();
                                match arrs2 {
                                    Some(ar2) => {
                                        for x in ar2 {
                                            let ox = x.as_str();
                                            match ox {
                                                Some(o) => {
                                                    let samplerx = self.between(&infotext, "Sampler: ".to_string());
                                                    let steps = self.between(&infotext, "Steps: ".to_string());
                                                    let seed = self.between(&infotext, "Seed: ".to_string());
                                                    let cfg_scale = self.between(&infotext, "CFG scale: ".to_string());
                                                    let modelx = self.between(&infotext, "Model: ".to_string());

                                                    let prompt = format!("{} (seed={},steps={},cfg_scale={},model={},sampler={})", arg_prompt, seed, steps, cfg_scale, modelx, samplerx);
                                                    let b = self.save_image(&output_path, &prompt, o.to_string());

                                                    if b {
                                                        println!("Success saving file");
                                                    } else {
                                                        println!("Fail saving file");
                                                    }
                                                    return b;
                                                },
                                                None => {
                                                    println!("Failed to get image string");
                                                }
                                            }
                                        }
                                    },
                                    None => {
                                        println!("Invalid Response: images is not list");
                                    }
                                }
                            },
                            None => {
                                println!("Invalid Response: not containing 'images'");
                            }
                        }
                    },
                    Err(x) => {
                        println!("{:?}", x);
                    }
                }
            },
            Err(x) => {
                println!("Invalid Response: {:?}", x);
            }
        }
        return false;
    }
}