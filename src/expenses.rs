use std::collections::HashSet;
use std::iter::FromIterator;


// find pair of numbers in values list that sum together to make total
fn _find_2_summands(value_set : &HashSet<i64>, total : i64) -> Option<(i64, i64)> {
    value_set.iter().filter(
        |&value| value_set.contains(&(total - value))
    ).next().map(
        |&summand| (summand, total - summand)
    )
}

// find triple of numbers in values list that sum together to make total
fn _find_3_summands(value_set : &HashSet<i64>, total : i64) -> Option<(i64, i64, i64)> {
    value_set.iter().filter_map(
        |&value| _find_2_summands(value_set, total - value)
    ).next().map(
        |(a, b)| (a, b, total - a - b)
    )
}

// find pair of numbers in values list that sum together to make total
pub fn find_2_summands(values : &Vec<i64>, total : i64) -> Option<(i64, i64)> {
    // Query set to avoid quadratic check
    let value_set : HashSet<i64> = 
      HashSet::from_iter(values.iter().cloned());
    _find_2_summands(&value_set, total)
}

// find triple of numbers in values list that sum together to make total
pub fn find_3_summands(values : &Vec<i64>, total : i64) -> Option<(i64, i64, i64)> {
    // Query set to avoid quadratic check
    let value_set : HashSet<i64> = 
      HashSet::from_iter(values.iter().cloned());
    _find_3_summands(&value_set, total)
}