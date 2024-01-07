#![feature(linked_list_remove)]
mod add2nums;
mod body_strings;
mod buy_sell_stock;
mod merge_sorted_arrays;
mod palindrome;
mod valid_parentheses;
mod lru_cache;

fn main() {
    add2nums::run(false);
    body_strings::run(false);
    valid_parentheses::run(false);
    buy_sell_stock::run(false);
    palindrome::run(false);
    merge_sorted_arrays::run(false);
}
