https://smallcultfollowing.com/babysteps/blog/2021/11/05/view-types/

从对复杂struct的理解上很有用处
逻辑上联系密切的一些field组成一个view

对外接口设计上也很好
field的名字不用暴露，（以后可以rename,add，remove）
只暴露一个集合

不过不知道能不能用 macro 简单实现一下？

https://crates.io/crates/readonly

至于用trait来表示view,我觉得很奇怪，反对

# 和 readonly 的区别
## readonly
pub        可读    可写 
readonly   可读    不可写 
<default>  不可读  不可写

readonly 针对单个field
view 针对一组fields

readonly 和 borrow ck 无关
view 与 borrow ck 有关

