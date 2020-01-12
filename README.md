# advent-2019

![GitHub Actions Badge](https://github.com/utter-step/advent-2019/workflows/CI/badge.svg)

Rusty Advent of Code 2019 solutions

## Current running time

```bash
utterstep@utterstep-nix:~/my/advent-2019$ head /proc/cpuinfo
processor	: 0
vendor_id	: GenuineIntel
cpu family	: 6
model		: 158
model name	: Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz
stepping	: 10
microcode	: 0xca
cpu MHz		: 4018.411
cache size	: 9216 KB
physical id	: 0

utterstep@utterstep-nix:~/my/advent-2019$ hyperfine --warmup 20 ./run_all.sh
Benchmark #1: ./run_all.sh
  Time (mean ± σ):      63.0 ms ±   1.3 ms    [User: 57.4 ms, System: 7.0 ms]
  Range (min … max):    60.1 ms …  66.1 ms    46 runs
```

## Growth points

* [day-3](./day-2): check, why precomputed sums looks slower, than brute force approach
* [day-7](./day-7): get rid of [permutohedron](https://crates.io/crates/permutohedron) crate
* [day-13](./day-13): more effective bot strategy (current time ~9ms because there are lot of redundant moves in the end)
* [day-19](./day-19): determine error sign — currently I'm not totaly sure that constants are good for every possible input (though they are quite coservative)
