#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Clone, Copy, Debug)]
// #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Self { width: w, height: h }
    }
    fn set_to_max(&mut self, other: &Self) {
        *self = self.max(other.clone());
    } 
}

fn main() {
    function();
}

pub fn function() {
    // 结构体
    println!("结构体");

    let mut user1 = User {
        email: String::from("zzc"),
        username: String::from("zzc"),
        active: true,
        sign_in_count: 1,
    };

    let _user2 = User {
        email: String::from("zzczzz"),
        ..user1.clone() // 克隆 user1 而不是移动部分字段
    };

    user1.email = String::from("123@qq.com");
    println!("---------------------------------");
    println!("{:?}", user1);
    println!("---------------------------------");
    println!("{:?}", _user2);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
    // 元组结构体
    println!("元组结构体");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {1} {0}", rect1.width, rect1.height);
    println!("----------------------------------");
    for i in 1..10 {
        print!("{}", i)
    }
    println!("\n");
    println!("----------------------------------");

    let w = 30;
    let h = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&Rectangle {
            width: w,
            height: h,
        })
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rec = Rectangle::square(3);
    println!("rec is {:?}", rec);

    // Example of using the max function
    let rect_max = rect1.max(rect2);
    println!("The larger rectangle is {:?}", rect_max);

    println!("----------------------------------");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
