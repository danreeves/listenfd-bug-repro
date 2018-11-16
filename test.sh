#!/bin/bash

cargo build && \
systemfd --no-pid -s http::3000 -- ./target/debug/listenfd-bug;

cargo build --release && \
systemfd --no-pid -s http::3000 -- ./target/release/listenfd-bug;
