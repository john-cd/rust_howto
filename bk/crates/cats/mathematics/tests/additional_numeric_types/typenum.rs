// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! # Type-Level Numbers with `typenum`
// //!
// //! This example demonstrates the use of the `typenum` crate to perform
// //! type-level arithmetic and define structures with compile-time
// //! dimensions.
// #![feature(generic_const_exprs)]

// use std::marker::PhantomData;
// // use std::ops::Index;
// use std::ops::IndexMut;

// use typenum::NonZero;
// use typenum::Prod;
// use typenum::Sum;
// // Type-level number representation.
// use typenum::U2;
// use typenum::U3;
// // Compile time unsigned integers.
// use typenum::Unsigned;

// // `typenum` provides type-level numbers evaluated at compile time.
// // It depends only on `libcore`.

// // Define a generic struct that uses type-level numbers
// // without runtime overhead.

// /// ## Assertions
// ///
// /// The `Assert` struct and `IsTrue` trait are used to perform compile-time
// /// assertions.
// struct Assert<const COND: bool> {}
// trait IsTrue {}

// impl IsTrue for Assert<true> {}

// /// ## Matrix (Take 1)
// // struct Matrix<T, R, C, const N: usize>
// // where R: Unsigned + NonZero, C: Unsigned + NonZero,
// //     Assert<{ Self::N == R::USIZE * C::USIZE }>: IsTrue,
// // {
// //     data: [T; N],
// //     // Use PhantomData to track the type-level dimensions
// //     rows: PhantomData<R>,
// //     cols: PhantomData<C>,
// // }

// // impl<T, R: Unsigned + NonZero, C: Unsigned + NonZero, const N: usize>
// // Matrix<T, R, C, N>  {
// //     fn new(data: [T; N]) -> Self {
// //         assert_eq!(data.len(), R::to_usize() * C::to_usize()); //
// // guaranteed! //         Matrix {
// //             data,
// //             rows: PhantomData,
// //             cols: PhantomData,
// //         }
// //     }

// //     fn rows(&self) -> usize {
// //         R::USIZE
// //     }

// //     fn cols(&self) -> usize {
// //         C::USIZE
// //     }
// // }

// // impl<T, R: Unsigned + NonZero, C: Unsigned + NonZero, const N: usize>
// //     Index<(usize, usize)> for Matrix<T, R, C, N>
// // {
// //     type Output = T;

// //     fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
// //         let index = row * self.cols() + col;
// //         &self.data[index]
// //     }
// // }

// // impl<T, R: Unsigned + NonZero, C: Unsigned + NonZero, const N: usize>
// //     IndexMut<(usize, usize)> for Matrix<T, R, C, N>
// // {
// //     fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut
// // Self::Output {
// //         let index = row * self.cols() + col;
// //         &mut self.data[index]
// //     }
// // }

// /// ## Matrix (Take 2)
// ///
// /// This is a refined version of the `Matrix` struct that uses type-level
// /// numbers to define its dimensions.
// struct Matrix<T, R, C> {
//     /// Use PhantomData to track the type-level dimensions
//     rows: PhantomData<R>,
//     /// Use PhantomData to track the type-level dimensions
//     cols: PhantomData<C>,
//     /// A raw pointer to the underlying data. This requires `unsafe`
//     /// operations when accessing the data.
//     data: *const T,
// }

// /// ## RightSized Trait
// ///
// /// This trait ensures that the matrix data has the correct size.
// trait RightSized<T, const N: usize> {
//     // const N: usize;
//     // type D; // = [T; Self::N]; // unstable default associated type
//     fn new(data: [T; N]) -> Self;
// }

// impl<T, R, C, const N: usize> RightSized<T, N> for Matrix<T, R, C>
// where
//     R: Unsigned + NonZero,
//     C: Unsigned + NonZero,
//     Assert<{ N == R::USIZE * C::USIZE }>: IsTrue,
// {
//     // const N: usize = R::USIZE * C::USIZE;
//     // type D = [T; {R::USIZE * C::USIZE}]; // unstable generic params in
//     // const expressions
//     fn new(data: [T; N]) -> Self // [T; Self::N] does not work
//     {
//         Matrix {
//             data: data.as_ptr(),
//             rows: PhantomData,
//             cols: PhantomData,
//         }
//     }
// }

// // ## Matrix Implementation
// //
// // This implementation provides methods to get the number of rows and
// // columns of the matrix.
// impl<T, R, C> Matrix<T, R, C>
// where
//     R: Unsigned + NonZero,
//     C: Unsigned + NonZero,
// {
//     const LEN: usize = R::USIZE * C::USIZE;

//     fn rows(&self) -> usize {
//         R::USIZE
//     }

//     fn cols(&self) -> usize {
//         C::USIZE
//     }
// }

// // impl<T, R: Unsigned + NonZero, C: Unsigned + NonZero>
// //     Index<(usize, usize)> for Matrix<T, R, C>
// // {
// //     type Output = T;

// //     fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
// //         let index = row * self.cols() + col;
// //         &self.data[index]
// //     }
// // }

// // impl<T, R: Unsigned + NonZero, C: Unsigned + NonZero>
// //     IndexMut<(usize, usize)> for Matrix<T, R, C>
// // {
// //     fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut
// // Self::Output { //         let index = row * self.cols() + col;
// //         &mut self.data[index]
// //     }
// // }

// /// This function demonstrates the use of type-level numbers and the `Matrix`
// /// struct.
// fn main() {
//     // Define type-level integers
//     type Two = U2;
//     type Three = U3;

//     // Perform type-level arithmetic
//     type Five = Sum<Two, Three>;
//     type Six = Prod<Two, Three>;

//     // Get the runtime value of type-level integers
//     println!("Two: {}", Two::to_i32());
//     println!("Two * Three = Six: {}", Six::to_i32());

//     // Type-level assertions
//     assert_eq!(Five::to_i32(), 5);

//     // Create a 2x3 matrix
//     let mut matrix_2x3: Matrix<f64, U2, U3> =
//         Matrix::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

//     // println!("Matrix 2x3 element at (1,2): {}", matrix_2x3[(1, 2)]);

//     // matrix_2x3[(1, 1)] = 10.0;

//     // Compile-time dimension checking - can't have negative number of rows
//     // Uncomment to see compile-time error
//     // let invalid_matrix = Matrix::<typenum::N2, U2>::new([1.0, 2.0, 3.0,
//     // 4.0,     // 5.0, 6.0]);
// }

// #[test]
// fn test() {
//     main();
// }
// // TODO FINISH NOW
// // https://users.rust-lang.org/t/making-a-type-level-type-function-with-typenum-crate/107008
// // https://github.com/paholg/typenum/tree/main
// // https://docs.rs/typenum/latest/typenum/
// // https://crates.io/crates/generic-array/

// // Need #![feature(generic_const_exprs)] and the nightly toolchain?
// // Use associated types

// // ALSO
// // const generics:
// // https://practice.course.rs/generics-traits/const-generics.html
// // https://www.awwsmm.com/blog/what-are-const-generics-and-how-are-they-used-in-rust
// // struct Assert<const COND: bool> {}
// // trait IsTrue {}

// // impl IsTrue for Assert<true> {}

// // trait IsOdd<const N: i32> {}

// // impl<const N: i32> IsOdd<N> for i32 where Assert<{N % 2 == 1}>: IsTrue {}

// // fn do_something_odd<const N: i32>() where i32: IsOdd<N> {}

// // fn do_something() {
// //     do_something_odd::<19>();
// //     do_something_odd::<42>(); // does not compile
// // }
