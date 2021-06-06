use std::{env, fs::File, io::Write, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).expect("Could not create file");

    let max_dimensions = option_env!("MAX_DIMENSIONS");
    let max_dimensions = max_dimensions
        .map_or(Ok(10_000), str::parse)
        .expect("Could not parse MAX_DIMENSIONS");

    write!(&mut f, "const MAX_DIMENSIONS: usize = {};", max_dimensions)
        .expect("Could not write file");
    println!("cargo:rerun-if-env-changed=MAX_DIMENSIONS");
}
