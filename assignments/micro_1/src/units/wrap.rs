//! Wrapper around a value and a unit.

use std::ops::Div ;
use std::fmt ;

use units::* ;

/// Trait for values.
#[allow(dead_code)]
pub trait Val: Clone + Copy {
  /// Conversion from an f64.
  fn from_f64(f64) -> Self ;
}
impl Val for f64 {
  fn from_f64(x: f64) -> Self { x as f64 }
}
impl Val for f32 {
  fn from_f64(x: f64) -> Self { x as f32 }
}
impl Val for usize {
  fn from_f64(x: f64) -> Self { x as usize }
}
impl Val for isize {
  fn from_f64(x: f64) -> Self { x as isize }
}

/// Wraps a value and a unit.
#[derive(Clone, Copy)]
pub struct U<Uni: Unit> {
  /// Value.
  val: f64,
  /// Unit.
  uni: Uni,
}
impl<Uni: Unit> U<Uni> {
  /// Creates a wrapped value.
  #[inline]
  pub fn mk(val: f64, uni: Uni) -> Self {
    U { val: val, uni: uni }
  }
  /// The value stored.
  pub fn val(& self) -> f64 { self.val }
  /// The unit of the value.
  pub fn uni(& self) -> Uni { self.uni }
  /// Prints with prefix.
  pub fn print(& self, pref: & 'static str) {
    println!("{} {}", pref, self)
  }
}

impl U<Length> {
  /// Converts to meters.
  pub fn to_meters(mut self) -> Self {
    self.val = self.val * self.uni.to_meters() ;
    self.uni = Length::M ;
    self
  }
  /// Converts to feet.
  pub fn to_feet(mut self) -> Self {
    self.val = self.val * self.uni.to_feet() ;
    self.uni = Length::Ft ;
    self
  }
  /// Converts to miles.
  pub fn to_miles(mut self) -> Self {
    self.val = self.val * self.uni.to_miles() ;
    self.uni = Length::Mi ;
    self
  }
}

impl U<Time> {
  /// Converts to seconds.
  pub fn to_seconds(mut self) -> Self {
    self.val = self.val * self.uni.to_seconds() ;
    self.uni = Time::S ;
    self
  }
  /// Converts to minutes.
  pub fn to_minutes(mut self) -> Self {
    self.val = self.val * self.uni.to_minutes() ;
    self.uni = Time::M ;
    self
  }
  /// Converts to hours.
  pub fn to_hours(mut self) -> Self {
    self.val = self.val * self.uni.to_hours() ;
    self.uni = Time::H ;
    self
  }
}

impl U< Frac<Length, Time> > {
  /// Converts to miles per hour.
  pub fn to_miph(mut self) -> Self {
    self.val = self.val * self.uni.to_miph() ;
    self.uni = Frac::mk(Length::Mi, Time::H) ;
    self
  }
  /// Converts to meter per hour.
  pub fn to_mph(mut self) -> Self {
    self.val = self.val * self.uni.to_mph() ;
    self.uni = Frac::mk(Length::M, Time::H) ;
    self
  }
  /// Converts to meter per second.
  pub fn to_mps(mut self) -> Self {
    self.val = self.val * self.uni.to_mps() ;
    self.uni = Frac::mk(Length::M, Time::S) ;
    self
  }
}


impl<Uni1: Unit, Uni2: Unit> Div<
  U<Uni2>
> for U<Uni1> {
  type Output = U< Frac<Uni1, Uni2> > ;
  fn div(self, den: U<Uni2>) -> Self::Output {
    U {
      val: self.val / den.val,
      uni: Frac::mk(self.uni, den.uni),
    }
  }
}


impl<Uni: Unit> fmt::Display for U<Uni> {
  fn fmt(& self, fmt: & mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "{}{}", self.val(), self.uni().to_str())
  }
}