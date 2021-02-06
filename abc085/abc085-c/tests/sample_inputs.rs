use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"9 45000
"#,
        )
        .tee_output()
        .expect_success();
    assert_correctness(9, 45000, output.stdout_str());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"20 196000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1 -1 -1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1000 1234000
"#,
        )
        .tee_output()
        .expect_success();
    assert_correctness(1000, 1234000, output.stdout_str());
    assert!(output.stderr_str().is_empty());
}

fn assert_correctness(vol: usize, total: usize, s: &str) {
    let mut iter = s.split_whitespace();
    let ten_k: usize = iter.next().unwrap().parse().unwrap();
    let five_k: usize = iter.next().unwrap().parse().unwrap();
    let one_k: usize = iter.next().unwrap().parse().unwrap();
    assert_eq!(one_k + five_k + ten_k, vol);
    assert_eq!(one_k * 1000 + five_k * 5000 + ten_k * 10000, total);
}
