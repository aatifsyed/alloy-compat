<!-- cargo-rdme start -->

Conversions between [`ethereum_types`] and [`alloy_primitives`].

```rust
use alloy_compat::Compat as _;

// from alloy to ethereum_types
let address = alloy::address!("deadbeefdeadbeefdeadbeefdeadbeef00000000");
let _: eth::Address = address.compat();

// from ethereum_types to alloy
let hash = eth::H256::zero();
let _: alloy::B256 = hash.compat();

// integers are supported
let int = eth::U128::MAX;
assert_eq!(alloy::U128::MAX, int.compat());
```

<!-- cargo-rdme end -->
