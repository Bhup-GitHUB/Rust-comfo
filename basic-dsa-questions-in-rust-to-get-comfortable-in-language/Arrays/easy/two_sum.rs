fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i, j];
            }
        }
    }
    vec![] 
}

fn main() {
    let res = two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", res);
}
