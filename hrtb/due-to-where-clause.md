src/test/ui/hrtb/due-to-where-clause.rs

```
fn main() {
    test::<FooS>(&mut 42); //~ ERROR implementation of `Foo` is not general enough
}

trait Foo<'a> {}

struct FooS<'a> {
    data: &'a mut u32,
}

impl<'a, 'b: 'a> Foo<'b> for FooS<'a> {}

fn test<'a, F>(data: &'a mut u32) where F: for<'b> Foo<'b> {}
```

源代码有点冗余，我们先去掉一些不相关的部分：依然保留错误

```
fn main() {
    test::<FooS>(); //~ ERROR implementation of `Foo` is not general enough
}

trait Foo<'a> {}

struct FooS<'a> {
    data: &'a mut u32,
}

impl<'a, 'b: 'a> Foo<'b> for FooS<'a> {}

fn test<F>() where F: for<'b> Foo<'b> {}
```

我们猜测是 impl 的 'b 'a 乱套了，所以我们去掉 'b

```
fn main() {
    test::<FooS>(); //~ ERROR implementation of `Foo` is not general enough
}

trait Foo<'a> {}

struct FooS<'a> {
    data: &'a mut u32,
}

impl<'a> Foo<'a> for FooS<'a> {}

fn test<F>() where F: for<'b> Foo<'b> {}
```
依然报错！

最后把FooS 结构体的定义修改一下


```
fn main() {
    test::<FooS>(); //~ ERROR implementation of `Foo` is not general enough
}

trait Foo<'a> {}

struct FooS {
}

impl<'a> Foo<'a> for FooS {}

fn test<F>() where F: for<'b> Foo<'b> {}
```

编译通过
