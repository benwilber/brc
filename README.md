Naive Rust implement of the [1 billion rows challenge](https://github.com/gunnarmorling/1brc)

### Create the measurements.txt file (13 GB)

```shell
$ cargo run --release -- create_measurements
```

### Run it

```shell
$ time cargo run --release -- run
real    1m18.636s
user    1m16.830s
sys 0m1.800s
```
