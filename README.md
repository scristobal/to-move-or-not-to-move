# To move or not to move (stack edition)

What is faster, move ownership arround or using a smart pointer?

```
enrich by move BigStruct
                        time:   [26.082 ms 26.114 ms 26.149 ms]
                        change: [-0.7355% -0.4322% -0.1780%] (p = 0.00 < 0.05)
                        Change within noise threshold.

enrich by Rc<RefCell<BigStruct>>
                        time:   [24.567 ms 24.602 ms 24.646 ms]
                        change: [+0.6825% +0.8927% +1.1089%] (p = 0.00 < 0.05)
                        Change within noise threshold.

```



run it with <https://github.com/bheisler/cargo-criterion>
