pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut output: Vec<i32> = vec![];
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    let start = i;
                    let end = j;
                    output.push(start.to_string().parse::<i32>().unwrap());
                    output.push(end.to_string().parse::<i32>().unwrap());
                    break;
                }
            }
        }
        println!("Output: {:?}", output);
        output
    }
}