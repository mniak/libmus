.PHONY=run
dev: node_modules/
	WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev

node_modules/:
	npm install

.PHONY=build
build:
	cargo tauri build --bundles=deb,rpm --verbose

.PHONY=build-run
run: build
	WEBKIT_DISABLE_DMABUF_RENDERER=1 ./src-tauri/target/release/musigym
