https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/adjustment/enum.Adjust.html

```
struct S;

impl S{
    fn foo(& self){
    }
}
```


```
[clippy_lints/src/borrow_deref_ref.rs:59] cx.typeck_results().expr_adjustments(e) = [
    Borrow(Ref(ReErased, Not)) -> &S,
]
{
    let s = S;
    s.foo();
}
```

```
[clippy_lints/src/borrow_deref_ref.rs:59] cx.typeck_results().expr_adjustments(e) = [
    Deref(None) -> S,
    Borrow(Ref(ReErased, Not)) -> &S,
]
{
    let s = &S;
    s.foo();
}
```

```
[clippy_lints/src/borrow_deref_ref.rs:59] cx.typeck_results().expr_adjustments(e) = [
  Deref(None) -> S,
  Borrow(Ref(ReErased, Not)) -> &S,
]
{
    let s = &mut S;
    s.foo();
}
```

```
[clippy_lints/src/borrow_deref_ref.rs:59] cx.typeck_results().expr_adjustments(e) = [
    Deref(None) -> &S,
    Deref(None) -> S,
    Borrow(Ref(ReErased, Not)) -> &S,
]
{
    let s = &&S;
    s.foo();
}
```



```
[clippy_lints/src/borrow_deref_ref.rs:59] cx.typeck_results().expr_adjustments(e) = [
    NeverToAny -> usize,
]
fn foo() -> usize {
    let x: ! = {
        return 123
    };
}
```




```
[clippy_lints/src/borrow_deref_ref.rs:59] cx.typeck_results().expr_adjustments(e) = [
    Deref(None) -> [usize; 3],
    Borrow(Ref(ReErased, Not)) -> &[usize; 3],
    Pointer(Unsize) -> &[usize],
]

let a: [usize;3] = [1,2,3];
let y: &[usize] = &a;
```


```
[clippy_lints/src/borrow_deref_ref.rs:59] cx.typeck_results().expr_adjustments(e) = [
    Pointer(MutToConstPointer) -> *const usize,
]
let x : *mut usize = &mut 1;
let y : *const usize = x;
```

