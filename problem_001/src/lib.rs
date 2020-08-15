// Multiples of 3 and 5
//
// Problem 1
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.assert_eq!
//
// Find the sum of all the multiples of 3 or 5 below 1000.

//! # Project Euler
//!
//! A library for Project Euler problems

pub use project_euler::find_sum_of_multiples;

pub mod project_euler {
    use std::cmp;
    /// Finds the sum of a given set of multiples for a number.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = problem_001::find_sum_of_multiples(3, 5, 10);
    /// assert_eq!(result, 23);
    /// ```
    pub fn find_sum_of_multiples(multiple_a: i32, multiple_b: i32, number: i32) -> i32 {
        // straight forward solution
        /*
        let mut sum = 0;

        for index in cmp::min(multiple_a, multiple_b)..number {
            if index % multiple_a == 0 || index % multiple_b == 0 {
                sum += index;
            }
        }

        sum
        */

        // FP-style
        (cmp::min(multiple_a, multiple_b)..number)
            .filter(|index| index % multiple_a == 0 || index % multiple_b == 0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::project_euler::*;

    #[test]

    // If we list all the natural numbers below 10 that are multiples of 3 or 5,
    // we get 3, 5, 6, and 9. The sum of these multiples is 23.
    fn test_find_sum_of_multiples_of_3_and_5_for_10() {
        assert_eq!(find_sum_of_multiples(3, 5, 10), 23);
    }

    #[test]
    fn test_find_sum_of_multiples_of_3_and_5_for_1000() {
        assert_eq!(find_sum_of_multiples(3, 5, 1000), 233168);
    }
}
