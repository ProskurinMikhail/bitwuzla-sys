use std::env;
use std::fs;
use std::os::linux::raw::stat;
use std::str;
use std::path::{Path, PathBuf};
use std::process::Command;

    fn run_command(description: &str, command: &mut Command) {
        println!("*** {}", description);

        let mut myStatus = command.status().unwrap();

        if !myStatus.success() {
            panic!(
                "*** ERROR in action `{}`, exit status {}\n*** Command: {:?}",
                description,
                myStatus,
                command,
            );
        }
        
    }

fn main() {
    let dir_bitwuzla: PathBuf;    
    dir_bitwuzla = env::current_dir().unwrap().join("bitwuzla");
    if !dir_bitwuzla.exists() {
        // run_command("Git clone", Command::new("git").arg("clone").arg("https://github.com/bitwuzla/bitwuzla"));
        // run_command("cd", Command::new("cd").arg("bitwuzla/"));  
        // run_command("configure", Command::new("./configure.py").arg(""));  
        // run_command("cd", Command::new("cd").arg("build/"));                                         
        // run_command("Ninja ", Command::new("ninja").arg(""));    
        let mut my_Command = Command::new("git").arg("clone").arg("https://github.com/bitwuzla/bitwuzla").status().unwrap();
        // let waiting = my_Command.wait().unwrap();

        let mut my_Command = Command::new("./configure.py").current_dir(&dir_bitwuzla).status().unwrap();
        // my_Command.current_dir(dir_bitwuzla);
        // my_Command.spawn().unwrap();

        // let mut child = Command::new("sleep").arg("10").spawn().unwrap();
        // let _result = child.wait().unwrap();
        let dir_tmp = env::current_dir().unwrap().join("bitwuzla/build");
        let mut my_Command = Command::new("ninja").current_dir(dir_tmp).status().unwrap();

        // my_Command.current_dir(dir_tmp);
        // my_Command.spawn().unwrap();  

        // let mut child = Command::new("sleep").arg("30").spawn().unwrap();
        // let _result = child.wait().unwrap();
    }          

    // let dir_bitwuzla: PathBuf;    
    // dir_bitwuzla = env::current_dir().unwrap().join("bitwuzla");
    println!("cargo:include={}", dir_bitwuzla.join("include").display());
    println!("cargo:rustc-link-search=native={}", dir_bitwuzla.join("build/src").display());
    println!("cargo:rustc-link-lib=static=bitwuzla");

    let headers_dir = PathBuf::from("bitwuzla/include");
    let headers_dir_canonical = fs::canonicalize(headers_dir).unwrap();
    let include_path = headers_dir_canonical.to_str();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path.unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}