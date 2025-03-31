.PHONY=run
run: node_modules/
	WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev

node_modules/:
	npm install
