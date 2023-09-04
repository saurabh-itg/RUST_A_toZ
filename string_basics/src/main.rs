fn main() {
    let str1 = "Hello, ";
    let str2 = "Rust!";
    let result = format!("{}{}", str1, str2);
    println!("{}", result);
    length();
    substring();
    cases();
    replace_char();
    split_join();
    whitespace_removal();

}
fn length() {
    let text = "Rust Programming";
    let length = text.len();
    println!("string is : '{}'",text);
    println!("Length of the string: {}", length);
}
fn substring() {
    let sentence = "Rust is fun!";
    let substring = &sentence[0..4];  // Extracts "Rust"
    println!("Substring: {}", substring);
}
fn cases() {
    let text = "Rust Programming";
    let upper = text.to_uppercase();
    let lower = text.to_lowercase();
    println!("Uppercase: {}", upper);
    println!("Lowercase: {}", lower);
}
fn replace_char() {
    let text = "Hello, world!";
    let replaced = text.replace("world", "Rustacians");
    println!("{}", replaced);
}
fn split_join() {
    let text = "Rust,is,awesome";
    let parts: Vec<&str> = text.split(",").collect();
    let joined = parts.join(" ");
    println!("Joined string: {}", joined);
}
fn whitespace_removal() {
    let input = "   Rust   ";
    let trimmed = input.trim();
    println!("Trimmed: '{}'", trimmed);
}