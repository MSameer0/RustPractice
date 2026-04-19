fn main() {
    let x = 4;
    println!("x is: {}", x);

    let arr = [1, 2, 3, 4, 5]; // arrays always have to be same data type unlike tuple

    {
        let x = x - 2;
        println!("x is: {}", x);
    }

    let x = x + 1;
    println!("x is: {}", x);

    let x = "hello";
    println!("x is: {}", x);

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);

    let mut tup = (1, true, "hello");

    tup.1 = false;

    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    println!("{}", arr[1]);
}
