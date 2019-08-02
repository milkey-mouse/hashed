#![cfg_attr(feature = "nightly", feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), allow(deprecated))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::collections::hash_map::DefaultHasher;

#[cfg(not(feature = "std"))]
// TODO: why is DefaultHasher not in core‽
use core::hash::SipHasher as DefaultHasher;

use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

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
pub struct Hashed<T: ?Sized + Hash> {
    value: u64,
    hashee_type: PhantomData<T>,
}

impl<T: ?Sized + Hash> Debug for Hashed<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Hashed<{}>({:x})", type_name::<T>(), self.value)
    }
}

impl<T: Hash> From<T> for Hashed<T> {
    fn from(hashee: T) -> Self {
        Self::new(&hashee)
    }
}

impl<T: ?Sized + Hash> Into<u64> for Hashed<T> {
    fn into(self) -> u64 {
        self.value
    }
}

impl<T: ?Sized + Hash> Hashed<T> {
    /// Create a new Hashed<T> from &T. Note that this function doesn't consume
    /// the input, so if it is later modified the hash will not update. To stop
    /// these kinds of errors, it is recommended to use the From or Into traits
    /// instead of this function whenever possible.
    pub fn new(hashee: &T) -> Self {
        let mut hasher = DefaultHasher::new();
        hashee.hash(&mut hasher);
        Self {
            value: hasher.finish(),
            hashee_type: PhantomData,
        }
    }

    /// Get the actual hash value the Hashed<T> is wrapping.
    pub fn value(&self) -> u64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Hashed;

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

        let a = Hashed::new(&x);  // Hashed::new() allows us to keep using the hashee

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
}
