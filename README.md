# ClaSH
The ClaSH shell for my personal website

## Building
To build the ClaSH shell, run the following command:
```shell
cargo build --release
```

## Running
To run the ClaSH shell, you can do the following:

### Run the debug executable
```shell
cargo run
```

### Run the release executable
```shell
./target/release/clash
```

### Run the container
```shell
docker build -t clash .
docker run --rm -it -p 7681:7681 clash
```

### Run the container using docker-compose
```shell
docker compose up
```

## Creator
This project and shell is the property of [Alex Clarke](https://github.com/Dark-Alex-17)
