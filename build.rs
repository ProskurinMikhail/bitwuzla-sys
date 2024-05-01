use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
extern crate pkg_config;

fn main() {
    let dir_bitwuzla: PathBuf;
    dir_bitwuzla = env::current_dir().unwrap().join("bitwuzla");
    if !dir_bitwuzla.exists() {
        let mut my_command = Command::new("git")
            .arg("clone")
            .arg("https://github.com/bitwuzla/bitwuzla")
            .status()
            .unwrap();

        let mut my_command = Command::new("./configure.py")
            .arg("--static")
            .current_dir(&dir_bitwuzla)
            .status()
            .unwrap();

        let dir_tmp = env::current_dir().unwrap().join("bitwuzla/build");
        let mut my_command = Command::new("ninja")
            .arg("install")
            .current_dir(dir_tmp)
            .status()
            .unwrap();
    }

    // let pkg_conf = pkg_config::Config::new()
    //     .statik(true)
    //     .probe("bitwuzla")
    //     .unwrap();

    // let library = pkg_config::probe_library("bitwuzla").unwrap();

    // println!("cargo::rerun-if-changed=build.rs");
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     dir_bitwuzla.join("build/subprojects/cadical-rel-1.7.4/src").display()
    //  );
    // println!(
    //     "cargo:rustc-link-search=native=/usr/local/include",
    // );
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     dir_bitwuzla.join("build/src").display()
    // );

    // println!("cargo:rustc-link-lib=static=cadical");

    // println!("cargo:rustc-link-lib=static=bitwuzla");

    // println!("cargo:rustc-link-lib=static=bzlautil");

    // println!("cargo:rustc-link-lib=static=bitwuzlabv");
    // println!("cargo:rustc-link-lib=static=bitwuzlabb");
    // println!("cargo:rustc-link-lib=static=bitwuzlals");
    // println!("cargo:rustc-link-lib=static=bzlarng");

    // println!("cargo:rustc-link-lib=gmp");
    // println!("cargo:rustc-link-lib=stdc++");

    // println!("cargo:include={}", dir_bitwuzla.join("include").display());

    // let include_paths = pkg_conf
    //     .include_paths
    //     .iter()
    //     .map(|p| format!("-I{}", p.to_str().expect("")))
    //     .collect::<Vec<_>>();


    println!("cargo:include=/usr/local/include");
    println!("cargo:rustc-link-search=native=/usr/local/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=static=bitwuzla");
    println!("cargo:rustc-link-lib=static=bitwuzlals");
    println!("cargo:rustc-link-lib=static=bitwuzlabv");
    println!("cargo:rustc-link-lib=static=bitwuzlabb");
    println!("cargo:rustc-link-lib=gmp");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // .clang_args(&["-I/usr/local/include","-L/usr/local/lib/x86_64-linux-gnu", "-lbitwuzla", "-lbitwuzlals", "-lbitwuzlabv", "-lbitwuzlabb", "-lgmp"])
        // .clang_args(library.include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo::rerun-if-changed=build.rs");
}
