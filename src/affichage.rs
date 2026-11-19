use crate::structures::Plateau;
use crate::structures::Move;
use crate::divers;

pub fn afficher_plateau (p: &Plateau){
    println!("Tour du joueur {}", (p.nb_tour % 2));

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




// Affiche le plateau p, en marquant les cases destination (nouveau) de deplacements 
pub fn afficher_mouv (p: &Plateau, deplacements: &Vec<Move>){
    // 🆃🅲🅵🅳🆁 🅿 
    // Il n'y a pas de version minuscule, donc les lettres inversées 
    // montrent toujours les pièces adverses
    println!("{}", deplacements.len());
    print!(" +");
    for _ in 1..18 {
        print!("-");
    }

    println!("+");
    for i in 0..8 {
        print!("{}| ", 8-i);
        for j in 0..8 {
            let c: char = p.carte[7-i][j];
            if divers::est_dans_mouv((7-i, j), deplacements) {
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



