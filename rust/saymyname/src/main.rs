fn main() {
    let name = say_my_name("Slim Shady");
    println!("{}", name);
}


fn say_my_name(name: &str) -> String {
    return format!("My name is: {}", name);
}