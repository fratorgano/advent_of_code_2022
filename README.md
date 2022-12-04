# 🎄 Advent of Code 2022 🎄

I'm again going to participate in the Advent of Code challenge; I'll try to stick with Rust this year too.  
I'm a little more time-constrained, so I'll allow myself to switch to a different language for more complex problems.  
I don't do this challenge for the leaderboard, so if I'm stuck on a problem, I'll look up tips to help me along the way since my main goal is learning.

🏆 Goal: get all stars before the end of the year! 🏆

## ✨ New this year ✨ 
I added a template for my solutions that is available in the template folder so that I can start to solve the problem faster!

The template contains 2 main files: 
- src/main.rs - contains the code that will run and benchmark the solution
- src/solution.rs - contains the template for the functions and tests that will actually solve the challenge

I used the amazing [cargo-generate](https://github.com/cargo-generate/cargo-generate),  I created a cargo-generate config file at `~/.cargo/cargo-generate` with
```
[favorites.aoc]
description = "Favorites for AOC 2022 template"
vcs = "None"
path = "../template"
```

So I can just run `cargo generate aoc` to interactively create the new crate for a solution each day.



## ❄️ How to use ❄️
`cargo run -p day**` - Runs a specific day

`cargo run -p day** --release` - Runs a specific day with compiler optimizations

`cargo test -p day**` - Tests a specific day

`cargo test` - Tests all

## 🥛 Results 🍪
| Day | Part 1 Time | Part 1 Rank | Part 1 Runtime[^1] | Part 2 Time | Part 2 Rank | Part 2 Runtime[^1] |
|:-:|-:|-:|-:|-:|-:|-:|
|  1 | 02:33:34 |  21725 |  12.2µs | 02:43:17 |  21176 |  16.3µs |
|  2 | 02:39:03 |  28771 | 107.9µs | 03:24:06 |  31073 | 106.3µs |
|  3 | 04:51:33 |  36874 | 350.3µs | 05:38:02 |  36646 | 320.6µs |
|  4 | 07:09:23 |  52797 | - | 07:19:39 |  51312 | - |
<!--|  1 | 00:13:19 |  5740 |  19.5µs | 00:21:33 |  5187 |  20.7µs | -->

## 🎅 Have a Wonderful Holiday Season, Everyone! 🎅 

![koch flakes](https://raw.githubusercontent.com/fratorgano/advent_of_code_2020/main/snow.gif)


[^1]: `cargo run -p day** --release`, does not include the reading of the input file but includes parsing.
