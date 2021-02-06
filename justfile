usage:
	@just -l
init-project +PROBLEM:
    rsync -av base/ {{PROBLEM}} --exclude target --exclude .git
    ln -s `PWD`/base/target ./{{PROBLEM}}/target
test:
    cd {{invocation_directory()}};cargo clean -p base
    cd {{invocation_directory()}};cargo test
