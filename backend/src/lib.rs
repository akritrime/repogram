#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
// extern crate serde_json;


mod posts;

macro_rules! pipe {
    ( $x:expr $( => $func:path )* ) => {
        {
            let ret = $x;
            $(
                let ret = $func(ret);
            )*
            ret
        }
    };
}

pub(crate) fn root_prefix(prefix: &str) -> String {
    format!("api/{}", prefix)
}

fn server() -> rocket::Rocket {
    pipe! {
        rocket::ignite()
        => posts::routes 
    }
}

pub fn it_works() -> rocket::error::LaunchError {
    server().launch()
}