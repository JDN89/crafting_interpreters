// tests/integration_test.rs

extern crate rulox;

use rulox::tree_walker::interpreter::Interpreter;
use rulox::user_interface::run;

// Helper function to remove whitespace for comparison
fn remove_whitespace(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("")
}

fn convert_to_string(output: Vec<u8>) -> String {
    String::from_utf8_lossy(&output)
        .lines()
        .map(|line| line)
        .collect::<Vec<&str>>()
        .join(" ")
}


#[test]
fn test_scope() {
    // SETUP
    let mut interpreter = Interpreter::new();
    // GIVEN
    let input = String::from(
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

    // WHEN

    match run(&input, &mut interpreter) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    // THEN
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

    let output_str = convert_to_string(output);

    assert_eq!(output_str, processed_expected.trim());
}

#[test]
fn test_grouping() {
    //given
    let mut interpreter = Interpreter::new();
    let input = String::from(
        r#"
        var a = ((1 + 3) * (6-3))/2;
print a;"#,
    );
    let expected = r#" 6
    "#;
    let processed_expected = remove_whitespace(expected);

    //WHEN
    match run(&input, &mut interpreter) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    //THEN
    let output = interpreter.get_outpout();

    let output_str = convert_to_string(output);

    assert_eq!(output_str, processed_expected.trim());
}


#[test]
fn function_declaration() {
    //given
    let mut interpreter = Interpreter::new();
    let input = String::from(
        r#"
        fun sayhi (first,last) { print "Hi, " + first + "" + last + "!"; }
        sayhi("dear","reader");
"#,
    );
    let expected = r#" Hi, dearreader!
    "#;
    let processed_expected = remove_whitespace(expected);

    //WHEN
    match run(&input, &mut interpreter) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    //THEN
    let output = interpreter.get_outpout();

    let output_str = convert_to_string(output);

    assert_eq!(output_str, processed_expected.trim());
}

#[test]
fn return_statement() {
    //given
    let mut interpreter = Interpreter::new();
    let input = String::from(r#"fun foo() {return 1;} print foo(); "#);
    let expected = r#"Hello "#;
    let processed_expected = remove_whitespace(expected);

    //WHEN
    match run(&input, &mut interpreter) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    //THEN
    let output = interpreter.get_outpout();

    let output_str = convert_to_string(output);

    assert_eq!(output_str, processed_expected.trim());
}

#[test]
fn return_statement_v2() {
    //given
    let mut interpreter = Interpreter::new();
    let input = String::from(r#" fun fib(n) {
     if (n <= 1) return n;
    return fib(n - 2) + fib(n - 1);
}
for (var i = 2; i < 4; i = i + 1) {
  print fib(i);
}  "#);
    let expected = r#"Hello "#;
    let processed_expected = remove_whitespace(expected);

    //WHEN
    match run(&input, &mut interpreter) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    //THEN
    let output = interpreter.get_outpout();

    let output_str = convert_to_string(output);

    assert_eq!(output_str, processed_expected.trim());
}

#[test]
fn smaller_or_equals() {
    //given
    let mut interpreter = Interpreter::new();
    let input = String::from(r#" var a =1; if (a <=1) print a; else print "hello";  "#);
    let expected = r#"Hello "#;
    let processed_expected = remove_whitespace(expected);

    //WHEN
    match run(&input, &mut interpreter) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    //THEN
    let output = interpreter.get_outpout();

    let output_str = convert_to_string(output);

    assert_eq!(output_str, processed_expected.trim());
}