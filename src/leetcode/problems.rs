use leetcode::solution::Solution;

fn min_distance_table(chars1: &Vec<char>, chars2: &Vec<char>) -> usize {
    let mut table: Vec<usize> = vec![];
    let width = chars1.len() + 1;
    let height = chars2.len() + 1;
    let mut i = 0;
    while i < width {
        table.push(i);
        i += 1;
    }
    let mut j = 1;
    while j < height {
        table.push(j);
        i = 1;
        while i < width {
            println!("({}, {})", i, j);
            let dist = if chars1[i - 1] == chars2[j - 1] {
                table[(j - 1) * width + i - 1]
            } else {
                if let Some(&dist) = vec![
                    table[(j - 1) * width + i - 1],
                    table[j * width + i - 1],
                    table[(j - 1) * width + i],
                ].iter().min() {
                    dist + 1
                }
                else { panic!("should not be"); }
            };
            table.push(dist);
            i += 1;
        }
        j += 1;
    }
    table[table.len() - 1]
}

fn min_distance_recursive(chars1: &Vec<char>, chars2: &Vec<char>) -> usize {
    fn md(chars1: &Vec<char>, chars2: &Vec<char>, cache: Vec<Option<usize>>, p1: usize, p2: usize) -> (Vec<Option<usize>>, usize) {
        println!("({}, {})", p1, p2);
        if p1 == 0 { (cache, p2) }
        else if p2 == 0 { (cache, p1) }
        else if let Some(known_md) = cache[(p2 - 1) * chars1.len() + p1 - 1] { (cache, known_md) }
        else if chars1[p1 - 1] == chars2[p2 - 1] {
            let (mut cache, r) = md(chars1, chars2, cache, p1 - 1, p2 - 1);
            cache[(p2 - 1) * chars1.len() + p1 - 1] = Some(r);
            (cache, r)
        }
        else {
            let (cache, r1) = md(chars1, chars2, cache, p1 - 1, p2 - 1);
            let (cache, r2) = md(chars1, chars2, cache, p1, p2 - 1);
            let (mut cache, r3) = md(chars1, chars2, cache, p1 - 1, p2);
            if let Some(&r) = vec![r1, r2, r3].iter().min() {
                cache[(p2 - 1) * chars1.len() + p1 - 1] = Some(r + 1);
                (cache, r + 1)
            }
            else { panic!("should not be"); }
        }
    }
    let cache_size = chars1.len() * chars2.len();
    let cache: Vec<Option<usize>> = vec![None; cache_size];
    md(chars1, chars2, cache, chars1.len(), chars2.len()).1
}


impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();
        min_distance_recursive(&chars1, &chars2) as i32
        //min_distance_table(&chars1, &chars2) as i32
    }
}

pub fn min_distance() {
    println!(
        "Solution::min_distance(\"horse\".to_string(), \"ros\".to_string()) = {}",
        Solution::min_distance("horse".to_string(), "ros".to_string())
    );
    println!(
        "Solution::min_distance(\"aa\".to_string(), \"aa\".to_string()) = {}",
        Solution::min_distance("aa".to_string(), "aa".to_string())
    );
}
