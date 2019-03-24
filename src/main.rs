mod leetcode;
mod rs_book;
mod udemy;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target:usize = if args.len() >= 2 { args[1].parse::<usize>().expect("not a number") } else { 0 };
    println!("");

    match target {
        101 => rs_book::cht2::guessing_game(),

        201 => udemy::cht2::sec_2_7_core_data_types(),
        202 => udemy::cht2::sec_2_8_operators(),
        203 => udemy::cht2::sec_2_9_scope_shadowing(),
        204 => udemy::cht2::sec_2_10_const(),
        205 => udemy::cht2::sec_2_11_stack_heap(),
        206 => udemy::cht3::sec_3_12_if(),
        207 => udemy::cht3::sec_3_13_while_loop(),

        301 => leetcode::interview_easy::max_profit(),
        302 => leetcode::interview_easy::remove_duplicates(),

        401 => leetcode::interview_medium::three_sum(),

        _ => help(),
    }
}

fn help() {
    println!("Please choose what you want to run:");
    println!("
        101 => rs_book::cht2::guessing_game(),

        201 => udemy::cht2::sec_2_7_core_data_types(),
        202 => udemy::cht2::sec_2_8_operators(),
        203 => udemy::cht2::sec_2_9_scope_shadowing(),
        204 => udemy::cht2::sec_2_10_const(),
        205 => udemy::cht2::sec_2_11_stack_heap(),
        206 => udemy::cht3::sec_3_12_if(),
        207 => udemy::cht3::sec_3_13_while_loop(),

        301 => leetcode::interview_easy::max_profit(),
        302 => leetcode::interview_easy::remove_duplicates(),

        401 => leetcode::interview_medium::three_sum(),

        _ => help(),
    ");
}
