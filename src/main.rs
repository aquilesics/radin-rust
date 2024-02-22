// #[macro_use] extern crate rocket;
// use serde::Deserialize;
use std::{env, ffi::{OsStr, OsString}, str::FromStr};




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

fn main() -> (){
    
    struct SfCred {
        sf_id:String,
        sf_sct:String 
    }

    let sf_t  = SfCred{
        sf_id : get_env( "sf_id" ),
        sf_sct: get_env( "sf_sct" ) 
    };

    println!("{}\n{}",sf_t.sf_id,sf_t.sf_sct)
}