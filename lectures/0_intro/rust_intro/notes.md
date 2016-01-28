# Cargo

* create binary project: `cargo new --bin project_name`
  Creates a file `Cargo.toml` with the info about the project

* build a project: `cargo build`

* run a project: `cargo run`
  Binary will be in `target/debug/project_name`

* generate documentation: `cargo doc`
  **NB**: will not generate anything if the *crate* has no visible item


# Comments

Notion of *item*: functions, custom types, modules...

* comments ignored by the compiler

    * single-line: `//`

    * multi-line: `/* ... */`

* documentation comments for following item

    * single-line: `///`

    * multi-line: `/** ... */`

* documentation comments for enclosing item

    * single-line: `//!`

    * multi-line: `/*! ... */`


# Primitive types

* introduce printing stuff

* integers and floats: why so many variants?
  very useful for low-level, high-efficiency code
  We won't use these variants that much, and rely mostly on `usize` and `isize`

* tuples: aggregates values of different types
  can *project* the tuple (select elements):

  ```rust
  let tuple = (7, true) ;
  assert!( tuple.1 == true )
  ```

  can be viewed as an ordered list of elements of different type with a fixed length

* unit: type with only one value: `()`
  reminiscent of `void` in other programming languages
  can understand as *empty tuple*: nothing

* arrays: collection of elements of the same type stored in contiguous memory
  `=>` fast access to elements
  Size is known at compile time and is **part of the type of the array**
  For instance: `[u32 ; 7]`
  **Not growable**

* slices: *window* on an array (or more generally a sequence of elements)
  Same as an array but size is **not** known at compile time
  Size is thus not in the type: `& [u32]`
  Actually a two-word object:

    * pointer to the first element, and

    * length of the slice


# Variable bindings

* type annotations are pretty much **never** recessary: types are inferred

* can do more than just bind a variable: can use patterns (deconstruction)
  ```rust
  let (an_int, a_bool) = (7, true) ;
  ```
  Very useful for custom types (*cf.* later)

* mutability: defensive approach, default is **non-mutable**
  safety first: mutating something the dev declared as non-mutable should be illegal
  can't check that if everything is mutable by default

* initialization: can't use variables if they're not initialized
  ```rust
  let blah: u32 ;
  let something = blah + 2 ; // <- does not compile
  ```
  Can initialize only once (if not mutable)

* bindings have a *scope*, they live in a *block* `{ ... }` and die when exiting the block (unless returned by the block)
  ```rust
  let x = 7 ;
  {
    let y = 9 ;
    println!("x = {}, y = {}", 7, 9)
  }
  println!("x = {}, y = {}", 7, 9) // does not compile
  ```

* shadowing is when the same identifier is bound to a different value:
  ```rust
  let x = 7 ;
  let x = "a string" ;
  println!("{}", x)
  ```

# Functions

Functions returning nothing:

```rust
fn main() {
  print_sum(5, 6);
}

fn print_sum(x: i32, y: i32) {
  println!("sum is: {}", x + y);
}
```

Declaring the type of the arguments is **mandatory**, although full program inference is possible (but not very readable).

Functions return **exactly one** value:

```rust
fn add_one(x: i32) -> i32 {
  x + 1
}
```

No semicolon? Because Rust is primarily expression-based.

An expression corresponds (returns) to a **value**. A block `{ ... }` is an
expression.

Only two kinds of statements:

* declaration statements: `let blah = 7 ;`

* expression statements: transforms an expression in a statement: `7 + 1 ;`

Everything else is an expression, **including** assignments:
```rust
let mut x = 0 ;
x = 7 // <- has type `()`
```

**Early returns**: `return`
```rust
fn blah(x: i32) -> i32 {
  return x ;
  x + 1 // <- unreachable
}
```

Although it is bad style, one can write
```rust
fn blah(x: i32) -> i32 {
  return x + 1
}
```


# Flow Control

## Ite

Condition does not need parenthesis.

Followed by a block.

```rust
fn blah(n: usize) {
  if n < 0 {
      print!("{} is negative", n)
  } else if n > 0 {
      print!("{} is positive", n)
  } else {
      print!("{} is zero", n)
  }
}

let n = 5 ;

blah(n) ;
```

Ite-s **are expressions**:

```rust
let n = 7 ;
let n = {
  if n > 15 {
    println!("n is too big ({}), clamping", n) ;
    15
  } else if n < 3 {
    println!("n is too small ({}), clamping", n) ;
    3
  } else {
    n
  }
} ;
```

## Loops

Infinite loop: `loop`. Can break out with `break`.

