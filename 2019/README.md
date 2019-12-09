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

## Day 5

39 minutes, rank 479

No real Rust surprises today. Turned day 2's inline integral opcodes into an
ADT. An ADT optimizes for adding an operation but we're probably going to be
adding more opcodes than operations, so I should refactor this into a class
hierarchy -- a Trait in Rust. I'll also systematize the parameter reading and
mode handling so it's not spammed out across all the opcode definitions.

## Day 6

50 minutes, rank 1583

Lifetimes. :(

I had also been trying to avoid reading the input into a single string, which
was premature optimization. `.split()` is still not the right way to split into
lines, though...

## Day 7

Over 1.5 hours.

Missed that phases were used exactly once...

Threads! That took a long time.

## Day 8

38 minutes

Argh, forgot to check for transparency in part 2. :-P
