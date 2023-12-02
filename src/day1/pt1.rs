use std::fs;


fn day1() {
    let lines: Vec<String> =  fs::read_to_string("../input.txt").expect("File not found!").lines().map(String::from).collect();

    for line in lines {
        println!("{}", line);
    }


    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;

        day1();
    }
}