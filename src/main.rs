mod sentences;

fn main() {
    let welcome_message = sentences::welcome_message();
    println!("Welcome back. {welcome_message}");
}
