/*
    LeetCode-1: TwoSum
    Author    : Irfan Ghat

    - Given an array of integers nums and an integer target,
      return indices of the two numbers such that they add up to target.
    - You may assume that each input would have exactly one solution,
       and you may not use the same element twice.
    - You can return the answer in any order.

    Example 1:
    Input: nums = [2,7,11,15], target = 9
    Output: [0,1]
    Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
*/

mod solution;

use std::io;

fn main() -> () {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    // let nums: Vec<i32> = vec![3,2,4];
    // let nums: Vec<i32> = vec![3,3];

    let mut target = String::new();
    println!("Please provide a target >");
    io::stdin().read_line(&mut target).unwrap();

    match target.trim().parse::<i32>() {
        Ok(target) => {
            solution::Solution::two_sum(nums, target);
        }
        Err(e) => println!("ERROR::{}", e)
    }
}
