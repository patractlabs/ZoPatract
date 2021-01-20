# Getting Started

## Installation

### Remix online IDE

To write your first SNARK program, check out the ZoPatract plugin in the [Remix online IDE](https://remix.ethereum.org)!

### One-line installation

We provide one-line installation for Linux, MacOS and FreeBSD:

```bash
curl -LSfs get.zoprat.es | sh
```

### Docker

ZoPatract is available on Dockerhub.

```bash
docker run -ti zopatract/zopatract /bin/bash
```

From there on, you can use the `zopatract` CLI.

### From source

You can build ZoPatract from [source](https://github.com/ZoPatract/ZoPatract/) with the following commands:

```bash
git clone https://github.com/ZoPatract/ZoPatract
cd ZoPatract
cargo +nightly build --release
cd target/release
```

## Hello ZoPatract!

First, create the text-file `root.zop` and implement your program. In this example, we will prove knowledge of the square root `a` of a number `b`:

```zopatract
{{#include ../../zopatract_cli/examples/book/factorize.zop}}
```

Some observations:
- The keyword `field` is the basic type we use, which is an element of a given prime field.
- The keyword `private` signals that we do not want to reveal this input, but still prove that we know its value.

Then run the different phases of the protocol:

```bash
# compile
zopatract compile -i root.zop
# perform the setup phase
zopatract setup
# execute the program
zopatract compute-witness -a 337 113569
# generate a proof of computation
zopatract generate-proof
# export a solidity verifier
zopatract export-verifier
```

The CLI commands are explained in more detail in the [CLI reference](reference/cli.md).
