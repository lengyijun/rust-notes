---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
---

# **Use Abella to prove equivalence of datalog rules**

In Polonius (the new Rust borrow checker)

---
# Borrow checker in Rust
```
#include<iostream>     
#include<vector>
using namespace std;

int main(){                          fn main() {            
  vector<int> v={12};                    let mut v=vec![12];
  auto x=&v[0];                          let x=&v[0];
  cout<<*x<<endl;                        dbg!(x);
                                     
  v.reserve(100);                        v.reserve(100);
  cout<<*x<<endl;                        dbg!(x);
}                                    }                       
```











---

# Datalog engines
swi(scala)

racket: 
https://docs.racket-lang.org/datalog/Tutorial.html
no negative？

souffle (c++): parallel, Rust is using

bddbddb (java): use binary decision diagram

https://github.com/vmware/differential-datalog
gnu-prolog gprolog (C)


---
# Proof assistant

- Isabella
- Lean3 Lean4
- lambda prolog
    Teyjus
    ELPI（OCaml）
    Abella（OCaml）

A lot of under developing

--- 
# What is Polonius
```rust
cargo  rustc  -- -Zpolonius
// or
rustc -Zpolonius main.rs 
```


--- 

# Where Polonius can help

```rust
fn get_or_insert(
    map: &mut HashMap<u32, String>,
) -> &String {
    match map.get(&22) {
        Some(v) => v,
        None => {
            map.insert(22, String::from("hi"));
            &map[&22]
        }
    }
}
```

--- 
```
error[E0502]: cannot borrow `*map` as mutable because also borrowed as immutable
 --> src/main.rs:11:13
  |
3 |   map: &mut HashMap<u32, String>,
  |        - let's call the lifetime of this reference `'1`
4 | ) -> &String {
5 |   match map.get(&22) {
  |         --- immutable borrow occurs here
6 |     Some(v) => v,
  |                - returning this value requires `*map` is borrowed for `'1`
7 |     None => {
8 |       map.insert(22, String::from("hi"));
  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

---

# Desugared 

```rust
fn get_or_insert<'a>(
    map: &'a mut HashMap<u32, String>,
) -> &'a String {
    match HashMap::get(&*map, &22) {
        Some(v) => v,
        None => {
            HashMap::insert(&mut *map, 22, String::from("hi"));
            &map[&22]
        }
    }
}
```
https://nikomatsakis.github.io/rust-belt-rust-2019/#72
https://www.youtube.com/watch?v=_agDeiWek8w&t=329s&ab_channel=RustBeltRustConference

---

# Polonius can't deal with 1
```
struct S;

fn main() {
    let s=S;
    let mut v:Vec<&S>=vec![];
    v.push(&s);
    v.pop();
    drop(s);
    // although v has remove the last element,
    // but still throw error
    v;
}
```


---

# Polonius can't deal with 2

```
use std::collections::binary_heap::BinaryHeap ;

fn main() {
    let mut heap = BinaryHeap::new();
    if let Some(_) = heap.peek_mut() {
        heap.push(4);
    };
}
```

https://github.com/rust-lang/rust/issues/70797

---

# How Polonius work?

![bg left:40% 80%](liveness.drawio.png )


--- 
# Benefit of my proof

We don't need to worry about the correctness of datafrog-opt.
Currently, we rely on a lot of tests to confirm the equivalence.

Helpful to verify new datalog rules.

