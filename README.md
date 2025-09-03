# 胖喵必快

<https://github.com/fm-elpac/pmbs>

![CI](https://github.com/fm-elpac/pmbs/actions/workflows/ci.yml/badge.svg)

(每分钟) 创建 btrfs 快照, 并自动清理.

Make btrfs snapshot (every minute), and auto clean.

正式名称: 紫腹巨蚊 (Toxorhynchites gravelyi) 系列 澳大利亚海神草 (Posidonia
australis) 软件

---

镜像 (mirror):

- <https://bitbucket.org/fm-elpac/pmbs/>
- <https://codeberg.org/fm-elpac/pmbs>
- <https://notabug.org/fm-elpac/pmbs>
- <https://framagit.org/fm-elpac/pmbs>
- <https://git.disroot.org/fm-elpac/pmbs>
- <https://gitlink.org.cn/fm-elpac/pmbs>

胖喵必快使用 `root` 权限创建 **只读** btrfs 快照, 所以普通用户 (无 root)
无权限删除快照. 因此可以防止误删文件 (可以从快照找回).

pmbs run as `root` to create **readonly** btrfs snapshot, so normal user (no
root) can not delete snapshot. So can prevent delete files by mistake (can
recovery from snapshot).

## 安装 (install)

- ArchLinux (AUR): TODO

  或者从 [release](https://github.com/fm-elpac/pmbs/releases) 下载安装包:

  Or download pre-compiled package from
  [release](https://github.com/fm-elpac/pmbs/releases):

  ```sh
  sudo pacman -U pmbs-bin-0.1.0a2-1-x86_64.pkg.tar.zst
  ```

- Fedora CoreOS (RPM):

  从 [release](https://github.com/fm-elpac/pmbs/releases) 下载安装包:

  Download pre-compiled package from
  [release](https://github.com/fm-elpac/pmbs/releases):

  ```sh
  sudo rpm-ostree install pmbs-0.1.0a2-1.fc42.x86_64.rpm
  ```

  然后重启系统.

  And reboot.

---

安装之后 (after install):

- (1) 编写配置文件, 比如:

  Edit config file, for example:

  ```sh
  cd /etc/pmbs
  sudo cp home.toml.zh.example home.toml  # or home.toml.en.example
  env EDITOR=nano sudo -e home.toml
  ```

  原则上, 保留规则应该按顺序编写, 上一条规则的间隔时间 (`time`)
  应该比下一条更短, 比如 `1m` (分钟), `5m`, `1h` (小时), `1d` (天). 同时保留个数
  (`n`) 应该大于 0.

  Keep rules should be written in order: `time` of one rule should be shorter
  than the next rule, for example: `1m` (minute), `5m`, `1h` (hour), `1d` (day).
  Number to keep (`n`) should be larger than 0.

- (2) 启用 systemd 服务 (定期快照/清理):

  Enable systemd timer (snapshot/clean):

  ```sh
  sudo systemctl enable --now pmbs-snapshot.timer
  sudo systemctl enable --now pmbs-clean.timer
  ```

---

- 列出指定 subvol 的快照, 比如:

  List snapshots of a subvol, for example:

  ```sh
  pmbs ls /home
  ```

- 可以直接使用 `btrfs` 命令删除快照:

  You can delete a snapshot with `btrfs` command:

  <https://wiki.archlinux.org/title/Btrfs#Deleting_a_subvolume>

## 常见问题 (FAQ)

TODO

## 文档 (doc)

- 主要设计文档: [doc/pmbs.md](./doc/pmbs.md)

TODO

## LICENSE

`MIT`
