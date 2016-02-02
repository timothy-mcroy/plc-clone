/*! Evaluation module.

Contains the code we will use to test your code. It uses a Random Number
Generator (RNG) to generate values. (See [at the bottom of this
page](#credits) for more info.)

The code in this module will only work once you have completed task 3. It is
deactivated by default, by the `#[cfg(test)]` that tells the compiler to
**not** compile the code unless the `test` flag is passed to the compiler.

So, once you have completed task 3 you can remove / comment that line to test
your code.

The tasks for the assignment follow. I recommend you read [my advices at
the bottom of this page](#advices).

# Task 0 (2 points)

Document everything you do. Write a comment explaining what you're doing
whenever you implement a trait, a function, modify my code... Most of the time
one or two lines is probably enough, just say what you're doing.

# Task 1 (2 points)

Add a `Km` variant for kilometers to [`Length`][len], and the appropriate
constructor and conversion functions `kilometers` and `to_kilometers`
respectively.

# Task 2 (3 points)

Write a `Prod` struct that represents the product of two units in the spirit
of [`Frac`][frac]. Implement the [`std::ops::Mul`][mul] trait for the wrapper
[`U`][u] using `Prod`.
Also, implement the [`std::ops::Neg`][neg] trait for `U`.

# Task 3 (5 points)

Abstract the type of the values stored in the `U` wrapper using the
[`Val`][val] trait that's in the same file. This will break everything, fix it.

### Hints

**1.** The traits in the [`helpers` module][help] will need to change. I
suggest you remove the `ToF64` trait. With the type of the values abstracted it
is now possible to implement the functions directly in the trait.

At first, you can simply slice the code away by commenting the module
declaration in `units::mod.rs`. Failure to update the helper traits to
obtain the same feature will cost you 1 point.

**2.** Because of the abstraction, some things we were doing before on `f64`s
won't work anymore as is. Arithmetic operations for instance. You will need to
ask the value type to implement the traits you need.

Remember that the return type of operators is an implicit type parameter. For
instance, to require multiplication over some type `T`, by a value of type `T`
and returning a value of type T, the constraint is

```
use std::ops::Mul ;

impl<
  T: Mul<T, Output = T>, ...
> ... { ... }
```

# Extra credit (1 points)

Type `usize` implements trait `Val` ([see here][val for usize]), which means
it can be used in a `U` wrapper. But you implemented `Neg`ation over `U` in
task 3, which is illegal over an unsigned (hence positive) integer. Isn't this
unsafe?

>
> Answer the question **prefixing each line with a `>`** (*quoting* in
> markdown) by replacing this quoted text.
> Note that you can write code in quotes if you want to:
>
> ```
> fn factorial(mut n: usize) -> usize {
>   let mut res = 1 ;
>   while n > 0 {
>     res * n ;
>     n -= 1
>   } ;
>   res
> }
> ```
>


# Advices

The [documentation of the standard library][doc] is your friend. You can just
hit the 's' key on any documentation page to search for a trait, a type... It's
very useful to implement traits, you can basically copy paste the signature and
just fill the inside. Well, once you got the type parameters right that is.

The rust compiler is also your friend. You may find what it says confusing
at first, but if you take the time to understand, it's actually giving you
almost all the information you need to solve the problem. Sometimes it actually
gives you the code it thinks you meant (he can be wrong though).

# Credits

The *crate* (library) I use for RNG is [rand][rand]. The line `rand = "0.3.13"`
is all you need to add to the file `Cargo.toml` at the root of the cargo
project to use it. You can check out the `Cargo.toml` of this project for
instance. To use it, you add `extern crate rand` at the top-most file of your
project. For this one, it's `main.rs`.


[len]: ../enum.Length.html (Length enum)
[frac]: ../struct.Frac.html (Frac struct)
[u]: ../struct.U.html (U struct)
[val]: ../trait.Val.html (Val trait)
[mul]: https://doc.rust-lang.org/stable/std/ops/trait.Mul.html (Mul trait)
[neg]: https://doc.rust-lang.org/stable/std/ops/trait.Neg.html (Neg trait)
[help]: ../helpers/index.html (helpers module)
[val for usize]: ../../src/units/units/wrap.rs.html#20-22 (Val for usize)
[doc]: https://doc.rust-lang.org/std/ (Rust standard library documentation)
[rand]: https://crates.io/crates/rand/ (The rand library)
*/


/// Evaluation module.
///
/// Uncomment the `#[cfg(test)]` once you have completed task 3 and want to
/// test your code.
#[cfg(test)]
mod eval {

  use rand::Rng ;
  use rand::os::OsRng ;
  use std::process::exit ;

  use units::* ;

  type W<T> = U<f64, T> ;

  /// Generates a random length unit.
  fn rng_length<R: Rng>(rng: & mut R) -> W<Length> {
    match rng.next_u32() % 4 {
      0 => U::mk(rng.next_f64(), Length::M),
      1 => U::mk(rng.next_f64(), Length::Km),
      2 => U::mk(rng.next_f64(), Length::Ft),
      3 => U::mk(rng.next_f64(), Length::Mi),
      _ => unreachable!(),
    }
  }

