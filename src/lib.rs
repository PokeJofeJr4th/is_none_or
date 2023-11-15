//! # Checking conditions with the `IsNoneOr` trait
//! The provided `is_none_or` method is a mirror to the core library's `is_some_and` method.
//! It returns `true` if the option is a [`None`] or the option is [`Some`] and the value
//! inside of it matches a predicate.
//! # Examples
//!
//! ```
//! use is_none_or::IsNoneOr;
//! let x: Option<u32> = Some(2);
//! assert_eq!(x.is_none_or(|x| x > 1), true);
//!
//! let x: Option<u32> = Some(0);
//! assert_eq!(x.is_none_or(|x| x > 1), false);
//!
//! let x: Option<u32> = None;
//! assert_eq!(x.is_none_or(|x| x > 1), true);
//! ```

pub trait IsNoneOr<T> {
    fn is_none_or(self, f: impl Fn(T) -> bool) -> bool;
}

impl<T> IsNoneOr<T> for Option<T> {
    /// Returns `true` if the option is a [`None`] or the option is [`Some`] and the value
    /// inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use is_none_or::IsNoneOr;
    /// let x: Option<u32> = Some(2);
    /// assert_eq!(x.is_none_or(|x| x > 1), true);
    ///
    /// let x: Option<u32> = Some(0);
    /// assert_eq!(x.is_none_or(|x| x > 1), false);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.is_none_or(|x| x > 1), true);
    /// ```
    #[must_use]
    #[inline]
    fn is_none_or(self, f: impl Fn(T) -> bool) -> bool {
        match self {
            None => true,
            Some(t) => f(t),
        }
    }
}
