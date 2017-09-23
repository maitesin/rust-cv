# Oscar Forner's Curriculum
[![Build Status](https://travis-ci.org/maitesin/rust-cv.svg?branch=master)](https://travis-ci.org/maitesin/rust-cv)

Rust application that shows my resume in a terminal UI fashion

![gif](https://raw.githubusercontent.com/maitesin/rust-cv/master/cv.gif)

You can run it locally with `docker run -it maitesin/resume`


## How to build the application locally

```
rustup target add i686-unknown-linux-musl
cargo build --target i686-unknown-linux-musl --release
```

## Build the Docker image

```
docker build -t DOCKER_HUB_USERNAME/resume .
docker tag resume DOCKER_HUB_USERNAME/resume
docker login
docker push DOCKER_HUB_USERNAME/resume
```

Or set up an automated build on the Docker Hub!
