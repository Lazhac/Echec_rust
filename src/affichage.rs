use crate::structures::Plateau;
use crate::structures::Move;

pub fn afficher_plateau (p: &Plateau){
    print!(" +");
    for _ in 1..18 {
        print!("-");
    }

    println!("+");
    for i in 0..8 {
        print!("{}| ", 8-i);
        for j in 0..8 {
            if (p.carte[7-i][j] == ' ') && ((i+j) %2 == 1){
                print!("• "); // ·
            } else {
                print!("{} ", p.carte[7-i][j]);
            }
        }
        println!("|");
    }

    print!(" +");
    for _ in 1..18 {
        print!("-");
    }
    println!("+");

    print!("  ");
    for c in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        print!(" {c}");
    }
    println!("\n");
}


// Test si pos fait partie des destinations de m_vec
fn est_dans_mouv (pos: (usize, usize), m_vec: &Vec<Move>) -> bool{
    for m in m_vec {
        if pos == m.nouveau {return true;}
    }
    false
}


// Affiche le plateau p, en marquant les cases destination (nouveau) de deplacements 
pub fn afficher_mouv (p: &Plateau, deplacements: &Vec<Move>){
    // 🆃🅲🅵🅳🆁 🅿 
    // Il n'y a pas de version minuscule, donc les lettres inversées 
    // montrent toujours les pièces adverses
    print!(" +");
    for _ in 1..18 {
        print!("-");
    }

    println!("+");
    for i in 0..8 {
        print!("{}| ", 8-i);
        for j in 0..8 {
            let c: char = p.carte[7-i][j];
            if est_dans_mouv((7-i, j), deplacements) {
                if c == ' ' {print!("█ ");}
                else if c.to_lowercase().to_string() == "t" {print!("🆃 ");}
                else if c.to_lowercase().to_string() == "c" {print!("🅲 ");}
                else if c.to_lowercase().to_string() == "f" {print!("🅵 ");}
                else if c.to_lowercase().to_string() == "d" {print!("🅳 ");} 
                else if c.to_lowercase().to_string() == "r" {print!("🆁 ");}
                else if c.to_lowercase().to_string() == "p" {print!("🅿 ");}
            }
            else if (c == ' ') && ((i+j) %2 == 1){
                print!("• "); // ·
            } else {
                print!("{} ", c);
            }
        }
        println!("|");
    }

    print!(" +");
    for _ in 1..18 {
        print!("-");
    }
    println!("+");

    print!("  ");
    for c in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        print!(" {c}");
    }
    println!("\n");
}



