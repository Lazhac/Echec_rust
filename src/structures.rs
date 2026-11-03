pub struct Plateau {
    pub carte: [[char; 8]; 8], //blanc : minuscule - noir: majuscule
    pub en_passant_b: [bool; 8],
    pub en_passant_n: [bool; 8],
    pub rock_b: [bool; 2],
    pub rock_n: [bool; 2],
    pub nb_tour: u32,
}
//pion : 

pub struct Move {
    pub joueur: bool,
    pub ancien: (usize,usize), // ancienne position
    pub nouveau: (usize,usize),// nouvelle position
    pub rock: (bool, bool),
    pub manger: (usize,usize), // position du pion mangé (surtout si en passant)
    pub en_passant: [bool; 8],
}

