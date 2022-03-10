# Fixed size 256-bit math library

[wip]

## Example

```rust
let x = Uint256::from_hex("0xeff123");
let y = Uint256::from(78);
let mut z = Uint256::default();
z = x + y;
z = x - y;
z = x & y;
z = x | y;
```

Inspired by [uint256.go](https://pkg.go.dev/github.com/holiman/uint256).
