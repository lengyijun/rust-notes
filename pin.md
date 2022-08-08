绝大多数类型都不在意是否被移动，因此它们都自动实现了 Unpin 特征
一个例外就是：async/await 生成的 Future 没有实现 Unpin


你可以通过以下方法为自己的类型添加 !Unpin 约束：
1. 使用文中提到的 std::marker::PhantomPinned 
2. 使用nightly 版本下的 feature flag

可以将值固定到栈上，也可以固定到堆上
  将 !Unpin 值固定到栈上需要使用 unsafe
  将 !Unpin 值固定到堆上无需 unsafe ，可以通过 Box::pin 来简单的实现


Pin<P> wraps a pointer type P (e.g. &mut T, Box<T>)

