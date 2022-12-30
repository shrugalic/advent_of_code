Inspired by [Chris Biscardi's video](https://youtu.be/NnIZjcO2g20) I added [dhat](https://docs.rs/dhat/) to part 3 for heap profiling.

#### Initial result for part 1:

```sh
❯ cargo run --bin day03 --release --features dhat-heap
…
dhat: Total:     73,068 bytes in 2,010 blocks
dhat: At t-gmax: 40,588 bytes in 1,002 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
```

#### After changing my `Vec<Vec<bool>>` with `Vec<BitVec>`

It uses _more_ heap space, because apparently it stores the BitVec as a usize, which is a bit of a waste given our binary numbers have length 5 (for the example) or 12 (for the input).

```sh
dhat: Total:     81,064 bytes in 1,010 blocks
dhat: At t-gmax: 56,584 bytes in 1,002 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
```