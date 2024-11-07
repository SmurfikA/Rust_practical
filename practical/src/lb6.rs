#[test]
fn test1() {
    println!("Success!");
}
#[test]
fn test() {
    let s: &str = "hello, world";
    greetings(s)
}
fn greetings(s: &str) {
    println!("{}",s)
}
