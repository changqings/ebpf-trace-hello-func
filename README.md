# print-hello

## Prerequisites

1. Install bpf-linker: `cargo install bpf-linker`

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag.

## Build Userspace

```bash
cargo build
```

## Run

```bash
RUST_LOG=info cargo xtask run
```

## 补充

### go 代码

go.mod

```
module hello

go 1.22
```

main.go

```
package main

import (
        "fmt"
        "time"
)


func hello(n int) {
        fmt.Println("hello",n)
}

func main() {
        fmt.Println("main start")

        for i := range 1000 {
                hello(i)
                time.Sleep(time.Second * 1)
        }
}
```

### program.attach() 参数

```
#在 user space 代码中`print-hello/src/main.rs`中
    program.attach(
        Some("main.hello"),
        0,
        "/home/shen/go-ws/hello/hello",
        opt.pid,
    )?;
```

### 参数解释：

`fn_name`: `Option<&str>`

- uprobe 要检测的程序内的函数名，使用 readelf -s <elf 可执行程序>来获取函数名
- go build 默认编译时，对函数名进行了优化，比如 runtime.main.func2.即不是代码是的函数名，所以要查出来 elf 中正确的函数名
- 这里可以使用 go build 时使用构建参数来实现，其它程序请自行百度

`offset`: `u64`

- 目标的偏移量，这里我使用了 0，应该还有其它用法，待查

`target`: `T`

- 二进制可执行文件 or 库名的绝对路径

`pid`: `Option<pid_t>`

- 可选使用 pid 来识别目标，这里可以限制 target，即不但要满足 target，还是满足此 pid，才会进行连接

### 性能损耗

会有性能损耗，但是相对于传统的追踪技术，此影响较小。
性能损耗由多种因素决定，比如被追踪的函数调用频率、eBPF 程序的复杂度、系统的负载情况等
请酌情使用
