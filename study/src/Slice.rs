fn main() {
    function();
}

pub fn function() {
    println!("------------------------------------");
    println!("Slice切片");
    //    first_word();
    //String Slices
    //Slice是特殊引用类型fat指针包含元数据
    //[起始索引..结束索引]不包括结束索引位置的元素
    //let s = String::from("hello world");
    //let = first_word(&s);
    //let hello: &str = &s[0..5];
    //let world: &str = &s[6..11];
    //let s2: &String = &s;
    let s = String::from("Hello");
    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);
    let len = s.len();
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);
    let slice = &s[0..len];
    println!("{}", slice);

    let slice = &s[..];
    println!("{}", slice);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("------------------------------------");
}
/*
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item = b' ' {
            return i;
        }
    }
    s.len()
}
*/
