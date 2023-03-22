use std::{env, path::PathBuf, fs};

use lulu::disassemble;

#[test]
fn it_should_disassemble_hello_world_sample_with_debug_data() {
    let compiled_dir: PathBuf = env::current_dir()
        .unwrap()
        .join("tests")
        .join("compiled")
        .join("hello_world.lu");

    let file = fs::read(compiled_dir).unwrap();
    let dis = disassemble(file);

    assert_eq!(dis.function.instructions.len(), 4);
    assert_eq!(dis.function.constants.len(), 2);
    assert_eq!(dis.function.prototypes.len(), 0);
    assert_eq!(dis.function.line_positions.len(), 4);
    assert_eq!(dis.function.locals.len(), 0);
    assert_eq!(dis.function.up_values.len(), 0);
}

#[test]
fn it_should_disassemble_hello_world_sample_without_debug_data() {
    let compiled_dir: PathBuf = env::current_dir()
        .unwrap()
        .join("tests")
        .join("compiled")
        .join("hello_world.lus");

    let file = fs::read(compiled_dir).unwrap();
    let dis = disassemble(file);

    assert_eq!(dis.function.instructions.len(), 4);
    assert_eq!(dis.function.constants.len(), 2);
    assert_eq!(dis.function.prototypes.len(), 0);
    assert_eq!(dis.function.line_positions.len(), 0);
    assert_eq!(dis.function.locals.len(), 0);
    assert_eq!(dis.function.up_values.len(), 0);
}
