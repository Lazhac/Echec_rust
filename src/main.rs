pub mod structures;
use crate::structures::Plateau;


fn plateau_vide () -> Plateau{
    Plateau {
        carte: [[' '; 8]; 8],
        en_passant_b: [false; 8],
        en_passant_n: [false; 8],
        rock_b: [false; 2],
        rock_n: [false; 2],
        nb_tour: 0
    }
}


fn main() {
    let  _p = plateau_vide();
    
}

