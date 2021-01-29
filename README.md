# pacifica

200 times faster than "ghq list -p" **but incompatible**.

## Benchmark
**191x faster** ≒ 1.185 / 0.0062
```bash 
hyperfine './target/release/pacifica'
Benchmark #1: ./target/release/pacifica
  Time (mean ± σ):       6.2 ms ±   0.5 ms    [User: 1.5 ms, System: 3.6 ms]
  Range (min … max):     5.4 ms …   8.3 ms    325 runs
  
hyperfine 'ghq list -p'
Benchmark #1: ghq list -p
Time (mean ± σ):      1.185 s ±  0.334 s    [User: 1.347 s, System: 10.284 s]
Range (min … max):    1.006 s …  2.128 s    10 runs
 ```


ghq version 1.1.5 (rev:HEAD)
## Why so fast?
It's because we're making the process super simple. It's not compatible with "ghq list -p" because it doesn't do all kinds of checks and stuff. Instead, it's fast!


## Command
```bash
pacifica #default read $HOME/src/**/**
```

## Config
```bash
export PACIFICA_PATH = "/Users/example/src"
pacifica
```

## Installation
```bash
git clone git@github.com:serinuntius/pacifica.git
cd pacifica
cargo install --path .
```

## License
[MIT](https://github.com/serinuntius/pacifica/blob/master/LICENCE)


## Author
- [serinuntius](https://github.com/serinuntius)
