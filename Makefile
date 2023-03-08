
run:
	cargo build
	cargo test
	g++ -std=c++17 main.cpp -o main -l cpp_bincode_issue -L ./target/debug
	./main
