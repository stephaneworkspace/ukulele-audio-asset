use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cc::Build::new()
        .flag("-g")
        .flag("-Wall")
        .file(Path::new(&dir).join("src/lamec/convert.c"))
        .compile("lamec")
}
