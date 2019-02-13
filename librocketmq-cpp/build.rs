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

    let _output = Command::new("sh")
        .arg("rocketmq-client-cpp/build.sh")
        .output()
        .unwrap();

    //let hello = output.stdout;
    //println!("{:?}", hello);
}
