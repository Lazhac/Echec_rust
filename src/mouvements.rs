use crate::structures::Plateau;
use crate::structures::Move;

pub fn mouv (p: &mut Plateau, m: Move){
    if m.joueur {
        p.en_passant_b = [false; 8];
        p.en_passant_n = m.en_passant;
        p.rock_b[0] = p.rock_b[0] && m.rock.0;
        p.rock_b[1] = p.rock_b[1] && m.rock.1;
    } else {
        p.en_passant_n = [false; 8];
        p.en_passant_b = m.en_passant;
        p.rock_n[0] = p.rock_n[0] && m.rock.0;
        p.rock_n[1] = p.rock_n[1] && m.rock.1;
    }
    p.nb_tour = p.nb_tour + 1;
    p.carte[m.manger.0][m.manger.1] = ' ';
    p.carte[m.nouveau.0][m.nouveau.1] = p.carte[m.ancien.0][m.ancien.1];
    p.carte[m.ancien.0][m.ancien.1] = ' ';
}




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

fn mvt_tour(plt: &Plateau, pos: (usize,usize)) {
    let c = plt.carte[pos.0][pos.1];
    assert!(couleur_piece(c) == (plt.nb_tour % 2)); //c'est le bon joueur qui joue
}//pour connaître le joueur, on regarde le tour de jeu - si pair blanc, si impair noir

//tests
pub fn test_couleur_piece(){
    let c1:char =' ';
    let c2:char ='p';
    let c3:char ='Q';
    let mut res:u32 = couleur_piece(c1);
    assert!(res==2); // attendu : 2
    res = couleur_piece(c2);
    assert!(res==0); // attendu : 0
    res = couleur_piece(c3);
    assert!(res==1); // attendu : 1
}

// test si pos est dans les limites de la carte
fn dans_carte(pos: (isize,isize)) -> bool {
    pos.0 >= 0 && pos.0 < 8 && pos.1 >= 0 && pos.1 < 8 
}

// Renvoie l'adversaire de j
fn autre_joueur (j: u32) -> u32 {
    (1- (j as i32)) as u32
}

// Renvoie l'ensemble des moves possible par le cavalier en position pos.
pub fn cavalier (p: &Plateau, pos: (usize,usize)) -> Vec<Move> {
    mvt_tour(p, pos);
    assert!(p.carte[pos.0][pos.1] == 'c' || p.carte[pos.0][pos.1] == 'C');

    let joueur = p.nb_tour % 2;

    let mut res: Vec<Move> = Vec::with_capacity(8);

    let possibilites: [(isize,isize); 8] = [(2,1), (2,-1), (1,2), (1,-2), (-1,2), (-1,-2), (-2,1), (-2,-1)];
    for (i, j) in possibilites {
        if dans_carte(((pos.0 as isize) + i, (pos.1 as isize) + j)) 
        && couleur_piece(p.carte[((pos.0 as isize)+i) as usize][((pos.1 as isize)+j) as usize]) != joueur {
            res.push(
                 Move {
                    joueur: joueur == 0,
                    ancien: pos, // ancienne position
                    nouveau: (((pos.0 as isize)+i) as usize, ((pos.1 as isize)+j) as usize),// nouvelle position
                    rock: (true, true),
                    manger: (((pos.0 as isize)+i) as usize, ((pos.1 as isize)+j) as usize), // position du pion mangé (surtout si en passant)
                    en_passant: [true; 8],
                }
            );
        }
    }   

    res
}


// Renvoie l'ensemble des moves possible par le fou en position pos.
pub fn fou (p: &Plateau, pos: (usize,usize)) -> Vec<Move> {
    mvt_tour(p, pos);
    assert!(p.carte[pos.0][pos.1] == 'f' || p.carte[pos.0][pos.1] == 'F');

    let joueur = p.nb_tour % 2;

    let mut res: Vec<Move> = Vec::with_capacity(8);

    
    for (signe_i, signe_j) in [(1, 1), (1, -1), (-1, 1), (-1, -1)]{    
        for k in 1..8 {
            if dans_carte(((pos.0 as isize) + (k*signe_i), (pos.1 as isize) + (k*signe_j))) {
                if couleur_piece(p.carte[((pos.0 as isize)+(k*signe_i)) as usize][((pos.1 as isize)+(k*signe_j)) as usize]) == joueur {break;}
                else if couleur_piece(p.carte[((pos.0 as isize)+(k*signe_i)) as usize][((pos.1 as isize)+(k*signe_j)) as usize]) == autre_joueur(joueur) {
                    res.push(
                        Move {
                            joueur: joueur == 0,
                            ancien: pos, // ancienne position
                            nouveau: (((pos.0 as isize)+(k*signe_i)) as usize, ((pos.1 as isize)+(k*signe_j)) as usize),// nouvelle position
                            rock: (true, true),
                            manger: (((pos.0 as isize)+(k*signe_i)) as usize, ((pos.1 as isize)+(k*signe_j)) as usize), // position du pion mangé (surtout si en passant)
                            en_passant: [true; 8],
                        }
                    );
                    break;
                }
                else {
                    res.push(
                        Move {
                            joueur: joueur == 0,
                            ancien: pos, // ancienne position
                            nouveau: (((pos.0 as isize)+(k*signe_i)) as usize, ((pos.1 as isize)+(k*signe_j)) as usize),// nouvelle position
                            rock: (true, true),
                            manger: (((pos.0 as isize)+(k*signe_i)) as usize, ((pos.1 as isize)+(k*signe_j)) as usize), // position du pion mangé (surtout si en passant)
                            en_passant: [true; 8],
                        }
                    );
                }
            }
        }   
    }
    res
}










