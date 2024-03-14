// #[macro_use] extern crate rocket;
// use serde::Deserialize;
use std::{env, ffi::{OsStr, OsString}, str::FromStr};
use ::http::{Request,Response};



// #[get("/")]
// fn index() -> &'static str{
   
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _{
//     rocket::build().mount("/", routes![index])
// }

fn get_env( k:&str ) -> String {
    match env::var_os(k) {
        Some(val) => val.into_string().unwrap(),
        None => String::from("")
    }
}
#[derive(Debug)]
struct SfCred {
    sf_id:String,
    sf_sct:String
}

impl SfCred {
    fn get_token(&self) -> String{
        format!("id {}",2)
    }
    
}

fn main() -> (){

    let sf_t  = SfCred{
        sf_id : get_env( "sf_id" ),
        sf_sct: get_env( "sf_sct" ),
    };

    println!("{}",sf_t.get_token());

    let rqst = Request::get("https://www.rust-lang.org/").body(()).unwrap();

    println!("{:?}",rqst)
}