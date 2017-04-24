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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConvType<T> {
    Auto(T),
    Hira(T),
    Kata(T),
}

impl<T> ConvType<T> {
    pub fn map<U, F>(self, f: F) -> ConvType<U>
        where F: Fn(T) -> U // each type represents a single function
    {
        self.map3w(&f, &f, &f)
    }

    pub fn map3w<U, A, H, K>(self, a: A, h: H, k: K) -> ConvType<U>
        where A: Fn(T) -> U,
              H: Fn(T) -> U,
              K: Fn(T) -> U
    {
        match self {
            ConvType::Auto(v) => ConvType::Auto(a(v)),
            ConvType::Hira(v) => ConvType::Hira(h(v)),
            ConvType::Kata(v) => ConvType::Kata(k(v)),
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            ConvType::Auto(v) => v,
            ConvType::Hira(v) => v,
            ConvType::Kata(v) => v,
        }
    }
}
