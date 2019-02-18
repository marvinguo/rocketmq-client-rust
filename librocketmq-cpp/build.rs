use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command};


fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!(
            "The `{}` directory is empty, did you forget to pull the submodules?",
            name
        );
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

fn main() {
    fail_on_empty_directory("rocketmq-client-cpp");

    println!("cargo:rustc-link-lib=dylib=rocketmq-client-cpp");
    println!("cargo:rustc-link-search=native=/rocketmq-client-cpp/bin");

    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        .header("./rocketmq-client-cpp/include/CMessage.h")
        .header("./rocketmq-client-cpp/include/CProducer.h")
        .header("./rocketmq-client-cpp/include/CPullConsumer.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    let _output = Command::new("sh")
        .arg("rocketmq-client-cpp/build.sh")
        .output()
        .unwrap();

    //let hello = output.stdout;
    //println!("{:?}", hello);
}
