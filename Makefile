all:
	cargo build --release

test:
	cargo test --release -- --nocapture

size13:
	cargo run --release --features size13
size15:
	cargo run --release --features size15
size17:
	cargo run --release --features size17

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: all clean fclean re
