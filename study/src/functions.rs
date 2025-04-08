pub fn function() {
    println!("-------------------------------------");
    println!("Hello world");

    anpther_function();
    print_labeled_measurement(5, 'h');
    control_flow();
}

fn anpther_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//control flow
fn control_flow() {
    let number = 3;
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    println!("-------------------------------------");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
}
