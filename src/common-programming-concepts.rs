fn main() {
    const MAX: u32 = 10_000;
    println!("{}", MAX);
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = multiplication(4, 30);

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;

    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {}",
        sum,
        difference,
        product,
        quotient,
        remainder,
        t,
        c,
        z,
        heart_eyed_cat,
        tup.0,
        tup.1,
        tup.2
    );
    let months: [&str; 12]= ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    let first = months[0];
    let second = months[1];
    println!("{} {}", first, second);
    let spaces = "  ";
    let spaces_len= spaces.len();
    let spaces_len= spaces_len as i32;
    example(spaces_len);


    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

}
fn example(x: i32) {
    println!("{}", x);
}
fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}
