# To move or not to move

What is faster, move ownership arround or using a smart pointer?

```
Benchmarking enrich by move BigStruct: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 22.4s, or reduce sample count to 20.
enrich by move BigStruct
                        time:   [229.00 ms 233.28 ms 238.80 ms]

Benchmarking enrich by Rc<RefCell<BigStruct>>: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 27.3s, or reduce sample count to 10.
enrich by Rc<RefCell<BigStruct>>
                        time:   [270.71 ms 271.25 ms 271.94 ms]
```

well the truth is that move ownership arround is faster than using a smart pointer. But it really does not matter because the BigStruct is actually a bunch of vectors so it is already in the heap.

run it with <https://github.com/bheisler/cargo-criterion>
