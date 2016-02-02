//! Helper traits.

use units::U ;
use units::{ Length, Time, Frac } ;

/// Convenience trait for units over primitive types.
pub trait ToUnits {
  /// Turns into meters.
  fn meters(& self)  -> U<Length> ;
  /// Turns into feet.
  fn feet(& self)    -> U<Length> ;
  /// Turns into miles.
  fn miles(& self)   -> U<Length> ;
  /// Turns into seconds.
  fn seconds(& self) -> U<Time>   ;
  /// Turns into minutes.
  fn minutes(& self) -> U<Time>   ;
  /// Turns into hours.
  fn hours(& self)   -> U<Time>   ;
  /// Turns into miles per hour.
  fn miph(& self)    -> U<Frac<Length, Time>> ;
  /// Turns into miles per second.
  fn mips(& self)    -> U<Frac<Length, Time>> ;
  /// Turns into meter per hour.
  fn mph(& self)     -> U<Frac<Length, Time>> ;
  /// Turns into meter per second.
  fn mps(& self)     -> U<Frac<Length, Time>> ;
}



/// Convenience trait for units over f64.
impl ToUnits for f64 {
  fn meters(& self)  -> U<Length> { Length::meters(* self) }
  fn feet(& self)    -> U<Length> { Length::feet(* self)   }
  fn miles(& self)   -> U<Length> { Length::miles(* self)  }
  fn seconds(& self) -> U<Time>   { Time::seconds(* self)  }
  fn minutes(& self) -> U<Time>   { Time::minutes(* self)  }
  fn hours(& self)   -> U<Time>   { Time::hours(* self)    }
  fn miph(& self)    -> U<Frac<Length, Time>> {
    U::mk(* self, Frac::mk( Length::Mi, Time::H ))
  }
  fn mips(& self)    -> U<Frac<Length, Time>> {
    U::mk(* self, Frac::mk( Length::Mi, Time::S ))
  }
  fn mph(& self)     -> U<Frac<Length, Time>> {
    U::mk(* self, Frac::mk( Length::M, Time::H ))
  }
  fn mps(& self)     -> U<Frac<Length, Time>> {
    U::mk(* self, Frac::mk( Length::M, Time::S ))
  }
}


/// Can convert to an `f64`.
pub trait ToF64 {
  /// Converts `self` to an `f64`.
  fn to_f64(& self) -> f64 ;
}

impl<T: ToF64> ToUnits for T {
  fn meters(& self)  -> U<Length> { self.to_f64().meters()  }
  fn feet(& self)    -> U<Length> { self.to_f64().feet()    }
  fn miles(& self)   -> U<Length> { self.to_f64().miles()   }
  fn seconds(& self) -> U<Time>   { self.to_f64().seconds() }
  fn minutes(& self) -> U<Time>   { self.to_f64().minutes() }
  fn hours(& self)   -> U<Time>   { self.to_f64().hours()   }
  fn miph(& self)    -> U<Frac<Length, Time>> { self.to_f64().miph() }
  fn mips(& self)    -> U<Frac<Length, Time>> { self.to_f64().mips() }
  fn mph(& self)     -> U<Frac<Length, Time>> { self.to_f64().mph() }
  fn mps(& self)     -> U<Frac<Length, Time>> { self.to_f64().mps() }
}

impl ToF64 for usize {
  fn to_f64(& self) -> f64 { * self as f64 }
}