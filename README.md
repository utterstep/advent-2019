# advent-2019

[![GitHub Actions Badge](https://github.com/utter-step/advent-2019/workflows/CI/badge.svg)](https://github.com/utter-step/advent-2019/actions?query=workflow%3ACI)

Rusty Advent of Code 2019 solutions

## Current running time

```console
utterstep@utterstep-nix:~/my/advent-2019$ head /proc/cpuinfo
processor       : 0
vendor_id       : GenuineIntel
cpu family      : 6
model           : 158
model name      : Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz
stepping        : 10
microcode       : 0xca
cpu MHz         : 4018.411
cache size      : 9216 KB
physical id     : 0

utterstep@utterstep-nix:~/my/advent-2019$ taskset -c 3 hyperfine --warmup 20 ./target/release/run-all
Benchmark #1: ./target/release/run-all
  Time (mean ± σ):     121.3 ms ±   0.5 ms    [User: 106.6 ms, System: 14.6 ms]
  Range (min … max):   120.4 ms … 122.8 ms    24 runs
```

Currently `day-12` (2nd part) is taking ~50% of all running time (~68ms, down from ~400ms initially).

## Growth points

* [day-3](./day-2): check, why precomputed sums looks slower, than brute force approach
* [day-7](./day-7): get rid of [permutohedron](https://crates.io/crates/permutohedron) crate
* [day-13](./day-13): more effective bot strategy (current time ~9ms because there are lot of redundant moves in the end)
* [day-14](./day-14): compute `SolutionPrecalc` once for both parts
* [day-17](./day-17): current solution looks like tailored to the specific class of cases. Rewrite in more generic way OR prove that it's generic
* [day-19](./day-19): determine error sign — currently I'm not totaly sure that constants are good for every possible input (though they are quite conservative)
