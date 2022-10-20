use std::io;

fn exercise1() {
    println!("Exercise 1");

    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let o_size = org_arr.len();
    let s_size = sub_arr.len();
    for i in 0..o_size {
        let mut j = 0;
        while j < s_size && sub_arr[j] == org_arr[i+j] {
            j += 1;
        }
        if j == s_size {
            println!("{:?} is a sublist of {:?} in order", sub_arr, org_arr);
            return ();
        }
    }
    println!("{:?} isn't a sublist of {:?} in order", sub_arr, org_arr);
}

fn exercise2 () {
    println!("Exercise 2");
    
    let input = "adbcdaDd";
    println!("Input: {}", input);

    println!("Enter an character: ");
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.pop();
    line.pop();

    let char: char = line.parse::<char>().unwrap();

    let count = input.matches(char.to_ascii_lowercase()).count() + input.matches(char.to_ascii_uppercase()).count();
    let mut result = input.replace(char.to_ascii_lowercase(), "");
    result = result.replace(char.to_ascii_uppercase(), "");

    println!("Character {} repeat: {} , input changed: {}",char, count, result);
}

fn main() {
    exercise1();
    exercise2();
}