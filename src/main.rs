mod strings;

fn main() {
    println!("Hello, world!");
    const DATA: i32 = 42;
    println!("DATA: {}", DATA);

    let mut x = 5;
    println!("x: {}", x);

    x = 10;
    println!("x: {}", x);

    let y = 5;
    println!("y: {}", y);

    let y = y + 1;
    println!("y: {}", y);

    let y = y * 2;
    println!("y: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("x: {}, y: {}", x, y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false;
    println!("t: {}, f: {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("a: {}, b: {}, c: {}", a, b, c);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);

    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("first: {}, second: {}", first, second);

    let arr = [3; 5];
    println!("arr: {:?}", arr);

    let arr = [1, 2, 3, 4, 5];
    let mut index = 10;
    index = if index < arr.len() { index } else { arr.len() - 1 };
    let element = arr[index];
    println!("The value of element is: {}", element);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    strings::string();
}

fn plus_one(p0: i32) -> i32 {
    p0 + 1
}

fn five() -> i32 {
    5
}