# Cross compilation / cross platform code

Steps:

```
$ systemctl start docker
$ cargo install cross --git https://github.com/cross-rs/cross
$ cross test --target x86_64-unknown-linux-gnu
$ cross test --target x86_64-pc-windows-gnu
$ cross build --examples --target x86_64-unknown-linux-gnu
$ cross build --examples --target x86_64-pc-windows-gnu
```

## Code quality

```
$ cargo install cargo-checkmate
$ cargo checkmate git-hook install
```