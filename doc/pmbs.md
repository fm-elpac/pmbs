# 胖喵必快 (pmbs) 主要设计文档

```ignore
胖喵必快 (pmbs)

正式名称: 紫腹巨蚊 (Toxorhynchites gravelyi) 系列
    澳大利亚海神草 (Posidonia australis) 软件
```

目录:

- 1 问题背景

- 2 源代码目录结构

- 3 命令行环境变量

- 4 快照目录结构

- 5 systemd timer

- 6 自动清理

## 1 问题背景

本软件的主要设计用途是 **防误删**. 也就是不小心删除自己的重要文件, 造成数据丢失.

典型的 **定期备份** (backup) 是不够的, 因为间隔时间通常较长,
比如每周或每天备份一次. **实时同步** (sync) 文件 (比如给一个目录设置镜像)
也是不够的, 因为原文件被删除之后, 镜像也会被同步删除.

**快照** (snapshot) 是一个很好的解决方案, 特别是 `btrfs` 快照. 因为 btrfs 的 CoW
(写时复制) 特性, 快照是轻量级, 快速的. 只有快照后变动的文件,
才会占用额外的存储空间. 通过使用 `root` 创建 **只读** 快照,
普通用户无权限删除快照 (只能读取), 从而可以在很大程度上防止误删除.
如果发生了误删, 普通用户可以直接读取之前的快照, 恢复文件 (复制回来),
恢复过程无需 root, 进一步降低了风险 (避免误操作破坏快照, 导致数据彻底丢失).

snapper (<http://snapper.io/>) 是一个现有的自动 btrfs 快照开源软件,
本来考虑的就是直接使用 snapper. 然而 snapper 只支持 **每小时** 快照一次,
间隔时间太长了. 对窝来说, 丢失最近一小时的数据是不可接受的. 因此,
只能自己再写一个软件了, 目标是实现 **分钟级** 快照 (Per Minute Btrfs Snapshot),
如果不小心发生了误删, 数据丢失通常在 1 分钟以内, 这是可以接受的.

注意, 快照 **不能替代备份** !! 如果发生 **硬件损坏** (比如磁盘故障), 操作系统
BUG (比如 Linux 内核, btrfs 文件系统的 BUG) 等情况, 仍然会发生数据丢失损坏,
快照对此无能为力.

---

根据上述使用场景, 只读快照需要以 `root` 在后台定期自动创建. root 是 Linux
系统中的超级用户权限, 如果出问题可能造成严重的破坏, 因此必须谨慎使用.

所以选择使用 `rust` (<https://www.rust-lang.org/>) 编程语言来编写本软件,
避免使用 `unsafe`, 同时尽量保持代码和功能的简单 (简单的东西更不容易出 BUG).

创建快照 (删除快照) 功能, 实现方法是直接调用 `btrfs` 命令行, 比如
`btrfs subvol snapshot -r` 和 `btrfs subvol delete`.

定期执行功能, 使用 systemd timer 触发
(<https://wiki.archlinux.org/title/Systemd>). 因为 systemd 是 Linux
系统的关键基础系统组件 (PID 1), 所以稳定性应该是很高的, 不容易出问题.

大量快照会占用大量的存储空间, 因此 **自动清理** 功能也是很重要的,
根据配置自动删除旧的快照.

注意: 目前本软件 **不会检测剩余存储空间**, 存在耗尽存储空间的风险, 需要多关注,
经常查看剩余存储空间. 因为本软件认为, 相比耗尽存储空间的风险,
数据丢失是更严重的情况, 更不能接受, 因此及时创建新的快照是更优先的事情.

## 2 源代码目录结构

- `src/`: 本软件的源代码 (rust).

- `doc/`: 文档.

- `systemd-unit/`: systemd 服务文件, 用于实现 systemd timer.

- `etc-pmbs/`: 配置文件示例.

- `build-aur/`: 用于构建 ArchLinux 软件包 (含有 AUR 的 `PKGBUILD`).

- `build-rpm/`: 用于构建 RPM 软件包 (用于 Fedora CoreOS).

- `.github/`: CI (自动化测试编译).

## 3 命令行环境变量

本软件的命令行参数是很简单的, 这可以简化解析命令行的代码. 更多配置选项通过
**环境变量** (env var) 实现.

[`config::ConfigEnv`] 定义了所有使用的环境变量:

| 环境变量         | 默认值          | 说明                           |
| :--------------- | :-------------- | :----------------------------- |
| `PMBS_DIR_ETC`   | `/etc/pmbs`     | 存放配置文件 (`*.toml`) 的目录 |
| `PMBS_DIR_LOG`   | `/var/log/pmbs` | 写入日志文件的目录             |
| `PMBS_BIN_BTRFS` | `btrfs`         | btrfs 命令                     |
| `RUST_LOG`       | `info`          | 输出日志级别 (`env_logger`)    |

## 4 快照目录结构

比如:

```sh
> ls -ali .pmbs
总计 4
929570 drwxr-xr-x 1 root root   20  9月 2日 03:48 ./
   256 drwxr-xr-x 1 root root   54  8月28日 23:32 ../
929571 drwxr-xr-x 1 root root 4080  9月 2日 03:48 2025/
944869 lrwxrwxrwx 1 root root   15  9月 2日 03:48 latest -> 2025/1756756128/
> ls -ali .pmbs/2025

   256 drwxr-xr-x 1 root root   54  8月28日 23:32 1756756005/
   256 drwxr-xr-x 1 root root   54  8月28日 23:32 1756756066/
   256 drwxr-xr-x 1 root root   54  8月28日 23:32 1756756128/
```

- 在目标 `subvol` (被快照的 subvol) 之下直接创建 `.pmbs` 目录, 比如
  `/home/.pmbs`

- 快照 (snapshot) 路径类似 `.pmbs/2025/1756756128/`

  其中 `1756756128` 是时间戳 (`UNIX_EPOCH` 开始的秒数), `2025` 是对应的年.

- 符号链接 `.pmbs/latest` 指向最新的快照.

## 5 systemd timer

有 2 个 systemd timer (以及对应的 service):

- `pmbs-snapshot.timer` (`pmbs-snapshot.service`) 定期执行:

  ```sh
  pmbs config snapshot
  ```

  读取所有配置文件, 并创建对应的快照.

  每分钟 (60 秒) 执行一次, 时间精度 1 秒, 随机延迟 1 秒.

- `pmbs-clean.timer` (`pmbs-clean.service`) 定期执行:

  ```sh
  pmbs config clean
  ```

  读取所有配置文件, 并执行自动清理.

  每 10 分钟执行一次, 时间精度 1 分钟, 随机延迟 1 分钟.

## 6 自动清理

TODO
