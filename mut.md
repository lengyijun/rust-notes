
```
fn main(){
    let mut num = 32_u32;

    let a = &mut num;

    // let b : &mut _ = a; // case 1 : ok
    // let b = &mut *a;    // case 2 : ok 
    // let b = a;          // case 3 : wrong

    b;  
    a;  
}
```

上面3 个case 中，1 和 2 是完全等价的
case 3 触发了move, 所以报错
case 1 2 是reborrow ， 不是move,所以不报错


inspired by 
https://haibane-tenshi.github.io/rust-reborrowing/

