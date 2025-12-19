The untyped lambda calculus is a Turing-complete language expressed in terms of function abstraction and function application. This implementation uses De Brujin indices instead of the traditional λx.y format.

In addition to the pure calculus, I've added the ability to define macros/replacements with the `:=` operator. For example, `True := λλ2` would define the True constantly as it is usually written, and `0 := λλ1 would define the `False` constant. You can find examples of programs in the [lc](https://github.com/siIverfish/lcalc2/tree/master/lc) folder.
