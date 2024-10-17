use std::time::Instant;

fn main() {
    let start: Instant = Instant::now();
    loop {
        println!("again!");
        break;
    }

    let mut counter: i64 = 0;
    let result = loop {
        counter += 1;
        if counter == 100000 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    // -------------
    // You can label a loop by putting `'some_name:` before the loop
    // The break without a label will break the inner loop
    // The break with a label will break the outer loop

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    // You can implement contidion on while loop, if the condition is not true, the loop will not run
    let mut number = 3;
    while number <= 10 {
       println!("{}!", number);
         number += 1;
    }


    // You can also use while construct to loop over elements of a collecion, such as an array
    // However if we put an index that doesn't exist in the array, the code would panic
    let a: [i8; 5] = [10, 20, 30, 40, 50];
    let mut index: i8 = 0;

    while index < 5 {
        println!("The value is: {}", a[index as usize]);
        index += 1;
    }

    // To iter over the array, do the following:
    for element in a {
        println!("The value in iteration is: {}", element);
    }

    // Another way to use Range from standart library, it will start with number and end right
    // before the end number, rev() is to reverse
    for number in (1..4).rev() {
        println!("From Range: {}!", number);
    }

}
