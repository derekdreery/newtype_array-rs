//!# `newtype_array`
//!
//!This crate has a single macro, `newtype_arrays`, that will create transparent newtypes for arrays,
//!and implement standard traits for them. It will be redundant when generic cosntants land, in the
//!mean time it means you can use large arrays on stable rust.
//!
//!# Examples
//!
//!```rust
//!#[macro_use]
//!extern crate newtype_array;
//!
//!use std::collections::HashMap;
//!
//!# fn main() {
//!// Sha385 length
//!newtype_array!(pub struct Array48(pub 48));
//!// Sha512 length
//!newtype_array!(pub struct Array64(pub 64));
//!
//!// We've got `Clone` and `PartialEq`/`Eq`
//!let arr1 = Array48([0u8; 48]);
//!let arr2 = arr1.clone();
//!assert_eq!(arr1, arr2);
//!
//!// `Hash` is implemented as well
//!let mut map = HashMap::new();
//!map.insert(arr1, "hello");
//!# }
//!```

// Lifted from rust/src/libcore/array.rs.

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_slice_eq1 {
    ($Lhs: ty, $Rhs: ty) => {
        __impl_slice_eq1! { $Lhs, $Rhs, Sized }
    };
    ($Lhs: ty, $Rhs: ty, $Bound: ident) => {
        impl<'a, 'b, A: $Bound, B> PartialEq<$Rhs> for $Lhs where A: PartialEq<B> {
            #[inline]
            fn eq(&self, other: &$Rhs) -> bool { self[..] == other[..] }
            #[inline]
            fn ne(&self, other: &$Rhs) -> bool { self[..] != other[..] }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_slice_eq2 {
    ($Lhs: ty, $Rhs: ty) => {
        __impl_slice_eq2! { $Lhs, $Rhs, Sized }
    };
    ($Lhs: ty, $Rhs: ty, $Bound: ident) => {
        __impl_slice_eq1!($Lhs, $Rhs, $Bound);

        impl<'a, 'b, A: $Bound, B> PartialEq<$Lhs> for $Rhs where B: PartialEq<A> {
            #[inline]
            fn eq(&self, other: &$Lhs) -> bool { self[..] == other[..] }
            #[inline]
            fn ne(&self, other: &$Lhs) -> bool { self[..] != other[..] }
        }
    }
}

#[doc(hidden)]
// macro for implementing n-element array functions and operations
#[macro_export]
macro_rules! __array_impls {
    ($name:ident, $size:expr) => {
        impl<T> AsRef<[T]> for $name<T> {
            #[inline]
            fn as_ref(&self) -> &[T] {
                &self.0[..]
            }
        }

        impl<T> AsMut<[T]> for $name<T> {
            #[inline]
            fn as_mut(&mut self) -> &mut [T] {
                &mut self.0[..]
            }
        }

        impl<T> ::std::borrow::Borrow<[T]> for $name<T> {
            fn borrow(&self) -> &[T] {
                &self.0
            }
        }

        impl<T> ::std::borrow::BorrowMut<[T]> for $name<T> {
            fn borrow_mut(&mut self) -> &mut [T] {
                &mut self.0
            }
        }

        impl<T: ::std::hash::Hash> ::std::hash::Hash for $name<T> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                ::std::hash::Hash::hash(&self.0[..], state)
            }
        }

        impl<T: ::std::fmt::Debug> ::std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&&self.0[..], f)
            }
        }

        impl<'a, T> IntoIterator for &'a $name<T> {
            type Item = &'a T;
            type IntoIter = ::std::slice::Iter<'a, T>;

            fn into_iter(self) -> ::std::slice::Iter<'a, T> {
                self.0.iter()
            }
        }

        impl<'a, T> IntoIterator for &'a mut $name<T> {
            type Item = &'a mut T;
            type IntoIter = ::std::slice::IterMut<'a, T>;

            fn into_iter(self) -> ::std::slice::IterMut<'a, T> {
                self.0.iter_mut()
            }
        }

        // NOTE: some less important impls are omitted to reduce code bloat
        __impl_slice_eq1! { $name<A>, $name<B> }
        //__impl_slice_eq2! { $name<A>, [B] }
        //__impl_slice_eq2! { $name<A>, &'b [B] }
        //__impl_slice_eq2! { $name<A>, &'b mut [B] }
        // __impl_slice_eq2! { [A; $N], &'b [B; $N] }
        // __impl_slice_eq2! { [A; $N], &'b mut [B; $N] }

        impl<T:Eq> Eq for $name<T> { }

        impl<T:PartialOrd> PartialOrd for $name<T> {
            #[inline]
            fn partial_cmp(&self, other: &$name<T>) -> Option<::std::cmp::Ordering> {
                PartialOrd::partial_cmp(&&self[..], &&other[..])
            }
            #[inline]
            fn lt(&self, other: &$name<T>) -> bool {
                PartialOrd::lt(&&self[..], &&other[..])
            }
            #[inline]
            fn le(&self, other: &$name<T>) -> bool {
                PartialOrd::le(&&self[..], &&other[..])
            }
            #[inline]
            fn ge(&self, other: &$name<T>) -> bool {
                PartialOrd::ge(&&self[..], &&other[..])
            }
            #[inline]
            fn gt(&self, other: &$name<T>) -> bool {
                PartialOrd::gt(&&self[..], &&other[..])
            }
        }

        impl<T:Ord> Ord for $name<T> {
            #[inline]
            fn cmp(&self, other: &$name<T>) -> ::std::cmp::Ordering {
                Ord::cmp(&&self[..], &&other[..])
            }
        }

        // Indexing

        impl<T> ::std::ops::Index<usize> for $name<T> {
            type Output = T;

            fn index(&self, idx: usize) -> &T {
                &self.0[idx]
            }
        }

        impl<T> ::std::ops::IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, idx: usize) -> &mut T {
                &mut self.0[idx]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::Range<usize>> for $name<T> {
            type Output = [T];

            fn index(&self, index: ::std::ops::Range<usize>) -> &[T] {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::IndexMut<::std::ops::Range<usize>> for $name<T> {
            fn index_mut(&mut self, index: ::std::ops::Range<usize>) -> &mut [T] {
                &mut self.0[index]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::RangeFrom<usize>> for $name<T> {
            type Output = [T];

            fn index(&self, index: ::std::ops::RangeFrom<usize>) -> &[T] {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::IndexMut<::std::ops::RangeFrom<usize>> for $name<T> {
            fn index_mut(&mut self, index: ::std::ops::RangeFrom<usize>) -> &mut [T] {
                &mut self.0[index]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::RangeTo<usize>> for $name<T> {
            type Output = [T];

            fn index(&self, index: ::std::ops::RangeTo<usize>) -> &[T] {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::IndexMut<::std::ops::RangeTo<usize>> for $name<T> {
            fn index_mut(&mut self, index: ::std::ops::RangeTo<usize>) -> &mut [T] {
                &mut self.0[index]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::RangeInclusive<usize>> for $name<T> {
            type Output = [T];

            fn index(&self, index: ::std::ops::RangeInclusive<usize>) -> &[T] {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::IndexMut<::std::ops::RangeInclusive<usize>> for $name<T> {
            fn index_mut(&mut self, index: ::std::ops::RangeInclusive<usize>) -> &mut [T] {
                &mut self.0[index]
            }
        }

        impl<T> ::std::ops::Index<::std::ops::RangeToInclusive<usize>> for $name<T> {
            type Output = [T];

            fn index(&self, index: ::std::ops::RangeToInclusive<usize>) -> &[T] {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::IndexMut<::std::ops::RangeToInclusive<usize>> for $name<T> {
            fn index_mut(&mut self, index: ::std::ops::RangeToInclusive<usize>) -> &mut [T] {
                &mut self.0[index]
            }
        }
        impl<T> ::std::ops::Index<::std::ops::RangeFull> for $name<T> {
            type Output = [T];

            fn index(&self, index: ::std::ops::RangeFull) -> &[T] {
                &self.0[index]
            }
        }

        impl<T> ::std::ops::IndexMut<::std::ops::RangeFull> for $name<T> {
            fn index_mut(&mut self, index: ::std::ops::RangeFull) -> &mut [T] {
                &mut self.0[index]
            }
        }

        impl<T> From<[T; $size]> for $name<T> {
            fn from(from: [T; $size]) -> $name<T> {
                $name(from)
            }
        }

        impl<T> Into<[T; $size]> for $name<T> {
            fn into(self) -> [T; $size] {
                self.0
            }
        }
    }
}

#[macro_export]
macro_rules! newtype_array {
    (pub struct $name:ident(pub $size:expr)) => {
        #[derive(Copy, Clone)]
        pub struct $name<T>(pub [T; $size]);
        __array_impls!($name, $size);
    };
    (pub struct $name:ident($size:expr)) => {
        #[derive(Copy, Clone)]
        pub struct $name<T>([T; $size]);
        __array_impls!($name, $size);
    };
    (struct $name:ident($size:expr)) => {
        #[derive(Copy, Clone)]
        pub struct $name<T>([T; $size]);
        __array_impls!($name, $size);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn it_works() {

        // Sha385 length
        newtype_array!(pub struct Array48(pub 48));
        // Sha512 length
        newtype_array!(pub struct Array64(pub 64));

        // We've got `Clone` and `PartialEq`/`Eq`
        let arr1: Array48<u8> = [0; 48].into();
        let arr2 = arr1.clone();
        assert_eq!(arr1, arr2);

        // `Hash` is implemented as well
        let mut map = HashMap::new();
        map.insert(arr1, "hello");
    }
}
