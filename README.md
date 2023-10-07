<div align="center">
  <h1>
    Counter-Strike Music Control
  </h1>
  <p>A CLI-Tool to automatically play/pause background music in between Counter-Strike rounds.</p>
</div>

---

## Description

This program expects that your background music is already playing. It will pause the music when a round starts and unpause when you are dead, in the menu/console or in between rounds. It simulates pressing of the media-keys so make sure they do what they should.

## Usage

You can build the program with `cargo build`.

In order to function, a gamestate integration configuration file is needed. To automatically generate one, you can run the following command

```text
cs-music-control.exe -g
```

This will automatically find your CS installation folder.

You can also provide your own CS installation path.

```text
cs-music-control.exe -g <CS_INSTALLATION_PATH>
```

To start the program, just run

```text
cs-music-control.exe
```

## Credits

- [gsi-csgo](https://github.com/sam-ai56/gsi-csgo) for the payload structs
- [u/Bkid](https://www.reddit.com/r/GlobalOffensive/comments/cjhcpy/game_state_integration_a_very_large_and_indepth/) for a great explanation of GSI
