
dyn 只与 Box Arc 连用，比如

# 不需要所有权
&dyn Trait

# 需要所有权
Box<dyn Trait>
Arc<dyn Trait>
Rc<dyn Trait>

