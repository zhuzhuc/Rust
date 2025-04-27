use csv_to_json::models::structs::HousePrice;
use csv_to_json::functions::read_csv;
fn main() {
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
    /* 
    crate::m1::m2::method1();//绝对路径
    crate::x1::x2::method2();//绝对路径
    m1::m2::method1();//相对路径
    x1::x2::method2();//相对路径
    */

    let y = csv_to_json::models::types::YesOrNo::Yes;
    println!("{:?}", y);
    let y = csv_to_json::models::types::YesOrNo::No;
    println!("{:?}", y);
    
    let house_price = HousePrice {
        price: 100000,
        area: String::from("100m2"),
        bed_rooms: 3,
        main_road: csv_to_json::models::types::YesOrNo::Yes,
    };
    println!("{:?}", house_price);

    read_csv(&String::from("house_price.csv"));
}
/*
mod m1 {
    pub mod m2{
        pub fn method1() {
            println!("m1::m2::method1");
        }
    }
}

mod x1 {
    fn method3() {
        // x2::method2();//相对路径
        self::x2::method2();
    }
    pub mod x2 {
        pub fn method2() {
            super::super::m1::m2::method1();//相对路径
            println!("x1::x2::method2");
        }
    }
}
*/