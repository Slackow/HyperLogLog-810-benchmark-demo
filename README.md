# HyperLogLog Demo

This program serves as a simple benchmark of HyperLogLog, Using its Rust package.

To run it do the following, after making sure you have git and cargo installed:
```bash
git clone https://github.com/Slackow/HyperLogLog-810-benchmark-demo.git hll-demo
cd hll-demo

cargo build --release
# The parameter "16" in this case refers to the precision,
# and can be any value from 4-16 (default: 16)
./target/release/hll-demo 16

# you can also do the following:
cargo run --release 16
```