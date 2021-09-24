#[cfg(test)]
use std::any::type_name;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Display;


mod structs_and_enums;
mod struct_constructors;
mod user_interaction_functions;

//ui byggaren behöver bara interakter med max 3 funktioner.

// 1. make move . returnar boardstate efter move och ifall draget resulterar i schackmatt
// 2. get possible moves för att se möjliga drag för en pjäs på en viss position
// 3. get board state - behövs möjlgtvis inte eftersom att make_move returnerar hela spelobjektet
// 4. build_spel för att starta ett game



//fn main() {
 

    fn add_usize_and_int(u: usize, i: i64) -> i64 {
        u as i64 + i
    }

    struct Possible_Move {
        destination: (usize, usize),
        kill_move: bool,
        trip: Option<(usize, usize)>,
        start: (usize, usize),
        is_horse: bool,
        castle: Option<((usize, usize), (usize, usize))>,
    }

    #[derive(PartialEq, Clone)]
    pub struct SchackSpel {
        turn: u64,
        board_state: [[Option<Piece>; 8]; 8],
        white_king_position: (usize, usize),
        black_king_position: (usize, usize),
        //black_pieces: Vec<Piece>,
        //white_pieces: Vec<Piece>,
    }

    fn build_spel() -> SchackSpel {
        //let mut board_state : [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        let mut board_state: Vec<Vec<Option<Piece>>> = Vec::new();
        //let mut board_state: Vec<Vec<Option<Piece>>> = Vec::new();

        let mut board_state = [
            [
                Some(build_piece(PieceType::tower, Color::white)),
                Some(build_piece(PieceType::pawn, Color::white)),
                None,
                None,
                None,
                None,
                Some(build_piece(PieceType::pawn, Color::black)),
                Some(build_piece(PieceType::tower, Color::black)),
            ],
            [
                Some(build_piece(PieceType::horse, Color::white)),
                Some(build_piece(PieceType::pawn, Color::white)),
                None,
                None,
                None,
                None,
                Some(build_piece(PieceType::pawn, Color::black)),
                Some(build_piece(PieceType::horse, Color::black)),
            ],
            [
                Some(build_piece(PieceType::knight, Color::white)),
                Some(build_piece(PieceType::pawn, Color::white)),
                None,
                None,
                None,
                None,
                Some(build_piece(PieceType::pawn, Color::black)),
                Some(build_piece(PieceType::knight, Color::black)),
            ],
            [
                Some(build_piece(PieceType::queen, Color::white)),
                Some(build_piece(PieceType::pawn, Color::white)),
                None,
                None,
                None,
                None,
                Some(build_piece(PieceType::pawn, Color::black)),
                Some(build_piece(PieceType::queen, Color::black)),
            ],
            [
                Some(build_piece(PieceType::king, Color::white)),
                Some(build_piece(PieceType::pawn, Color::white)),
                None,
                None,
                None,
                None,
                Some(build_piece(PieceType::pawn, Color::black)),
                Some(build_piece(PieceType::king, Color::black)),
            ],
            [
                Some(build_piece(PieceType::knight, Color::white)),
                Some(build_piece(PieceType::pawn, Color::white)),
                None,
                None,
                None,
                None,
                Some(build_piece(PieceType::pawn, Color::black)),
                Some(build_piece(PieceType::knight, Color::black)),
            ],
            [
                Some(build_piece(PieceType::horse, Color::white)),
                Some(build_piece(PieceType::pawn, Color::white)),
                None,
                None,
                None,
                None,
                Some(build_piece(PieceType::pawn, Color::black)),
                Some(build_piece(PieceType::horse, Color::black)),
            ],
            [
                Some(build_piece(PieceType::tower, Color::white)),
                Some(build_piece(PieceType::pawn, Color::white)),
                None,
                None,
                None,
                None,
                Some(build_piece(PieceType::pawn, Color::black)),
                Some(build_piece(PieceType::tower, Color::black)),
            ],
        ];

        SchackSpel {
            board_state: board_state,
            turn: 0,
            white_king_position: (3, 0),
            black_king_position: (3, 7),
        }
    }

    impl SchackSpel {
        fn check_if_empty(&self, x: i64, y: i64) -> bool {
            if self.board_state[x as usize][y as usize].is_none() {
                true
            } else {
                false
            }
        }

        pub fn make_move(
            mut self,
            start: (usize, usize),
            destination: (usize, usize),
            color: Color,
        ) -> SchackSpel {
            if self.board_state[start.0][start.1].is_none()
                || self
                    .return_piece_in_position(start.0 as i64, start.1 as i64)
                    .color
                    != color
            {
                return self;
            }

            let possible_moves = self.get_possible_moves(color, start, false);
            let mut move_to_be_made:&Possible_Move;
            let mut move_found = false;


            for i in possible_moves {
                if i.as_ref().unwrap().destination == destination {
                    move_found = true;
                    move_to_be_made = i.as_ref().unwrap();
                    break;
                }
            }

            if move_found == true {
                self.board_state[destination.0][destination.1] =
                    self.board_state[start.0][start.1].clone();

                self.board_state[start.0][start.1] = None;

                if move_to_be_made.trip != None{
                    self.board_state[move_to_be_made.trip.unwrap().0][move_to_be_made.trip.unwrap().1] = None

                }

                //self. flytta pjäsen från start till destionation
                // ta bort pjäsen från start
            }

            let mut color_opposite:Color;
            if color == Color::white{
                color_opposite = Color::black;
            }
            else{
                color_opposite = Color::white;
            }

            //loopa igenom brädet  och kör get possible moves för alla 
            //pjäser av motsatt färg 
            // ifall inga möjliga drag existerar så vinner color

        

            println!("{:?}",self.board_state);

            self
        }

        fn return_piece_in_position(&self, x: i64, y: i64) -> &Piece {
            //if !self.check_if_empty(x,y){
            self.board_state[x as usize][y as usize].as_ref().unwrap()

            //}
        }
        fn build_move(
            &self,
            color: Color,
            destination: (usize, usize),
            kill_move: bool,
            trip: Option<(usize, usize)>,
            start: (usize, usize),
            is_horse: bool,
            castle: Option<((usize, usize), (usize, usize))>,
        ) -> Option<Possible_Move> {
            if self.does_move_cause_check(color, start, destination.0, destination.1, trip, castle)
            {
                Option::None
            } else {
                Some(Possible_Move {
                    destination: destination,
                    kill_move: kill_move,
                    trip: trip,
                    start: start,
                    is_horse: is_horse,
                    castle: castle,
                })
            }
        }

        fn does_move_cause_check(
            &self,
            color: Color,
            position: (usize, usize),
            new_x: usize,
            new_y: usize,
            trip: Option<(usize, usize)>,
            castle: Option<((usize, usize), (usize, usize))>,
        ) -> bool {
            let mut simulated_board = self.board_state.clone();

            let pos_x = position.0;
            let pos_y = position.1;

            simulated_board[new_x][new_y] = simulated_board[pos_x][pos_y].clone();

            if trip != None {
                simulated_board[trip.unwrap().0][trip.unwrap().1] = None;
            }

            if castle != None {
                let castle_unwrapped = castle.unwrap();
                let start = castle_unwrapped.0;
                let end = castle_unwrapped.1;

                simulated_board[end.0][end.1] = simulated_board[start.0][start.1].clone();
                simulated_board[start.0][start.1] = None;
            }

            self.check_check_simulated(color, &simulated_board)
            //SchackSpel::check_check_no_borrow(game_simulation, color)
        }

        fn check_check(game: &SchackSpel, color: Color) -> bool {
            if color == Color::white {
                if game
                    .get_threatened_positions(color)
                    .contains(&game.white_king_position)
                {
                    return true;
                } else {
                    return false;
                }
            } else {
                if game
                    .get_threatened_positions(color)
                    .contains(&game.black_king_position)
                {
                    return true;
                } else {
                    return false;
                }
            }
        }
        fn check_check_no_borrow(game: SchackSpel, color: Color) -> bool {
            if color == Color::white {
                if game
                    .get_threatened_positions(color)
                    .contains(&game.white_king_position)
                {
                    return true;
                } else {
                    return false;
                }
            } else {
                if game
                    .get_threatened_positions(color)
                    .contains(&game.black_king_position)
                {
                    return true;
                } else {
                    return false;
                }
            }
        }

        fn check_check_simulated(
            &self,
            color: Color,
            simulated_board: &[[Option<Piece>; 8]; 8],
        ) -> bool {
            if color == Color::white {
                //jag har lagt in så att kungen inte kan gå in i schack så därför kan du inte råka sätta dig i schack genom att flytta kungen
                if self
                    .simulated_get_threatened_positions(color, simulated_board)
                    .contains(&self.white_king_position)
                {
                    return true;
                } else {
                    return false;
                }
            } else {
                if self
                    .simulated_get_threatened_positions(color, simulated_board)
                    .contains(&self.black_king_position)
                {
                    return true;
                } else {
                    return false;
                }
            }
        }

        fn get_threatened_positions(&self, color: Color) -> Vec<(usize, usize)> {
            let mut threatened_positions = Vec::new();

            let mut color_opposite: Color;

            if color == Color::white {
                color_opposite = Color::black;
            } else {
                color_opposite = Color::white;
            }

            for (x, a) in self.board_state.iter().enumerate() {
                for (y, b) in a.iter().enumerate() {
                    if !&self.check_if_empty(x as i64, y as i64) {
                        if self.return_piece_in_position(x as i64, y as i64).color != color {
                            for i in &self.get_possible_moves(
                                color_opposite,
                                (x as usize, y as usize),
                                true,
                            ) {
                                if i.as_ref().unwrap().kill_move {
                                    threatened_positions.push(i.as_ref().unwrap().destination);
                                }
                            }
                        }
                    }
                }
            }

            threatened_positions
        }
        fn simulated_get_threatened_positions(
            &self,
            color: Color,
            simulated_board: &[[Option<Piece>; 8]; 8],
        ) -> Vec<(usize, usize)> {
            let mut threatened_positions = Vec::new();
            let mut color_opposite: Color;

            if color == Color::white {
                color_opposite = Color::black;
            } else {
                color_opposite = Color::white;
            }

            //kom på att jag måste inkrementera pjäsernas antal drag osv.. kan påverka men inte säker tror ej

            for (x, a) in simulated_board.iter().enumerate() {
                for (y, b) in a.iter().enumerate() {
                    if simulated_board[x as usize][y as usize].is_none() {
                        if simulated_board[x as usize][y as usize]
                            .as_ref()
                            .unwrap()
                            .color
                            != color
                        {
                            for i in self.simulated_get_possible_moves(
                                &simulated_board,
                                color_opposite,
                                (x as usize, y as usize),
                                true,
                            ) {
                                if i.as_ref().unwrap().kill_move {
                                    threatened_positions.push(i.as_ref().unwrap().destination);
                                }
                            }
                        }
                    }
                }
            }

            threatened_positions
        }
        fn simulated_get_possible_moves(
            &self,
            simulated_board: &[[Option<Piece>; 8]; 8],
            color: Color,
            position: (usize, usize),
            get_threatened: bool,
        ) -> Vec<Option<Possible_Move>> {
            let x = position.0;
            let y = position.1;
            let piece = &simulated_board[x][y];
            let piece_unwrapped;

            let mut possible_moves = Vec::new();

            //let checked = SchackSpel::check_check(&self, color);
            //ej klar . börja jobba här
            //self.test_ownership();

            if piece.is_none() {
                println!("empty position");
                return possible_moves;
            }
            // if x>8  x<0  y>8 y<0{
            //     println!("outside board");
            //     return possible_moves;

            // }
            else {
                piece_unwrapped = piece.as_ref().unwrap();
            }

            //om spelarens färg inte matchar valda pjäsen... break

            if piece_unwrapped.piece_type == PieceType::pawn {
                if piece_unwrapped.number_of_moves == 0 {
                    if color == Color::black {
                        if simulated_board[x][y - 2].is_none()
                            && simulated_board[x][y - 1].is_none()
                        {
                            possible_moves.push(self.build_move(
                                color,
                                (x, y - 2),
                                false,
                                Option::None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                        }
                    } else {
                        if simulated_board[x][y + 2].is_none()
                            && simulated_board[x][y + 1].is_none()
                        {
                            possible_moves.push(self.build_move(
                                color,
                                (x, y + 2),
                                false,
                                Option::None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                        }
                    }
                } else {
                    for direction in &piece_unwrapped.moving_directions {
                        let new_x = x as i64 + direction.x;
                        let new_y = y as i64 + direction.y;

                        if 8 > new_x && new_x > -1 {
                            if 8 > new_y && new_y > -1 {
                                if simulated_board[new_x as usize][new_y as usize].is_none() {
                                    possible_moves.push(self.build_move(
                                        color,
                                        (new_x as usize, new_y as usize),
                                        false,
                                        Option::None,
                                        (x as usize, y as usize),
                                        false,
                                        Option::None,
                                    ));
                                }
                            }
                        }
                    }
                    for direction in &piece_unwrapped.kill_directions {
                        let new_x = x as i64 + direction.x;
                        let new_y = y as i64 + direction.y;

                        if 8 > new_x && new_x > -1 {
                            if 8 > new_y && new_y > -1 {
                                if get_threatened {
                                    possible_moves.push(self.build_move(
                                        color,
                                        (new_x as usize, new_y as usize),
                                        true,
                                        Option::None,
                                        (x, y),
                                        false,
                                        Option::None,
                                    ));
                                } else {
                                    if simulated_board[new_x as usize][new_y as usize]
                                        .as_ref()
                                        .unwrap()
                                        .color
                                        != color
                                    {
                                        possible_moves.push(self.build_move(
                                            color,
                                            (new_x as usize, new_y as usize),
                                            true,
                                            Option::None,
                                            (x, y),
                                            false,
                                            Option::None,
                                        ));
                                    } else if simulated_board[new_x as usize][new_y as usize - 1]
                                        .as_ref()
                                        .unwrap()
                                        .color
                                        != color
                                    {
                                        if simulated_board[new_x as usize][new_y as usize - 1]
                                            .as_ref()
                                            .unwrap()
                                            .number_of_moves
                                            == 1
                                        {
                                            if simulated_board[new_x as usize][new_y as usize - 1]
                                                .as_ref()
                                                .unwrap()
                                                .has_double_jumped
                                            {
                                                possible_moves.push(self.build_move(
                                                    color,
                                                    (new_x as usize, new_y as usize),
                                                    true,
                                                    Option::Some((
                                                        new_x as usize,
                                                        new_y as usize - 1,
                                                    )),
                                                    (x, y),
                                                    false,
                                                    Option::None,
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if piece_unwrapped.piece_type == PieceType::horse {
                for direction in &piece_unwrapped.moving_directions {
                    let new_x = direction.x + x as i64;
                    let new_y = direction.y + y as i64;

                    if simulated_board[new_x as usize][new_y as usize].is_none() {
                        possible_moves.push(self.build_move(
                            color,
                            (new_x as usize, new_y as usize),
                            true,
                            Option::None,
                            (x, y),
                            true,
                            Option::None,
                        ));
                    } else if simulated_board[x as usize][y as usize]
                        .as_ref()
                        .unwrap()
                        .color
                        != color
                    {
                        possible_moves.push(self.build_move(
                            color,
                            (new_x as usize, new_y as usize),
                            true,
                            Option::None,
                            (x, y),
                            true,
                            Option::None,
                        ));
                    }
                }
            } else {
                for direction in &piece_unwrapped.moving_directions {
                    let mut i = 1;

                    while 8 > i * add_usize_and_int(x,direction.x)           //direction.x + x
                        && -1 < i * add_usize_and_int(x,direction.x)//direction.x + x
                        && 8 > i * add_usize_and_int(y,direction.y)
                        && -1 < i * add_usize_and_int(y,direction.y)
                    //direction.y + y
                    {
                        let new_x = direction.x + x as i64;
                        let new_y = direction.y + y as i64;

                        if simulated_board[new_x as usize][new_y as usize].is_none() {
                            possible_moves.push(self.build_move(
                                color,
                                (new_x as usize, new_y as usize),
                                true,
                                Option::None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                            i = i + 1;
                        } else if simulated_board[new_x as usize][new_y as usize]
                            .as_ref()
                            .unwrap()
                            .color
                            != color
                        {
                            possible_moves.push(self.build_move(
                                color,
                                (new_x as usize, new_y as usize),
                                true,
                                None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                            i = i + 1;
                        } else {
                            break;
                        }
                    }
                }
            }

            //possible_moves.retain(|x| x == Option::Possible_Move);
            possible_moves.retain(|x| x.is_some());

            possible_moves
        }

        fn get_possible_moves(
            &self,
            color: Color,
            position: (usize, usize),
            get_threatened: bool,
        ) -> Vec<Option<Possible_Move>> {
            let x = position.0;
            let y = position.1;
            let piece = &self.board_state[x][y];
            let piece_unwrapped;

            let mut possible_moves = Vec::new();

            //let checked = SchackSpel::check_check(&self, color);
            //ej klar . börja jobba här
            //self.test_ownership();

            if piece.is_none() {
                println!("empty position");
                return possible_moves;
            }
            // if x>8  x<0  y>8 y<0{
            //     println!("outside board");
            //     return possible_moves;

            // }
            else {
                piece_unwrapped = piece.as_ref().unwrap();
            }


            if piece_unwrapped.piece_type == PieceType::pawn {
                if piece_unwrapped.number_of_moves == 0 {
                    if color == Color::black {
                        if self.board_state[x][y - 2].is_none()
                            && self.board_state[x][y - 1].is_none()
                        {
                            possible_moves.push(self.build_move(
                                color,
                                (x, y - 2),
                                false,
                                Option::None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                        }
                    } else {
                        if self.board_state[x][y + 2].is_none()
                            && self.board_state[x][y + 1].is_none()
                        {
                            possible_moves.push(self.build_move(
                                color,
                                (x, y + 2),
                                false,
                                Option::None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                        }
                    }
                } else {
                    for direction in &piece_unwrapped.moving_directions {
                        let new_x = x as i64 + direction.x;
                        let new_y = y as i64 + direction.y;

                        if 8 > new_x && new_x > -1 {
                            if 8 > new_y && new_y > -1 {
                                if self.board_state[new_x as usize][new_y as usize].is_none() {
                                    possible_moves.push(self.build_move(
                                        color,
                                        (new_x as usize, new_y as usize),
                                        false,
                                        Option::None,
                                        (x as usize, y as usize),
                                        false,
                                        Option::None,
                                    ));
                                }
                            }
                        }
                    }
                    for direction in &piece_unwrapped.kill_directions {
                        let new_x = x as i64 + direction.x;
                        let new_y = y as i64 + direction.y;

                        if 8 > new_x && new_x > -1 {
                            if 8 > new_y && new_y > -1 {
                                if get_threatened {
                                    possible_moves.push(self.build_move(
                                        color,
                                        (new_x as usize, new_y as usize),
                                        true,
                                        Option::None,
                                        (x, y),
                                        false,
                                        Option::None,
                                    ));
                                } else {
                                    if self.board_state[new_x as usize][new_y as usize]
                                        .as_ref()
                                        .unwrap()
                                        .color
                                        != color
                                    {
                                        possible_moves.push(self.build_move(
                                            color,
                                            (new_x as usize, new_y as usize),
                                            true,
                                            Option::None,
                                            (x, y),
                                            false,
                                            Option::None,
                                        ));
                                    } else if self.board_state[new_x as usize][new_y as usize - 1]
                                        .as_ref()
                                        .unwrap()
                                        .color
                                        != color
                                    {
                                        if self.board_state[new_x as usize][new_y as usize - 1]
                                            .as_ref()
                                            .unwrap()
                                            .number_of_moves
                                            == 1
                                        {
                                            if self.board_state[new_x as usize][new_y as usize - 1]
                                                .as_ref()
                                                .unwrap()
                                                .has_double_jumped
                                            {
                                                possible_moves.push(self.build_move(
                                                    color,
                                                    (new_x as usize, new_y as usize),
                                                    true,
                                                    Option::Some((
                                                        new_x as usize,
                                                        new_y as usize - 1,
                                                    )),
                                                    (x, y),
                                                    false,
                                                    Option::None,
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if piece_unwrapped.piece_type == PieceType::king {
                let threatened_positions = self.get_threatened_positions(color);

                // castle logic

                // kollar om kungen har rört på sig

                if !SchackSpel::check_check(&self, color) {
                    if self
                        .return_piece_in_position(x as i64, y as i64)
                        .number_of_moves
                        == 0
                    {
                        if color == Color::white {
                            if self.return_piece_in_position(0, 0).number_of_moves == 0 {
                                if self.board_state[1][0] == None
                                    && self.board_state[2][0] == None
                                    && self.board_state[3][0] == None
                                {
                                    possible_moves.push(self.build_move(
                                        color,
                                        (0, 0),
                                        false,
                                        Option::None,
                                        (4, 0),
                                        false,
                                        Option::Some(((0, 0), (4, 0))),
                                    ));
                                }
                            }
                            if self.return_piece_in_position(7, 0).number_of_moves == 0 {
                                if self.board_state[6][0] == None && self.board_state[5][0] == None
                                {
                                    possible_moves.push(self.build_move(
                                        color,
                                        (7, 0),
                                        false,
                                        Option::None,
                                        (4, 0),
                                        false,
                                        Option::Some(((7, 0), (4, 0))),
                                    ));
                                }
                            }
                        } else {
                            if self.return_piece_in_position(0, 7).number_of_moves == 0 {
                                if self.board_state[1][7] == None
                                    && self.board_state[2][7] == None
                                    && self.board_state[3][7] == None
                                {
                                    possible_moves.push(self.build_move(
                                        color,
                                        (0, 7),
                                        false,
                                        Option::None,
                                        (4, 7),
                                        false,
                                        Option::Some(((0, 7), (4, 7))),
                                    ));
                                }
                            }
                            if self.return_piece_in_position(7, 7).number_of_moves == 0 {
                                if self.board_state[6][7] == None && self.board_state[5][7] == None
                                {
                                    possible_moves.push(self.build_move(
                                        color,
                                        (7, 7),
                                        false,
                                        Option::None,
                                        (4, 7),
                                        false,
                                        Option::Some(((7, 7), (4, 7))),
                                    ));
                                }
                            }
                        }
                    }
                }

                for direction in &piece_unwrapped.moving_directions {
                    let new_x = direction.x + x as i64;
                    let new_y = direction.y + y as i64;

                    if !threatened_positions.contains(&(new_x as usize, new_y as usize)) {
                        if self.check_if_empty(new_x, new_y) {
                            possible_moves.push(self.build_move(
                                color,
                                (new_x as usize, new_y as usize),
                                true,
                                Option::None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                        } else if self.return_piece_in_position(new_x, new_y).color != color {
                            possible_moves.push(self.build_move(
                                color,
                                (new_x as usize, new_y as usize),
                                true,
                                Option::None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                        }
                    }
                }
            } else if piece_unwrapped.piece_type == PieceType::horse {
                for direction in &piece_unwrapped.moving_directions {
                    let new_x = direction.x + x as i64;
                    let new_y = direction.y + y as i64;

                    if self.check_if_empty(new_x, new_y) {
                        possible_moves.push(self.build_move(
                            color,
                            (new_x as usize, new_y as usize),
                            true,
                            Option::None,
                            (x, y),
                            true,
                            Option::None,
                        ));
                    } else if self.return_piece_in_position(new_x, new_y).color != color {
                        possible_moves.push(self.build_move(
                            color,
                            (new_x as usize, new_y as usize),
                            true,
                            Option::None,
                            (x, y),
                            true,
                            Option::None,
                        ));
                    }
                }
            } else {
                for direction in &piece_unwrapped.moving_directions {
                    let mut i = 1;

                    while 8 > i * add_usize_and_int(x,direction.x)           //direction.x + x
                        && -1 < i * add_usize_and_int(x,direction.x)//direction.x + x
                        && 8 > i * add_usize_and_int(y,direction.y)
                        && -1 < i * add_usize_and_int(y,direction.y)
                    //direction.y + y
                    {
                        let new_x = direction.x + x as i64;
                        let new_y = direction.y + y as i64;

                        if self.check_if_empty(new_x, new_y) {
                            possible_moves.push(self.build_move(
                                color,
                                (new_x as usize, new_y as usize),
                                true,
                                Option::None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                            i = i + 1;
                        } else if self.board_state[new_x as usize][new_y as usize]
                            .as_ref()
                            .unwrap()
                            .color
                            != color
                        {
                            possible_moves.push(self.build_move(
                                color,
                                (new_x as usize, new_y as usize),
                                true,
                                None,
                                (x, y),
                                false,
                                Option::None,
                            ));
                            i = i + 1;
                        } else {
                            break;
                        }
                    }
                }
            }

            //possible_moves.retain(|x| x == Option::Possible_Move);
            possible_moves.retain(|x| x.is_some());

            possible_moves
        }
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

    fn build_direction(x: i64, y: i64) -> Direction {
        Direction { x: x, y: y }
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

    fn build_piece(piece_type: PieceType, color: Color) -> Piece {
        let mut range_cap = RangeCap::uncapped;

        let mut moving_directions = Vec::new();
        let mut kill_directions = Vec::new();

        let mut has_double_jumped = false;

        if piece_type == PieceType::tower {
            range_cap = RangeCap::uncapped;

            moving_directions.push(build_direction(0, 1));
            moving_directions.push(build_direction(0, -1));
            moving_directions.push(build_direction(1, 0));
            moving_directions.push(build_direction(-1, 0));

            //kill_directions = moving_directions.clone();
        } else if piece_type == PieceType::knight {
            range_cap = RangeCap::uncapped;
            moving_directions.push(build_direction(1, 1));
            moving_directions.push(build_direction(1, -1));
            moving_directions.push(build_direction(-1, 1));
            moving_directions.push(build_direction(-1, -1));

            //kill_directions = moving_directions.clone();
        } else if piece_type == PieceType::queen {
            range_cap = RangeCap::uncapped;
            //knight moving directions
            moving_directions.push(build_direction(1, 1));
            moving_directions.push(build_direction(1, -1));
            moving_directions.push(build_direction(-1, 1));
            moving_directions.push(build_direction(-1, -1));
            //tower moving directions
            moving_directions.push(build_direction(0, 1));
            moving_directions.push(build_direction(0, -1));
            moving_directions.push(build_direction(1, 0));
            moving_directions.push(build_direction(-1, 0));

            //kill_directions = moving_directions.clone();
        } else if piece_type == PieceType::king {
            range_cap = RangeCap::capped(1);
            //knight moving directions
            moving_directions.push(build_direction(1, 1));
            moving_directions.push(build_direction(1, -1));
            moving_directions.push(build_direction(-1, 1));
            moving_directions.push(build_direction(-1, -1));
            //tower moving directions
            moving_directions.push(build_direction(0, 1));
            moving_directions.push(build_direction(0, -1));
            moving_directions.push(build_direction(1, 0));
            moving_directions.push(build_direction(-1, 0));

            //kill_directions = moving_directions.clone();
        } else if piece_type == PieceType::horse {
            range_cap = RangeCap::capped(1);

            moving_directions.push(build_direction(1, 2));
            moving_directions.push(build_direction(2, -1));
            moving_directions.push(build_direction(-1, -2));
            moving_directions.push(build_direction(-2, 1));

            //kill_directions = &moving_directions;
        } else if piece_type == PieceType::pawn {
            range_cap = RangeCap::capped(1);

            if color == Color::white {
                moving_directions.push(build_direction(0, 1));

                kill_directions.push(build_direction(-1, 1));
                kill_directions.push(build_direction(1, 1));
            } else {
                moving_directions.push(build_direction(0, -1));

                kill_directions.push(build_direction(-1, -1));
                kill_directions.push(build_direction(1, -1));
            }
        }

        Piece {
            piece_type: piece_type,
            color: color,
            moving_directions: moving_directions,
            kill_directions: kill_directions,
            range_cap: range_cap,
            number_of_moves: 0,
            has_double_jumped: has_double_jumped,
        }
    }

    // let spel = SchackSpel{
    //     turn:2,
    //     board_state: [[Some(build_piece(PieceType::pawn,Color::black)),None],[None,None]]
    //     //board_state: [[[None,None,None,None,None,None,None,None],[None,None,None,None,None,None,None,None],[None,None,None,None,None,None,None,None],[None,None,None,None,None,None,None,None],],

    // };
//}
