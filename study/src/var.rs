pub fn function() {
    println!("This is some function from var.rs");

    println!("Hello, world!");

    let mut x = 5; //可变
    println!("The value of x is: {}", x);
    //常量不可变，类型必须被标注，只能绑定常量表达式
    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
