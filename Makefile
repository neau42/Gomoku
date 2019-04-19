# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: no <no@student.42.fr>                      +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2019/04/19 05:30:12 by no                #+#    #+#              #
#    Updated: 2019/04/19 05:32:03 by no               ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

all:
	cargo build --release

run: 
	cargo run --release

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
