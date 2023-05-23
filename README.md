
### Xilinx

```
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc
cargo build --target aarch64-unknown-linux-gnu --release
```


### Raspberry Pi (3B+)

```
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnueabihf-gcc
cargo build --target armv7-unknown-linux-gnueabihf --release
```