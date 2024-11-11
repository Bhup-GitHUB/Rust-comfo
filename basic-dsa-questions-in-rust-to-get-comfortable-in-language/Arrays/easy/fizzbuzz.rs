fn fizzbuzz(n: i32) {
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        }
    }
}

fn main() {
    fizzbuzz(15); 
}
