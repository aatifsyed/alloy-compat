//! Conversions between [`ethereum_types`] and [`alloy_primitives`].
//!
//! ```
//! # use alloy_primitives as alloy; use ethereum_types as eth;
//! use alloy_compat::Compat as _;
//!
//! // from alloy to ethereum_types
//! let address = alloy::address!("deadbeefdeadbeefdeadbeefdeadbeef00000000");
//! let _: eth::Address = address.compat();
//!
//! // from ethereum_types to alloy
//! let hash = eth::H256::zero();
//! let _: alloy::B256 = hash.compat();
//!
//! // integers are supported
//! let int = eth::U128::MAX;
//! assert_eq!(alloy::U128::MAX, int.compat());
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

/// Convert between an [`ethereum_types`] type and an [`alloy_primitives`] type.
///
/// See [module documentation](mod@self) more.
pub trait Compat {
    fn compat<T>(self) -> T
    where
        Self: sealed::Compat<T>;
}

impl<T> Compat for T {
    fn compat<U>(self) -> U
    where
        Self: sealed::Compat<U>,
    {
        sealed::Compat::compat(self)
    }
}

mod sealed {
    use alloy_primitives::FixedBytes;

    pub trait Compat<T> {
        fn compat(self) -> T;
    }

    macro_rules! compat_fixed_bytes {
        ($($(#[$meta:meta])* $alloy:path : $eth:path);* $(;)?) => {
            $(
                $(#[$meta])*
                impl Compat<$eth> for $alloy {
                    fn compat(self) -> $eth {
                        let alloy_primitives::FixedBytes(bytes) = self;
                        $eth(bytes)
                    }
                }
                $(#[$meta])*
                impl Compat<$alloy> for $eth {
                    fn compat(self) -> $alloy {
                        let $eth(bytes) = self;
                        alloy_primitives::FixedBytes(bytes)
                    }
                }
            )*
        };
    }

    compat_fixed_bytes! {
        // alloy_primitives::B32 : ethereum_types::H32;
        alloy_primitives::B64 : ethereum_types::H64;
        alloy_primitives::B128 : ethereum_types::H128;
        // #[allow(deprecated)] alloy_primitives::B160 : ethereum_types::H160;
        alloy_primitives::B256 : ethereum_types::H256;
        // alloy_primitives::B264 : ethereum_types::H264;
        alloy_primitives::B512 : ethereum_types::H512;
        // alloy_primitives::B520 : ethereum_types::H520;
    }

    macro_rules! compat_uint {
        ($($(#[$meta:meta])* $alloy:path : $eth:path);* $(;)?) => {
            $(
                $(#[$meta])*
                impl Compat<$eth> for $alloy {
                    fn compat(self) -> $eth {
                        let bytes = self.to_le_bytes::<{<$alloy>::BYTES}>();
                        <$eth>::from_little_endian(&bytes)
                    }
                }
                $(#[$meta])*
                impl Compat<$alloy> for $eth {
                    fn compat(self) -> $alloy {
                        let mut bytes = [0u8; {<$alloy>::BYTES}];
                        self.to_little_endian(&mut bytes);
                        <$alloy>::from_le_bytes(bytes)
                    }
                }
            )*
        };
    }

    compat_uint! {
        alloy_primitives::U64 : ethereum_types::U64;
        alloy_primitives::U128 : ethereum_types::U128;
        alloy_primitives::U256 : ethereum_types::U256;
        alloy_primitives::U512 : ethereum_types::U512;
    }

    impl Compat<ethereum_types::Address> for alloy_primitives::Address {
        fn compat(self) -> ethereum_types::Address {
            let Self(FixedBytes(bytes)) = self;
            ethereum_types::H160(bytes)
        }
    }
    impl Compat<alloy_primitives::Address> for ethereum_types::Address {
        fn compat(self) -> alloy_primitives::Address {
            let Self(bytes) = self;
            alloy_primitives::Address(FixedBytes(bytes))
        }
    }

    impl Compat<ethereum_types::Bloom> for alloy_primitives::Bloom {
        fn compat(self) -> ethereum_types::Bloom {
            let alloy_primitives::Bloom(alloy_primitives::FixedBytes(src)) = self;
            ethereum_types::Bloom(src)
        }
    }
    impl Compat<alloy_primitives::Bloom> for ethereum_types::Bloom {
        fn compat(self) -> alloy_primitives::Bloom {
            let ethereum_types::Bloom(src) = self;
            alloy_primitives::Bloom(alloy_primitives::FixedBytes(src))
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    use alloy_primitives::{self as alloy, fixed_bytes};
    use ethereum_types as eth;

    #[test]
    fn address() {
        let alloy = alloy::address!("deadbeefdeadbeefdeadbeefdeadbeef00000000");
        assert_eq!(format!("{alloy:x}"), format!("{:x}", alloy.compat()));
        assert_eq!(alloy, alloy.compat().compat::<alloy::Address>());
    }

    #[test]
    fn u128() {
        let alloy = alloy::U128::MAX - alloy::U128::from(1);
        let eth = eth::U128::MAX - eth::U128::from(1);
        assert_eq!(alloy, eth.compat());
        assert_eq!(eth, alloy.compat());
    }

    #[test]
    fn bloom() {
        const CHUNK: usize = 8;
        let mut alloy = fixed_bytes!();
        for chunk in alloy.chunks_exact_mut(CHUNK) {
            chunk.copy_from_slice(b"deadbeef");
        }
        alloy
            .last_chunk_mut::<CHUNK>()
            .unwrap()
            .copy_from_slice(b"f0000000");
        let alloy = alloy::Bloom(alloy);
        assert_eq!(
            serde_json::to_value(alloy).unwrap(),
            serde_json::to_value(alloy.compat()).unwrap()
        );
    }
}
