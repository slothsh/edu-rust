
// CodeWars : Fundamentals - Rust Training
// Positive Sum
// https://www.codewars.com/kata/5715eaedb436cf5606000381/train/rust
// Author: Stefan Olivier
/* -
    Task:
    You get an array of numbers, return the sum of all of the positives ones.
    Example [1,-4,7,12] => 1 + 7 + 12 = 20
    Note: if there is nothing to sum, the sum is default to 0.
*/

fn positive_sum(slice: &[i32]) -> i32 {
    let mut acc: i32 = 0;
    for x in slice {
        if x > &0 {
            acc += x;
        }
    }

    return acc;
}

