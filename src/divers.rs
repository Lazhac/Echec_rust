use crate::structures::Plateau;

pub fn plateau_vide () -> Plateau{
    let mut p: Plateau = Plateau {
        carte: [[' '; 8]; 8],
        en_passant_b: [false; 8],
        en_passant_n: [false; 8],
        rock_b: [false; 2],
        rock_n: [false; 2],
        nb_tour: 0
    };
    p.carte[0] = ['t', 'c', 'f', 'd', 'r', 'f', 'c', 't'];
    p.carte[1] = ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'];
    p.carte[7] = ['T', 'C', 'F', 'D', 'R', 'F', 'C', 'T'];
    p.carte[6] = ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'];

    p
}