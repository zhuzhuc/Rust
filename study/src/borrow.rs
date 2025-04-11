fn main() {
    function();
}

pub fn function() {
    println!("------------------------------------");
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);

    println!("------------------------------------");
    let a1 = String::from("Hello");
    let a2 = String::from("zzc");
    println!("a1: {}", a1);
    println!("a1的地址:{:p}", &a1);
    greet2(&a1, &a2);
    let b = format!("{} {}", a1, a2);
    println!("------------------------------------");
    yyy();
    xxx();
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

// 引用
fn greet2(a1: &String, a2: &String) {
    println!("{} {}!", a1, a2);
    let address_in_a1: *const String = a1 as *const String;
    println!("{a1}");
    println!("a1存的内容: {:p}", address_in_a1);
    println!("a1的地址: {:p}", &a1);
}
fn yyy() {
    let x = Box::new(1);
    let y = x;
    //此时box的所有权已经转移给y
    println!("y: {}", y);
    let r1 = &y;
    let r2 = &y;
    println!("r1: {r1}, r2: {r2}");
}

fn xxx() {
    //let mut v: Vec<i32> = vec![1, 2, 3]; //v R+W+O
    //let num: &i32 = &v[2]; // v R, num R+O, *num R
    //println!("Third element is {]", *num); //v R+W+O
    //    v.push(4); //此时v用完之后没有任何权限
    let mut v = vec![1, 2, 3];
    v.push(4); // 修改完 v 后，再借用

    let num = &v[2]; // 现在才借用，不冲突
    println!("Third element is {}", *num);
}
