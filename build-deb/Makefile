# build .deb for pmbs (ubuntu)

deb:
	- rm -r deb
	mkdir -p deb/DEBIAN
	cp control deb/DEBIAN
	cp conffiles deb/DEBIAN

	mkdir -p deb/usr/bin
	cp ../target/release/pmbs deb/usr/bin

	mkdir -p deb/usr/lib/systemd/system
	cp ../systemd-unit/* deb/usr/lib/systemd/system

	mkdir -p deb/etc/pmbs
	cp ../etc/* deb/etc/pmbs

	mkdir -p deb/usr/share/doc/pmbs
	cp ../LICENSE deb/usr/share/doc/pmbs
	cp ../LICENSE deb/usr/share/doc/pmbs/copyright

	cp ../README.md deb/usr/share/doc/pmbs
	cp ../doc/pmbs.md deb/usr/share/doc/pmbs

	gzip --best -nc changelog.Debian > changelog.Debian.gz
	mv changelog.Debian.gz deb/usr/share/doc/pmbs

	dpkg-deb --root-owner-group --build deb
	mv deb.deb pmbs.deb
.PHONY: deb
