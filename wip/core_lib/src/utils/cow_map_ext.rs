// [do we need?](https://github.com/john-cd/rust_howto/issues/1429)

// use std::borrow::Borrow;
// use std::borrow::Cow;

// // <https://stackoverflow.com/questions/72179485/how-to-compose-two-calls-to-regexreplace-all>

// pub trait CowMapExt<'a, B>
// where
//     B: 'a + ToOwned + ?Sized,
// {
//     fn map<F>(self, f: F) -> Self
//     where
//         F: for<'b> FnOnce(&'b B) -> Cow<'b, B>;
// }

// impl<'a, B> CowMapExt<'a, B> for Cow<'a, B>
// where
//     B: 'a + ToOwned + ?Sized,
// {
//     fn map<F>(self, f: F) -> Self
//     where
//         F: for<'b> FnOnce(&'b B) -> Cow<'b, B>,
//     {
//         match self {
//             Cow::Borrowed(v) => f(v),
//             Cow::Owned(v) => Cow::Owned(f(v.borrow()).into_owned()),
//         }
//     }
// }