```rust
let mut count = 0 ;

loop {
  count += 1 ;

  if count == 3 { println!("count is three") } ;

  if count == 5 {
    println!("count is five, that's enough") ;
    break
  }
}
```

```rust
'outer: loop {
  println!("outer loop") ;
  'inner: loop {
    println!("entering inner loop") ;

    // break

    break 'outer
  }
  println!("unreachable")
}
println!("exited outer loop")
```

## While

```rust
let mut n = 7 ;
while n > 0 {
  n -= 1 ;
  println!("n: {}", n) ;
}
println!("n: {}", n)
```

## For

```rust
for n in & [ 2, 3, 5, 7, 11, 13, 17 ] {
  println!("n: {}", n)
}
```

```rust
for n in 0..10 {
  println!("n: {}", n)
}
```

More generally, `for` loops work over **iterators**.

## Pattern matching

```rust
struct Cplx { a: f64, b: f64 }

fn blah(a: f64, b: f64) -> Result<Cplx, & 'static str> {
  match (a,b) {
    // Match against a concrete value.
    (0f64,0f64) => Err("a and b are zero, I don't like that"),
    // Partial match with wild card.
    (1f64,_) => Err("a is one"),
    // Variable binding with guard.
    (a,b) if a == b => Err("a is the same as b"),
    (a,b) => {
      let cplx = Cplx { a: a, b: b } ;
      // Let's match again:
      match cplx {
        // Destructuring with guard.
        Cplx { a, b } if a == - b => Err("unreachable"),
        // Destructuring, omitting some fields.
        Cplx { a, ..} if a == 1f64 => Err("unreachable"),
        // Destructuring with renaming.
        Cplx { a: the_a, b: the_b } if the_a == 0f64 && the_b == 0f64 => Err(
          "unreachable"
        ),
        // Match the whole thing.
        cplx => Ok(cplx),
      }
    }
  }
}

fn main() {
  for a in 0..10 {
    for b in 0..10 {
      let (a,b) = (a as f64, b as f64) ;
      print!("blah({}, {}): ", a, b) ;
      match blah(a, b) {
        Ok(cplx) => println!(
          "success: {} + {}.i", cplx.a, cplx.b
        ),
        Err(msg) => println!("error: {}", msg),
      }
    }
  }
}
```



# Custom types

Only two of them (with variations): `struct`s and `enum`s.

## Structs

### Classic structs

Gather data as *fields* of a structure

```rust
struct Cplx {
  a: f64, b: f64
}
```

(Fields are private by default, but can be accessed by any code in the same
module.)

Can destructure in variable binding:

```rust
let cplx = Cplx { a: 5f64, b: 7f64 } ;
let Cplx { a: the_a, b: the_b } = cplx ;
```

### Tuple structs

Basically named tuples (except we can have implementations over them).

```rust
struct Cplx (f64, f64) ;
```

Can also destructure in variable binding:

```rust
let cplx = Cplx(5f64, 7f64) ;
let Cplx(the_a, the_b) = cplx ;
```

### Unit structs

Useful for generics.

```rust
struct Meter ;
struct Feet ;
```

## Enums

An `enum` is a type that has *variants*: it can be different things. Any
variant which is valid as a `struct` is valid as an `enum`.

```rust
enum Movement {
  // Unit variant.
  Static,
  // Linear with time.
  Linear(f64),
  // Changes over time.
  Dynamic { initial: f64, coeff: f64 },
}

let lin = Movement::Linear(42f64) ;
let dyn = Movement::Dynamic { initial: 0f64, coeff: 1f64 } ;
```

Variants are not imported in the scope by default. You can do so with `use`:

```rust
use Movement::* ;
let lin = Linear(42f64) ;
let dyn = Dynamic { initial: 0f64, coeff: 1f64 } ;
```



# Methods

Methods are functions attached to *objects*. Methods are defined under an
`impl` block. As usual, methods can be static, if they're not then their first
parameter is a (reference) on `self`.

Note also the special type `Self` refering to the underlying (concrete) type.

```rust
impl Movement {
  // Three static methods.
  pub static() -> Self { Movement::Static }
  pub linear(spd: f64) -> Self { Movement::Linear(spd) }
  pub dynamic(i: f64, c: f64) -> Self {
    Movement::Dynamic { initial: i, coeff: c }
  }

  // Non-static methods.
  print(& self) { // <- use `self` first
    match * self {
      Static => println!("| static"),
      Linear(spd) => println!("| linear({})", spd),
      Dynamic { initial: init, coeff } => println!(
        "| dynamic({},{})", init, coeff
      ),
    }
  }
}
```

```rust
let sta = Movement::static() ;
sta.print() ;
```


# Traits and generics

