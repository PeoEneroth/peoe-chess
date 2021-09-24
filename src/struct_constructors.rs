mod structs_and_enums




impl Schackspel{


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
}




    fn build_direction(x: i64, y: i64) -> Direction {
        Direction { x: x, y: y }
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