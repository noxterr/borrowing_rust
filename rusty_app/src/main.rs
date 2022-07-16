fn main() {
    println!("Hello, world!");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    let guess: i8 = -5;
    println!("The value of guess is: {}", guess);

    // Calling many many functions - at once
    function_caller();

    // conditional logic

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // This is an explicit tup. is like an struct on C. Very kind of
    let explicit_tup: (i32, f64, u8) = (500, 6.4, 1); // Here we explicitly assign each number its type

    // This in an implicit tup, is like setting up an array
    let implicit_tup = (500, 6.4, 1);

    // x = 500 - y = 6.4 - z = 1
    let (_x, _y, _z) = implicit_tup; // Now i am accessing the data of the tup, creating a variable for each one in the tup, that will be assigned the tup value at its position


    println!("Values of x,y and z are: {} {} and {}", _x, _y, _z);

    // I can access tup's variable like I am accessing an array, but more with an object-like syntactically approach like so
    let _five_hundred = explicit_tup.0;
    let _six_point_four = explicit_tup.1;
    let _one = explicit_tup.2;

    // I eventually want to sum every number in the tup (not looping because is I don't know it right now)
    // I create a mutable variable, then I sum every variable.
    // I cannot sum variables of different types (like int, or unsigned int with floats). I then convert all to f64 (float)
    let mut _sum = ( _five_hundred as f64 )+ ( _one as f64 ) + _six_point_four;

    println!("Summing the whole tup content should give me {}", _sum);
}

fn function_caller() {
    another_function();
    another_function_param(129);
    print_labeled_measurement(5, 'h');
    testing_function();
}

fn another_function() {
    println!("Another function.");
}

fn another_function_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn testing_function() {
    println!("The 5 value is: {}", five());
}

fn five() -> i32 {
    5
}