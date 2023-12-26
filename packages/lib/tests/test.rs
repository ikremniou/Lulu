use std::{env, io::Read, path::PathBuf, process::Command};

use lulu_lib::disassemble;

fn compile_lua(file_name: &str, is_debug: bool) -> Vec<u8> {
    let test_location = env::current_dir().unwrap().join("tests").join("assets");
    let test_file_path: PathBuf = test_location.join(file_name);
    let compiled_file_path: PathBuf = test_location.join(format!("{}c", file_name));

    let lua_compiler_path: PathBuf = env::current_dir()
        .unwrap()
        .join("..")
        .join("..")
        .join("extern")
        .join("bin")
        .join("luac");

    let mut binding = Command::new(lua_compiler_path);
    binding.arg("-o").arg(&compiled_file_path);
    if is_debug {
        binding.arg("-s");
    }
    binding.arg(test_file_path);
    binding.output().unwrap();

    let mut file = std::fs::File::open(&compiled_file_path).unwrap();
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();
    std::fs::remove_file(compiled_file_path).unwrap();
    return content;
}

#[test]
fn it_should_disassemble_hello_world_sample_with_debug_data() {
    let file = compile_lua("hello_world.lua", false);
    let dis = disassemble(file).unwrap();

    assert_eq!(dis.function.instructions.len(), 4);
    assert_eq!(dis.function.constants.len(), 2);
    assert_eq!(dis.function.prototypes.len(), 0);
    assert_eq!(dis.function.line_positions.len(), 4);
    assert_eq!(dis.function.locals.len(), 0);
    assert_eq!(dis.function.up_values.len(), 0);
}

#[test]
fn it_should_disassemble_hello_world_sample_without_debug_data() {
    let file = compile_lua("hello_world.lua", true);
    let dis = disassemble(file).unwrap();

    assert_eq!(dis.function.instructions.len(), 4);
    assert_eq!(dis.function.constants.len(), 2);
    assert_eq!(dis.function.prototypes.len(), 0);
    assert_eq!(dis.function.line_positions.len(), 0);
    assert_eq!(dis.function.locals.len(), 0);
    assert_eq!(dis.function.up_values.len(), 0);
}

#[test]
fn it_should_disassemble_fib_with_debug_data() {
    let file = compile_lua("fib.lua", false);
    let dis = disassemble(file).unwrap();

    // one global function defined only
    assert_eq!(dis.function.prototypes.len(), 1);
}

#[test]
fn it_should_disassemble_local_func_with_debug_data() {
    let file = compile_lua("local_func.lua", true);
    let dis = disassemble(file).unwrap();

    // one global function and 2 local functions
    assert_eq!(dis.function.prototypes.len(), 3);
    assert_eq!(dis.function.locals.len(), 0)
}

#[test]
fn it_should_disassemble_local_vars_with_debug_data() {
    let file = compile_lua("local_vars.lua", false);
    let dis = disassemble(file).unwrap();

    // 7 local vars defined. 0 functions defined
    assert_eq!(dis.function.prototypes.len(), 0);
    assert_eq!(dis.function.locals.len(), 7);
}

#[test]
fn it_should_disassemble_math_with_debug_data() {
    let file = compile_lua("math.lua", true);
    let dis = disassemble(file).unwrap();

    assert_eq!(dis.function.constants.len(), 21);
}

#[test]
fn it_should_disassemble_use_fib_with_debug_data() {
    let file = compile_lua("use_fib.lua", false);
    let dis = disassemble(file).unwrap();

    assert_eq!(dis.function.up_values.len(), 0);
}

#[test]
fn it_should_disassemble_up_values_with_debug_data() {
    let file = compile_lua("up_values.lua", true);
    let dis = disassemble(file).unwrap();

    assert_eq!(dis.function.up_values.len(), 0);
    assert_eq!(dis.function.prototypes.len(), 1);
}
