use std::io;

fn main() {
    const MY_NAME_HERE: &str = "Frank";
    println!("My name is {}", MY_NAME_HERE);

    let mut x= 6; //any un-assingned interger, takes the 32bit signed integer value by default
    println!("X is {}", x);

    {
        let x: u64 = 20 - 5;
        println!("inside block: {}", x);
    }

    x = 8;
    println!("X is now {}", x);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Faild to real line");
    println!("{}", input);

}
