# Chain Reaction

An action game to dodge chain recation particles.

- For Bevy Jam #4 - https://itch.io/jam/bevy-jam-4

![Main Menu](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_01.png)
![Gameplay](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_03.png)
![Help](https://raw.githubusercontent.com/sibevin/chain-reaction/main/screenshots/cr_02.png)

## Links

- itch.io: https://sibevin.itch.io/chain-reaction
- Github: https://github.com/sibevin/chain-reaction

## Development

### Build web

1. Make sure wasm-bindgen-cli is installed. `cargo install wasm-bindgen-cli`
2. `cargo build --release --target wasm32-unknown-unknown`
3. `cp -r wasm build-web`
4. `cp -r assets build-web`
5. `wasm-bindgen --no-typescript --out-name bevy_game --out-dir build-web --target web target/wasm32-unknown-unknown/release/chain-reaction.wasm`

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

## License

Released under GPLv3
