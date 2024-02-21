mod avideb;
mod token;
mod scanner;

fn main() {
    avideb::visible();

    let mut l = scanner::Scanner::new("main".into(), "repl".into(), |pos, msg| {
        println!("Lexing Error: {msg}. line {}, column {}, {}", pos.line, pos.column, pos.filename);
    });

    println!("tok = {:?}", l.scan());
}
