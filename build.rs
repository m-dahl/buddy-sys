// build.rs

use std::path::PathBuf;
use std::fs;
use std::env;
use std::io::Write;
use bindgen::callbacks::*;

#[derive(Debug, PartialEq, Clone)]
struct Macros {
}

impl ParseCallbacks for Macros {
    // change defines to i32 for the bdd operations
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            "bddop_and" => Some(IntKind::Int),
            "bddop_xor" => Some(IntKind::Int),
            "bddop_or" => Some(IntKind::Int),
            "bddop_nand" => Some(IntKind::Int),
            "bddop_nor" => Some(IntKind::Int),
            "bddop_imp" => Some(IntKind::Int),
            "bddop_biimp" => Some(IntKind::Int),
            "bddop_diff" => Some(IntKind::Int),
            "bddop_less" => Some(IntKind::Int),
            "bddop_invimp"=> Some(IntKind::Int),
            _ => None,
        }
    }
}

fn main() {
    let config_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("include");
    fs::create_dir_all(&config_dir).unwrap();
    let mut config_h = fs::File::create(config_dir.join("config.h")).unwrap();
    let vendor_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("vendor");

    println!("cargo:include={}", env::join_paths(&[&config_dir, &vendor_dir]).unwrap().to_str().unwrap());

    write!(config_h, r#"
        #define MAJOR_VERSION 2
        #define MINOR_VERSION 4
        #define PACKAGE_VERSION "2.4"
        "#,
    ).unwrap();

    cc::Build::new()
        .include(&config_dir)
        .include(&vendor_dir)
        .pic(true)
        .file("vendor/bddio.c")
        .file("vendor/bddop.c")
        .file("vendor/bvec.c")
        .file("vendor/cache.c")
        .file("vendor/fdd.c")
        .file("vendor/imatrix.c")
        .file("vendor/kernel.c")
        .file("vendor/pairs.c")
        .file("vendor/prime.c")
        .file("vendor/reorder.c")
        .file("vendor/tree.c")
        .compile("buddy");

    let builder = bindgen::Builder::default()
        .header("vendor/bdd.h")
        .header("vendor/fdd.h")
        .whitelist_type("BDD")
        .whitelist_type("(b|f)dd.*")
        .whitelist_function("(b|f)dd.*")
        .whitelist_var("(b|f)dd.*")
        .parse_callbacks(Box::new(Macros {}))
        .derive_copy(false);

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bdd.rs"))
        .expect("Couldn't write bindings!");
}
