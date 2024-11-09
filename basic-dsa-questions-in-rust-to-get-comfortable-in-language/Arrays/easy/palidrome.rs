fn is_palindrome(str: &str) {
    let arr: Vec<char> = str.chars().collect();
    let reversed_arr: Vec<char> = arr.iter().rev().cloned().collect();

    if arr == reversed_arr {
        println!("palindrome");
    } else {
        println!("not palindrome");
    }
}

fn main() {
    is_palindrome("cddc");
    is_palindrome("abcd");
    is_palindrome("acacd");
    is_palindrome("Hello");
}
