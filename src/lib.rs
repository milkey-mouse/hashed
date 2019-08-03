#![cfg_attr(feature = "nightly", feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), allow(deprecated))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::collections::hash_map::DefaultHasher;

#[cfg(not(feature = "std"))]
// TODO: why is DefaultHasher not in core‽
use core::hash::SipHasher as DefaultHasher;

use core::fmt::{self, Debug, Formatter, LowerHex};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

#[cfg(feature = "truncate")]
pub use num_traits::AsPrimitive;

#[cfg(not(feature = "truncate"))]
pub trait AsPrimitive<T> {
    fn as_(self) -> T;
}

#[cfg(not(feature = "truncate"))]
impl<T> AsPrimitive<T> for T {
    fn as_(self) -> T {
        self
    }
}

#[cfg(feature = "nightly")]
fn type_name<T: ?Sized>() -> &'static str {
    unsafe { core::intrinsics::type_name::<T>() }
}

#[cfg(not(feature = "nightly"))]
fn type_name<T: ?Sized>() -> &'static str {
    "?"
}

// TODO: be generic over Hasher

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HashedGeneric<T, V>
where
    T: ?Sized + Hash,
    V: AsPrimitive<u64> + Copy,
    u64: AsPrimitive<V>,
{
    value: V,
    hashee_type: PhantomData<T>,
}

pub type Hashed<T> = HashedGeneric<T, u64>;

#[cfg(feature = "truncate")]
pub type Hashed32<T> = HashedGeneric<T, u32>;
#[cfg(feature = "truncate")]
pub type Hashed16<T> = HashedGeneric<T, u16>;
#[cfg(feature = "truncate")]
pub type Hashed8<T> = HashedGeneric<T, u8>;

impl<T, V> Debug for HashedGeneric<T, V>
where
    T: ?Sized + Hash,
    V: AsPrimitive<u64> + LowerHex + Copy,
    u64: AsPrimitive<V>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Hashed<{}>({:x})", type_name::<T>(), self.value)
    }
}

impl<T, V> Default for HashedGeneric<T, V>
where
    T: Hash + Default,
    V: AsPrimitive<u64> + Copy,
    u64: AsPrimitive<V>,
{
    fn default() -> Self {
        T::default().into()
    }
}

impl<T, V> From<T> for HashedGeneric<T, V>
where
    T: Hash,
    V: AsPrimitive<u64> + Copy,
    u64: AsPrimitive<V>,
{
    fn from(hashee: T) -> Self {
        Self::new(&hashee)
    }
}

// Due to the limitations of the Into trait, this always converts into a u64.
// To obtain the inner value as its "native" data type (it may be truncated),
// use the Hashed::value() function instead.
impl<T, V> Into<u64> for HashedGeneric<T, V>
where
    T: ?Sized + Hash,
    V: AsPrimitive<u64> + Copy,
    u64: AsPrimitive<V>,
{
    fn into(self) -> u64 {
        self.value.as_()
    }
}

impl<T, V> HashedGeneric<T, V>
where
    T: ?Sized + Hash,
    V: AsPrimitive<u64> + Copy,
    u64: AsPrimitive<V>,
{
    /// Create a new Hashed<T> from &T. Note that this function doesn't consume
    /// the input, so if it is later modified the hash will not update. To stop
    /// these kinds of errors, it is recommended to use the From or Into traits
    /// instead of this function whenever possible.
    pub fn new(hashee: &T) -> Self {
        let mut hasher = DefaultHasher::new();
        hashee.hash(&mut hasher);
        Self {
            value: hasher.finish().as_(),
            hashee_type: PhantomData,
        }
    }

    /// Get the actual hash value the Hashed<T> is wrapping.
    pub fn value(&self) -> V {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hashed() {
        let _ = Hashed::from("hello");
    }

    #[test]
    fn into_hashed() {
        let _: Hashed<_> = "hello".into();
    }

    #[test]
    #[cfg(feature = "std")]
    fn new_hashed_str() {
        let mut x = String::from("hello");

        let a = Hashed::new(&x); // Hashed::new() allows us to keep using the hashee

        x.push_str("world");

        let b = Hashed::new(&x);

        assert_ne!(a, b);
    }

    #[test]
    fn new_hashed_int() {
        let mut x = 1337;

        let a = Hashed::new(&x);

        x += 30000;

        let b = Hashed::new(&x);

        assert_ne!(a, b);
    }

    #[test]
    fn hashed_eq() {
        let a = 1337;
        let b = 1337;

        assert_eq!(a, b);
        assert_eq!(Hashed::from(a), Hashed::from(b));
    }

    #[test]
    fn hashed_not_eq() {
        let a = "hello";
        let b = "world";

        assert_ne!(a, b);
        assert_ne!(Hashed::from(a), Hashed::from(b));
    }

    #[test]
    fn into_u64() {
        let a = 1337;
        let b = 1337;

        let a_hash_value: u64 = Hashed::from(a).into();
        let b_hash_value: u64 = Hashed::from(b).into();

        assert_eq!(a, b);
        assert_eq!(a_hash_value, b_hash_value);
    }

    #[test]
    #[cfg(feature = "truncate")]
    fn hashed_eq_32() {
        let a = 1337;
        let b = 1337;

        assert_eq!(a, b);
        assert_eq!(Hashed32::from(a), Hashed32::from(b));
    }

    #[test]
    #[cfg(feature = "truncate")]
    fn hashed_eq_16() {
        let a = 1337;
        let b = 1337;

        assert_eq!(a, b);
        assert_eq!(Hashed16::from(a), Hashed16::from(b));
    }

    #[test]
    #[cfg(feature = "truncate")]
    fn hashed_not_eq_32() {
        let a = "hello";
        let b = "world";

        assert_ne!(a, b);
        assert_ne!(Hashed32::from(a), Hashed32::from(b));
    }

    #[test]
    #[cfg(feature = "truncate")]
    fn hashed_not_eq_16() {
        let a = "hello";
        let b = "world";

        assert_ne!(a, b);
        assert_ne!(Hashed16::from(a), Hashed16::from(b));
    }

    // not doing a hashed_eq_8 because 8-bit hashes are rather likely to collide

    #[test]
    fn default_impl() {
        let x: Hashed<_> = Some(1337).into();
        let default = Hashed::default();  // default for Option is None

        assert_ne!(x, default);
    }
}
