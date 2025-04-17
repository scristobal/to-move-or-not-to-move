# To move or not to move (stack edition)

What is faster, move ownership arround or using a smart pointer?

```
enrich by move BigStruct
                        time:   [2.0516 ms 2.0531 ms 2.0548 ms]
                        change: [-93.457% -93.449% -93.441%] (p = 0.00 < 0.05)
                        Performance has improved.

enrich by Rc<RefCell<BigStruct>>
                        time:   [2.0148 ms 2.0179 ms 2.0220 ms]
                        change: [-93.295% -93.278% -93.260%] (p = 0.00 < 0.05)
                        Performance has improved.
```



run it with <https://github.com/bheisler/cargo-criterion>
