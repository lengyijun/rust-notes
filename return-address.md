1. 如果返回值是Copy, 那么return会复制一份
2. 如果返回值是zero size, 那么return后地址会不一样
3. 如果返回值是move的， 那么地址一样

```
// move， 地址一样
fn main() {
    let s = foo();
    println!("{:p}", &s);
}

fn foo() -> String {
    let s = String::new();
    println!("{:p}", &s);
    s
}

```

```
// Copy, 地址不一样
fn main() {
    let x = &12;
    println!("{:p}", &x);
    let y = foo(x);
    println!("{:p}", &y);
}

fn foo(x: &u32) -> &u32 {
    x
}

```

```
// zero size
// 地址不一样
struct S;

fn main() {
    let s = foo();
    println!("{:p}", &s);
}

fn foo() -> S {
    let s = S;
    println!("{:p}", &s);
    s
}

```


