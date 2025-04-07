fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let a = [1, 2, 3, 4, 5];
    let a[i32,5] = [1, 2, 3, 4, 5];
    let a[3,5] = [3, 3, 3, 3, 3];

    let months={
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
    };

    let first = months[0];
    let second = months[1];

    let index = [12, 13, 14, 15, 16];
    let month = months[index[15]];//panic // Crossing boundaries
    println!("{}, {}, {}",tup.0, tup.1, tup.2));
}
