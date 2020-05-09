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

utterstep@utterstep-nix:~/my/advent-2019$ hyperfine --warmup 20 "bash ./run_all.sh"
Benchmark #1: bash ./run_all.sh
  Time (mean ± σ):      69.8 ms ±   0.6 ms    [User: 63.7 ms, System: 7.8 ms]
  Range (min … max):    68.8 ms …  71.3 ms    42 runs
```

## Growth points

* benchmark: currently I'm measuring not only solutions, but also bash startup. Rewrite benchmark to increase accuracy
* [day-3](./day-2): check, why precomputed sums looks slower, than brute force approach
* [day-7](./day-7): get rid of [permutohedron](https://crates.io/crates/permutohedron) crate
* [day-13](./day-13): more effective bot strategy (current time ~9ms because there are lot of redundant moves in the end)
* [day-17](./day-17): current solution looks like tailored to the specific class of cases. Rewrite in more generic way OR prove that it's generic
* [day-19](./day-19): determine error sign — currently I'm not totaly sure that constants are good for every possible input (though they are quite conservative)
