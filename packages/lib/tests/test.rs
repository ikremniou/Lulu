use std::{env, path::PathBuf, process::Command};

use lulu_lib::disassemble;

fn compile_lua(test_file: PathBuf, is_debug: bool) -> Vec<u8> {
    let lua_compiler_path: PathBuf = env::current_dir()
        .unwrap()
        .join("..")
        .join("..")
        .join("extern")
        .join("bin")
        .join("luac");

    let mut binding = Command::new(lua_compiler_path);
    binding.arg("-o").arg("-");
    if is_debug {
        binding.arg("-s");
    }
    binding.arg(test_file);

    let output = match binding.output() {
        Ok(output) => output,
        Err(e) => panic!("Failed to execute command: {e}"),
    };
    return output.stdout;
}

#[test]
fn it_should_disassemble_hello_world_sample_with_debug_data() {
    let test_file: PathBuf = env::current_dir()
        .unwrap()
        .join("tests")
        .join("assets")
        .join("hello_world.lua");
    let file = compile_lua(test_file, false);
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
    let test_file: PathBuf = env::current_dir()
        .unwrap()
        .join("tests")
        .join("assets")
        .join("hello_world.lua");

    let file = compile_lua(test_file, true);
    let dis = disassemble(file);

    assert_eq!(dis.function.instructions.len(), 4);
    assert_eq!(dis.function.constants.len(), 2);
    assert_eq!(dis.function.prototypes.len(), 0);
    assert_eq!(dis.function.line_positions.len(), 0);
    assert_eq!(dis.function.locals.len(), 0);
    assert_eq!(dis.function.up_values.len(), 0);
}
