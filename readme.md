# Hidutil wrapper

As I have a sticky capslock key, that no one uses anyway, this small hidutil wrapper switches
the capslock key with the tab key on macOS.

## How to use

```sh
cargo install --path .
```

Then map, to map it and none, to undo it

```sh
hidutil-wrapper map
```

```sh
hidutil-wrapper none
```
