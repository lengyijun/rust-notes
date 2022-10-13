```
trait Base {}
trait Derived: Base {
    fn upcast_base(self: Box<Self>) -> Box<dyn Base>;
    fn as_base(&self) -> &dyn Base;
}
```

其中直接返回 Self 

https://github.com/rust-lang/rfcs/pull/3324

