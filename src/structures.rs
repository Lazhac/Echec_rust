struct Plateau {
    carte: [[char; 8]; 8],
    en_passant_b: [bool; 8],
    en_passant_n: [bool; 8],
    rock_b: [bool; 2],
    rock_n: [bool; 2],
    nb_tour: u32
}

struct Move {
    joueur: bool
    ancien: (u8,u8), // ancienne position
    nouveau: (u8,u8),// nouvelle position
    rock: (bool, bool),
    manger: (u8,u8), // position du pion mangé (surtout si en passant)
    en_passant: [bool; 8]
}