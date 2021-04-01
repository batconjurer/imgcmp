use clap::{App, Arg};

mod average_hash;
mod errors;
use crate::average_hash::are_images_equal;
use std::path::Path;


fn main() {
    let matches = App::new("imgcmp")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line tool to test if images are equal")
        .arg(
            Arg::with_name("image_one")
                .takes_value(true)
                .required(true)
                .help("File path of an image")
        )
        .arg(
            Arg::with_name("image_two")
                .takes_value(true)
                .required(true)
                .help("File path of an image")
        )
        .get_matches();
    if are_images_equal(Path::new(matches.value_of("image_one").unwrap()),
                        Path::new(matches.value_of("image_two").unwrap())).unwrap() {
        println!("Pictures are the same");
    } else{
        println!("Pictures are different");
    }
}
