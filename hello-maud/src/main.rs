#![feature(proc_macro_hygiene)]

extern crate actix;
extern crate actix_web;
extern crate maud;

use maud::{html, Markup};
use actix_web::{App, server, Path, http::Method};

fn index(params: Path<(String, u32)>) -> Markup {
    html! {
        h1 { "Hello " (params.0) " with id " (params.1) "!"}
    }
}

fn main() {
    let sys = actix::System::new("maud-example");

    server::new(move || {
        App::new()
            .resource("/user/{name}/{id}", |r| {
                r.method(Method::GET).with(index)
            })
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    let _ = sys.run();
}
