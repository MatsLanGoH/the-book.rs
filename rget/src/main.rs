use clap::{Arg, App};

fn main() {
   let matches = App::new("Rget")
       .version("0.1.0")
       .author("Left Blank <leftblank@fornow.com")
       .about("wget clone written in Rust")
       .arg(Arg::with_name("URL")
                   .required(true)
                   .index(1)
                   .help("url to download"))
       .get_matches();
   let url = matches.value_of("URL").unwrap();
   println!("{}", url);
}
