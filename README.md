
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


### Key and stuff
Keys were generated with:
```
openssl rand -base64 32.
```

loosely following these guidelines:

[https://rocket.rs/v0.5-rc/guide/configuration/](https://rocket.rs/v0.5-rc/guide/configuration/)

```
ROCKET_SECRET_KEY=+KS94aFsclxip0qY54kx82g5APnF4a2pdyMiDhny9M10wfbuYPS6dZp1OaaKTceo5WDCUbWaNqD1Z8v3tEEfOg== ./target/release/pisat-api
```