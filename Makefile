.PHONY: test install_test_deps

test:
	mkdir -p cover | true
	rm cover/* | true
	LLVM_PROFILE_FILE=cover/ex-scylla-driver.profraw MIX_ENV=test mix test --cover
	grcov cover/ex-scylla-driver.profraw -s ./native/ex_scylla -b ./_build/test/lib/ex_scylla/priv/native -t lcov --branch --ignore-not-existing --ignore "$$HOME/.cargo/*" -o cover/rust-lcov.info
	cat cover/lcov.info >> cover/total-lcov.info
	cat cover/rust-lcov.info >> cover/total-lcov.info
	lcov --rc branch_coverage=1 --ignore-errors mismatch --summary cover/total-lcov.info

install_test_deps:
	cargo install grcov
	rustup component add llvm-tools-preview