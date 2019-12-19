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

## Day 9

33 minutes, rank 564

I probably spent too much time turning the relative base into a field of the
state.

## Day 10

Way too long

I implemented a fancy way of enumerating the rationals in order, but I think I
could have just scanned for all the existing asteroids and sort|uniq'ed their
angles.

## Day 11

1:13, rank 1478

## Day 12

OMG, so much time trying to optimize the simulation, not realizing that each
axis was independent and so could be simulated to it repeat point independently.

## Day 13

Part 1: 6:40 min; Part 2 took longer.

I thought maybe I could just play the game manually, but there were a lot of
blocks, so I had to teach the program to play for me.

## Day 17

Part 1: 00:44:27 rank 1307; Part 2: 01:08:29 rank 305

The second part was just me manually writing out the instructions to hardcode
into my program. Did other people try to have the program compute the
instructions?

## Day 19

Started about 10 minutes late due to finishing part 13. Still got finished the
whole thing at 9:36 for rank 168!

This was very brute force: Try putting the ship at (1,1), if the bottom left
corner isn't in the tractor beam, shift right by a unit. If the top right
doesn't fit now, shift down. Repeat until they both fit. My one problem was that
Rust's `Rect((x,y), (100,100))` has a min_x..max_x of 101 units. :-P
