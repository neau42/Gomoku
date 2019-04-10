#[cfg(test)]
mod tests {
    use crate::models::gameboard::*;
    use crate::controllers::game::print_all_values;
    use crate::models::ia::*;
    use std::time::Instant;
    use std::collections::HashMap;

    macro_rules! negascout_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut gameboard = Gameboard::new();
                let depth = $value as u8;
                let mut ia = IA::new(depth);
    			let mut all_values: Vec<(usize, usize, isize)> = Vec::new();
    			let mut map_board: HashMap<[u64; SIZE], isize> = HashMap::new();
                let stone = WHITE;
                let timer = Instant::now();
                gameboard.make_move(9,9, BLACK);
                println!("time for make first move = {:?}", timer.elapsed());
                ia.negascout(&mut gameboard, stone, ia.depth, (std::i64::MIN + 1) as isize, std::i64::MAX as isize,&mut map_board, &mut all_values, stone);
                let best_move = gameboard.selected_move.unwrap();
                println!("time for apply negascout search whith {}-depth= {:?}", depth, timer.elapsed());
                println!("number of negascout call whith {}-depth= {:?}", depth, ia.counter);
                // print_all_values(&gameboard.cells, &all_values);
                // gameboard.make_move(best_move.0, best_move.1, BLACK);
                // println!("time = {:?}", timer.elapsed());
                // println!("gameboard = {:?}", best_move);
                // println!("----------------------------");

            }
        )*
        }
    }

    negascout_tests! {
        negascout_1: (1),
        negascout_2: (2),
        negascout_3: (3),
        negascout_4: (4),
        negascout_5: (5),
        negascout_6: (6),
        negascout_7: (7),
        negascout_8: (8),
        negascout_9: (9),
        negascout_10: (10),
    }
}
