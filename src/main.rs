use std::io;

fn main() {
    println!("Hello, world!");
    let mut st = String::new();
    st = String::from("MIRUNA");
    //let letter = 's';
    // let tup = (33, 4.44, "mm");
    // let (x, y, z) = tup;
    // let a = (1, 4.3, 'x');
    // let b = a.0;
    // println!("{b}");
    // let v = [1, 2, 3, 4];
    // let first = v[0];
    // println!("{first}");
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("Failed!");
    // let index: usize = index.trim().parse().expect("Failed!");
    // let el = v[index];
    // println!("{el}");
    function();
    function2(5);
    function3(4, 'c');
    function4(String::from("MMM"));
    let y = {
        let x = 1;
        x + 1
    };
    println!("{y}");
    let mut b = five();
    println!("{b}");
    b = plusone(b);
    println!("{b}");
    let mut x = String::new();
    println!("Enter number:");
    io::stdin().read_line(&mut x).expect("Failed!");
    let mut x: usize = x.trim().parse().expect("Failed!");
    if x < 10 {
        println!("<10");
    } else {
        println!(">=10");
    }
    let mut sum = 0;
    x = loop {
        if x > 0 {
            sum = sum + x % 10;
            x = x / 10;
        } else {
            break sum;
        }
    };
    println!("{x}");
}
fn function() {
    println!("FUNTIOOOON");
}
fn function2(x: i32) {
    println!("{x}");
}
fn function3(nr: i32, c: char) {
    println!("{nr}, {c}");
}
fn function4(s: String) {
    println!("{s}");
}
fn five() -> i32 {
    5
}
fn plusone(x: i32) -> i32 {
    x + 1
}
