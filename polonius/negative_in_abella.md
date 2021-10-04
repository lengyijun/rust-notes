# 如何在abella中表示否定

## 方法一：
```
Type origin_live_on_entry origin -> point -> prop. 
/* At any point, the origin must either be live or dead */                                                         
Theorem Axiom2:                                                                                                    
  forall Origin,                                                                                                   
  forall Point,                                                                                                    
  (origin_live_on_entry Origin Point ) \/ ( origin_live_on_entry Origin Point -> false).                           
skip. 
```
### 优点：
只引入一个Axiom
如果出现
```
H1: origin_live_on_entry Origin Point
H2: origin_live_on_entry Origin Point -> false
```
可以`search` 自动推导错误

### 缺点：
一个Type和一个Hyposis是对立的，感觉有点奇怪




## 方法二：
```
Type origin_live_on_entry origin -> point -> prop. 
Type not_origin_live_on_entry origin -> point -> prop. 

Theorem OriginAxiom:                                                           
  forall Origin,
  forall Point,        
  origin_live_on_entry Origin Point ->                   
  not_origin_live_on_entry Origin Point -> false.        
skip.                       
                            
Theorem Axiom2:                 
  forall Origin,                
  forall Point,                                                                                                                                   
  (origin_live_on_entry Origin Point ) \/ ( not_origin_live_on_entry Origin Point ).    
skip.    
```

### 优点：
origin_live_on_entry 和 not_origin_live_on_entry 非常对称

### 缺点：
多引入了一个Type
引入了2个Axiom

```
H1: origin_live_on_entry Origin Point
H2: not_origin_live_on_entry Origin Point
```
不能自动推导错误，需要`apply OriginAxiom to H1 H2`



## TODO
如果是互斥的三个状态，应该怎么描述？
这时候，自动search就变成很重要
