# To move or not to move (stack edition)

What is faster, move ownership arround, reference the struct or using a smart pointer?

```
To move or not to move/enrich by move BigStruct
                        time:   [26.435 ms 26.444 ms 26.453 ms]
To move or not to move/enrich by Rc<RefCell<BigStruct>>
                        time:   [24.227 ms 24.281 ms 24.352 ms]
To move or not to move/enrich by mutable reference of BigStruct
                        time:   [24.183 ms 24.194 ms 24.206 ms]
```



run it with <https://github.com/bheisler/cargo-criterion>
