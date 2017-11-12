extern crate ll1_lexer;

use std::env;

use ll1_lexer::list_lexer;
use ll1_lexer::token;

fn main() {
    let input = env::args().nth(1).unwrap();
    let mut output = String::new();
    output = drive(input, output);

    println!("{}", output)
}

fn drive(input: String, mut output: String) -> String {
    let mut lexer = list_lexer::ListLexer::new(input);
    let mut token = lexer.next_token();

    while token.token_type != token::TokenType::EOF {
        output.push_str(format!("{}\n", token).as_str());
        token = lexer.next_token()
    }

    output.push_str(format!("{}", token).as_str());
    output
}

#[test]
fn test_drive_1() {
    let input = String::from("[a, b]");
    let mut output = String::new();
    let expected =
r#"<'[',LBrack>
<'a',Name>
<',',Comma>
<'b',Name>
<']',RBrack>
<'<EOF>',<EOF>>"#;

    output = drive(input, output);
    assert_eq!(expected, output.as_str());
}

#[test]
fn test_drive_2() {
    let input = String::from("[usagi,[pyon,  poin], b]");
    let mut output = String::new();
    let expected =
r#"<'[',LBrack>
<'usagi',Name>
<',',Comma>
<'[',LBrack>
<'pyon',Name>
<',',Comma>
<'poin',Name>
<']',RBrack>
<',',Comma>
<'b',Name>
<']',RBrack>
<'<EOF>',<EOF>>"#;

    output = drive(input, output);
    assert_eq!(expected, output.as_str());
}
