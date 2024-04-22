# This is a template for my Bevy projects, which have to be build with VSCode on Windows.

## **Crates**

1. **Bevy**
2. **bevy_embedded_assets**: embed assets folder inside binary files (including web)
3. **bevy_inspector_egui**: to inspect entities in the game world.

## Details

Project contains ball.png asset file and main.rs file with code for scene setup and basic 2D character controller.

_.vscode_ contains *tasks.json* that is used for executing rust code with shortcut: <code>CTRL + SHIFT + B</code>.

_.cargo_ contains code for dynamic linking and faster compiling.

_rust-toolchain.toml_ is used for turning on the rust nightly compiler.

## WASM Build

**Run the following to build for web:** 

1. Install or update _**wasm-bindgen**_ , _**wasm-bindgen-cli**_ , **_wasm-opt_** dependencies: <code>cargo update -p wasm-bindgen</code>, <code>cargo update -p wasm-bindgen-cli</code>
2. Comment dynamic linking and optimisations in **cargo.toml** and execute <code>cargo build --release --target wasm32-unknown-unknown</code>.
3. Optimise the wasm build <code>wasm-opt -Oz out/bevy_project_template_bg.wasm --output out/bevy_project_template_bg.wasm</code>
