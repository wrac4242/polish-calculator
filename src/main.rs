mod stack;
use stack::Stack;
mod operations;
use std::io;

fn help() {
    println!();
    println!("-----");
    println!();
    
    println!("General controls are:");
    println!("print: prints the top of the stack");
    println!("exit: exits the calculator");
    println!("switch: switches the top two items on the stack");
    println!("help: prints this dialogue");
    println!("pop: removes the top element and outputs it");

    println!();
    println!("-----");
    println!();

    println!("The operators are: ");
    println!("add: takes the top two items on the stack and `second + top` is pushed back to the stack");
    println!("sub: takes the top two items on the stack and `second - top` is pushed back to the stack");
    println!("mul: takes the top two items on the stack and `second * top` is pushed back to the stack");
    println!("div: takes the top two items on the stack and `second / top` is pushed back to the stack");
    println!("mod: takes the top two items on the stack and `second % top` is pushed back to the stack");

    println!("-----");
    println!();
}

fn main() {
    println!("Initializing stack");
    let mut stack = Stack::new();

    println!("welcome to the polish calculator");

    help();

    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .expect("Couldn't read line"); 

        match line.as_str().trim() {
            "print" => println!("{}", stack.peak()),
            "exit" => break,
            "help" => help(),

            "switch" => operations::switch(&mut stack),
            "pop" => println!("{}", stack.pop()),

            "add" => operations::add(&mut stack),
            "sub" => operations::sub(&mut stack),
            "mul" => operations::mult(&mut stack),
            "div" => operations::div(&mut stack),
            "mod" => operations::modulo(&mut stack),

            e => {
                let push = e.trim().parse::<i64>();
                match push {
                    Ok(e) => stack.push(e),
                    Err(e) => println!("Error {}, cannot push", e)
                }
            }
        }
    }
}
