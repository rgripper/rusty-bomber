## Prerequisites

```
cargo install cargo-make
```

```
rustup target add wasm32-unknown-unknown
```

## How to play?
You can download and try it [here](https://github.com/rgripper/rusty-bomber/releases), but currently only the win10 version is compiled.

You can also try it online directly [here](https://rgripper.github.io/rusty-bomber/), but due to network speed and other reasons, the loading time may be very long.

```
    Arrows: move
    Space: set bomb
    Enter: start or restart
    Esc: Exit the game
```
    
## Build and serve WASM version

```
cargo make -p release wasm-serve
```

then point your browser to http://127.0.0.1:4000/

## Build and run native version

```
cargo run --features native-release --release
```

## Bundle and deploy WASM version

```shell
cargo make -p release build-wasm-bundled
cd web
yarn # should really do it only once to install webpack stuff
yarn build
yarn deploy
```

## Developer

```
cargo run --features native
```

## Assets and Attribution

![Bomb Party v4 screenshot](assets/bomb_party_v4.png)

Made by
most: [usr_share](https://opengameart.org/users/usrshare) at
https://opengameart.org/content/bomb-party-the-complete-set

door: [awesomeduck](https://opengameart.org/users/awesomeduck) at
https://opengameart.org/content/wall-door-tileset

speed icon: [antum_deluge](https://opengameart.org/users/antumdeluge) at
https://opengameart.org/content/cc0-footgear-icons

power icon: [victordelima](https://opengameart.org/users/victordelima) at
https://opengameart.org/content/16-bit-rpg-potion-pack

bomb icon: [sprite_attack](https://opengameart.org/users/spriteattack) at
https://opengameart.org/content/emotional-explosives

Licenses:
[CC 3.0](https://creativecommons.org/licenses/by/3.0/)
[CC 1.0](https://creativecommons.org/publicdomain/zero/1.0/)
