FROM rust:alpine

RUN apk add --no-cache gcc lld musl-dev curl openssl-dev
RUN mkdir /backend
WORKDIR /backend
COPY . .
RUN cargo install --path .
EXPOSE 8083

CMD ["kitter"]
