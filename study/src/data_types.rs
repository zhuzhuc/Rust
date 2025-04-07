pub fn function() {
    println!("This is some function from data_types.rs");

    // 字符串转数字
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    // 整型和浮点型
    // int -> u32
    // 无符号u开头，有符号i开头
    // f32 单精度 f64 双精度
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 54 % 5;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);

    // 布尔类型
    let t = true;
    let f: bool = false;

    // 字符类型
    let x = 'z';
    let y: char = 'z';

    // 元组（tuple）
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // 数组（array）
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // [类型; 长度]
    println!("Array: {:?}", a);

    // 月份数组
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = months[0];
    let second = months[1];
    println!("First month: {}", first);
    println!("Second month: {}", second);

    // 调用函数
    let result = add_numbers(5, 10);
    println!("5 + 10 = {}", result);

    // 调用函数并返回元组
    let (x, y) = get_coordinates();
    println!("x: {}, y: {}", x, y);

    println!("----------------------------------------");
    five_func();
    println!("----------------------------------------");
    bool_func();
    println!("----------------------------------------");
    while_func();
    println!("----------------------------------------");
}

// 函数例子：加法函数
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// 函数例子：返回一个元组
fn get_coordinates() -> (i32, i32) {
    (10, 20)
}

fn five(x: i32) -> i32 {
    x + 5
}
fn five_func() {
    let mut x = 0;
    x = x + five(6);
    println!("the value of x is: {}", x);
}

fn bool_func() {
    let number = 3;
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    let num = 6;
    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else if num % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }

    let conditions = true;

    let nummm = if conditions { 5 } else { 6 };
    println!("the value of nummm is: {}", nummm);
}

fn while_func() {
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is: {}", res);
    println!("----------------------------------------");
    let mut nn = 3;
    while nn != 0 {
        println!("{}!", nn);
        nn = nn - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    println!("----------------------------------------");

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    println!("----------------------------------------");
    for nm in (1..4).rev() {
        println!("{}", nm);
    }
    println!("LIFTOFF");
}
