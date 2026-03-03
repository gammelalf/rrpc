# rRPC

**gRPC for rust without `.proto` files**

## What

rRPC is gRPC where you declare your services and messages in normal rust code instead of `.proto` files.

### Example

```rust
pub struct GetRandomNumber;
impl Method for GetRandomNumber {
    const ID: &'static str = "GetRandomNumber";

    type IsRequestStreamed = False;
    type IsResponseStreamed = False;

    type Request = GetRandomNumberRequest;
    type Response = GetRandomNumberResponse;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRandomNumberRequest {
    pub min: Option<u64>,
    pub max: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRandomNumberResponse {
    pub number: u64,
}
```

```proto
syntax = "proto3";

service Rng {
  rpc GetRandomNumber (GetRandomNumberRequest) returns (GetRandomNumberResponse);
}

message GetRandomNumberRequest {
  optional fixed64 min = 1;
  optional fixed64 max = 2;
}

message GetRandomNumberResponse {
  required fixed64 number = 1;
}
```

## When

TL;DR server and client are written in rust

The API declaration is written in rust.
Therefore, this project should only be used when both server and client are written in rust.

When you need another language (or might need another language in the future), then you will have a better time with
proper gRPC.

## Why

1. You can use any `serde` compatible type for your messages and don't have to choose from the limited set of protobuf
   compatible types.

2. You don't need a build script (which depends on an external tools - `protoc`) to convert your declaration into rust
   code.

## How

gRPC extends HTTP2 with some core concepts required for an RPC.

It advertises to encode message using Protobuf but does not require it.

This project uses gRPC but replaces Protobuf with JSON.