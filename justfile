test:
	scripts/cargotest.sh

build *release:
	CROSS_CONTAINER_ENGINE=podman cross build --target=x86_64-pc-windows-gnu {{release}}

dbox-debug:
	distrobox-host-exec ./run.sh --debug

debug:
	./run.sh --debug

dbox-run:
	distrobox-host-exec ./run.sh --release

run:
	./run.sh --release