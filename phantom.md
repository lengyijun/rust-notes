```
use std::marker::PhantomData;


trait A<T>{}
trait B<X,T:A<X>>{}

struct C<X,Y:A<X>,T:B<X,Y>>{ 
  a : PhantomData<X>,
  b : PhantomData<Y>,
  c : T 
}
```

如果没有phantom这两个field，那么会报错


https://github.com/rust-lang/rust/issues/102810
PhantomData is less like Box<T> and more like [T; 0].

