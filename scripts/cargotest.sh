#!/bin/bash

trap "exit" INT TERM ERR
trap "kill 0" EXIT

node beetlerank/index.js &
sleep 1
cargo test
