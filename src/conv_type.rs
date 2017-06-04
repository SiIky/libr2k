/*
<????????> siiky: FnOnce is least restrictive in what you can
pass to it, and most restrictive in what you can do with it.

<???????> siiky: "less restrictive" may still be true; if
I understood the 'supertraits' mention here
https://doc.rust-lang.org/core/ops/trait.Fn.html then it
looks like a function that accepts an FnOnce could also
accept an Fn or FnMut, but not the other way around

<????????> siiky: FnOnce is least restrictive in what can be
passed to a function that takes T:FnOnce, and most restrictive
in what that function can do with it.

<????????> (sarnold: It was mostly good, but there was a
potential confusion that I was talking about flexibility in
the arguments to the FnOnce)
 */

use self::ConvType::*;

/// Helper type to streamline the logic of converting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConvType<T> {
    /// Auto-detect by case
    Auto(T),
    /// Convert everything to hiragana
    Hira(T),
    /// Convert everything to katakana
    Kata(T),
}

impl<T> ConvType<T> {
    /// Call a function on the contained value.
    ///
    /// ```
    /// use r2k::conv_type::ConvType::*;
    ///
    /// let f = |_| 42;
    ///
    /// let x = Auto(21).map(&f);
    /// assert_eq!(x, Auto(42));
    ///
    /// let y = Hira(40).map(&f);
    /// assert_eq!(y, Hira(42));
    ///
    /// let z = Kata(44).map(&f);
    /// assert_eq!(z, Kata(42));
    /// ```
    pub fn map<U, F>(self, f: F) -> ConvType<U>
        where F: Fn(T) -> U // each type represents a single function
    {
        self.map3w(&f, &f, &f)
    }

    /// Call a function on the contained value, depending on the variant.
    ///
    /// ```
    /// use r2k::conv_type::ConvType::*;
    ///
    /// let a = |a| a * 2;
    /// let h = |h| h + 2;
    /// let k = |k| k - 2;
    ///
    /// let x = Auto(21).map3w(&a, &h, &k);
    /// assert_eq!(x, Auto(42));
    ///
    /// let y = Hira(40).map3w(&a, &h, &k);
    /// assert_eq!(y, Hira(42));
    ///
    /// let z = Kata(44).map3w(&a, &h, &k);
    /// assert_eq!(z, Kata(42));
    /// ```
    pub fn map3w<U, A, H, K>(self, a: A, h: H, k: K) -> ConvType<U>
        where A: Fn(T) -> U,
              H: Fn(T) -> U,
              K: Fn(T) -> U
    {
        match self {
            Auto(v) => Auto(a(v)),
            Hira(v) => Hira(h(v)),
            Kata(v) => Kata(k(v)),
        }
    }

    /// Return the contained value.
    ///
    /// ```
    /// use r2k::conv_type::ConvType::*;
    /// let x = Auto(42);
    /// assert_eq!(42, x.unwrap());
    /// ```
    pub fn unwrap(self) -> T {
        match self {
            Auto(v) => v,
            Hira(v) => v,
            Kata(v) => v,
        }
    }
}
