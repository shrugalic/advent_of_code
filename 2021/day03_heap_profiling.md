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

#### After changing the `Vec<BitVec>` to a single `BitVec`

Since `BitVec` only seems to be efficient for larger vecs, I tried using a single `BitVec` for the whole input. To make this work, the `width` of an individual chunk needed to be passed around to consider where necessary. 

And instead of the elegant `retain(…)` function on `Vec` I did something complicated to avoid re-allocating new `BitVec`s while reducing the number of chunks in part 2.

However, it _does_ use considerably less memory than before, but simplicity / readability suffered accordingly. For this reason, I still prefer my original solution, because 4k vs 77k heap does not really matter for my use case. ;)

BTW, the size of the input.txt file is 13k.

```sh
dhat: Total:     4,072 bytes in 8 blocks
dhat: At t-gmax: 2,056 bytes in 2 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
```