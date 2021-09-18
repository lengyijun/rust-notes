```
subset(origin1,origin2,point2):-
  subset(origin1,origin2,point1),
  cfg_edge(point1,point2),
  origin_live_on_entry(origin1,point2),
  origin_live_on_entry(origin2,point2),
```

这条关系也挺神奇的，因为subset只进不出。
能断开origin1 <= origin2 只有origin1或者 origin2 失效

一日为师，终生为父
你可以尝试一下，能不能构造一个例子

```
subset(o1,o2,point1),
!subset(o1,o2,point2),
cfg_edge(point1,point2),
origin_live_at(o1,point2),
origin_live_at(o2,point2)
```
