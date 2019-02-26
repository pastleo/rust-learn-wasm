mod leetcode;
mod rs_book;
mod udemy;

fn main() {
    leetcode::array::remove_duplicates_test();
    leetcode::array::max_profit_test();

    rs_book::cht2::guessing_game();

    udemy::cht2::sec_2_7_core_data_types();
    udemy::cht2::sec_2_8_operators();
    udemy::cht2::sec_2_9_scope_shadowing();
    udemy::cht2::sec_2_10_const();
    udemy::cht2::sec_2_11_stack_heap();
    udemy::cht3::sec_3_12_if();
    udemy::cht3::sec_3_13_while_loop();
}
