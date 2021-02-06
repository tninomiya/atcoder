usage:
	@just -l
init-project CONTEST PROBLEM:
    mkdir -p {{CONTEST}}
    rsync -av base/ {{CONTEST}}/{{CONTEST}}-{{PROBLEM}} --exclude target --exclude .git
    ln -s `PWD`/base/target ./{{CONTEST}}/{{CONTEST}}-{{PROBLEM}}/target
test:
    cd {{invocation_directory()}};cargo clean -p base
    cd {{invocation_directory()}};cargo test
