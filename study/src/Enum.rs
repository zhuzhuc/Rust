use core::panic;

// 删除未使用的导入
// use crate::Slice::function as other_function;
use crate::operation::Operation;

fn main() {
    function();
}
#[derive(Debug)]
enum IpAddrking {
    V4(u8, u8, u8, u8), 
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// impl Message {
//     fn call(&self) {
//         println!("call");
//     }
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
 
pub fn function() {
    println!("------------------------------------");
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
    // 枚举
    println!("枚举");
    let x = 5;  
    let y = 10;
    let op = Operation::Divide;
    let result = match op {
        Operation::Add => x + y,
        Operation::Subtract => x - y,
        Operation::Multiply => x * y,
        Operation::Divide => x / y,
    };
    println!("Result: {}", result);

    print!("------------------------------------\n");
    let four = IpAddrking::V4(127, 0, 0, 1);
    let six = IpAddrking::V6(String::from("::1"));
    println!("four is {:?}", four);
    println!("six is {:?}", six);

    route(four);
    route(six);  

    println!("--------------------------------------");
    #[derive(Debug)] 
    enum IpAddr { 
        V4(String), 
        V6(String), 
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);
    // panic!("you win")
    println!("--------------------------------------");
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        ot => move_player(ot),
    }
    fn add_fancy_hat() { }
    fn remove_fancy_hat() { }
    fn move_player(num_spaces: u8) { }
    println!("--------------------------------------");
    let opt: Option<String> = Some(String::from("zzc"));//opt >- R + O
    match &opt {//opt >- R != O
        Some(s) => println!("some: {}", s),// opt R + O
        None => println!("None"),//opt R + O
    };
    println!("{:?}", opt);// opt =>!R and !O

    println!("--------------------------------------");
    let config_max = Some(99);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("None");
    }
}

fn route(ip_type: IpAddrking) { }

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}