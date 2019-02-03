use clap::{Arg, App};
use rget::parse_url;

fn main() {
   let matches = App::new("Rget")
       .version("0.1.0")
       .author("Matthias Lambrecht <mats.lan.pod@googlemail.com>")
       .about("wget clone written in Rust")
       .arg(Arg::with_name("URL")
                   .required(true)
                   .index(1)
                   .help("url to download"))
       .get_matches();
   let url = parse_url(matches.value_of("URL").unwrap());
   println!("{:?}", url);
}
