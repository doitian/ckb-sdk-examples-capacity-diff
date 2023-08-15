# ckb-sdk-examples-capacity-diff

This is an example lock script which verifies the witness matches the capacity difference.

- The script loads the witness for the first input in the script group using the WitnessArgs layout.
- The total input capacity is the sum of all the input cells in the script group.
- The total output capacity is the sum of all the output cells having the same lock script as the script group.
- The capacity difference is a 64-bit signed integer which equals to total output capacity minus total input capacity.
- The witness is encoded using two's complement and little endian.

## Build contracts

``` sh
capsule build
```

Run tests:

``` sh
capsule test
```
