fn chunk<T>(arr: Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut chunk = Vec::new();

    for (i, item) in arr.into_iter().enumerate() {
        chunk.push(item);

        if chunk.len() == size || i == arr.len() - 1 {
            result.push(chunk.clone());
            chunk.clear();
        }
    }

    result
}

fn main() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = chunk(input, 3);
    
    
    for sublist in result {
        println!("{:?}", sublist);
    }
}
