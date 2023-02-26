use std::{env, path::PathBuf, fs};

use lulu::disassemble;

#[test]
fn it_should_do() {
    let compiled_dir: PathBuf = env::current_dir()
        .unwrap()
        .join("tests")
        .join("compiled")
        .join("hello_world.lu");

    let file = fs::read(compiled_dir).unwrap();
    disassemble(file);
}
