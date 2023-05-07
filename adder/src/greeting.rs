pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn bad_greeting(name: &str) -> String {
    String::from("Hello")
}