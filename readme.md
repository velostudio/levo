# LEVO: web as it should be

`wasm` file generated using `levo-wasm-generator`.
`brotli` file is created from `wasm` file in `levo-server`

To run it:

`cd levo-server`
`cargo r --release`

to start demo server.

To run client:

`cd levo-client`
`SSLKEYLOGFILE=<PATH_TO_KEY_LOG_FILE> cargo r --release`

Then ssl key log file can be used with `Wireshark` to inspect HTTP3 packets.
