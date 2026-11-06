pub mod structures;
pub mod affichage;
pub mod divers;

use crate::structures::Plateau;
use crate::affichage::afficher_plateau;







fn main() {
    let  p = divers::plateau_vide();
    afficher_plateau(p);
    
}

