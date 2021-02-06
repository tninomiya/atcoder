usage:
	@just -l
init-project +PROBLEM:
    rsync -av base/ {{PROBLEM}} --exclude target --exclude .git
    ln -s `PWD`/base/target ./{{PROBLEM}}/target
test:
    cargo clean -p base
    cargo test
