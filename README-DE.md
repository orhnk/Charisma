> **Note** _Englisch_: I suck at german, I am a complete beginer who is trying
> to learn some stuff by adapting my daily life to it. I'd love to see any
> suggestions. Also when writing this text I got a lot of help from the Google
> Translate. So that makes it even worse.

> **Note** _Deustch_: Ich bin schlecht in Deutsch, Ich bin eine absoluter
> Anfänger, der versucht etwas zu lernen. Ich doch so mit adaptieren meine leben
> zu das. Ich würde mich über jeden freuen, der einen Vorschlag hat. Auch diese
> Text algemein habe geschreiben mit das google. Das macht es also noch
> schlimmer

<h1 align="center">Charisma</h1>
<p align="center">
  <img src="https://img.shields.io/github/license/utfeight/charisma">
  <img src="https://img.shields.io/crates/v/charisma">
  <img src="https://img.shields.io/crates/d/charisma">
  <img src="https://img.shields.io/badge/Built%20with%20Rust-ff3a29">
</p>

<img align="left" alt="Retro Vintage Robot" src="https://github.com/UTFeight/Charisma/assets/101834410/07dee25d-1e1a-4b47-a030-f6b7738f1d66" width="120px"/>

<br>

Charisma dient dazu, eine Brücke zwischen Dall-e Mini zu sein. Es ist gewohnt
Generieren Sie ASCII-Grafiken direkt von Ihrem Terminal aus. Zur Konvertierung
wird die Craiyon-API verwendet Deine Worte in die Realität umsetzen. Es ist in
den geliebten Rust geschrieben, mit mächtigen Werkzeugen damit.

Es ist avai verfügbar an crates.io:
[crates.io](https://crates.io/crates/charisma). für weitere Einzelheiten, bitte lese
[die Unterlagen](https://docs.rs/charisma/*/charisma/)

<br>
<br>

<h3 align="center">Merkmale</h3>

- **Frei**: nein erforderlich Zahlung
- **Schnell**: die Algorithmus auf diese Weise parallelisieren das du kann
  generieren eine bilder weniger als eine Sekunde (wenn du benutzt
  [`-N 60`](#help) oder mehr)
- **Leicht**: Homely CLI app with no bloat

<br>

<h2 align="center">Vitrine</h2>

<details><summary> <b>mehr</b></summary>
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

<h2 align="center">Einstieg</h2>

### Voraussetzungen

- **Cargo**: Dieses Projekt wird unter dem
  [cargo](https://github.com/rust-lang/cargo) erstellt und verteilt

### Installation

```shell
cargo install charisma
```

### Verwendung

```shell
charisma "rostiger Krabbenroboter" -c -C block # geschätzte Zeit: ~1min
```

---

> **Notiz**: Diese Befehle sind zu meinen Lernzwecken (lernen deutsch) so bitte
> schreiben Dieses an Englisch an das Terminal

<pre id="help">
Generieren ASCII-Grafiken mith künstliche Intelligenz 🦾🧠

Verwendung: charisma [Optionen] <PROMPT>

Arguments:
  <PROMPT>  Prompt zur Eingabe

Options:
  -n, --negatif-prompt <NEGATIF_PROMPT> Use AI to generate ascii art, but with a negative prompt [default: ]
  -N, --nummer-bilder  <NUMMER>          Number of images to generate when using AI [1..9] [default: 9]
  -m, --modell-typ     <MODEL_TYP>      Model to use in generation [default: general] [possible values: art, drawing, photo, general]
  -v                   <API_VERSION>     Model API version [default: 3] [possible values: 1, 3]
  -a, --api-zeichen    <ZEICHEN>           API token for premium users (Faster generation, No watermark)
  -w, --breite         <BREITE>           Width of the output image. Defaults to 128 if width and height are not specified
  -H, --höhe           <HÖHE>          Height of the output image, if not specified, it will be calculated to keep the aspect ratio
  -c, --farbe                            Whether to use colors in the output image
  -i, --umkehren                         Inverts the weights of the characters. Useful for white backgrounds
  -C, --zeichensatz    <ZEICHENSATZ>         Characters used to render the image, from transparent to opaque. Built-in charsets: block, emoji, default, russian, slight [default: default]
  -h, --hilfe                            Print help
  -V, --version                          Print version

            # Optionen:
            #   -n, --negative-prompt <NEGATIVE_PROMPT> Use AI to generate ascii art, but with a negative prompt [default: ]
            #   -N, --num-image       <NUMBER>          Number of images to generate when using AI [1..9] [default: 9]
            #   -m, --model-type      <MODEL_TYPE>      Model to use in generation [default: general] [possible values: art, drawing, photo, general]
            #   -v                    <API_VERSION>     Model API version [default: 3] [possible values: 1, 3]
            #   -a, --api-token       <TOKEN>           API token for premium users (Faster generation, No watermark)
            #   -w, --width           <WIDTH>           Width of the output image. Defaults to 128 if width and height are not specified
            #   -H, --height          <HEIGHT>          Height of the output image, if not specified, it will be calculated to keep the aspect ratio
            #   -c, --color                             Whether to use colors in the output image
            #   -i, --invert                            Inverts the weights of the characters. Useful for white backgrounds
            #   -C, --charset         <CHARSET>         Characters used to render the image, from transparent to opaque. Built-in charsets: block, emoji, default, russian, slight [default: default]
            #   -h, --help                              Print help
            #   -V, --version                           Print version
</pre>

<h2 align="center">Anerkennung</h2>

<ul style="text-align: center;">
  <li>

[Craiyon](https://www.craiyon.com/blog): [TOU](https://www.craiyon.com/terms)

</li>
</ul>
