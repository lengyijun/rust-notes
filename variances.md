# Variance

Variance是Rust中绕不开的问题。
Variance问题就是subtype和supertype的关系。
比如Cat是subtype, Animal是supertype. 因为Cat具有Animal所有的特征和函数。

比如 fn ff(x: Animal) 里，传一个Cat给他就问题不大

fn ff(x: &Animal) 里，传一个&Cat给他也可以

fn ff(x: &mut Animal)里，传一个&mut Cat就问题很大，因为可能会把Cat改成Dog

所以简单的从 （Cat是SubType,Animal是SuperType）， 不能推导出 （&mut Cat是SubType,&mut Animal是SuperType）

那么到底 & T，&mut T的variance和T是什么关系？有三种可能covariant,invariant,contravariant.


|   |                      |     'a    |         T         |     U     |
|---|--------------------- |:---------:|:-----------------:|:---------:|
| * | `&'a T `             | covariant | covariant         |           |
| * | `&'a mut T`          | covariant | invariant         |           |
| * | `Box<T>`             |           | covariant         |           |
|   | `Vec<T>`             |           | covariant         |           |
| * | `UnsafeCell<T>`      |           | invariant         |           |
|   | `Cell<T>`            |           | invariant         |           |
| * | `fn(T) -> U`         |           | contravariant     | covariant |
|   | `*const T`           |           | covariant         |           |
|   | `*mut T`             |           | invariant         |           |
|   | Box<dyn Trait<T>>[1] |           | invariant         |           |

[1]: rust/src/test/ui/variance/variance-covariant-arg-object.rs

用具体的例子：

|SubType      | SyperType    | Right？  |
| -------     | ---------    | -------- |
|Cat          | Animal       | ok       |
|&Cat         | &Animal      | ok       |
|&mut Cat     | &mut Animal  | false    |
|&mut Animal  | &mut Cat     | false    |
|Box<Cat>     | Box<Animal>  | ok       |
|Vec<Cat>     | Vec<Animal>  | ok       |
|Cell<Cat>    | Cell<Animal> | false    |
|Cell<Animal> | Cell<Cat>    | false    |





-----

我们分析一个在生命周期里运用variance的例子：

```
fn replace<T>(x:&mut T,y:&T){
	*x=y;
}

fn main(){
	let mut x:&'static str="";
	{
		let y="hello";
		replace(&mut x,&y);
	}
}
```



首先找T的类型。根据x,T只能是 `&'static str`

然后y的类型需要是是T的subtype

y的类型是`&'static str`的subtype

我们只考虑生命周期这个维度

y的生命周期是`'static`  的subtype

只有比 `'static`更长的生命周期才能是 `'static`的subtype

但显然y的生命周期很短，所以会报错。

注意：&mut x 引入了一个invariance，所以T被马上确定好了


所有的variance，最后都会归结到lifetime的比较？
variance 在polonius中也有用处
注意trait的比较不是variance的部分，是chalk，查询的部分，算法不一样


associated types are invariant

dyn 是一种特殊的associate type

src/test/ui/variance/variance-regions-direct.rs 提供了调试invariance的方法

https://doc.rust-lang.org/nomicon/subtyping.html

```
/// The `Context` of an asynchronous task.
///
/// Currently, `Context` only serves to provide access to a `&Waker`
/// which can be used to wake the current task.
#[stable(feature = "futures_api", since = "1.36.0")]
pub struct Context<'a> {
    waker: &'a Waker,
    // Ensure we future-proof against variance changes by forcing
    // the lifetime to be invariant (argument-position lifetimes
    // are contravariant while return-position lifetimes are
    // covariant).
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}
```

