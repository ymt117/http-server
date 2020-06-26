fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    let str = "Hello, world!";
    assert_eq!("Hello, world!", str);
}