# Disfetch

A discord neofetch generator.

![neofetch](https://user-images.githubusercontent.com/45378255/167285157-7a273777-cd84-485d-a6d0-c0da985f1c1b.png)

## Configuration

Just edit ``config.yaml``.

## Running


``cargo run`` will show you the result, but you can **not** copy from the terminal, you must use a tool like xclip or wl-copy:

```sh
cargo run | wl-copy # if you are using wayland
cargo run | xclip # on xorg
```

Then paste into discord.

## Avalable logos:

- logos/discord: The discord logo

- logos/arch: The arch linux logo
