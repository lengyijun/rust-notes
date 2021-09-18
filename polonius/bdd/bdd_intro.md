
一个函数是一个bdd图。
两个图可以做计算，得到新的图。
规则很多，但是图却可能很小 => save memory

两个图可以比较是否相同。相同的图表示规则不再改变，循环结束。

全称量词的时候，point要放在最后一个维度

非一个函数，在bdd中就是terminal中 1 和 0 互换

# bdd 中优化的方法
split 规则。自动split
最佳ordering用machine learning找到


# concern
是否会丢失信息？

# notes
universal 的规则要填充一些Point
比如 point有5个
000
001
010
011
100

我们还要加入
101
110
111

这样才能表示全所有的点，否则无法压缩图。

