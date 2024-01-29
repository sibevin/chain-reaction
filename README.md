# Chain Reaction

An action game to dodge chain reaction particles, at the same time you have to create your own chain.

![Cover](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_cover_1280x720.png)
![Gameplay](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_gameplay.gif)
![Main Menu](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_menu.png)
![Gameplay](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_gameplay.png)
![Help](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_help.png)
![Game Over](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_game_over.png)
![Achievement](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_achievement.png)
![Leaderboard](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_leaderboard.png)
![Settings](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_settings.png)

## Links

- itch.io: https://sibevin.itch.io/chain-reaction
- Github: https://github.com/sibevin/chain-reaction

## Credits

### Game Design

- Kait Wang

### Programming

- Kait Wang

### Art

- Miya

### Icon

- Phosphor Icons - https://phosphoricons.com/

### Font

- SYN Nova - https://www.fontsquirrel.com/fonts/syn-nova
- Silkscreen - https://kottke.org/plus/type/silkscreen/index.html
- VAG-HandWritten - https://www.fontsquirrel.com/fonts/VAG-HandWritten

### BGM

- Synthetic Deception - Loopable Epic Cyberpunk Crime Music - By GioeleFazzeri
  https://pixabay.com/music/suspense-synthetic-deception-loopable-epic-cyberpunk-crime-music-157454/

### SE

- Heavy Cineamtic Hit - By LordSonny - https://pixabay.com/sound-effects/heavy-cineamtic-hit-166888/
- pick - From Pixabay - https://pixabay.com/sound-effects/pick-92276/
- Item Pick Up - From Pixabay - https://pixabay.com/sound-effects/item-pick-up-38258/
- Glass Shatter 3 - From Pixabay - https://pixabay.com/sound-effects/glass-shatter-3-100155/
- Tada Military 3 - By floraphonic - https://pixabay.com/sound-effects/tada-military-3-183975/

## Development

### Build web version

1. Make sure wasm-bindgen-cli is installed. `cargo install wasm-bindgen-cli`
2. `cargo build --release --target wasm32-unknown-unknown`
3. `cp -r wasm build-web`
4. `cp -r assets build-web`
5. `wasm-bindgen --no-typescript --out-name bevy_game --out-dir build-web --target web target/wasm32-unknown-unknown/release/chain-reaction.wasm`

### Release

1. Update `README` and `CHANGE_LOG` if needed.
2. Update version in `Cargo.toml/lock`.
3. Add git tag `git tag -a "vX.Y.Z" -m "vX.Y.Z"`.
4. Push git tags `git push --tags` to trigger CI/CD.

## License

Released under GPLv3
