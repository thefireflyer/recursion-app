# CS240 | Final

|           |               |
| --------- | ------------- |
| Author    | Aidan Beil    |
| Date      | 20/3/2024     |
| Class     | CS240 2963    |
| Professor | Darrell Criss |

---

## Videos

- [Spell checker video](https://youtu.be/wyySKhGMwgY)
- [Recursion video]()

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
