#!/bin/bash

trap "kill 0" EXIT

node beetlerank/index.js &
sleep 1
cargo test
