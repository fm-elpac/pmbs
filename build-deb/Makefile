# build .deb for pmbs (ubuntu)

deb:
	- rm -r deb
	mkdir -p deb/DEBIAN
	cp control deb/DEBIAN
	cp conffiles deb/DEBIAN

	mkdir -p deb/usr/bin
	cp ../target/release/pmbs deb/usr/bin

	mkdir -p deb/usr/share/doc/pmbs
	cp ../LICENSE deb/usr/share/doc/pmbs/copyright

	gzip --best -nc changelog.Debian > changelog.Debian.gz
	mv changelog.Debian.gz deb/usr/share/doc/pmbs

	cd .. && make FROM=. TO=build-deb/deb install-config

	dpkg-deb --root-owner-group --build deb
	mv deb.deb pmbs.deb
.PHONY: deb
