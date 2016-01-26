//! This crate is a Rust tutorial for PLC 2016.

/// This is the entry point.
pub fn main() -> () {
  //! Comment for main inside main.
  println!("") ;
  println!("") ;

  easy::run() ;

  println!("") ;

  control_flow::run() ;

  println!("") ;
  ()
}

/// Module showcasing basing rust stuff.
pub mod easy {

  /// Runs basic rust stuff.
  pub fn run() {
    println!("|===| Life sucks") ;

    let x = 32 ;

    println!("| x: {}", x) ;

    {
      let y = ("blah", 45) ;

      println!("| y's **second** element: {}", y.1) ;

      // Destructuration.
      let (fst, snd) = y ;

      println!("| fst: {}, snd: {}", fst, snd) ;
    }

    // Can't access `fst` and `snd`, they went out of scope at the end of the
    // previous block.
    // println!("fst: {}", fst) ;
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^~~~~~ Does not compile.

    // An array. Note that this new binding shadows the previous binding for
    // `x`. Value and type are different. Previous binding for `x` is no more
    // accessible.
    let x: [ usize ; 7 ] = [ 2, 3, 5, 7, 11, 13, 17 ] ;
    //   ^^^^^^^^^^^^^^^~~~~ type annotation is not necessary

    // Access elements by index.
    println!("| x[1]: {}", x[1]) ;

    // Bounds in Rust are open to the right.
    println!("| & x[3..6]: {:?}", & x[3..6]) ;
    println!("| & x[3..]: {:?}", & x[3..]) ;

    println!("|===|") ;
    println!("") ;

  }

}


pub mod control_flow {

  pub fn run() {
    println!("|===| control flow") ;

    for n in 0..19 {
      println!("|=| ite({})", n) ;
      ite(n) ;
      println!("|")
    }

    let n = 7 ;
    println!("|=| named_loops({})", n) ;
    let res = named_loops(n) ;
    println!("|=| got {}", res) ;
    println!("|") ;

    let n = 11 ;
    println!("|=| return_loops({})", n) ;
    let res = return_loops(n) ;
    println!("|=| got {}", res) ;
    println!("|") ;

    let n = 13 ;
    println!("|=| while_example({})", n) ;
    let res = while_example(n) ;
    println!("|=| got {}", res) ;
    println!("|") ;

    println!("|=| for_example()") ;
    let res = for_example() ;
    println!("|=| got {}", res) ;
    println!("|") ;

    println!("|===|")
  }

  pub fn ite(n: usize) -> usize {
    if n > 15 {
      println!("| n is too big ({}), clamping", n) ;
      15
    } else if n < 3 {
      println!("| n is too tiny ({}), clamping", n) ;
      3
    } else {
      n
    }
  }

  pub fn named_loops(n: usize) -> usize {
    let mut count = n ;

    'outer: loop {
      println!("| entering 'outer") ;
      count = count * 2 ;

      'inner: loop {
        println!("| ('inner) count: {}", count) ;
        if count > 42 {
          println!("| breaking out of 'outer, thus exiting 'inner") ;
          break 'outer
        }
        count += 1 ;
      }

    }

    count
  }

  /// Careful, never returns.
  pub fn continue_loops(n: usize) -> usize {
    let mut count = n ;

    loop {
      println!("| count: {}", count) ;

      if count > 10 { continue }
      count += 1 ;
    }
  }

  pub fn return_loops(n: usize) -> usize {
    let mut count = n ;

    loop {
      println!("| count: {}", count) ;

      if count > 10 { return count }
      count += 1 ;
    }
  }

  pub fn while_example(n: usize) -> usize {
    let mut count = n ;

    while count < 10 { count += 1 }

    count
  }

  pub fn for_example() -> usize {
    let array = [1, 2, 3, 4, 5, 6, 7 ] ;

    let mut count = 0 ;

    for n in & array {
      count = count + n ;
    }

    count
  }

}