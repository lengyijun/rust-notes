# This post maybe wrong!

```
let mut z: u32;
let mut x: &'x u32;
let mut y: &'y u32;
if something {
y = x; // creates `'x subset-of 'y`.
}
if something {
x = &z; // creates {L0} in 'x constraint.
//
// at this point, we have
// `'x subset-of 'y` and `{L0} in `'x`,
// so we also have `{L0} in 'y` (wrong).
drop(x);
}
z += 1; // Polonius: false positive error
drop(y);
```

但这个已经修好了。通过要求每个origin都是活着。

不过这依然有问题，因为可能会导致基于polonius的分析不精确。

```
z=&_1
y=z
<- y is related to {_1,_2}
   which is wrong
z=&_2
use y
use z
```

要产生错误：
1. 在使用y之前，要`drop _2`, 这样才能触发错误
2. 在使用y之后，要`use z`，让z是活的，才能让subset的关系继承下去

Possible fix:

