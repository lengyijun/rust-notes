
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

