## Archive

Because of the latest API Changes done to the craiyon API this project stopped working

<h1 align="center">Charisma</h1>
<p align="center">
  <img src="https://img.shields.io/github/license/utfeight/charisma">
  <img src="https://img.shields.io/crates/v/charisma">
  <img src="https://img.shields.io/crates/d/charisma">
  <img src="https://img.shields.io/badge/Built%20with%20Rust-ff3a29">
</p>


<img align="left" alt="Retro Vintage Robot" src="https://github.com/UTFeight/Charisma/assets/101834410/07dee25d-1e1a-4b47-a030-f6b7738f1d66" width="120px"/>

<br>

Charisma serves a purpose of being a bridge between Dall-e mini. It's used to generate ASCII Art directly from your terminal. It uses Craiyon API to convert your words into reality. It's written in beloved rust with powerful tools come with it.


It's available as a rust crate on [crates.io](https://crates.io/crates/charisma). For further implementation details please read [the docs](https://docs.rs/charisma/*/charisma/)

<br>
<br>

<h3 align="center">Features</h3>

- **Free**: No payment required for any feature
- **Fast**: Algorithms parallelize in such a way that you can generate an image less than a second! (If you use [`-N 60`](#help) or more)
- **Simple**: Homely CLI app with no bloat

<br>

<h2 align="center">Showcase</h2>

<details><summary> <b>More</b></summary>
  <p align="center">
    <img src="https://github.com/UTFeight/Charisma/assets/101834410/36f0e333-79a2-4cc0-9dd9-a3c1d2e23c96">
    <br>
    <br>
    <img src="https://github.com/UTFeight/Charisma/assets/101834410/a01a68fb-49f1-499d-b185-f7b548dda21d">
    <br>
    <br>
    <img src="https://github.com/UTFeight/Charisma/assets/101834410/5916607d-92ec-4c13-a89e-731f96b41320">
  </p>
</details>


<p align="center">
  <img src="https://github.com/UTFeight/Charisma/assets/101834410/7c57dca9-c06d-4ddb-bc65-2afd4009c037">
</p>

<h2 align="center">Getting started</h2>

### Prerequisites
  - **Cargo:** This project is built and published using [cargo](https://github.com/rust-lang/cargo) package manager

### Installation
```shell
cargo install charisma
```

### Usage

```shell
charisma "rusty crab robot" -c -C block # Estimated time ~1min
```

---------------------------------------

<pre id="help">
Generate ASCII Art using AI 🦾🧠

Usage: charisma [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  Prompt to enter

Options:
  -n, --negative-prompt <NEGATIVE_PROMPT> Use AI to generate ascii art, but with a negative prompt [default: ]
  -N, --num-image       <NUMBER>          Number of images to generate when using AI [1..9] [default: 9]
  -m, --model-type      <MODEL_TYPE>      Model to use in generation [default: general] [possible values: art, drawing, photo, general]
  -v                    <API_VERSION>     Model API version [default: 3] [possible values: 1, 3]
  -a, --api-token       <TOKEN>           API token for premium users (Faster generation, No watermark)
  -w, --width           <WIDTH>           Width of the output image. Defaults to 128 if width and height are not specified
  -H, --height          <HEIGHT>          Height of the output image, if not specified, it will be calculated to keep the aspect ratio
  -c, --color                             Whether to use colors in the output image
  -i, --invert                            Inverts the weights of the characters. Useful for white backgrounds
  -C, --charset         <CHARSET>         Characters used to render the image, from transparent to opaque. Built-in charsets: block, emoji, default, russian, slight [default: default]
  -h, --help                              Print help
  -V, --version                           Print version
</pre>

<h2 align="center">Acknowledgments</h2>

<ul style="text-align: center;">
  <li>
  
  [Craiyon](https://www.craiyon.com/blog): [TOU](https://www.craiyon.com/terms)
  
  </li>
</ul> 
