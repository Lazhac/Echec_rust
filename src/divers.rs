use crate::structures::Plateau;

// initialise un plateau de début de partie
pub fn plateau_vide () -> Plateau{
    let mut p: Plateau = Plateau {
        carte: [[' '; 8]; 8],
        en_passant_b: [false; 8],
        en_passant_n: [false; 8],
        rock_b: [true; 2],
        rock_n: [true; 2],
        nb_tour: 0
    };
    p.carte[0] = ['t', 'c', 'f', 'd', 'r', 'f', 'c', 't'];
    p.carte[1] = ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'];
    p.carte[7] = ['T', 'C', 'F', 'D', 'R', 'F', 'C', 'T'];
    p.carte[6] = ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'];

    p
}



pub fn conv_str_to_pos (pos_string: String) -> Option<(usize, usize)> {
    if pos_string.len() != 2 {
        None
    } else {
        let j_char = pos_string.chars().nth(0).unwrap();
        let i_char = pos_string.chars().nth(1).unwrap();

        let i_opt = i_char.to_digit(10);
        let i: usize = match i_opt {
            Some(x) => (x-1) as usize,
            None => 10
        };

        let j: usize = 
            if j_char == 'A' {0}
            else if j_char == 'B' {1}
            else if j_char == 'C' {2}
            else if j_char == 'D' {3}
            else if j_char == 'E' {4}
            else if j_char == 'F' {5}
            else if j_char == 'G' {6}
            else if j_char == 'H' {7}
            else {10};

        if (i == 10) || (j == 10) {
            None
        }
        else {
            Some((i,j))
        }
        
    }
}