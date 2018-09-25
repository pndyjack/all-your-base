# All Your Base

Convert a number, represented as a sequence of digits in one base, to any other base.

Implement general base conversion. Given a number in base **a**,
represented as a sequence of digits, convert it to base **b**.

## About [Positional Notation](https://en.wikipedia.org/wiki/Positional_notation)

In positional notation, a number in base **b** can be understood as a linear
combination of powers of **b**.

The number 42, _in base 10_, means:

(4 _ 10^1) + (2 _ 10^0)

The number 101010, _in base 2_, means:

(1 _ 2^5) + (0 _ 2^4) + (1 _ 2^3) + (0 _ 2^2) + (1 _ 2^1) + (0 _ 2^0)

The number 1120, _in base 3_, means:

(1 _ 3^3) + (1 _ 3^2) + (2 _ 3^1) + (0 _ 3^0)

I think you got the idea!

_Yes. Those three numbers above are exactly the same. Congratulations!_

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. After you get the first test to
pass, open the tests source file which is located in the `tests` directory
and remove the `#[ignore]` flag from the next test and get the tests to pass
again. Each separate test is a function with `#[test]` flag above it.
Continue, until you pass every test.

If you wish to run all tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

To learn more about Rust tests refer to the [online test documentation][rust-tests]
