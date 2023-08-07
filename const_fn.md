const functions must (currently) be deterministic, 
don't get access to IO, 
don't get access to statics,
can't allocate on the heap. 

can panic

https://blog.yoshuawuyts.com/totality/#const-functions
https://doc.rust-lang.org/reference/const_eval.html

