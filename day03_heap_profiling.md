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