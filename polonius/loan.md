每个loan需要记住是mut 还是 immut 的
一个mut 的loan 可能被 不可变的origin持有


```
fn main(){
    let mut a = 0;
    let b = &mut a;  // bw0
    let c = &* b;    // 'c contains bw0
    c;
    b;
}
```

