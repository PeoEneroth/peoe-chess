    #[derive(PartialEq, Clone)]
    pub struct SchackSpel {
        turn: u64,
        board_state: [[Option<Piece>; 8]; 8],
        white_king_position: (usize, usize),
        black_king_position: (usize, usize),
        //black_pieces: Vec<Piece>,
        //white_pieces: Vec<Piece>,
    }



struct Possible_Move {
        destination: (usize, usize),
        kill_move: bool,
        trip: Option<(usize, usize)>,
        start: (usize, usize),
        is_horse: bool,
        castle: Option<((usize, usize), (usize, usize))>,
    }

 #[derive(PartialEq, Clone, Copy, Debug)]
    enum RangeCap {
        uncapped,
        capped(u64),
    }

    #[derive(PartialEq, Clone, Copy, Debug)]
    enum PieceType {
        king,
        pawn,
        knight,
        queen,
        tower,
        horse,
    }

    #[derive(PartialEq, Clone, Copy,Debug)]
    enum Color {
        black,
        white,
    }

    #[derive(PartialEq, Clone,Debug)]
    pub struct Direction {
        x: i64,
        y: i64,
    }

    

    
    #[derive(PartialEq, Clone, Debug)]
    pub struct Piece {
        piece_type: PieceType,
        color: Color,
        moving_directions: Vec<Direction>,
        kill_directions: Vec<Direction>,
        range_cap: RangeCap,
        number_of_moves: u64,
        has_double_jumped: bool,
    }

