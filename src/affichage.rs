use crate::structures::Plateau;

pub fn afficher_plateau (p: Plateau){
    print!("+");
    for _ in 1..18 {
        print!("-");
    }

    println!("+");
    for i in 0..8 {
        print!("| ");
        for j in 0..8 {
            if (p.carte[7-i][j] == ' ') && ((i+j) %2 == 1){
                print!("• "); // ·
            } else {
                print!("{} ", p.carte[7-i][j]);
            }
        }
        println!("|");
    }

    print!("+");
    for _ in 1..18 {
        print!("-");
    }
    println!("+");
}