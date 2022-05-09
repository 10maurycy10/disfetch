# Disfetch

A discord neofetch generator.

![neofetch](https://user-images.githubusercontent.com/45378255/167285157-7a273777-cd84-485d-a6d0-c0da985f1c1b.png)

## Configuration

Just edit ``config.yaml``.

## Running


``cargo run`` will show you the result, but you can **not** copy from the terminal, you must use a tool like `xclip` or `wl-copy` (`clip` on Microsoft Windows):

```sh
cargo run | wl-copy # if you are using wayland
cargo run | xclip # on xorg
cargo run | clip # on Windows
```

Then paste into discord.

## Avalable logos:

- logos/discord: The discord logo
- logos/discord\_small: The smaller discord logo 
- logos/arch: The arch linux logo
- logos/debian: The standard debian gnu/linux logo
- logos/windows_10: The Microsoft Windows 10 logo
- logos/windows_11: The Microsoft Windows 11 logo
