use factorial::Factorial;
use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    num::ParseIntError,
};

// I apologise in advance to anyone who needs to read this...

fn main() {
    let nums = loop {
        println!("Please enter a sequence of integers, separated by commas.");
        print!("> ");
        stdout().flush().expect("Failed to flush standard output.");
        match read_numbers() {
            Ok(result) => {
                break result;
            }
            Err(e) => {
                println!("An error occurred: {}", e);
            }
        }
    };

    let mut stack: Vec<Vec<i128>> = Vec::new();

    let done = |stack: &mut Vec<Vec<i128>>| {
        println!(
            "You're looking at a polynomial with degree {}.",
            stack.len() - 1
        );
        println!("Its differences look like this:");

        print_stack(&stack);

        let curr = stack.last_mut().expect("No arrays in stack");
        let mut diff = *curr.iter().min().expect("Buffer is empty");

        curr.push(diff);
        let mut index = stack.len() - 2usize;
        loop {
            let cr = &mut stack[index as usize];
            diff = *cr.last().expect("Empty stack level") + diff;
            cr.push(diff);
            if index == 0 {
                break;
            }
            index -= 1;
        }

        println!("Expanding to the next term:");
        print_stack(&stack);

        println!(
            "Therefore, your next number is {}",
            stack
                .first()
                .expect("Empty stack level")
                .last()
                .unwrap_or(&0)
        );

        let mut terms: Vec<String> = Vec::new();
        let mut counter = 0u128;
        for level in stack {
            let mut divisor = String::new();
            for n in 0..(counter) {
                if divisor.is_empty() {
                    divisor = "*".to_string();
                }
                divisor = format!("{}(x-{})", divisor, n);
            }

            terms.push(format!(
                "{}{}/{}",
                level.first().expect("Empty stack level"),
                divisor,
                counter.factorial()
            ));
            counter += 1;
        }
        println!("{}", terms.join(" + "));
    };

    stack.push(nums);
    loop {
        let curr = stack.last().expect("No arrays in stack");
        if same(curr) {
            done(&mut stack);
            break;
        }

        let mut counter: usize = 0;
        let mut temp: Vec<i128> = vec![0; curr.len() - 1];

        for slice in curr.windows(2) {
            temp[counter] = slice[1] - slice[0];
            counter += 1;
        }
        stack.push(temp);
    }
}

fn read_numbers() -> Result<Vec<i128>, ParseIntError> {
    let mut input = String::new();
    let _ = stdin()
        .read_line(&mut input)
        .expect("Failed to read standard input.");

    let mut nums: Vec<i128> = Vec::new();
    for entry in input.split(",") {
        match entry.trim().parse::<i128>() {
            Ok(n) => nums.push(n),
            Err(e) => return Err(e),
        }
    }

    return Ok(nums);
}

fn same<T: PartialEq>(vec: &Vec<T>) -> bool {
    let mut iter = vec.iter();
    let first = iter.next();
    iter.fold(first, |acc, item| {
        acc.and_then(|stored| if stored == item { Some(stored) } else { None })
    })
    .is_some()
}

fn max<'a, Q, T: Ord>(vec: &'a Vec<Q>, map: fn(q: &Q) -> T, def: T) -> T {
    return vec.iter().map(map).max().unwrap_or(def);
}

fn length<T: ToString>(t: T) -> usize {
    t.to_string().len()
}

fn print_stack<T: ToString + Display>(stack: &Vec<Vec<T>>) {
    let max = max(stack, |vec| max(vec, |i| length(i), 0), 0);

    let mut j = 0usize;
    while j < stack.len() {
        print!("{}", " ".repeat((j as f32 * max as f32 / 1.3333) as usize));
        for num in &stack[j] {
            print!(" {}{} ", " ".repeat(max - num.to_string().len()), num);
        }
        println!();
        j += 1;
    }
}
