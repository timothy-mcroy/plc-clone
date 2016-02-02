/*! Library providing wrapping values with units.

The actual wrapper is the `U` struct. Trait [`ToUnits`][to units] provides
helper function to turn primitive types into values with a unit.

# Tasks

See the [`eval` module][eval].

[to units]: helpers/trait.ToUnits.html (ToUnits trait)
[eval]: eval/index.html (Eval module)
*/

#![allow(dead_code)]
#![forbid(missing_docs)]

extern crate rand ;

mod units ;

pub use units::* ;
use units::helpers::* ;

pub mod eval ;

/// Testing entry point.
pub fn main() {
  println!("") ;
  println!("") ;
  println!("|===| Running tests.") ;

  let v_1 = 42f64.meters() ;
  let v_2 = 7f64.seconds() ;
  v_1.print("| v_1:") ;
  v_2.print("| v_2:") ;

  let v_3 = 19.miles() ;
  v_3.print("| v_3:") ;

  let v_4 = v_3 / 5f64.hours() ;
  v_4.print("| v_4:") ;

  let v_5 = 7.mph() ;
  v_5.print("| v_5:") ;

  let v_6 = 9.mps() ;
  v_6.print("| v_6:") ;

  let v_7 = 42.mips() ;
  v_7.print("| v_7:") ;

  let v = v_6.to_mps() ;
  v.print("| v_6 to mps:") ;

  let v = v.to_miph() ;
  v.print("| v_6 to miph:") ;

  let v = v.to_mph() ;
  v.print("| v_6 to mph:") ;

  println!("|===|") ;

  // eval::test() ;

  println!("")


}

