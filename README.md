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

origin   >> 43.23 kB (bytes: 43235)
minified >> 9.24 kB (bytes: 9241)
gzipped  >> 4.03 kB (bytes: 4028)
```

### License

MIT License
