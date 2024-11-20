# hotsave

## Summary

Immediately backup and restore a backup for a single file using global hotkeys.

## Purpose

An example use case for me personally is saving current game save states to restore them on a key press later. This is exceptionally useful
for challenging games, like the Souls series, where on death you would have to redo a time intensive and cumbersome running section to reach a
boss again.

While there are already some tools out there, I decided to do this application for myself for the following reasons:

- trying out cli related Rust crates
- implementing a CI/CD process for a compiled binary as learning
- concentrate on core functionality instead of providing additional features like save management
- no GUI
- fun

## Requirements

### Linux

Because this tool listens on global keystrokes, the user running this application needs to be in the **input** group.

## Installation

### Build from source

You can install this cli application from this **git repository** using **cargo**

```shell
cargo install --git https://github.com/robin-thoene/hotsave
```

### Prebuilt

To get a prebuilt binary check out the [release page](https://github.com/robin-thoene/hotsave/releases) and download it there.

## Customization

You can customize the application behavior by creating a config file located at **~/.config/hotsave/hotsave.toml** with the contents
(note that the values are examples):

```shell
save_file_key = "F7"
restore_file_key = "F8"
```

If you are unsure what the name of the key is you want to customize, start the app with the environment variable **RUST_LOG=debug**.
Just press the desired key, the debug log will show you the **KeyPress** value of that key that you need to use in the config.
