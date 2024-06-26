# Indivisible

Indivisible is an optimized prime number generator and tester written in Rust.

## Build & Installation

To build the project you will require the Rust compiler and build
system, `cargo`. At which point you simply run `cargo build` in the root
directory of the project. To create an optimized release build append
the `--release` flag to the previous command.

Once a release build has been compiled, you may install Indivisible and
the manpage documentation by running the `install` script. In a similar
manner, you can run the `uninstall` script to remove the previously
installed binary and documentation.

## Usage

The purpose of Indivisible is to find the nth prime and all the primes
before it. The basic usage is `indivisible <n>` where `n` is the ordinal
of the prime you'd like to find. To display all primes before `n`, you
can run verbose mode by using the `--verbose` or simply `-v` option.

Since Indivisible generates primes using previously computed primes, you
can also import prime numbers previously computed with the `--import` or
`-i` option. To store already computed primes you are expected to use
piping like any UNIX user would expect. Here is an example:

```bash
# store first 100 primes in ./primes
indivisible -v 100 > ./primes
# appends next 400 primes
indivisible -i ./primes -v 500 >> ./primes
# display the 600th prime
indivisible -i ./primes 600
```

## Legacy

This project was originally written in C. It can be found in the
[indivisible-legacy repository](https://gitlab.com/naortega/Indivisible-legacy)
on my GitLab.

## License

This project is licensed under the terms & conditions of the GNU General
Public License version 3 or greater (see `LICENSE` file for more
information).
