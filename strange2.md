
```
fn main() {
    let mut local:i32 = 0;
    let x = &mut local;
    println!("{:p}",x);
    let y:&mut i32 = &mut *x; // Reborrow x to y.
    println!("{:p}",y);
}
```

both x and y point to local

直觉上，y会复制一份，然而并没有

```
let local=&0;
let y=*local;
```
