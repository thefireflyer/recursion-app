# CS240 | Final

|           |               |
| --------- | ------------- |
| Author    | Aidan Beil    |
| Date      | 20/3/2024     |
| Class     | CS240 2963    |
| Professor | Darrell Criss |

[![Rust](https://github.com/thefireflyer/recursion-app/actions/workflows/rust.yml/badge.svg)](https://github.com/thefireflyer/recursion-app/actions/workflows/rust.yml)

## Videos

- [Spell checker video](https://youtu.be/wyySKhGMwgY)
- [Recursion video](https://youtu.be/ewVa0XAZUMw)

## Spell Checker

I have not made any changes to this project, but it is still available [here](https://github.com/thefireflyer/midterm/tree/master/SpellChecker).

## Code Library

I've only made a couple of small changes, but as usual, the source code is available [here](https://github.com/thefireflyer/cs-240-library) (rust implementations) and [here](https://github.com/thefireflyer/cs240-w5-w6-combined) (for C# implementations).

## Recursion

### Organization

- Iterative and recursive functions (as well as unit testing)
  - [`expo`](/recursion-lib/src/expo.rs)
  - [`int_to_bin`](/recursion-lib/src/int_to_bin.rs)
  - [`lcd`](/recursion-lib/src/lcd.rs)
- [Benchmarking code](/benches/bench_main.rs)
- [Benchmark report](/index.html)

### Approach

**Exponents**

I pretty much just wrote down the mathematical definition of exponentiation.

**Integers to binary**

This was trickier and I'm less happy with my result, but it works well enough. I basically looked a couple of example numbers and results and tried to look for recurring patterns. After playing around with modulo and similar, I ended with the idea that you could find the value associated with a bit and use that to check whether it should be on. I then slowly worked my way through the iterative implementation and then quickly wrote up the equivalent recursive code.

**LCD**

This was the most interesting and I mostly made progress by just trying stuff out in my head. I had already implemented the exponent algorithm and had originally wanted to support fractional exponents, but didn't get it to work, so I had already kind of been thinking about representing and working with fractions. I then mostly worked through what I remembered in middle school math class. Once I got the core ideas down, I starting implementing the iterative version, starting with just simple cases like 1/1 or similar. After getting that to work, I worked through cases like (n\*m)/d and then moved on to (n)/(d\*m). The last part was finding common factors. I just wrote down everything I knew in a mathematical sense and worked from that.

For all of the above, I got unit tests working pretty much out of the gate, before actually implementing the solutions.
I actually also wrote the benchmarking code before implementing most of them as well.

### Usage

`cargo bench`

> Run benchmarks

`cd recursion-lib; cargo test`

> ```
> running 6 tests
> test int_to_bin::tests::iterative ... ok
> test int_to_bin::tests::recursive ... ok
> test expo::tests::iterative ... ok
> test expo::tests::recursive ... ok
> test lcd::tests::iterative ... ok
> test lcd::tests::recursive ... ok
>
> test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.28s
>
>    Doc-tests recursion-lib
>
> running 0 tests
>
> test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
> ```
