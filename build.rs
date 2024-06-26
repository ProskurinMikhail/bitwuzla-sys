use std::env;

use std::path::PathBuf;
use std::process::Command;
extern crate pkg_config;

fn main() {
    let dir_bitwuzla: PathBuf;
    dir_bitwuzla = env::current_dir().unwrap().join("bitwuzla");
    if !dir_bitwuzla.exists() {
        let mut _my_command = Command::new("git")
            .arg("clone")
            .arg("https://github.com/bitwuzla/bitwuzla")
            .status()
            .unwrap();
    }
    if !dir_bitwuzla.join("build").exists() {
        let mut _my_command = Command::new("./configure.py") // --prefix
            .current_dir(&dir_bitwuzla)
            .status()
            .unwrap();

        let dir_tmp = env::current_dir().unwrap().join("bitwuzla/build");
        let mut _my_command = Command::new("ninja").current_dir(dir_tmp).status().unwrap();
    }

    let old = env::var("PKG_CONFIG_PATH");
    let pkg_config_dir: PathBuf = dir_bitwuzla.join("build/meson-uninstalled"); // --prefix path

    match old {
        Ok(ref s) => {
            let mut paths = env::split_paths(s).collect::<Vec<PathBuf>>();
            paths.push(pkg_config_dir);
            let paths = env::join_paths(paths).unwrap();
            env::set_var("PKG_CONFIG_PATH", paths)
        }
        Err(_) => env::set_var("PKG_CONFIG_PATH", pkg_config_dir),
    }

    let library = pkg_config::probe_library("bitwuzla").unwrap();

    env::set_var("PKG_CONFIG_PATH", old.unwrap_or_else(|_| "".into()));

    println!("cargo:rustc-link-lib=stdc++");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(
            library
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .clang_args(
            library
                .link_paths
                .iter()
                .map(|path| format!("-L{}", path.to_string_lossy())),
        )
        .clang_args(library.libs.iter().map(|name| format!("-l{}", name)))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo::rerun-if-changed=build.rs");
}
