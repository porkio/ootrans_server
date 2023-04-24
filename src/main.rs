mod controller;
mod model;
// mod utils;

#[macro_use]
extern crate rocket;

use core::result::Result::Ok;

use crate::model::request_args::InputArgs;
use crate::model::request_args::RequestArgs;
use jwt_simple::Error;
// use reqwest::Error;
use rocket::serde::json::{Json, Value};
use serde_json::json;

use crate::model::response::RespData;

#[get("/")]
fn index() -> &'static str {
    "hello world!"
}

#[post("/translate", format = "application/json", data = "<post_data>")]
async fn translate(post_data: Json<InputArgs>) -> Value {
    println!("{:?}", post_data);
    let input_args = InputArgs::new(&post_data);

    let request_args = RequestArgs::new(&input_args);

    println!("request_args: {:?}\n", request_args);

    let res = request_trans_api(&request_args).await;

    // println!("{:?}", res);

    match res {
        Ok(s) => {
            return json!(RespData {
                code: 200,
                msg: "Success!",
                data: s,
            })
        }
        Err(_) => {
            return json!(RespData {
                code: 500,
                msg: "Translate Error!",
                data: (),
            })
        }
    }
}

async fn request_trans_api(trans_args: &RequestArgs) -> Result<Option<Value>, Error> {
    let params = [
        ("q", trans_args.q.clone()),
        ("from", trans_args.from.clone()),
        ("to", trans_args.to.clone()),
        ("appKey", trans_args.get_app_key()),
        ("salt", trans_args.salt.clone()),
        ("sign", trans_args.get_sign()),
        ("signType", trans_args.sign_type.clone()),
        ("curtime", trans_args.curtime.clone()),
        ("ext", trans_args.ext.clone()),
    ];

    let client = reqwest::Client::new();

    let res = client
        .post("https://openapi.youdao.com/api")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let res_obj: Value = serde_json::from_str(&res)?;

    // println!("{}", res_obj["translation"][0]);
    // println!("{:#?}", res);

    Ok(Some(res_obj))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, translate])
}
