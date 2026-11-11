use crate::structures::Plateau;
use crate::structures::Move;
use crate::divers::conv_str_to_pos;
use std::collections::HashMap;
use std::sync::LazyLock;

pub static COUPS :LazyLock<HashMap<char,fn(&Plateau,(usize,usize)) ->Vec<Move>>> = LazyLock::new(|| 
    {
        HashMap::from([
            ('t', tour as fn(&Plateau, (usize, usize)) -> Vec<Move>),
            ('c', cavalier as _),
            ('f', fou as _),
            ('d', dame as _),
            //('r', roi as _),
            // ('p', pion as _),
            ('T', tour as _),
            ('C', cavalier as _),
            ('F', fou as _),
            ('D', dame as _),
            // ('R', roi as _),
            // ('P', pion as _),
        ])
    }
    );

// Applique simplement le coups, ne vérifie pas la victoire, l'égalité ou quoi que ce soit d'autre
pub fn mouv (p: &mut Plateau, m: Move){
    if m.joueur { //si c'est au joueur blanc de jouer
        p.en_passant_b = [false; 8]; // on ne peut faire en passant que immédiatement après avoir joué
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

//pour connaître le joueur, on regarde le tour de jeu - si pair blanc, si impair noir

// test si pos est dans les limites de la carte
fn dans_carte(pos: (isize,isize)) -> bool {
    pos.0 >= 0 && pos.0 < 8 && pos.1 >= 0 && pos.1 < 8 
}

// Renvoie l'adversaire de j
fn autre_joueur (j: u32) -> u32 {
    (1- (j as i32)) as u32
}

// Renvoie l'ensemble des moves possible par le cavalier en position pos.
fn cavalier (p: &Plateau, pos: (usize,usize)) -> Vec<Move> {
    assert!(p.carte[pos.0][pos.1] == 'c' || p.carte[pos.0][pos.1] == 'C');
    assert!(couleur_piece(p.carte[pos.0][pos.1]) == (p.nb_tour % 2)); //c'est le bon joueur qui joue

    let joueur = p.nb_tour % 2;

    let mut res: Vec<Move> = Vec::with_capacity(8);

    let possibilites: [(isize,isize); 8] = [(2,1), (2,-1), (1,2), (1,-2), (-1,2), (-1,-2), (-2,1), (-2,-1)];
    for (i, j) in possibilites {
        if dans_carte(((pos.0 as isize) + i, (pos.1 as isize) + j)) 
        && couleur_piece(p.carte[((pos.0 as isize)+i) as usize][((pos.1 as isize)+j) as usize]) != joueur {
            res.push(
                 Move {
                    joueur: joueur == 0, //pourquoi cette ligne ?
                    ancien: pos, // ancienne position
                    nouveau: (((pos.0 as isize)+i) as usize, ((pos.1 as isize)+j) as usize),// nouvelle position
                    rock: (true, true),
                    manger: (((pos.0 as isize)+i) as usize, ((pos.1 as isize)+j) as usize), // position du pion mangé (surtout si en passant)
                    en_passant: [true; 8], //pourquoi en passant à true ?
                }
            );
        }
    }   

    res
}


// Renvoie l'ensemble des moves possible par le fou en position pos.
fn fou (p: &Plateau, pos: (usize,usize)) -> Vec<Move> {
    assert!(couleur_piece(p.carte[pos.0][pos.1]) == (p.nb_tour % 2));
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

fn tour (p: &Plateau, pos: (usize,usize)) -> Vec<Move> {
    let c = p.carte[pos.0][pos.1];
    assert!(couleur_piece(c) == (p.nb_tour % 2)); //c'est le bon joueur qui joue
    assert!(p.carte[pos.0][pos.1] == 't' || p.carte[pos.0][pos.1] == 'T');

    let configs :[(isize,isize);4] = [(1,0),(0,-1),(-1,0),(0,-1)];
    let mut res: Vec<Move> = Vec::new();
    let mut x:isize;
    let mut y:isize;

    //mise a jour du rock (on regarde juste si la tour est à son emplacement de départ)
    let rock :(bool,bool) =
    if (p.nb_tour % 2) == 0 {
        if pos == conv_str_to_pos("A1".to_string()).unwrap() {(false,true)}
        else if pos == conv_str_to_pos("H1".to_string()).unwrap() {(true,false)}
        else {(true,true)}
    }
    else {
        if pos == conv_str_to_pos("A8".to_string()).unwrap() {(false,true)}
        else if pos == conv_str_to_pos("H8".to_string()).unwrap() {(true,false)}
        else {(true,true)}
    };

    for (i,j) in configs { //on avance tant que l'on peut dans toutes les dirrections
        x = (pos.0 as isize) + i;
        y = (pos.1 as isize) + j;
        while dans_carte((x,y)) && (p.carte[x as usize][y as usize] == ' ') { //tant qu'il sagt de cases vides
            res.push (
                    Move {
                            joueur: (p.nb_tour % 2) == 0,   // true = blanc, false = noir 
                            ancien: pos, // ancienne position
                            nouveau: (x as usize,y as usize),// nouvelle position
                            rock: (true,true), // on mets à false les positions (coté 0 ou 7) qui sont rendu impossible par ce coup
                            manger: (x as usize,y as usize), // position du pion mangé (surtout si en passant). Si rien n'est manger, manger=nouveau
                            en_passant: [false;8],
                    }
            );
            x = x + i;
            y = y + j;
        // si on est sortis de la boucle et encore sur le plateau => on a rencontré un pion à manger (coup à ajouter)
        if dans_carte((x,y)) {
            res.push(
                Move {
                    joueur: (p.nb_tour % 2) == 0,
                    ancien: pos,
                    nouveau: (x as usize,y as usize),
                    rock: (true,true),
                    manger: (x as usize,y as usize),
                    en_passant: [false;8]
                }
            )
        }
        }
    }
    return res;
}

fn dame (p: &Plateau, pos: (usize,usize)) -> Vec<Move> {
    let c = p.carte[pos.0][pos.1];
    assert!(couleur_piece(c) == (p.nb_tour % 2)); //c'est le bon joueur qui joue
    assert!(p.carte[pos.0][pos.1] == 'd' || p.carte[pos.0][pos.1] == 'D');

    let configs :[(isize,isize);8] = [(1,1),(1,-1),(-1,1),(-1,-1),(1,0),(0,1),(-1,0),(0,-1 )];
    let mut res: Vec<Move> = Vec::new();
    let mut x:isize;
    let mut y:isize;

    //mise a jour du rock (on regarde juste si la tour est à son emplacement de départ)

    for (i,j) in configs { //on avance tant que l'on peut dans toutes les dirrections
        x = (pos.0 as isize) + i;
        y = (pos.1 as isize) + j;
        while dans_carte((x,y)) && (p.carte[x as usize][y as usize] == ' ') { //tant qu'il sagt de cases vides
            res.push (
                    Move {
                            joueur: (p.nb_tour % 2) == 0,   // true = blanc, false = noir 
                            ancien: pos, // ancienne position
                            nouveau: (x as usize,y as usize),// nouvelle position
                            rock: (true,true), // on mets à false les positions (coté 0 ou 7) qui sont rendu impossible par ce coup
                            manger: (x as usize,y as usize), // position du pion mangé (surtout si en passant). Si rien n'est manger, manger=nouveau
                            en_passant: [false;8],
                    }
            );
            x = x + i;
            y = y + j;
        // si on est sortis de la boucle et encore sur le plateau => on a rencontré un pion à manger (coup à ajouter)
        if dans_carte((x,y)) {
            res.push(
                Move {
                    joueur: (p.nb_tour % 2) == 0,
                    ancien: pos,
                    nouveau: (x as usize,y as usize),
                    rock: (true,true),
                    manger: (x as usize,y as usize),
                    en_passant: [false;8]
                }
            )
        }
        }
    }
    return res;
}

//récupère tout les pions de joueur sauf le roi
fn loc_pions_saufroi(p: &Plateau,joueur :u32) -> Vec<(usize,usize)> { 
    let mut res :Vec<(usize,usize)> = Vec::new();
    for x in 0..=7 {
        for y in 0..=7 {
            if joueur == 0 && ['p','f','c','t','d'].contains(&p.carte[x][y]) {
                res.push((x as usize,y as usize));
            }
            else if joueur == 1 && ['P','F','C','T','D'].contains(&p.carte[x][y]) {
                res.push((x as usize,y as usize));
            }
        }
    }
    res 
}

/* 
fn roi (p: &Plateau, pos: (usize,usize)) -> Vec<Move> {
    let c = p.carte[pos.0][pos.1];
    let joueur = p.nb_tour % 2
    assert!(couleur_piece(c) == (joueur)); //c'est le bon joueur qui joue
    assert!(p.carte[pos.0][pos.1] == 'r' || p.carte[pos.0][pos.1] == 'R');

    //recuperation des coups de tout les pions adverses
    let positions :vec<(usize,usize)> = loc_pions_saufroi(p,autre_joueur(joueur));
    let adversaires :vec<Move> = Vec::new();
    for i in 0..position.len() {
        adversaire.append()
    }

    let configs :[(isize,isize);8] = [(1,1),(1,-1),(-1,1),(-1,-1),(1,0),(0,1),(-1,0),(0,-1 )];
    let mut res: Vec<Move> = Vec::new();
    let mut x:isize;
    let mut y:isize;


    for (i,j) in configs { //on avance tant que l'on peut dans toutes les dirrections
        x = (pos.0 as isize) + i;
        y = (pos.1 as isize) + j;
        if dans_carte((x,y)) { //pas à se soucier de la présence ou non d'un pion 
        res.push(
            Move {
                joueur: joueur == 0,
                ancien: pos,
                nouveau: (x as usize,y as usize),
                rock: (true,true),
                manger: (x as usize,y as usize),
                en_passant: [false;8]
            }

        )
        }
        if (p.nb_tour % 2) == 0 {
            if pos == conv_str_to_pos("E8".to_string()).unwrap() { // roi à la bonne place
                if 
            }
        }
        }
    return res;
}*/

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







