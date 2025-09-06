Name:      pmbs
Version:   0.1.0a3
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
# skip

%install
mkdir -p %{buildroot}/usr/bin/
install -Dm755 -t %{buildroot}/usr/bin/ %{_topdir}/SOURCES/pmbs
mkdir -p %{buildroot}/usr/lib/systemd/system/
install -Dm644 -t %{buildroot}/usr/lib/systemd/system/ %{_topdir}/SOURCES/pmbs-snapshot.service
install -Dm644 -t %{buildroot}/usr/lib/systemd/system/ %{_topdir}/SOURCES/pmbs-snapshot.timer
install -Dm644 -t %{buildroot}/usr/lib/systemd/system/ %{_topdir}/SOURCES/pmbs-clean.service
install -Dm644 -t %{buildroot}/usr/lib/systemd/system/ %{_topdir}/SOURCES/pmbs-clean.timer
mkdir -p %{buildroot}/etc/pmbs/
install -Dm644 -t %{buildroot}/etc/pmbs/ %{_topdir}/SOURCES/home.toml.zh.example
install -Dm644 -t %{buildroot}/etc/pmbs/ %{_topdir}/SOURCES/home.toml.en.example

%files
/usr/bin/pmbs
/usr/lib/systemd/system/pmbs-snapshot.service
/usr/lib/systemd/system/pmbs-snapshot.timer
/usr/lib/systemd/system/pmbs-clean.service
/usr/lib/systemd/system/pmbs-clean.timer
/etc/pmbs/home.toml.zh.example
/etc/pmbs/home.toml.en.example

%changelog
# TODO
