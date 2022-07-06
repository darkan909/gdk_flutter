extern crate cheddar;

fn main() {
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("gdk_flutter").expect("malformed module path")
        .run_build("target/include/rusty.h");
}