```
// ok
fn foo<'a>()-> fn(&'a u32){
    fn b<'b>(x : &'b u32) {}
    b
}

fn main(){
    let a= 0;
    let f = foo();
    let b= 0;
    f(&a);
    f(&b);
}
```

```
// fail
fn foo<'a>()-> impl Fn(&'a u32){
    fn b<'b>(x : &'b u32) {}
    b
}

fn main(){

    let f = foo();

    let a= 0;
    f(&a);

}
```

```
// ok
fn foo()-> impl for<'a> Fn(&'a u32){
    fn b<'b>(x : &'b u32) {}
    b
}

fn main(){

    let f = foo();

    let a= 0;
    f(&a);

}
```
