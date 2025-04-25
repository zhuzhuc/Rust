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
//如果一个值不拥有堆数据， 那么它可以在不移动的情况下被复制
//一个i32不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 string 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &String 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut String 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<String> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 Vec<String> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Vec<String> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Vec<String> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Vec<String>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Vec<String>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Vec<String>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Vec<String>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Vec<String>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Vec<String>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Vec<String>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Vec<String>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Vec<String>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Vec<String>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Vec<String>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Vec<String>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Vec<String>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制 
//一个 &Box<Box<Box<Box<Box<Vec<String>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Vec<String>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Box<Vec<String>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Box<Box<Vec<String>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Box<Vec<String>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制           
//一个 Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制      
//一个 &Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制       
//一个 &mut Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 拥有堆数据， 因此不能在不移动的情况下被复制
//一个 &Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制
//一个 &mut Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Box<Vec<String>>>>>>>>> 不拥有堆数据， 因此可以在不移动的情况下被复制