use leetcode::solution::Solution;

impl Solution {
    // https://leetcode.com/problems/remove-duplicates-from-sorted-array/
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        let mut ni = 0;
        let mut i = 0;
        i += 1;
        while i < nums.len() {
            if nums[ni] != nums[i] {
                ni += 1;
                nums[ni] = nums[i];
            }
            i += 1;
        }

        return (ni + 1) as i32;
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 { return 0; }
        let mut profit = 0;
        let mut i = 1;
        while i < prices.len() {
            if prices[i] > prices[i-1] {
                profit += prices[i] - prices[i-1];
            }
            i += 1;
        }
        return profit;
    }
}

pub fn remove_duplicates() {
    let mut nums = vec![1,1,2];
    println!("nums = {:?}", nums);
    println!(
        "remove_duplicates(vec![1,1,2]) = {}",
        Solution::remove_duplicates(&mut nums)
        );
    println!("nums = {:?}", nums);
}

pub fn max_profit() {
    println!(
        "max_profit(vec![7,1,5,3,6,4]) = {}",
        Solution::max_profit(vec![7,1,5,3,6,4])
        );
}
