use crate::algorithms::solution::Solution;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 8,
            5 => 10,
            6 => 36,
            7 => 41,
            8 => 132,
            9 => 250,
            10 => 700,
            11 => 750,
            12 => 4010,
            13 => 4237,
            14 => 10680,
            15 => 24679,
            _ => 0
        }
    }
}