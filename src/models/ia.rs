pub struct IA {
    pub depth: u8,
}

impl IA {
    pub fn new(depth: u8) -> IA {
        IA {
            depth,
        }
    }
}

    // pub fn alpha_beta_with_memory(&self, state: &mut Gameboard, depth: usize, mut alpha: isize, mut beta: isize) -> (isize, Option<Gameboard>) {
	// 	let mut bestmove: Option<Gameboard> = None;
	// 	if self.all_state.contains(state) {
    //         if state.lowerbound >= beta {
    //             return (state.lowerbound, Some(state.clone()));
    //         }
    //         if state.upperbound <= alpha {
    //             return (state.upperbound, Some(state.clone()));
    //         }
    //         alpha = max(alpha, state.lowerbound);
    //         beta = min(beta, state.upperbound);
    //     }
    //     if state.is_finish_state() || depth <= 0 {
    //         return /*winning score or*/ (state.eval(), Some(state.clone()));
    //     }
    //     // move bestmove ;
    //     let mut current = isize::from(std::i16::MIN);
    //     //for (each possible move m
    //     for mut new_state in state.expand(self.current_stone) {
    //         // make move m;
    //         let (mut score, tmp_state) = self.alpha_beta_with_memory(&mut new_state, depth - 1, -beta, -alpha);
	// 		score = -score;
    //         // unmake move m;
    //         if score >= current {
    //             current = score;
    //             bestmove = Some(new_state);
    //             if score >= alpha {
    //                 alpha = score;
    //                 if score >= beta {
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     if current <= alpha {
    //         state.upperbound = current;
    //     }
    //     if current >= beta {
    //         state.lowerbound = current;
    //     }
    //     return (current, bestmove);
    // }

    // pub fn mdtf(&mut self, mut g: isize, depth: usize) -> (isize, Option<Gameboard>) { //On utilise donc en général comme valeur de g la valeur retourné par l’algorithme lors d’une itération précédente
    //     let mut upperbound = isize::from(std::i16::MAX);
    //     let mut lowerbound = isize::from(std::i16::MIN);
	// 	let mut bestmove: Option<Gameboard> = None;
    //     while lowerbound != upperbound {
    //         let beta: isize = match lowerbound {
    //             g => g + 1,
    //             _ => g,
    //         };
    //         let (tmp_g, tmp) = self.alpha_beta_with_memory(&mut self.state.clone(), depth, beta - 1, beta);
	// 		bestmove = tmp;
	// 		g = tmp_g;
    //         if g < beta {
    //             upperbound = g;
    //         }
    //         else {
    //             lowerbound = g;
    //         }
    //     }
    //     (g, bestmove)
    // }