mod structs_and_enums;
mod struct_constructors;


impl SchackSpel{

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

        

            //println!("{:?}",self.board_state);

            self
        }

        //av en pjäs i position
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