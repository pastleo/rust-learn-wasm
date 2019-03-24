use leetcode::solution::Solution;
//use std::collections::HashMap;

impl Solution {
    pub fn three_sum(nums_input: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];

        // faster than 90%
        let mut nums = nums_input.clone();
        nums.sort();
        if nums.len() < 3 { return ans; }
        let mut i = 0;
        while i < nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            let wanted = -nums[i];
            while j < k {
                if wanted == nums[j] + nums[k] {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1; k -= 1;
                    while nums[j - 1] == nums[j] && j <= k { j += 1; }
                    while nums[k + 1] == nums[k] && j <= k { k -= 1; }
                }
                else if wanted > nums[j] + nums[k] { j += 1; }
                else { k -= 1; }
            }
            i += 1;
            while nums[i - 1] == nums[i] && i < nums.len() - 2 { i += 1; }
        }

        // faster than 40%
        /*
        let mut nums = nums_input.clone();
        nums.sort();
        if nums.len() < 3 { return ans; }
        let mut i = 0;
        while i < nums.len() - 2 {
            if i > 0 && nums[i - 1] == nums[i] { i += 1; continue; }
            let mut j = i + 1;
            let wanted = -nums[i];
            let mut matching: HashMap<i32, i32> = HashMap::new();
            let mut matched: HashMap<i32, bool> = HashMap::new();
            while j < nums.len() {
                let mut found = false;
                if let Some(nums_k) = matching.get(&nums[j]) {
                    if let None = matched.get(&nums[j]) {
                        ans.push(vec![nums[i], *nums_k, nums[j]]);
                        found = true;
                    }
                };
                if found {
                    matched.insert(nums[j], true);
                } else {
                    matching.insert(wanted - nums[j], nums[j]);
                }
                j += 1;
            }
            i += 1;
        }
        */

        ans
    }
}

pub fn three_sum() {
    println!(
        "Solution::three_sum(vec![-2,0,0,2,2]) = {:?}",
        Solution::three_sum(vec![-2,0,0,2,2])
    );
    println!(
        "Solution::three_sum(vec![-1,0,1,2,-1,-4]) = {:?}",
        Solution::three_sum(vec![-1,0,1,2,-1,-4])
    );
    println!(
        "Solution::three_sum(vec![0,2,2,3,0,1,2,3,-1,-4,2]) = {:?}",
        Solution::three_sum(vec![0,2,2,3,0,1,2,3,-1,-4,2])
    );
    println!(
        "Solution::three_sum(vec![0,0,0]) = {:?}",
        Solution::three_sum(vec![0,0,0])
    );
    println!(
        "Solution::three_sum(vec![]) = {:?}",
        Solution::three_sum(vec![])
    );
}
