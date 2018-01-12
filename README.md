# Oscar Forner's Curriculum
[![Build Status](https://travis-ci.org/maitesin/rust-cv.svg?branch=master)](https://travis-ci.org/maitesin/rust-cv)

Rust application that shows my resume in a terminal UI fashion

![gif](https://raw.githubusercontent.com/maitesin/rust-cv/master/cv.gif)

You can run it locally with `docker run -it maitesin/resume`


## How to build the application to be statically linked

```
rustup target add i686-unknown-linux-musl
cargo build --target i686-unknown-linux-musl --release
```

## Build the Docker image

```
docker build -t resume .
docker tag resume maitesin/resume
docker login
docker push maitesin/resume
```

## Automatically build
Since the change for multi-stage Dockerfile I set up an automatic build in DockerHub: https://hub.docker.com/r/maitesin/resume
