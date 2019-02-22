all:
	cargo build --release

test:
	cargo test --release -- --nocapture

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: all clean fclean re
