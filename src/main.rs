use lox::Lox;

mod lox;

fn main() {
    let mut l = Lox::new();
    l.run_file("test.lox".to_string());
}
