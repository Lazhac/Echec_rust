pub mod structures;
pub mod affichage;
pub mod divers;
pub mod mouvements;

use crate::structures::Plateau;
use crate::affichage::afficher_plateau;







fn main() {
    mouvements::test_couleur_piece();

    let mut p: Plateau = divers::plateau_vide();
    afficher_plateau(&p);

    p.carte[5][1] = 'f';

    let m_vec = mouvements::fou(&p, (5,1));
    affichage::afficher_mouv(&p, &m_vec);
    
}

