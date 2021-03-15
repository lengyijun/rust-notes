使用atomic的场景
1. static 全局变量，可以多个线程共同使用
2. 局部变量，用Arc<AtomicI8> 传递

