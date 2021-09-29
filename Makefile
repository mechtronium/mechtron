
the-docs:
	cd docs && hugo -D 
	skaffold -f skaffold-docs.yaml run 


install: 
	cd rust/starlane && cargo install --path .


