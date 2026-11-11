pub mod structures;
pub mod affichage;
pub mod divers;
pub mod mouvements;


use crate::{structures::Plateau, structures::Move};
use crate::mouvements::couleur_piece;
use crate::affichage::afficher_plateau;


use std::io::{self, BufRead};





fn main() {
    mouvements::test_couleur_piece();

    let mut p: Plateau = divers::plateau_vide();
    afficher_plateau(&p);

    let stdin = io::stdin();
    let mut selection = true; // true : on choisi la piece, false : on choisi la case où la mettre
    let mut m_vec: Vec<Move> = Vec::new();

    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let pos_opt = divers::conv_str_to_pos(l);
        match pos_opt {
            None => {
                     if selection {println!("Position non valide");}
                     else {selection = true}
                    },

            Some((i, j)) => {
                println!("{i}, {j}");
                if selection {
                    if couleur_piece(p.carte[i][j]) != (p.nb_tour % 2) {println!("Position non valide");}
                    else {
                        match mouvements::COUPS.get(&p.carte[i][j]) {
                            None => println!("Position non valide"),
                            Some(f) => {
                                m_vec = f(&p, (i,j));
                                affichage::afficher_mouv(&p, &m_vec);
                                selection = false;
                            }
                        };
                    }
                } else {
                    for m in m_vec.clone().into_iter() {
                        if (i,j) == m.nouveau {
                            mouvements::mouv(&mut p, m);  
                        }
                    }
                    selection = true;
                    affichage::afficher_plateau(&p); 
                }
                
            }
        }

    }
}

