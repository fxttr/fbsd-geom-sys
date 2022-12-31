use bindgen::Builder;
use std::env;
use std::path::PathBuf;

#[cfg(target_os = "freebsd")]
fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").expect("Variable OUT_DIR not set. Aborting."))
        .join("bindings.rs");
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG_PATH");
    println!("cargo:rustc-link-lib=geom");
    println!("cargo:rerun-if-changed=/usr/include/libgeom.h");
    println!("cargo:rerun-if-changed=/usr/include/sys/devicestat.h");

    Builder::default()
        .header("/usr/include/libgeom.h")
        .header("/usr/include/sys/devicestat.h")
        .whitelist_function("geom_.*")
        .whitelist_function("gctl_.*")
        .whitelist_function("g_.*")
        .whitelist_type("devstat_trans_flags")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Building bindings failed.")
        .write_to_file(out)
        .expect("Could not write bindings to OUT_DIR.");
}
