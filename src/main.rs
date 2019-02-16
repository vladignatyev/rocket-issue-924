#![allow(unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(type_alias_enum_variants)]

#![feature(const_str_len)]

#[macro_use] extern crate rocket;

use rocket::http::Status;

#[get("/<alias>")]
fn example(alias: String) -> Status {
    Status::Ok
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![example])
}


fn main() {
    rocket().launch();
}
