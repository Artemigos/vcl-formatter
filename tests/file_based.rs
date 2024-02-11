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
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    {
        let mut stdin = child.stdin.as_ref().unwrap();
        let mut writer = BufWriter::new(&mut stdin);
        writer.write_all(data).unwrap();
    }

    child.wait_with_output().unwrap()
}

fn verify_formatting(input: &[u8], expected: &[u8], indent: &str) {
    let output = process_vcl(input, indent);
    assert_eq!(&output.stdout[..], expected);
}

fn verify_error(input: &[u8], expected: &[u8], indent: &str) {
    let output = process_vcl(input, indent);
    assert_eq!(&output.stderr[..], expected);
}

fn verify_unchanged(data: &[u8], indent: &str) {
    verify_formatting(data, data, indent)
}

const EXAMPLE_GOOD: &[u8] = include_bytes!("files/example/good.vcl");
const NESTED_EXPR_GOOD: &[u8] = include_bytes!("files/nested_expr/good.vcl");
const CORRUPTED_UNKNOWN_TOKEN: &[u8] = include_bytes!("files/corrupted/unknown_token.vcl");
const CORRUPTED_UNEXPECTED_TOKEN: &[u8] = include_bytes!("files/corrupted/unexpected_token.vcl");

#[test]
fn example_stays_unchanged() {
    verify_unchanged(EXAMPLE_GOOD, "4")
}

#[test]
fn nested_expr_stays_unchanged() {
    verify_unchanged(NESTED_EXPR_GOOD, "4")
}

#[test]
fn correct_unknown_token_error() {
    verify_error(
        CORRUPTED_UNKNOWN_TOKEN,
        b"Error: Unknown token (line=3, column=1)\n",
        "4",
    );
}

#[test]
fn correct_unexpected_token_error() {
    verify_error(
        CORRUPTED_UNEXPECTED_TOKEN,
        b"Error: Unexpected token \"none\" (line=3, column=1)\n",
        "4",
    );
}
