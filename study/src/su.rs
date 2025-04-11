fn read(y: bool) {
    if y {
        println!("y is true");
    }
}
pub fn function() {
    println!("this is onwer");
    println!("---------------------------------------");

    let x = true;
    read(x);
    main();
    main2();
}
//调用函数时，rust为被调用的函数分配一个Stack Frame。当调用结束时，rust释放该Satck Frame。
//Box内存释放原则
//(几乎正确）如果一个变量绑定到一个box， 当rust释放变量当frame时，rust也会释放box的堆内存
//（完全正确）如果一个变量拥有一个box，当rust释放该变量的frame时，rust会释放该box的堆内存(heap)
fn main() {
    let first = String::from("zzc");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}
fn add_suffix(mut name: String) -> String {
    name.push_str("zz");
    name
}
fn add_suffix2(mut s: String) -> String {
    s.push_str(" world");
    s
}
fn main2() {
    let s = String::from("hello");
    let s2 = add_suffix2(s);
    println!("{}", s2);
}
