fn main() {
    function();
}

//修复所有权常见错误
pub fn function() {
    println!("-----------------------------------------");
    println!("修复所有权常见错误");
    let value = return_a_string();
    println!("The value is: {}", value);

}
fn return_a_string() -> &'static str{
    "Hello zzc"
}