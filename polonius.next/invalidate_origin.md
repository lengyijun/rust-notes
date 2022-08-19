对于 x : &mut u32 的使用要注意
会产生 `invalidate_origin` 的 fact
对于 &mut *x 生成的中间origin, 要 invalidate 掉


invalidated 是传染病，会从一个origin传染给另一个origin
invalidated 也是遗传病，会往下传
liveness是遗传病，会往上传
但liveness不是传染病，不会传给别的origin

