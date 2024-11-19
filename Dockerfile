FROM rust:latest AS builder
WORKDIR /usr/src/clash
COPY . .

RUN cargo build --release

FROM ubuntu:latest
RUN apt-get update && apt-get install -y \
  ttyd \
  bash \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/clash/target/release/clash /bin/alexjclarke/clash
RUN useradd aclarke

USER aclarke
WORKDIR /home/aclarke
ENV TERM=xterm
CMD ["/bin/ttyd", "--max-clients", "5", "--writable", "--port", "7681", "/bin/alexjclarke/clash"]
EXPOSE 7681
