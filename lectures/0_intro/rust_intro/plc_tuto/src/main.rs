//! This crate is a Rust tutorial for PLC 2016.

/// This is the entry point.
pub fn main() -> () {
  //! Comment for main inside main.
  use units::* ;

  println!("") ;
  println!("") ;

  let val = U::mk( 5f64, Length::Feet ) ;

  println!("val = {}", val) ;

  println!("") ;
  println!("") ;
  ()
}


pub mod units {
  use std::fmt ;

  /// Wrapper around a value storing the unit of the value.
  pub struct U {
    val: f64,
    uni: Length,
  }
  impl U {
    /// Creates a new wrapper around a value.
    pub fn mk(val: f64, uni: Length) -> Self {
      U { val: val, uni: uni }
    }

    /// A string representation of the value and its unit.
    pub fn to_str(& self) -> String {
      format!("{}{}", self.val, self.uni.to_str())
    }

    /// Converts the value to meters.
    pub fn to_meters(self) -> Self {
      let factor = self.uni.to_meters() ;
      U::mk( self.val * factor, Length::Meters )
    }
    /// Converts the value to feet.
    pub fn to_feet(self) -> Self {
      let factor = self.uni.to_feet() ;
      U::mk( self.val * factor, Length::Feet )
    }
    /// Converts the value to miles.
    pub fn to_miles(self) -> Self {
      let factor = self.uni.to_miles() ;
      U::mk( self.val * factor, Length::Miles )
    }
  }

  impl fmt::Display for U {
    fn fmt(& self, fmt: & mut fmt::Formatter) -> fmt::Result {
      write!(fmt, "{}{}", self.val, self.uni.to_str())
    }
  }

  /// A length.
  pub enum Length {
    Meters,
    Feet,
    Miles,
  }
  impl Length {
    /// A string representation of the value and its unit.
    pub fn to_str(& self) -> String {
      match * self {
        Length::Meters => "m".to_string(),
        Length::Feet => "ft".to_string(),
        Length::Miles => "mi".to_string(),
      }
    }

    /// Factor to convert to meters.
    pub fn to_meters(& self) -> f64 {
      match * self {
        Length::Meters => 1.0,
        Length::Feet   => 0.3048,
        Length::Miles  => 1_609.344,
      }
    }
    /// Factor to convert to feet.
    pub fn to_feet(& self) -> f64 {
      match * self {
        Length::Meters => 3.281,
        Length::Feet   => 1.0,
        Length::Miles  => 5_280.0,
      }
    }
    /// Factor to convert to miles.
    pub fn to_miles(& self) -> f64 {
      match * self {
        Length::Meters => 0.00062137,
        Length::Feet   => 0.000189,
        Length::Miles  => 1.0,
      }
    }
  }

}
