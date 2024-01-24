# ext4 FS bare metal driver


## Features
- read
- readdir

## Toolchain
- rust

## Build & Run

```
$ cargo b 
```
```
$ dd if=/dev/zero of=hd.img bs=1M count=128
$ mkfs.ext4 hd.img -L TEST -b 4096
$ cargo r --example=test -- -d hd.img

```

## License

- MIT License
