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

pub fn remove_duplicates_test() {
    let mut nums = vec![1,1,2];
    println!("nums = {:?}", nums);
    println!(
        "remove_duplicates(vec![1,1,2]) = {}",
        remove_duplicates(&mut nums)
    );
    println!("nums = {:?}", nums);
}

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
pub fn max_profit_test() {
    println!(
        "max_profit(vec![7,1,5,3,6,4]) = {}",
        max_profit(vec![7,1,5,3,6,4])
    );
}


pub fn three_sum(nums_input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums_input.clone();
    nums.sort();
    let mut ans: Vec<Vec<i32>> = vec![];
    if nums.len() < 3 { return ans; }
    let mut i = 0;
    while i < nums.len() - 2 {
        if i > 0 && nums[i - 1] == nums[i] { i += 1; continue; }
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
    }
    ans
}

pub fn run_three_sum() {
    println!(
"three_sum(vec![-2,0,0,2,2]) = {:?}",
three_sum(vec![-2,0,0,2,2])
);
    println!(
"three_sum(vec![]) = {:?}",
three_sum(vec![])
);
}
