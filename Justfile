dev:
    if test ! -d node_modules; then \
        npm install; \
    fi
    WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev

build:
	cargo tauri build --bundles=deb,rpm --verbose

run: build
	WEBKIT_DISABLE_DMABUF_RENDERER=1 ./src-tauri/target/release/musigym
