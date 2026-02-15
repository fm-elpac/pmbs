Name:      pmbs
Version:   0.1.0
Release:   1%{?dist}
Summary:   Make btrfs snapshot (every minute), and auto clean
License:   MIT
URL:       https://github.com/fm-elpac/pmbs
Requires:  btrfs-progs

%description
pmbs: Make btrfs snapshot (every minute), and auto clean

%prep
# skip

%build
cd %{_topdir}/SOURCES/ && tar -xvf pmbs-src.tar

%install
install -Dm755 -t %{buildroot}/usr/bin/ %{_topdir}/SOURCES/pmbs

cd %{_topdir}/SOURCES/ && make FROM=. TO=%{buildroot} install-config

%files
/usr/bin/pmbs
/usr/lib/systemd/system/pmbs-snapshot.service
/usr/lib/systemd/system/pmbs-snapshot.timer
/usr/lib/systemd/system/pmbs-clean.service
/usr/lib/systemd/system/pmbs-clean.timer
/usr/lib/systemd/user/pmbs-rsync-home.service.example
/usr/lib/systemd/user/pmbs-rsync-home.timer.example
/etc/pmbs/home.toml.zh.example
/etc/pmbs/home.toml.en.example
/etc/pmbs/rsync/rsync-home.sh.example
/etc/pmbs/rsync/rsync-home-exclude.txt.example
/usr/share/licenses/pmbs/LICENSE
/usr/share/doc/pmbs/README.md
/usr/share/doc/pmbs/pmbs.md

%changelog
# TODO
