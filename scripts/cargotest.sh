#!/bin/bash

trap "kill 0" EXIT

cd beetlerank
node index.js &
sleep 1
cargo test
