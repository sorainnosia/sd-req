use reqwest;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::io;
use std::env;
use std::io::*;
use crate::http::{http_fast_reqwest, path, stringops, types};
use std::io::Write;
use serde_json::{Value};
use base64::{*, engine::general_purpose};
use close_file::Closable;
use sanitize_filename;

#[derive(Clone, Debug)]
pub struct sd_reqo {
    pub is_running: Arc<Mutex<bool>>,
    pub json: Arc<Mutex<Option<Value>>>,
    pub running_thread: Arc<Mutex<i32>>
}

impl sd_reqo {
    pub fn new() -> sd_reqo {
        let obj = sd_reqo { is_running: Arc::new(Mutex::new(false)), json : Arc::new(Mutex::new(None)), running_thread: Arc::new(Mutex::new(0))};
        return obj;
    }

    fn set_config(&self) {
        let config = r#"
        {
            "url" : "http://127.0.0.1:7860",
            "negative_prompt" : "",
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
            "s_noise" : 1
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

    fn save_image(&self, prompt: &String, mut base64: String) -> bool {
        let mut i = 1;
        
        let prompt2 = sanitize_filename::sanitize(prompt);
        let mut fname = format!("{}/{} - {}.png", "output".to_string(), prompt2.to_string(), i);
        while path::is_file_exists(fname.to_string()) {
            i = i + 1;
            fname = format!("{}/{} - {}.png", "output".to_string(), prompt2.to_string(), i);
        }

        let i = stringops::index_of(&base64, ",".to_string(), 0, false);
        if i >= 0 {
            let c = stringops::count(&base64);
            base64 = stringops::substring(&base64, i + 1, c - i - 1);
        }

        path::create_dir_all("output".to_string());
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
                        println!("{:?}", x);
                    }
                }
            },
            Err(x) => {
                println!("Fail to decode base64, {:?}", x);
            }
        }
        return false;
    }

    fn get_string(&self, k: &Value, name: String, default: String) -> String {
        match k.get(name.as_str()) {
            Some(x) => {
                match x.as_str() {
                    Some (y) => {
                        return y.to_string();
                    },
                    None => {}
                }
            },
            None => {}
        }

        match k.get(name.as_str()) {
            Some(x) => {
                match x.as_bool() {
                    Some (y) => {
                        return y.to_string();
                    },
                    None => {}
                }
            },
            None => {}
        }

        match k.get(name.as_str()) {
            Some(x) => {
                match x.as_i64() {
                    Some (y) => {
                        return y.to_string();
                    },
                    None => {}
                }
            },
            None => {}
        }
        return default;
    }

    fn print_header(&self) {
        println!("sd-req 0.1.0");
        println!("Stable Diffusion WebUI API Requestor");
        println!("");
    }

    fn print_help(&self) {
        self.print_header();
        println!("Arguments");
        println!("   [repeat/norepeat] [prompt] [count] [CONFIGS..]");
        println!("Example 1");
        println!("   repeat \"rock in a river\" 5");
        println!("Example 2");
        println!("   repeat \"rock in a river\" 1 seed 5 negative_prompt \"sand\" steps 50");
        println!("Example 3");
        println!("   norepeat \"rock in a river\" 1");
    }

    pub fn sdcall(&self) {
        let mut args:Vec<String> = std::env::args().collect();
        args.remove(0);

        let mut repeat = false;
        let mut arg_prompt = String::new();
        let mut arg_count = 0;

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
            }
        }
        if args.len() > 1 {
            arg_prompt = args[1].to_string();
        }
        if args.len() > 2 {
            let x = args[2].to_string();
            (_, arg_count) = types::str_to_i32(x);
        }

        let mut first = true;
        while first || repeat
        {
            self.set_config();
            if arg_prompt == String::new() {
                print!("{}", "Input prompt : ");
                io::stdout().flush().unwrap();
                let mut prompt = String::new();
                io::stdin().read_line(&mut arg_prompt)
                    .expect("failed to read from stdin");    
            }
            if arg_prompt.to_string() == String::from("\r\n") {
                println!("End");
                break;
            }
            
            if arg_count <= 0 {
                print!("{}", "Input amount : ");
                io::stdout().flush().unwrap();
                let mut input_text = String::new();
                io::stdin().read_line(&mut input_text)
                    .expect("failed to read from stdin");
            
                (_, arg_count) = types::str_to_i32(input_text.trim().to_string());
            }

            if arg_count == 0 { arg_count = 1; }
            
            let mut negative_prompt = String::from("");
            let mut steps = "20".to_string();
            let mut width = "512".to_string();
            let mut height = "512".to_string();
            let mut cfg_scale = "7".to_string();
            let mut sampler_index = "Euler".to_string();
            let mut restore_faces = "false".to_string();
            let mut denoising_strength = "0".to_string();
            let mut firstphase_width = "0".to_string();
            let mut firstphase_height = "0".to_string();
            let mut seed = "-1".to_string();
            let mut subseed = "-1".to_string();
            let mut subseed_strength = "0".to_string();
            let mut seed_resize_from_h = "-1".to_string();
            let mut seed_resize_from_w = "-1".to_string();
            let mut s_churn = "0".to_string();
            let mut s_tmax = "0".to_string();
            let mut s_tmin = "0".to_string();
            let mut s_noise = "1".to_string();
            let mut eta = "0".to_string();
            let mut tiling = "false".to_string();
            let mut n_iter = "1".to_string();
            let mut batch_size = "1".to_string();
            let mut url = String::from("http://127.0.0.1:7860");
            {
                let j = &*self.json.lock().unwrap();
                match j {
                    Some(k) => {
                        url = self.get_string(k, "url".to_string(), url.to_string());
                        steps = self.get_string(k, "steps".to_string(), steps.to_string());
                        width = self.get_string(k, "width".to_string(), width.to_string());
                        height = self.get_string(k, "height".to_string(), height.to_string());
                        cfg_scale = self.get_string(k, "cfg_scale".to_string(), cfg_scale.to_string());
                        sampler_index = self.get_string(k, "sampler_index".to_string(), sampler_index.to_string());
                        negative_prompt = self.get_string(k, "negative_prompt".to_string(), negative_prompt.to_string());
                        restore_faces = self.get_string(k, "restore_faces".to_string(), restore_faces.to_string());
                        tiling = self.get_string(k, "tiling".to_string(), tiling.to_string());
                        denoising_strength = self.get_string(k, "denoising_strength".to_string(), denoising_strength.to_string());
                        firstphase_width = self.get_string(k, "firstphase_width".to_string(), firstphase_width.to_string());
                        firstphase_height = self.get_string(k, "firstphase_heigth".to_string(), firstphase_height.to_string());
                        seed = self.get_string(k, "seed".to_string(), seed.to_string());
                        subseed = self.get_string(k, "subseed".to_string(), subseed.to_string());
                        subseed_strength = self.get_string(k, "subseed_strengh".to_string(), subseed_strength.to_string());
                        seed_resize_from_h = self.get_string(k, "seed_resize_from_h".to_string(), seed_resize_from_h.to_string());
                        seed_resize_from_w = self.get_string(k, "seed_resize_from_w".to_string(), seed_resize_from_w.to_string());
                        s_churn = self.get_string(k, "s_churn".to_string(), s_churn.to_string());
                        s_tmax = self.get_string(k, "s_tmax".to_string(), s_tmax.to_string());
                        s_tmin = self.get_string(k, "s_tmin".to_string(), s_tmin.to_string());
                        s_noise = self.get_string(k, "s_noise".to_string(), s_noise.to_string());
                        eta = self.get_string(k, "eta".to_string(), eta.to_string());
                        n_iter = self.get_string(k, "n_iter".to_string(), n_iter.to_string());
                        batch_size = self.get_string(k, "batch_size".to_string(), batch_size.to_string());
                    },
                    None => {}
                }
            }

            let mut json = HashMap::new();
            json.insert("prompt", arg_prompt.as_str());
            json.insert("negative_prompt", negative_prompt.as_str());
            json.insert("steps", steps.as_str());
            json.insert("width", width.as_str());
            json.insert("height", height.as_str());
            json.insert("cfg_scale", cfg_scale.as_str());
            json.insert("batch_size", batch_size.as_str());
            json.insert("n_iter", n_iter.as_str());
            json.insert("sampler_index", sampler_index.as_str());
            json.insert("tiling", tiling.as_str());
            json.insert("restore_faces", restore_faces.as_str());
            json.insert("denoising_strength", denoising_strength.as_str());
            json.insert("firstphase_width", firstphase_width.as_str());
            json.insert("firstphase_height", firstphase_height.as_str());
            json.insert("seed", seed.as_str());
            json.insert("subseed", subseed.as_str());
            json.insert("subseed_strength", subseed_strength.as_str());
            json.insert("seed_resize_from_h", seed_resize_from_h.as_str());
            json.insert("seed_resize_from_w", seed_resize_from_w.as_str());
            json.insert("eta", eta.as_str());
            json.insert("s_churn", s_churn.as_str());
            json.insert("s_tmax", s_tmax.as_str());
            json.insert("s_tmin", s_tmin.as_str());
            json.insert("s_noise", s_noise.as_str());

            let mut args2:Vec<String> = std::env::args().collect();
            if args2.len() > 0 { args2.remove(0); }
            if args2.len() > 0 { args2.remove(0); }
            if args2.len() > 0 { args2.remove(0); }
            if args2.len() > 0 { args2.remove(0); }

            let mut x = 0;
            while x < args2.len() {
                let key = args2[x].to_string();
                if json.contains_key(key.as_str()) && x + 1 < args2.len() {
                    json.remove(key.as_str());

                    json.insert(args2[x].as_str(), args2[x + 1].as_str());
                }
                x = x + 2;
            }

            while arg_count > 0 {
                let url3 = format!("{}/sdapi/v1/txt2img", url.to_string());
                self.call_api(url3.to_string(), &arg_prompt, json.clone());
                
                arg_count = arg_count - 1;
            }

            arg_prompt = String::from("");
            first = false;
        }
    }

    fn call_api(&self, url3: String, arg_prompt: &String, json: HashMap<&str, &str>) -> bool {
        println!("Requesting {}", url3.to_string());
        let str = http_fast_reqwest::post_json(1, url3.to_string(), "POST".to_string(), json.clone());
        match str {
            Ok(x) => {
                let jsonx = serde_json::from_str::<Value>(x.as_str());
                match jsonx {
                    Ok(j) => {
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
                                                    let b = self.save_image(&arg_prompt, o.to_string());

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
                                        println!("Couldn't parse json");
                                    }
                                }
                            },
                            None => {
                                println!("Couldn't parse json");
                            }
                        }
                    },
                    Err(x) => {
                        println!("{:?}", x);
                    }
                }
            },
            Err(x) => {
                println!("{:?}", x);
            }
        }
        return false;
    }
}