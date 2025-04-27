mod Slice;
mod Struct;
mod Enum;
mod borrow;
mod data_types;
mod fixed;
mod functions;
mod su;
mod var;
mod operation;
fn main() {
    println!("Hello, world!");
    let zzc_art = r#"
 ________  ________  ________     
|\_____  \|\_____  \|\   ____\    
 \|___/  /|\|___/  /\ \  \___|    
     /  / /    /  / /\ \  \       
    /  /_/__  /  /_/__\ \  \____  
   |\________\\________\ \_______\
    \|_______|\|_______|\|_______|                   
   "#;
    println!("{}", zzc_art);

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

    data_types::function();
    var::function();
    functions::function();
    su::function();
    borrow::function();
    fixed::function();
    Slice::function();
    Struct::function();
    Enum::function();
}
