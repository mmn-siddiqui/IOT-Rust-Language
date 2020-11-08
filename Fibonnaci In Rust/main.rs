fn main(){
    let number = 8;

    let result = calculate_fibonacci_through_recursion(number);
    println!("Recursion: The fibonacci number of {} is {}", number, result);

    let result = calculate_fibonacci_through_while_loop(number);
    println!("While Loop: The fibonacci number of {} is {}", number, result);

    let result = calculate_fibonacci_through_loop(number);
    println!("Loop: The fibonacci number of {} is {}", number, result);

    let result = calculate_fibonacci_through_forloop(number);
    println!("For: The fibonacci number of {} is {}", number, result);
}

fn calculate_fibonacci_through_recursion(value:u32) -> u32{
    if value == 1 {
        1
    }
    else{
        value * calculate_fibonacci_through_recursion(value - 1)
    }
}

fn calculate_fibonacci_through_while_loop(value:u32) -> u32 {
    let mut counter = value;
    let mut answer = value;
    
    while counter > 1 {        
        answer = answer * (counter - 1);
        counter -= 1;
    };

    answer
}

fn calculate_fibonacci_through_loop(value:u32) -> u32 {
    let mut counter = value;
    let mut answer = value;
   
    loop {
        if counter < 1 {
            break 0;
        }
        else if counter == 1 {
            break answer * 1;
        }
        else {            
            answer = answer * (counter - 1);
            counter -= 1;
        }
    };

    answer
}

fn calculate_fibonacci_through_forloop(value:u32) -> u32 {
    let counter = value;
    let mut result = value;

    for element in (1..counter).rev() {        
        result = result * element;        
    }

    result
}

