fn main() {
    const MY_NAME_HERE: u32 = 100;
    println!("My name is {}", MY_NAME_HERE);

    let mut x= 6;
    println!("X is {}", x);

    {
        let x = 20 - 5;
        println!("inside block: {}", x);
    }

    x = 8;
    println!("X is now {}", x);
}
