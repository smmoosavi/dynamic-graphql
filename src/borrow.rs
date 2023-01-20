// use std::borrow::Borrow;
//
// pub enum Borrowed<'a, T> {
//     Owned(T),
//     Ref(&'a T),
// }
//
// impl<'a, T> Borrow<T> for Borrowed<'a, T> {
//     fn borrow(&self) -> &T {
//         match self {
//             Borrowed::Owned(ref t) => t,
//             Borrowed::Ref(t) => t,
//         }
//     }
// }
