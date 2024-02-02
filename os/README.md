# 查看目前 Rust 编译器支持哪些基于 RISC-V 的目标平台
```shell
rustc --print target-list | grep riscv
```


# 指定架构平台运行
```shell
cargo run --target riscv64gc-unknown-none-elf
```

# 给 rustc 添加一个target 
> 使得我们之后在 cargo build 的时候不必再加上 --target 参数的一个小 trick
```shell
rustup target add riscv64gc-unknown-none-elf
```
## 目录下新建 .cargo 目录，并在这个目录下创建 config 文件，并在里面输入如下内容
```text
# os/.cargo/config
[build]
target = "riscv64gc-unknown-none-elf"
```