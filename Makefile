build-core:
	npm run build:core

watch-rs: 
	npm run watch:rs

build: build-core watch-rs

run: 
	npm run prod