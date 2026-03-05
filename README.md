# Indivisible

Indivisible is an optimized prime number generator and tester using a segmented
version of the [Sieve of
Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).

## Build & Installation

Build a binary release using the following command:

```console
$ cargo build -r
```

Once compiled, you can install it using the `install` script. By default this
will install to `/usr/local/{bin,share}`, but you can optionally change the
`PREFIX` used as follows:

```console
$ PREFIX="~/.local" ./install
```

Additionally there is an `uninstall` script available. If you have used a custom
`PREFIX` this must also be specified to uninstall in the same manner. To see
where you've installed it you can always use the following command:

```console
$ which indivisible
```

## Legacy

This project was originally written in C. It can be found in the
[indivisible-legacy repository](https://code.ortegas.org/nortega/indivisible-legacy).

## License

This project is licensed under the terms & conditions of the GNU General
Public License version 3 or greater (see `LICENSE` file for more
information).
