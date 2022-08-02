对于 x : &mut u32 的使用要注意
会产生 `invalidate_origin` 的 fact
对于 &mut *x 生成的中间origin, 要 invalidate 掉

