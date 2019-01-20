#![cfg(test)]

use crate::problem1::{sum, dedup, filter};
use crate::problem2::mat_mult;
use crate::problem3::sieve;
use crate::problem4::{hanoi, Peg};

//
// Problem 1
//

// Sum

#[test]
fn test_sum_negative() {
    let array = [-1, -2, -3, -4, -5];
    assert_eq!(sum(&array), -15);
}

// Dedup

#[test]
fn test_dedup_same() {
    let vs = vec![1, 1, 1, 1, 1];
    assert_eq!(dedup(&vs), vec![1]);
}

// filter

fn positive_predicate(x: i32) -> bool {
    x > 0
}

#[test]
fn test_filter_positive() {
    let vs = vec![1, -2, 3, -4, 5, -6];
    assert_eq!(filter(&vs, &positive_predicate), vec![1, 3, 5]);
}

//
// Problem 2
//

#[test]
#[should_panic]
fn test_rows_columns_not_match() {
    let mat1 = vec![vec![5.; 4]; 3];
    let mat2 = vec![vec![5.; 4]; 3];
    mat_mult(&mat1, &mat2);
}

#[test]
fn test_mat_mult_normal_case() {
    let mat1 = vec![vec![1., 2., 3., 4.], vec![5., 6., 7., 8.]];
    let mat2 = vec![
        vec![1., 2., 3.],
        vec![4., 5., 6.],
        vec![7., 8., 9.],
        vec![10., 11., 12.],
    ];
    let result = vec![vec![70., 80., 90.], vec![158., 184., 210.]];
    assert_eq!(mat_mult(&mat1, &mat2), result);
}

//
// Problem 3
//

#[test]
fn test_sieve_less_than_two() {
    let v: Vec<u32> = vec![];
    assert_eq!(v, sieve(1));
    assert_eq!(v, sieve(0));
}
//
// Problem 4
//

#[test]
fn test_hanoi_3_disks() {
    let result = hanoi(3, Peg::A, Peg::B, Peg::C);
    assert_eq!(
        vec![
            (Peg::A, Peg::C),
            (Peg::A, Peg::B),
            (Peg::C, Peg::B),
            (Peg::A, Peg::C),
            (Peg::B, Peg::A),
            (Peg::B, Peg::C),
            (Peg::A, Peg::C),
        ],
        result
    );
    assert_eq!(7, result.len());
}