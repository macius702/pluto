# Pluto: Building Backend Services on Chain

<a href="https://github.com/pu0238/pluto" target="_new">
    <img src="https://img.shields.io/badge/GitHub-pu0238/pluto-blue.svg" alt="GitHub">
</a>

Pluton is an HTTP router implemented on the Internet Computer to facilitate working with HTTP. This router is a framework that allows you to write REST API applications on the blockchain in almost the same way as in Web 2

Pluto is an open-source project that empowers developers to build backend services on the chain. It provides a framework and tools to facilitate the creation of decentralized applications (dApps) with a focus on the backend logic and functionality.

## Features
- Easy-to-use router
- Cors support
- Validation
## Getting Started

To get started with Pluto simply add pluto to "dependencies" in your Cargo.toml

``` toml
[dependencies]
pluto = { git = "https://github.com/pu0238/pluto.git" }
```

## Coming soon
- Authentication and secure tokens
- CLI (project generator, files generator)
- Tests
- Website + Docs
- Examples
- Swagger integration
- Response cache
- Blob response


## Useful commands

You can use the following commands with `dfx`:

- Get the canister ID: `dfx canister id [canisterName]`
- Get the webserver port: `dfx info webserver-port`
- Send a GET request to the canister: `curl http://[canisterId].localhost:[replicaPort]`
- Send a POST request with JSON data to the canister: `curl -X POST -H "Content-Type: application/json" -d "{ \"hello\": \"world\" }" http://[canisterId].localhost:[replicaPort]`
- `curl -X POST -H "Content-Type: application/json" -d "{ \"hello\": \"world\" }" "http://$(dfx canister id d_backend).localhost:$(dfx info webserver-port)"`

You can also use `curl` to send requests to specific URLs:

- `curl http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943`
- `curl "https://[canisterId].ic0.app"`
- `curl -X GET http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943/sample.json`

The following URLs are working:

- `http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943/sample.json`
- `http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943/index.html`