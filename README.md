# Marko Dujmovic's Curriculum

Rust application that shows my resume in a terminal UI fashion

First you need to build the Docker image with `docker build -t my_resume .`

You can run it locally with `docker run -it -p 8000 my_resume` (instead of '8000' you can change port)


## How to build the application to be statically linked

```
rustup target add i686-unknown-linux-musl
cargo build --target i686-unknown-linux-musl --release
```

## Build the Docker image

```
docker build -t my_resume .
docker tag resume maitesin/resume
docker login
docker push maitesin/resume
```

