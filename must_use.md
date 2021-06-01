must_use 如果用在struct上，表示这个struct的实例必须被接收

```
#[must_use]
strust S;

// wrong
S{};
// right
let s=S{};
```

同理，enum，union

must_use 用在函数上，表示函数的返回值必须被接收

```
#[must_use]
fn f()->i32{0}

// wrong
f();
// right
let a=f();
```