  /// Generates a random time unit.
  fn rng_time<R: Rng>(rng: & mut R) -> W<Time> {
    match rng.next_u32() % 3 {
      0 => U::mk(rng.next_f64(), Time::S),
      1 => U::mk(rng.next_f64(), Time::M),
      2 => U::mk(rng.next_f64(), Time::H),
      _ => unreachable!(),
    }
  }

  /// Generates a non zero something.
  fn non_0<
    R: Rng, Uni: Unit, F: Fn(& mut R) -> W<Uni>
  >(
    rng: & mut R, f: F
  ) -> W<Uni> {
    loop {
      match f(rng) {
        res if res.val() != 0.0 => return res,
        _ => ()
      }
    }
  }

  /// Test function.
  pub fn test() {

    let total = 100 ;

    println!("") ;
    println!("|===| Testing functionalities on {} random testcases", total) ;
    println!("|") ;

    let rng = & mut match OsRng::new() {
      Ok(rng) => rng,
      Err(e) => {
        println!("| error creating RNG:") ;
        println!("| > {}", e) ;
        exit(2)
      },
    } ;

    let div_failed = test_div(total, rng) ;
    let mul_failed = test_mul(total, rng) ;
    let neg_failed = test_neg(total, rng) ;

    let score = (3 * total - div_failed - mul_failed - neg_failed) / 3 ;

    println!("| score: {} / {}", score, total) ;
    println!("|") ;

    println!("|===|") ;
    println!("")

  }

  macro_rules! test {
    (
      $cnt:ident ($u3:expr)
      for ($val:expr, $uni:expr) with $desc:expr
    ) => (
      {
        let (val_failed, uni_failed) = (
          $u3.val() != $val, $u3.uni() != $uni
        ) ;

        if val_failed || uni_failed {
          $cnt += 1 ;
          println!("  |=| error:") ;
          println!("    | > {} = {}", $desc, $u3) ;
          println!("    | u3:  {}", $u3.val()) ;
          println!("    | val: {}", $val) ;
          if val_failed { println!("    | > value is wrong") } ;
          if uni_failed { println!("    | >  unit is wrong") } ;
          println!("  |=|")
        }
      }
    ) ;
  }


  /// Tests division.
  /// Returns the number of failed test cases.
  pub fn test_div<R: Rng>(total: usize, rng: & mut R) -> usize {
    let mut failed = 0 ;

    println!("|=| Testing division") ;

    for _ in 0..(total / 2) {

      let len = rng_length(rng) ;
      let tim = rng_time(rng) ;

      test!(
        failed (len / tim) for (
          len.val() / tim.val(), Frac::mk(len.uni(), tim.uni())
        ) with format!("{} / {}", tim, len)
      ) ;

      test!(
        failed (tim / len) for (
          tim.val() / len.val(), Frac::mk(tim.uni(), len.uni())
        ) with format!("{} / {}", len, tim)
      ) ;
    } ;

    match failed {
      0 => println!("|=| success \\(^o^)/"),
      _ => println!("|=| failed {}/{} /(T_T)\\", failed, total),
    } ;

    println!("|") ;

    failed
  }


  /// Tests multiplication.
  /// Returns the number of failed test cases.
  pub fn test_mul<R: Rng>(total: usize, rng: & mut R) -> usize {
    let mut failed = 0 ;

    println!("|=| Testing multiplication") ;

    for _ in 0..(total / 2) {

      let len = rng_length(rng) ;
      let tim = rng_time(rng) ;

      test!(
        failed (len * tim) for (
          len.val() * tim.val(), Prod::mk(len.uni(), tim.uni())
        ) with format!("{} * {}", len, tim)
      ) ;

      test!(
        failed (tim * len) for (
          tim.val() * len.val(), Prod::mk(tim.uni(), len.uni())
        ) with format!("{} * {}", tim, len)
      ) ;
    } ;

    match failed {
      0 => println!("|=| success \\(^o^)/"),
      _ => println!("|=| failed {}/{} /(T_T)\\", failed, total),
    } ;

    println!("|") ;

    failed
  }


  /// Tests negation.
  /// Returns the number of failed test cases.
  pub fn test_neg<R: Rng>(total: usize, rng: & mut R) -> usize {
    let mut failed = 0 ;

    println!("|=| Testing negation") ;

    for _ in 0..(total / 2) {

      let len = rng_length(rng) ;
      test!(
        failed (- len) for (
          - len.val(), len.uni()
        ) with format!("-{}", len)
      ) ;


      let tim = rng_time(rng) ;
      test!(
        failed (- tim) for (
          - tim.val(), tim.uni()
        ) with format!("-{}", tim)
      ) ;
    } ;

    match failed {
      0 => println!("|=| success \\(^o^)/"),
      _ => println!("|=| failed {}/{} /(T_T)\\", failed, total),
    } ;

    println!("|") ;

    failed
  }

}