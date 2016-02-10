//! This crate is a Rust tutorial for PLC 2016.

// mod helpers ;
// pub use helpers::* ;

mod units ;
pub use units::* ;
pub use units::helpers::* ;

/// This is the entry point.
fn main() {
  //! Comment for main inside main.

  println!("") ;
  println!("") ;
  println!("|===| Running tests.") ;

  let v_1 = 42.meters() ; // U::mk(42f64, Length::Meters) ;
  let v_2 = 7.seconds() ;
  v_1.print("| v_1:") ;
  v_2.print("| v_2:") ;

  let v_3 = U::mk(19f64, Length::Miles) ;
  v_3.print("| v_3:") ;

  let v_4 = v_3 / v_2 ;
  v_4.print("| v_4:") ;

  let v_5 = 7.mph() ;
  v_5.print("| v_5:") ;

  let v_6 = U::mk(9f64, Frac::mk(Length::Meters, Time::Seconds)) ;
  v_6.print("| v_6:") ;

  let v_7 = U::mk(42f64, Frac::mk(Length::Miles, Time::Seconds)) ;
  v_7.print("| v_7:") ;

  let v = v_6.to_mps() ;
  v.print("| v_6 to mps:") ;

  let v = v.to_miph() ;
  v.print("| v_6 to miph:") ;

  let v = v.to_mph() ;
  v.print("| v_6 to mph:") ;

  let v = v.to_mps() ;
  v.print("| v to mph:") ;

  println!("|===|") ;

  println!("") ;
  println!("")
}

