# Notes on the daily problems

## Day 1

Intro to rust:
* How do you iterate over the lines in stdin?
* What's for loop syntax?
* How do you parse an integer from a string?

## Day 2

* Argh, mixing up positions and values.
* Indices are usize, but my inputs might be signed? How safe is the `as` cast?
  Will it correctly panic on overflow?
* Part 2: brute force should work.
* Oh, right, Rust moves into by-value function arguments. Do I take by reference
  or make the caller copy? Caller-copies might be more idiomatic since it would
  let the caller be more efficient...
* The `mut` on a function parameter goes in a weird place, before the variable
  name. Oh well.

## Day 3

1:21

Spent too much time building nice datastructures for the paths and worrying
about how to slice a String of unicode characters when the actual strings could
only have ASCII.

## Day 4

14 minutes

How do you format a number into a string? Didn't find `.to_string()` while
coding.
