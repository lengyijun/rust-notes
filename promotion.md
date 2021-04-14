const promotion的作用：
1. 减少了栈上变量的分配，而是分配在常量区

局限：
只能用在不变的变量。如果发生了改变(mut)，就不能放在常量区了


```
// ok
// 没有const promotion
fn main() {
    let g=1;
    let x = &g; 
    let y = unsafe { &mut *(x as *const i32 as *mut i32) };
    *y = 42; 
}
```


```
// This should fail even without validation/SB
// compile-flags: -Zmiri-disable-validation -Zmiri-disable-stacked-borrows

// core dumped
// 有const promotion
fn main() {
    let x = &1; // the `&1` is promoted to a constant, but it used to be that only the pointer is marked static, not the pointee
    let y = unsafe { &mut *(x as *const i32 as *mut i32) };
    *y = 42;  //~ ERROR read-only
    assert_eq!(*x, 42);
}

```

const promotion 要从mir中看。

https://github.com/rust-lang/const-eval/blob/master/promotion.md


```
// 有const promotion
let a=&1;

// 没有const promotion
let a=&mut 1;
```

|                            | const promotion | binary size |
| -------------------------- | --------------- | ----------- |
| let a=&[0;5<<20];          | yes             | 3M          |
| let a=&mut [0;5<<20];      | no              | 3M          |
| let a=[0;5<<20]            | no              | 3M          |
| let mut a=[0;5<<20]        | no              | 3M          |
|                            |                 |             |
| let a=&[0,1,....5<<20]     | yes             | 24M         |
| let a=&mut [0,1,....5<<20] | no              | 24M         |
| let a=[0,1,....5<<20]      | no              | 24M         |
| let mut a=[0,1,....5<<20]  | no              | 24M         |

test under cargo build --release
