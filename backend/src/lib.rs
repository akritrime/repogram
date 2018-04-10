#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod posts;

#[get("/")]
fn index<'a>() -> &'a str {
    "hello, world!"
}

fn server() -> rocket::Rocket {
    let rkt = rocket::ignite();
    let rkt = rkt.mount("api/", routes![index]);
    let rkt = posts::routes(rkt);
    rkt
}

pub fn it_works() -> rocket::error::LaunchError {
    server().launch()
}