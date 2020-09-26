# URL shortener
This is a URL shortener written in [Rust](https://www.rust-lang.org/) using [Rocket framework](https://rocket.rs/).

This project is a result of my first steps in Rocket and Rust in general, too. The code is based on the [Rocket exmaples](https://github.com/SergioBenitez/Rocket/tree/master/examples).

# Setup & running
1. Install Rust following https://rustup.rs/.
2. Configure Rust nightly as your default toolchain.
```
$ rustup default nightly
```
3. Install `cargo-watch`.
```
$ cargo install cargo-watch
```
4. Install SQLite - macOS:
```
$ brew install sqlite
```
5. Run
```
$ cargo watch -x run
```

# API
## Get all links
`[GET] /api/v1/links`
```
$ curl -X GET http://localhost:8000/api/v1/links
[{"alias":"TfgMq","id":4,"is_active":true,"url":"http://google.com"},{"alias":"aymrB","id":3,"is_active":false,"url":"https://www.bbc.com/"}]
```

## Get a single link
`[GET] /api/v1/links/<alias>`
```
$ curl -X GET http://localhost:8000/api/v1/links/TfgMq
{"alias":"TfgMq","id":4,"is_active":true,"url":"http://google.com"}
```

## Add a new link
`[POST] /api/v1/links`
```
$ curl -X POST \
  http://localhost:8000/api/v1/links \
  -H 'Content-Type: application/json' \
  -d '{
    "url": "http://google.com"
}'
{"link":"http://localhost:8000/redirect/t5ku4","status":"Successfully added a link"}
```
