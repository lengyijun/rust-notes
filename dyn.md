
dyn 只与 Box Arc 连用，比如

# 不需要所有权
&dyn Trait

# 需要所有权
Box<dyn Trait>
Arc<dyn Trait>
Rc<dyn Trait>


Question:
Box<> 如何导出一个 &dyn Trait 的引用？
Answer: &*

```
trait T{}
struct S;
impl T for S{}

let s = S;
let x : &dyn T = &s;

let s = Box::new(S);
let x : Box<dyn T> = s;

let y : &dyn T = &*s;
```


Question:
&dyn 和 &impl 的区别？
Answer：
impl 和 范型差不多(用在函数签名里)
  但是范型比 impl 更好一点，见 file:///home/lyj/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/reference/types/impl-trait.html

dyn 会有heap 分配和 dynamic dispatch,
所以尽量用 impl

&impl 不能用在类型标注
&dyn 可以用在类型标注
```
use std::fmt::Debug;

fn main(){
    let y : &impl Debug = &1;  // error
    let y : &dyn Debug = &1;   // ok
}
```

impl 只是影响使用
dyn 是影响内存分配的


```
trait Trait {}

fn foo(arg: impl Trait) {
}

struct S;

impl Trait for S{}

impl Trait for Box<dyn Trait> {}

fn main(){
   let s : Box<dyn Trait> = Box::new(S);
    // let s = S;
    foo(s);
}
```

------

async fn f(…) -> Ret

is sugar for 

fn f(…) -> impl Future<Output = Ret>


--- 

dyn 允许异构集合
file:///home/lyj/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types


--- 

dyn 相当于存在量词

https://rust-lang.github.io/a-mir-formality/blog

## dyn 详解
https://quinedot.github.io/rust-learning/dyn-trait.html

