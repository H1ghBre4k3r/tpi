# TPI

An small (and WIP) CLI for interacting with your local TuringPi 2 Cluster, written in Rust.

## DISCLAIMER

USE THIS SOFTWARE AT YOUR OWN RISK! I DO NOT TAKE ANY RESPONSIBILITY FOR DAMAGED HARDWARE.

### Prerequisites

In order to be able to compile this software, you need an `armv7-unknown-linux-gnueabihf`-toolchain available on your system. To achieve that, first install the Rust target for that:

```shell
rustup target add armv7-unknown-linux-gnueabihf
```

Secondly, you need some environment variables set up:

```
export CC_armv7_unknown_linux_gnueabihf=armv7-unknown-linux-gnueabihf-gcc
export CXX_armv7_unknown_linux_gnueabihf=armv7-unknown-linux-gnueabihf-g++
export AR_armv7_unknown_linux_gnueabihf=armv7-unknown-linux-gnueabihf-ar
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=armv7-unknown-linux-gnueabihf-gcc
```

These are for correctly linking the program (and doing other cross compilation stuff).

### Compilation

Now, you can compile the CLI via cargo:

```shell
cargo build --release
```

This creates the executable in the `target` folder:

```
./target/armv7-unknown-linux-gnueabihf/release/tpi
```

You can now transfer this file to you Turing Pi 2 BMC, e.g., by using SCP:

```shell
scp target/armv7-unknown-linux-gnueabihf/release/tpi turing:~
```

### Usage

Currently, the CLI only exposes a very light CLI:

```
# ./tpi --help
Software for interacting with the Turing Pi 2.

Usage: tpi [OPTIONS] --node <NODE>

Options:
  -n, --node <NODE>    The node to interact with. Without '--power' this command will print useful node information
  -p, --power <POWER>  By specifying this argument, you either power on or power off the specified node [possible values: on, off]
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

You can get power information about a node:

```
# ./tpi -n 1
Power status of node 1: On
```

..., turn a node off...

```
# ./tpi -n 3 -p off
```

..., or turn a node on:

```
# ./tpi -n 3 -p on
```
