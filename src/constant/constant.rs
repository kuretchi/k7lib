/// A trait for marker types that represents a constant value.
///
/// See the [`constant!`] macro for more details.
///
/// [`constant!`]: ../macro.constant.html
pub trait Constant<T> {
  /// Returns the constant value.
  fn get() -> T;
}

/// Creates a marker type that represents a constant value.
///
/// # Syntax
///
/// ```ignore
/// constant! {
///   // Compile-time constant value
///   const NAME: TYPE = EXPR;
///
///   // Runtime constant value
///   static NAME: TYPE = EXPR;
/// }
/// ```
///
/// If it contains `static`'s one, it works only in functions.
///
/// # How it works
///
/// This macro defines an empty `enum` type that implements [`Constant<TYPE>`]
/// with an accosiated function `fn get() -> TYPE` which returns the given `EXPR`.
///
/// ## `const`
///
/// `EXPR` will simply be declared as a normal Rust's `const` value in a private scope.
///
/// ## `static`
///
/// It declares a thread local storage in a private scope, and immediately sets `EXPR` to that.
///
/// # Examples
///
/// ```
/// # use spella::constant;
/// # use spella::constant::Constant;
/// fn mod_mul<Mod: Constant<u32>>(x: u32, y: u32) -> u32 {
///   x * y % Mod::get()
/// }
///
/// fn main() {
///   let x = 10;
///
///   constant! {
///     const MOD1: u32 = 2;
///     static MOD2: u32 = x;
///   }
///
///   assert_eq!(mod_mul::<MOD1>(7, 3), 7 * 3 % 2);
///   assert_eq!(mod_mul::<MOD2>(7, 3), 7 * 3 % x);
/// }
/// ```
///
/// [`Constant<TYPE>`]: ./constant/trait.Constant.html
#[macro_export]
macro_rules! constant {
  () => {};
  ($(#[$attr:meta])* $vis:vis const $Name:ident: $T:ty = $val:expr; $($rest:tt)*) => {
    $(#[$attr])*
    $vis enum $Name {}

    impl $crate::constant::Constant<$T> for $Name {
      fn get() -> $T {
        const VAL: $T = $val;
        VAL
      }
    }

    constant! { $($rest)* }
  };
  ($(#[$attr:meta])* $vis:vis static $Name:ident: $T:ty = $val:expr; $($rest:tt)*) => {
    $(#[$attr])*
    $vis enum $Name {}

    {
      use ::std::cell::Cell;
      use ::std::option::Option::{self, None, Some};

      ::std::thread_local! {
        static VAL: Cell<Option<$T>> = Cell::new(None);
      }
      VAL.with(|val| val.set(Some($val)));

      impl $crate::constant::Constant<$T> for $Name {
        fn get() -> $T {
          VAL.with(|val| val.get().expect("constant not yet initialized"))
        }
      }
    }

    constant! { $($rest)* }
  };
}
