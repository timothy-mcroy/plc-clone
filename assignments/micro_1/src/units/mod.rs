/// The unit module.

use std::cmp::PartialEq ;

mod wrap ;
pub use self::wrap::* ;
pub mod helpers ;

/// Trait for units.
pub trait Unit: Clone + Copy + PartialEq {
  /// String representation of a unit.
  fn to_str(& self) -> String ;
}

/// Length units.
#[derive(Clone, Copy, PartialEq)]
pub enum Length {
  /// Meters.
  M,
  /// Feet.
  Ft,
  /// Miles.
  Mi,
}
impl Length {
  /// Wraps a value in meters.
  pub fn meters(v: f64) -> U<Length> {
    U::mk(v, Length::M)
  }
  /// Wraps a value in feet.
  pub fn feet(v: f64) -> U<Length> {
    U::mk(v, Length::Ft)
  }
  /// Wraps a value in feet.
  pub fn miles(v: f64) -> U<Length> {
    U::mk(v, Length::Mi)
  }

  /// Factor to convert to meters.
  pub fn to_meters(& self) -> f64 {
    match * self {
      Length::M => 1.0,
      Length::Ft => 0.3048,
      Length::Mi => 1_609.344,
    }
  }
  /// Factor to convert to feet.
  pub fn to_feet(& self) -> f64 {
    match * self {
      Length::M => 3.281,
      Length::Ft => 1.0,
      Length::Mi => 5_280.0,
    }
  }
  /// Factor to convert to miles.
  pub fn to_miles(& self) -> f64 {
    match * self {
      Length::M => 0.00062137,
      Length::Ft => 0.000189,
      Length::Mi => 1.0,
    }
  }
}
impl Unit for Length {
  fn to_str(& self) -> String {
    match * self {
      Length::M => "m".to_string(),
      Length::Ft => "ft".to_string(),
      Length::Mi => "mi".to_string(),
    }
  }
}

/// Time units.
#[derive(Clone, Copy, PartialEq)]
pub enum Time {
  /// Seconds.
  S,
  /// Minutes.
  M,
  /// Hours.
  H,
}
impl Time {
  /// Wraps a value in seconds.
  pub fn seconds(v: f64) -> U<Time> {
    U::mk(v, Time::S)
  }
  /// Wraps a value in minutes.
  pub fn minutes(v: f64) -> U<Time> {
    U::mk(v, Time::M)
  }
  /// Wraps a value in hours.
  pub fn hours(v: f64) -> U<Time> {
    U::mk(v, Time::H)
  }

  /// Factor to convert to seconds.
  pub fn to_seconds(& self) -> f64 {
    match * self {
      Time::S => 1.0,
      Time::M => 60.0,
      Time::H => 3_600.0,
    }
  }
  /// Factor to convert to minutes.
  pub fn to_minutes(& self) -> f64 {
    match * self {
      Time::S => 1.0 / 60.0,
      Time::M => 1.0,
      Time::H => 60.0,
    }
  }
  /// Factor to convert to hours.
  pub fn to_hours(& self) -> f64 {
    match * self {
      Time::S => 1.0 / 3_600.0,
      Time::M => 1.0 / 60.0,
      Time::H => 1.0,
    }
  }
}
impl Unit for Time {
  fn to_str(& self) -> String {
    match * self {
      Time::S => "s".to_string(),
      Time::M => "mn".to_string(),
      Time::H => "h".to_string(),
    }
  }
}

/// Fraction of units.
#[derive(Clone, Copy, PartialEq)]
pub struct Frac<Num, Den> {
  /// Numerator.
  num: Num,
  /// Denominator.
  den: Den,
}
impl<Num, Den> Frac<Num, Den> {
  /// Creates a fraction.
  pub fn mk(num: Num, den: Den) -> Self {
    Frac { num: num, den: den }
  }
  /// Numerator.
  #[inline]
  pub fn num(& self) -> & Num { & self.num }
  /// Denominator.
  #[inline]
  pub fn den(& self) -> & Den { & self.den }
}
impl Frac<Length, Time> {
  /// Factor to convert to miles per hour.
  pub fn to_miph(& self) -> f64 {
    self.num.to_miles() / self.den.to_hours()
  }
  /// Factor to convert to meter per hour.
  pub fn to_mph(& self) -> f64 {
    self.num.to_meters() / self.den.to_hours()
  }
  /// Factor to convert to meter per second.
  pub fn to_mps(& self) -> f64 {
    self.num.to_meters() / self.den.to_seconds()
  }
}
impl<Num: Unit, Den: Unit> Unit for Frac<Num, Den> {
  fn to_str(& self) -> String {
    format!("({}/{})", self.num.to_str(), self.den.to_str())
  }
}



