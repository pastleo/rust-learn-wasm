use leetcode::solution::Solution;

impl Solution {
    pub fn max_common_length(word1: String, word2: String) -> i32 {
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();
        let mut max = 0;

        let mut i = 0;
        while i < chars1.len() {
            let mut j = 0;
            while j < chars2.len() {
                let mut k = 0;
                while
                    i + k < chars1.len() &&
                    j + k < chars2.len() &&
                    chars1[i + k] == chars2[j + k]
                {
                    k += 1;
                }
                if k > max { max = k; }

                j += 1;
            }
            i += 1;
        }
        max as i32
    }
}

pub fn max_common_length() {
    println!(
        "Solution::max_common_length(\"pastleo\".to_string(), \"leo\".to_string()) = {}",
        Solution::max_common_length("pastleo".to_string(), "leo".to_string())
    );
}
