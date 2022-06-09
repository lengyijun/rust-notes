```
use std::collections::HashMap;

struct Foo<'a> {
    map: HashMap<usize, &'a str>
}

impl<'a> Foo<'a> {
    fn new() -> Foo<'a> { panic!() }
    fn insert(&'a mut self) { } 
}
fn main() {
    let mut foo = Foo::new();
    foo.insert();
    foo.insert(); //~ ERROR cannot borrow
}
```
错误



```
use std::collections::HashMap;

struct Foo<'a> {
    map: HashMap<usize, &'a str>
}

impl<'a> Foo<'a> {
    fn new() -> Foo<'a> { panic!() }
    fn insert(&mut self) { }
}
fn main() {
    let mut foo = Foo::new();
    foo.insert();
    foo.insert(); 
}

```
正确
