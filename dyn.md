
dyn 只与 Box Arc 连用，比如

# 不需要所有权
&dyn Trait

# 需要所有权
Box<dyn Trait>
Arc<dyn Trait>
Rc<dyn Trait>


Question:
Box<> 如何导出一个 &dyn Trait 的引用？

```
trait T{}
struct S;
impl T for S{}

let s = S;
let x : &dyn T = &s;

let s = Box::new(S);
let x : Box<dyn T> = s;

let y : &dyn T = ?
```

