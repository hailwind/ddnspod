# ddnspod

## A ddns cli for dnspod
基于 [dnspod-lib](https://github.com/hangj/dnspod-lib)
使用配置文件进行配置更新方式。
从指定网卡查询IP地址，为每一个子域名根据类型设置IP地址。

可以运行在openwrt上。

编译方法(For OpenWRT X86_64)
```bash
cargo build --release --target=x86_64-unknown-linux-musl
```

运行方法
```bash
# ddnspod -h
Usage: ddnspod [OPTIONS]

Options:
  -c, --cfg <CFG>  配置文件路径 [default: /etc/ddnspod.json]
  -d, --dry        模拟运行
  -l, --lrun       循环运行还是一次性运行
  -h, --help       Print help
  -V, --version    Print version

# 标准运行后台运行模式示例命令为：
ddnspod -l

```
