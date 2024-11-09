use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for &num in &nums {
        if !seen.insert(num) {
            return true;
        }
    }
    false
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    println!("{}", contains_duplicate(nums)); 
}
