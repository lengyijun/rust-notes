It lets you run a large number of concurrent tasks on a small number of OS threads,

Thread pools can mitigate some of these costs, but not enough to support massive IO-bound workloads.

No built-in runtime is provided by Rust. Instead, runtimes are provided by community maintained crates.

idle threads consume system resources.

an async runtime uses a small amount of (expensive) threads to handle a large amount of (cheap) tasks

async Rust results in larger binary blobs due to the state machines generated from async functions and since each executable bundles an async runtime.

Whenever the compiler encounters an async function, it generates a state machine under the hood.

you can't directly call an async function from a sync function.

There are two main ways to use async: async fn and async blocks. 
Each returns a value that implements the Future trait.

Currently, Context only serves to provide access to a &Waker which can be used to wake the current task.

每当 move 导致的lifetime失效时，都应该考虑 Pin

Currently, async fn cannot be used in traits. 
The reasons for this are somewhat complex, but there are plans to remove this restriction in the future.

It is recommended to measure performance for your application when you are choosing between a single- and a multi-threaded runtime.

async 块里一定要有 .awaits 

https://www.zhihu.com/question/556880425
rust 有实现有栈协程的能力，但由于原子性和安全性的冲突、空间利用率、堆分配等问题，最后胜出的是无栈协程。

