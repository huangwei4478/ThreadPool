# ThreadPool 
a thread pool implemented in Rust

# how to run demo

```
cargo run
```

# how to use thread pool

```rust
let pool = ThreadPool::new(4);          // specify the count of workers (i.e. how many threads in this thread pool)
pool.execute(|| {
    handle_tedious_and_heavy_job();
});
```

# how it implements
    - use Arc<Mutex<mpsc::Receiver<Message>>> to transport the work to another thread;
    - use mpsc::channel to communicate among different threads;
    - each thread has its own infinite loop, looping for message to come;
    - all threads would come to an end while received with a 'end' message;
  
![screenshot.png](https://i.loli.net/2020/12/15/PwXxcloUVZjGTKM.png)


