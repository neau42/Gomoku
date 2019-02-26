mod controllers;

use controllers::gameplay::*;


fn main() {
    gameplay: GameplayController = GameplayController::new();

    match gameplay.init() {
        Ok(_) => gameplay.run_loop(),
        Err(error) => println!("Error : {}", error),
    }
}
