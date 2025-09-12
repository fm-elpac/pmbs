# pmbs 编译辅助命令

# 安装配置文件
# install -Dm755 target/release/pmbs $(TO)/usr/bin/
install-config:
	install -Dm644 -t $(TO)/usr/lib/systemd/system/ $(FROM)/systemd-unit/system/pmbs-snapshot.service
	install -Dm644 -t $(TO)/usr/lib/systemd/system/ $(FROM)/systemd-unit/system/pmbs-snapshot.timer
	install -Dm644 -t $(TO)/usr/lib/systemd/system/ $(FROM)/systemd-unit/system/pmbs-clean.service
	install -Dm644 -t $(TO)/usr/lib/systemd/system/ $(FROM)/systemd-unit/system/pmbs-clean.timer
	install -Dm644 -t $(TO)/usr/lib/systemd/user/ $(FROM)/systemd-unit/user/pmbs-rsync-home.service.example
	install -Dm644 -t $(TO)/usr/lib/systemd/user/ $(FROM)/systemd-unit/user/pmbs-rsync-home.timer.example

	install -Dm644 -t $(TO)/etc/pmbs/ $(FROM)/etc-pmbs/home.toml.en.example
	install -Dm644 -t $(TO)/etc/pmbs/ $(FROM)/etc-pmbs/home.toml.zh.example
	install -Dm644 -t $(TO)/etc/pmbs/rsync/ $(FROM)/etc-pmbs/rsync/rsync-home.sh.example
	install -Dm644 -t $(TO)/etc/pmbs/rsync/ $(FROM)/etc-pmbs/rsync/rsync-home-exclude.txt.example

	install -Dm644 -t $(TO)/usr/share/licenses/pmbs/ $(FROM)/LICENSE

	install -Dm644 -t $(TO)/usr/share/doc/pmbs/ $(FROM)/README.md 
	install -Dm644 -t $(TO)/usr/share/doc/pmbs/ $(FROM)/doc/pmbs.md
.PHONY: install-config

# 打包部分源文件 (方便编译)
pmbs-src:
	tar -cvf pmbs-src.tar systemd-unit/ etc-pmbs/ doc/ LICENSE README.md Makefile
.PHONY: pmbs-src
