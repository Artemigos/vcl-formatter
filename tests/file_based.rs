use std::{
    io::{BufWriter, Write},
    process::{Command, Output, Stdio},
};

fn process_vcl(data: &[u8], indent: &str) -> Output {
    let path = env!("CARGO_BIN_EXE_vcl-formatter");
    let child = Command::new(path)
        .arg("-i")
        .arg(indent)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    {
        let mut stdin = child.stdin.as_ref().unwrap();
        let mut writer = BufWriter::new(&mut stdin);
        writer.write_all(data).unwrap();
    }

    child.wait_with_output().unwrap()
}

fn verify_unchanged(data: &[u8], indent: &str) {
    let output = process_vcl(data, indent);
    assert_eq!(&output.stdout[..], data);
}

const EXAMPLE_GOOD: &[u8] = include_bytes!("files/example/good.vcl");
const NESTED_EXPR_GOOD: &[u8] = include_bytes!("files/nested_expr/good.vcl");

#[test]
fn example_stays_unchanged() {
    verify_unchanged(EXAMPLE_GOOD, "4")
}

#[test]
fn nested_expr_stays_unchanged() {
    verify_unchanged(NESTED_EXPR_GOOD, "4")
}
