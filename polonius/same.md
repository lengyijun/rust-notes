
```
fn main(){
    let a = &0;
    let b = a;
}

fn foo(){
    let a = &0;
    let b = &*a;
}
```

这两段代码的 mir 是一样的

