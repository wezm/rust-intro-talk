class: center, middle, ferris


Rust Introduction
=================

Wesley Moore
------------

![Ferris the Rustacean animation](images/ferris.gif)

---

# Agenda

1. Features
2. Tooling
3. Syntax
4. Demo
5. Questions

---

# Introduction

```rust
fn main() {
  println!("Rust Introduction");
}
```

> **Rust** is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. 

— [rust-lang.org](https://www.rust-lang.org/)

---

# Runs Blazingly Fast

* Complies to native code
* Same ballpark as C and C++
* Memory saftey without garbage collection
* Strong, statically typed lanuage with an emphasis on saftey and correctness
* No runtime

---

class: segfaults

# Prevents Segfaults

* No `NULL`, `nil` or other [billion dollar mistakes][billion-dollar-mistake].

![undefined method for nil:NilClass](images/undefined-method-nil.png)

This is not a thing. At all. Ever.

.center[[billion-dollar-mistake]: https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare]

---

# Guarantees Thread Safety

* Ownership model tracks owner and lifetime of memory.
* No data races: Compiler knows which thread owns what data.
* This leads to, "Fearless Concurrency".

---

# Strong Static Type System

* Do more at compile time, so fewer checks required at run time

--

* Concepts mostly the same as other languages. No need to learn an entirely new paradigm.

--

* Traits and generics instead of objects and inheritance.

--

* Type inference reduces the need to specifiy types.
  * Function definitions must have types makes reading and understanding code easier.

--

* Refactor with impunity.

---

# Tooling

Official distribution includes:

* `cargo`
  * build tool (no Makefiles)
  * package manager (like bundler)
  * test runner
  * documentation generator

---

## Releases

Generally managed with `rustup`:

* Official toolchain manager (like rbenv)
* New releases every 6 weeks
  * Commited to backwards compatibility for every release.

---

class: crates

# Crates

* Rust favours a small, stable standard library.
* Crates are the equivalient of Ruby gems. They are published to
  [crates.io](https://crates.io/).

![screenshot of crates.io](images/crates-io.png)

---

class: center, middle

Syntax
======

---

#  Variables (Bindings)

`let` bindings are mutable by default.

```rust,skt-basic-main
let number = 42; // immutable
let mut total = 0; // mutable
```

---

#  Functions

```rust
fn average(values: &[i32]) -> i32 {
    values.iter().sum::<i32>() / values.len() as i32
}

fn main() {
    average(&[1,2,3,4,5]);
}
```

---

#  Conditionals: If

```rust,skt-basic-main
let mut airconditioner;
let temperature: i32 = 42;

if temperature > 35 {
    airconditioner = true;
}
else {
    airconditioner = false;
}
```

---

#  Conditionals: Match

Can match structure and values:

```rust,skt-basic-main
let month = "jan";

match month {
    "jan" => 1,
    "feb" => 2,
    "mar" => 3,
    "apr" => 4,
    "may" => 5,
    "jun" => 6,
    "jul" => 7,
    "aug" => 8,
    "sep" => 9,
    "oct" => 10,
    "nov" => 11,
    "dec" => 12,
    _     => panic!("invalid month"),
};

```


---

#  Loops

```rust,skt-basic-main
let numbers = [1,2,3];

for i in numbers.iter() {
    // do something
}

for i in 0..10 {
    // do something
}

loop {
    // do something

    if numbers.len() > 2 {
      break;
    }
}

while numbers.len() < 2 {
    // do something
}
```

---

# Functional or imperative

You pay _no cost_ for using the higher level style.

```rust,skt-var-mean
fn variance_mean(data: &[f64], mean: f64) -> f64 {
    let mut sum = 0.;

    for d in data {
        sum += (d - mean).powf(2.0);
    }

    sum / data.len() as f64
}
```

compiles the identical machine code (I checked) as:

```rust,skt-var-mean
fn variance_mean(data: &[f64], mean: f64) -> f64 {
    data.into_iter()
        .map(|d| (d - mean).powf(2.0))
        .sum::<f64>() / data.len() as f64
}
```

---

#  enums

Type that represents one possibility of several variants. Variants may optionally
carry data.

```rust,skt-type-demo
enum SerialProtocol {
    Usb,
    Rs485,
    Rs232,
    I2C,
    Spi,
}
```

---

#  structs

Type that carries structured data.

```rust,skt-serial
struct Person {
    name: String,
    age: i32,
    favourite_serial_protocol: SerialProtocol,
}
```

---

# Option

Insead of `nil`/`NULL` we have `Option`.

* Used to represent something that may be absent.
* An enum that that looks like this:

```rust,skt-type-demo
enum Option<T> {
    Some(T),
    None
}
```

---

# Result

When something can succeed or fail with an error. There are no exceptions in
Rust, `Result` is how you handle errors.

* An enum that that looks like this:

```rust,skt-type-demo
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

---

class: center, middle

Demo
====

---

class: center, middle

Questions?
==========

---

Credits
=======

* Portions of this talk were derived from [A Very Brief Intro to Rust][rust-intro]  
  Copyright (c) 2016 Ashley Williams
* [Animated Ferris][animated-ferris] by A. L. Palmer, via [rustacean.net][rustacean].

[rust-intro]: https://github.com/rustbridge/a-very-brief-intro-to-rust
[animated-ferris]: https://www.behance.net/gallery/42774743/Rustacean
[rustacean]: http://rustacean.net/
