mod stack;
use stack::Stack;
mod operations;
use std::io;

fn main() {
    println!("Initialising stack");
    let mut stack = Stack::new();

    println!("welcome to the polish calculator");

    //gets input and sorts what is needed to do
    /*
    print outputs the top of the stack
    exit gets out of the loop
    add adds the top two values of the stack
    sub, mul, div, mod -> subtract, multiply, divide, modulo, same as add practically

    anything else is checked if its a number, if it is, pushed to stack
    if not will print an error
    */
    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .expect("Couldn't read line"); 

        match line.as_str().trim() {
            "print" => {println!("{}", stack.peak());},
            "exit" => break,

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
