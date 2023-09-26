<div align="center">
  <h1>
    Counter-Strike Music Control
  </h1>
  <p>A CLI-Tool to automatically play/pause background music in between Counter-Strike rounds.</p>
</div>

---

## Description

This program expects that your background music is already playing. It will pause the music when a round starts and unpause when you are dead or in between rounds. It simulates pressing of the media-keys so make sure they do what they should (on Windows this should work without a problem).

> [!WARNING]
> This program does not detect quitting from matches, so it will stay in the state it was before quitting.

## Usage

You can build the program with `cargo build`.

In order to function, a gamestate integration configuration file is needed. This file must be located at `<CS:GO-Folder-Path>\csgo\cfg`. To automatically generate one, you can run the following command on Windows

```text
cs-music-control.exe -g <CS-Folder-Path>
```

The CS:GO-Folder-Path usually is `C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive`.

To start the program on Windows, just run `cs-music-control.exe`.

## Credits

- [gsi-csgo](https://github.com/sam-ai56/gsi-csgo) for the payload structs
