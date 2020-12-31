#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_tuple_impl {
  ($m:ident [$($acc:tt)*] []) => {};
  ($m:ident [$($acc:tt)*] [$i:tt: $T:ident, $($rest:tt)*]) => {
    $m! { $($acc)* $i: $T, }
    $crate::__for_each_tuple_impl! { $m [$($acc)* $i: $T,] [$($rest)*] }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! for_each_tuple {
  ($m:ident) => {
    $crate::__for_each_tuple_impl! {
      $m []
      [0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8, 9: T9, 10: T10, 11: T11,]
    }
  };
}
