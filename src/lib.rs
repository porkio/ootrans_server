// extern crate dotenv;
// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
// use dotenv::dotenv;
// use rocket::serde::Deserialize;
// use rocket::FromForm;
// use serde_json::Value;
// use std::collections::HashMap;
// use std::env;
// use std::error::Error;
// use std::time::{SystemTime, UNIX_EPOCH};

// // 请求数据
// pub struct RequestArgs {
//     app_key: String,       // appKey
//     pub q: String,         // 查询内容
//     pub from: String,      // 源语言
//     pub to: String,        // 目标语言
//     pub curtime: String,   // 当前时间戳
//     pub salt: String,      // 密码盐
//     pub sign_type: String, // 签名类型
//     sign: String,          // 签名
//     pub strict: String,    // 是否严格按照指定from和to进行翻译
//     pub ext: String,       // 结果转mp3
// }

// impl RequestArgs {
//     pub fn new(input_args: &InputArgs) -> RequestArgs {
//         dotenv().ok();

//         let envs: HashMap<String, String> = env::vars().collect();

//         println!("{:?}", envs);

//         let app_key = envs.get("APP_KEY").unwrap();
//         let app_secret = envs.get("APP_SECRET").unwrap();

//         let curtime = match SystemTime::now().duration_since(UNIX_EPOCH) {
//             Ok(n) => n.as_secs().to_string(),
//             Err(error) => panic!("SystemTime before UNIX EPOCH!"),
//         };

//         let salt = match SystemTime::now().duration_since(UNIX_EPOCH) {
//             Ok(n) => n.as_millis().to_string(),
//             Err(error) => panic!("SystemTime before UNIX EPOCH!"),
//         };

//         let sign_content = format!(
//             "{}{}{}{}{}",
//             app_key, input_args.q, salt, curtime, app_secret
//         );

//         let mut hasher = Sha256::new();

//         hasher.input_str(&sign_content);

//         let sign = hasher.result_str();

//         RequestArgs {
//             app_key: app_key.to_string(),
//             q: input_args.q.to_string(),
//             from: input_args.from.to_string(),
//             to: input_args.to.to_string(),
//             curtime: curtime,
//             salt: salt,
//             sign_type: String::from("v3"),
//             sign: sign,
//             ext: if input_args.voice == true {
//                 String::from("mp3")
//             } else {
//                 String::from("")
//             },
//             strict: String::from("false"),
//         }
//     }

//     pub fn get_app_key(&self) -> String {
//         self.app_key.clone()
//     }

//     pub fn get_sign(&self) -> String {
//         self.sign.to_string()
//     }
// }

// // 用户输入数据
// #[derive(Deserialize, Debug)]
// pub struct InputArgs {
//     pub q: String,                   // 查询内容
//     pub from: String,                // 源语言
//     pub to: String,                  // 目标语言
//     pub voice: bool,                 // 翻译结果发音选择
//     pub speak_origin_language: bool, // 是否阅读源语言
//     pub speak_target_language: bool, // 是否阅读目标语言结果
// }

// impl InputArgs {
//     pub fn new(args: &Vec<String>) {
//         let shell_mode = args.contains(&String::from("-shell"));
//         let res_voice = args.contains(&String::from("-b"));

//         let to = args.contains(&String::from(":en"));
//         for (i, item) in args.iter().enumerate() {}

//         println!("shell_mode: {}, res_voice: {}", shell_mode, res_voice);
//     }
// }
