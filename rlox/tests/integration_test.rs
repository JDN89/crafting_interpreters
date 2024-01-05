// tests/integration_test.rs

extern crate _lox;

use _lox::tree_walker::interpreter::Interpreter;
use _lox::user_interface::run;


#[test]
fn test_scope() {
    let code_block_test = String::from(
        r#"var a = "global a";
var b = "global b";
var c = "global c";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;"#,
    );

    let mut interpreter = Interpreter::new();

    // Second test
    let _ = run(&code_block_test, &mut interpreter);
    let output = interpreter.get_outpout();
    let expected = r#" inner a
    outer b
    global c
    outer a
    outer b
    global c
    global a
    global b
    global c
    "#;
    let processed_expected = remove_whitespace(expected);

    let output_str = String::from_utf8_lossy(&output)
        .lines()
        .map(|line| line)
        .collect::<Vec<&str>>()
        .join(" ");

    println!("Actual Output:\n{}", output_str);
    println!("Expected Output:\n{}", processed_expected);

    assert_eq!(output_str, processed_expected.trim());
}

// Helper function to remove whitespace for comparison
fn remove_whitespace(input: &str) -> String {
    input.lines().map(|line| line.trim()).collect::<Vec<&str>>().join("")
}