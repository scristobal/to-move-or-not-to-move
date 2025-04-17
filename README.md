# To move or not to move

What is faster, move ownership arround, reference the struct or using a smart pointer?

```
To move or not to move/enrich by move BigStruct
                        time:   [199.27 ms 199.97 ms 200.69 ms]
To move or not to move/enrich by Rc<RefCell<BigStruct>>
                        time:   [213.31 ms 214.00 ms 214.72 ms]
To move or not to move/enrich by mutable reference of BigStruct
                        time:   [199.52 ms 200.21 ms 200.95 ms]
```

well the truth is that move ownership arround is faster than using a smart pointer. But it really does not matter because the BigStruct is actually a bunch of vectors so it is already in the heap.

run it with <https://github.com/bheisler/cargo-criterion>
