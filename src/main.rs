
use game::Rustvaders;

pub mod libsdl;
pub mod game;





fn main() {
   
   let mut game = Rustvaders::new(800, 600);
        
    game.mainloop();
}
