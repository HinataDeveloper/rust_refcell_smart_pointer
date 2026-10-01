// main.rs
// Date: Fri Oct 02 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning Chapter 15
// Goal: using RefCell and Rc smart pointer: Reference Cycle can leak Memory
// Dependency: Without dependency

// rustc 1.101.0-nightly (21b707e3f 2026-09-30)
// binary: rustc
// commit-hash: 21b707e3f97e0b522ebd2f277a862339625ad83f
// commit-date: 2026-09-30
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.1

// cargo 1.101.0-nightly (f3865b2a4 2026-09-29)
// release: 1.101.0-nightly
// commit-hash: f3865b2a4d1acc5276f6b3c67d0e057f4dab3928
// commit-date: 2026-09-29
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.8-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use std::{cell::RefCell, rc::Rc};

use ref_cell_smart_pointer::List::{self, Cons, Nil};

fn main() {
    println!("\n");

    let a: Rc<List> = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("reference count for a is: {}", Rc::strong_count(&a));
    println!("next item for a is: {:?}", a.tail());
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let b: Rc<List> = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("reference count for a is: {}", Rc::strong_count(&a));
    println!("reference count for b is: {}", Rc::strong_count(&b));
    println!("next item for b is: {:?}", b.tail());
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    if let Some(line) = a.tail() {
        *line.borrow_mut() = Rc::clone(&b);
    }

    println!(
        "reference count for b after changing a is: {}",
        Rc::strong_count(&b)
    );
    println!(
        "reference count for a after changing a is: {}",
        Rc::strong_count(&a)
    );

    println!("\nThe End ...\n");
}
