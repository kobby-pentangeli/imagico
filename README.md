# Imagico

A command-line program for hiding secret messages in PNG files.

The program currently has four(4) commands:

1. *Encode* a message into a PNG file;
2. *Decode* a message stored in a PNG file;
3. *Remove* a message from a PNG file;
4. *Print* a list of PNG chunks that can be searched for messages.

This project is based on the the [pngme book](https://picklenerd.github.io/pngme_book/).

### Setup

Local installation:

```bash
cargo install --path .
```

Remote installation:

```bash
cargo install --git https://github.com/kobby-pentangeli/imagico
```

### Running

- *Encode* (add) a secret message into a PNG file by overwriting the input file:

```bash
imapp encode ./some-file.png RuST "Your secret message here"
```

- *Encode* (add) a secret message into a PNG file without overwriting the original file:

```bash
imapp encode ./inputfile.png RuST "Your secret message here" ./outputfile.png
```

- *Decode* (show) a secret message:

```bash
imapp decode ./some-file.png RuST
```

- *Remove* the secret message:

```bash
imapp remove ./some-file.png RuST
```

- *Print* out all chunks in a PNG file:

```bash
imapp print ./some-file.png
```

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
