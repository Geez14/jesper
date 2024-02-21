mod avideb;
mod token;
mod scanner;

fn main() {
    avideb::visible();

    let mut l = scanner::Scanner::new("main".into(), "repl".into());

    println!("tok = {:?}", l.scan());
}
