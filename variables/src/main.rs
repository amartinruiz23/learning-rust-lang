fn main() {

    // Mutability

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of three hours in secods is: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing

    let x = 7;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    println!("The value of spaces before shadowing is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of spaces after shadowing is: {}", spaces);

}
