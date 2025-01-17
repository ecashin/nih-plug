# Crisp

This plugin adds a bright crispy top end to low bass sounds. The effect was
inspired by Polarity's [Fake Distortion](https://youtu.be/MKfFn4L1zeg) video.

## Download

You can download the development binaries for Linux, Windows and macOS from the
[automated
builds](https://github.com/robbert-vdh/nih-plug/actions/workflows/test.yml?query=branch%3Amaster)
page. Or if you're not signed in on GitHub, then you can also find the latest nightly
build [here](https://nightly.link/robbert-vdh/nih-plug/workflows/build/master).

The macOS version has not been tested and may not work correctly. You may also
have to [disable Gatekeeper](https://disable-gatekeeper.github.io/) to use the
VST3 version as Apple has recently made it more difficult to run unsigned code
on macOS.

### Building

Crisp depends on `libxcb-icccm`.

After installing [Rust](https://rustup.rs/), you can compile Crisp
as follows:

```shell
cargo xtask bundle crisp --release
```
