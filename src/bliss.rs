mod parse;
mod tokenizer;
use tokenizer::Lexer;
use parse::Parser;
fn main() {
    let s: String = String::from(r#" [4] { [1-2] "hello"; [3] "yes"; };"#);

    let mut l: Lexer = Lexer::new(s);
    l.init();
    println!("{:?}", l.get_output());
    let parser = Parser::new(l);
    println!("{}", parser.parse());
}