use crate::structures::Plateau;
use crate::structures::Move;

pub fn couleur_piece(c:char) -> u32 {
    //renvoie 0 si c'est une pièce qui appartient au joueur blanc, 1 au joueur noir, 2 sinon
    if c == ' '{ //cas d'erreur - il n'y a pas de pion sur la case
        2
    }
    else if c.is_ascii_lowercase(){
        0
    }
    else {
        1
    }
}

pub fn mvt_tour(plt: &Plateau, pos: (usize,usize)) {
    let c = plt.carte[pos.0][pos.1];
    assert!(couleur_piece(c) == (plt.nb_tour % 2)); //c'est le bon joueur qui joue
}//pour connaître le joueur, on regarde le tour de jeu - si pair blanc, si impair noir

//tests
pub fn test_mvt_couleur_piece(){
    let c1:char =' ';
    let c2:char ='p';
    let c3:char ='Q';
    let mut res:u32 = couleur_piece(c1);
    println!("{res}"); // attendu : 2
    res = couleur_piece(c2);
    println!("{res}"); // attendu : 0
    res = couleur_piece(c3);
    println!("{res}"); // attendu : 1
}










