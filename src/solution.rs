pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut output: Vec<i32> = vec![];
        let mut pointer: usize = 1;
        for (index, _num) in nums.iter().enumerate() {
            while pointer < nums.len() {

                // println!("{}", nums.len());
                println!("Main index: {index}");
                println!("Pointer index: {pointer}\n");

                if nums[index] + nums[pointer] == target {
                    let first_index: i32 = index.to_string().parse::<i32>().unwrap();
                    let last_index: i32 = index.to_string().parse::<i32>().unwrap();
                    output.push(first_index);
                    output.push(last_index);
                    break;
                }
                pointer += 1;
            }
        }
        println!("Output: {:?}", output);
        output
    }
}