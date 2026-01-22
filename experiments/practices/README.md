# How to Run the Priority Graph Initialization Test

## Quick Start

Navigate to the practices directory and run the test:

```bash
cd /root/agave/solana_virtual_machine/experiments/practices
cargo test test_priority_graph_initialization
```

## Run All Tests

To run all tests in the directory:
```bash
cargo test
```

## Run with Verbose Output

To see detailed output during the test:
```bash
cargo test test_priority_graph_initialization -- --nocapture
```

The `--nocapture` flag shows all `println!` output during the test.

## Expected Output

When the test passes successfully, you should see:
```
running 1 test
test tests::test_priority_graph_initialization ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## What the Test Does

This test demonstrates:
1. ✅ Creating a new `PrioGraph` instance
2. ✅ Verifying the graph starts empty
3. ✅ Inserting transactions with account access information
4. ✅ Popping transactions in priority order (highest priority first)
5. ✅ Verifying graph state after operations

## Troubleshooting

If you encounter compilation errors:
- Make sure you're in the practices directory: `cd /root/agave/solana_virtual_machine/experiments/practices`
- Try cleaning and rebuilding: `cargo clean && cargo test`
