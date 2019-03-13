//! Gomoku!
//! 
#[macro_use] extern crate conrod_derive;
mod models;
mod views;
mod controllers;
mod utils;
mod widgets;
mod traits;

use controllers::gameplay::*;
use conrod::*;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1024;

widget_ids! {
    pub struct WidgetIds {
        background,
        title,
        window_canvas,
        game_builder_canvas,
        dropdown_button_game_mode,
        toggle_button_weight_boxes,
        number_dialer_first_ia_depth,
        number_dialer_second_ia_depth,
        button_start,
		grid,
        text_turn,
        text_captures,
        text_last_move_time,
        button_quit,
        button_undo,
    }
}

fn main() {
    let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let widget_ids = WidgetIds::new(ui.widget_id_generator());

    let mut gameplay: GameplayController = GameplayController::new(WIDTH, HEIGHT, ui, widget_ids);

    gameplay.run()
}







// http://www.ffothello.org/informatique/algorithmes/

// Fig. 4 : fail-soft alpha-beta

// int alphabêta(int depth, int alpha, int bêta)
// {
//    if (game over or depth <= 0)
//       return winning score or eval();
//    move bestMove ;
//    int current = -INFINITY;
//    for (each possible move m) {
//       make move m;
//       int score = -alphabêta(depth - 1, -bêta, -alpha)
//       unmake move m;
//       if (score >= current) {
//          current = score;
//          bestmove = m;
//          if (score >= alpha){
//             alpha = score;
//             bestmove = m ;
//             if (score >= bêta)
//                break;
//          }
//       }
//    }
//    return current;
// }


// si alpha < current < bêta, alors current est la valeur minimax
// si current <= alpha, alors la vraie valeur minimax m vérifie :
// m <= current <= alpha
// si bêta <= current alors la vraie valeur minimax m vérifie :
// bêta <= current <= m


// int alphabêta(int depth, int alpha, int bêta)
// {
//    if (game over or depth <= 0)
//       return winning score or eval();
//    move bestMove = first move;
//    make move bestMove;
//    int current = -alphabêta(depth - 1, -bêta, -alpha);
//    unmake move bestMove;
//    if (current >= alpha)
//       alpha = current;
//    if (current < bêta) {
//       for (each remaining move m) {
//          make move m;
//          int score = -alphabêta(depth - 1, -(alpha+1), -alpha)
//          if (score > alpha && score < bêta)
//             score = -alphabêta(depth - 1, -bêta, -alpha)
//          unmake move m;
//          if (score >= current) {
//             current = score;
//             bestmove = m;
//             if (score >= alpha){
//                alpha = score;
//                if (score >= bêta)
//                   break;
//             }
//          }
//    }
//    return current;
// }

// si alpha < current < bêta, alors current est la valeur minimax
// si current <= alpha, alors la vraie valeur minimax m vérifie :
// m <= current <= alpha
// si bêta <= current alors la vraie valeur minimax m vérifie :
// bêta <= current <= m


// int MTDF(depth, init_g) 
// {
//    int g = init_g , bêta;
//    int upperbound = +INFINITY;
//    int lowerbound = -INFINITY;
//    do {
//       if (g == lowerbound)
//          bêta = g + 1 ;
//       else
//          bêta = g;
//       g = AlphaBêtaWithMemory(depth, bêta - 1, bêta);
//       if (g < bêta)
//          upperbound = g
//       else
//          lowerbound = g;
//       }
//    while (lowerbound != upperbound);
//    return g ;
// }


// ****************
// NegaSCiot
// int alphabêta(int depth, int alpha, int bêta)
// {
//    if (game over or depth <= 0)
//       return winning score or eval();
//    move bestMove = first move;
//    make move bestMove;
//    int current = -alphabêta(depth - 1, -bêta, -alpha);
//    unmake move bestMove;
//    if (current >= alpha)
//       alpha = current;
//    if (current < bêta) {
//       for (each remaining move m) {
//          make move m;
//          int score = -alphabêta(depth - 1, -(alpha+1), -alpha)
//          if (score > alpha && score < bêta)
//             score = -alphabêta(depth - 1, -bêta, -alpha)
//          unmake move m;
//          if (score >= current) {
//             current = score;
//             bestmove = m;
//             if (score >= alpha){
//                alpha = score;
//                if (score >= bêta)
//                   break;
//             }
//          }
//    }
//    return current;
// }