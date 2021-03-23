use std::{env, error::Error, path::PathBuf};

use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=asm-helpers");
    Build::new().file("asm.s").compile("asm-helpers");
    println!("cargo:rerun-if-changed=asm.s");
    Ok(())
}
