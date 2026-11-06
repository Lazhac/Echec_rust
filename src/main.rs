pub mod structures;
pub mod affichage;
pub mod divers;
pub mod mouvements;

use crate::mouvements::cavalier;
use crate::{mouvements::couleur_piece, structures::Plateau};
use crate::affichage::afficher_plateau;


use std::io::{self, BufRead};





fn main() {
    mouvements::test_couleur_piece();

    let mut p: Plateau = divers::plateau_vide();
    afficher_plateau(&p);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let pos_opt = divers::conv_str_to_pos(l);
        match pos_opt {
            None => println!("Position non valide"),
            Some((i, j)) => {
                println!("{i}, {j}");
                if couleur_piece(p.carte[i][j]) != (p.nb_tour % 2) {println!("Position non valide");}
                else {
                    let m_vec = 
                        if p.carte[i][j].to_ascii_lowercase() == 'c' {mouvements::cavalier(&p, (i,j))}
                        else {mouvements::fou(&p, (i,j))};
                    
                    affichage::afficher_mouv(&p, &m_vec);

                }
            }
        }

    }
}

