## js_sizers
> Show your javascript file size after minification and gzip

### Install

```sh
cargo install js_sizers
```

### Usage

```sh
js_sizers ./test.js
```

```sh
# Output
Reading file: ./test.js...

origin   >> 20.38 kB (20384 bytes)
minified >> 3.73 kB (3733 bytes)
gzipped  >> 1.89 kB (1894 bytes)
```

### Development

```sh
cargo build

touch test.js # create a test file as input and edit
cargo run ./test.js
```

### License

MIT License
