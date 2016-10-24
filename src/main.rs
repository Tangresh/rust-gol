use std::ops::Index;

#[derive(PartialEq, Eq, Debug)]
struct Position {
    x: i64,
    y: i64
}

impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position {
            x: x,
            y: y,
        }
    }

    fn get_neighbours(&self) -> Vec<Position> {
        let mut neighbours = vec!();
        for x in -1..2 {
            for y in -1..2 {
                let neighbour = Position::new(self.x + x, self.y + y);
                if (&neighbour != self) {
                    neighbours.push(neighbour);
                }
            }
        }
        neighbours
    }
}

trait WithLiveCells {
    fn is_alive(&self, pos: &Position) -> bool;
}

struct GameBoard {
    generations: Vec<Generation>
}
impl GameBoard {
    fn initialize_with(initial_generation: Generation) -> GameBoard {
        GameBoard{
            generations: vec!(initial_generation)
        }
    }

    fn get_current_generation(&self) -> &Generation {
        self.generations.index(self.generations.len() - 1)
    }

    fn advance_time(&mut self) {
        let mut new_generation: Vec<Position> = vec!();
        {
            let current_generation = self.get_current_generation();
            for pos in current_generation.live_cells() {
                let neighbours = pos.get_neighbours();
            }
        }
        self.generations.push(Generation::new(vec!()))
    }
}
impl WithLiveCells for GameBoard {
    fn is_alive(&self, pos: &Position) -> bool {
        self.get_current_generation().is_alive(pos)
    }
}

struct Generation {
    live_positions: Vec<Position>
}
impl Generation {
    fn new(live_positions: Vec<Position>) -> Generation {
        Generation {
            live_positions: live_positions
        }
    }

    fn build() -> GenerationBuilder {
        GenerationBuilder{
            live_positions: vec!()
        }
    }

    fn live_cells(&self) -> &Vec<Position> {
        &self.live_positions
    }
}
impl WithLiveCells for Generation {
    fn is_alive(&self, pos: &Position) -> bool {
        self.live_positions.contains(pos)
    }
}

struct GenerationBuilder {
    live_positions: Vec<Position>
}
impl GenerationBuilder {
    fn add(mut self, x: i64, y: i64) -> GenerationBuilder {
        let pos = Position::new(x, y);
        self.live_positions.push(pos);
        self
    }

    fn build(mut self) -> Generation {
        Generation::new(self.live_positions)
    }
}

#[cfg(test)]
#[macro_use(expect)] extern crate expectest;

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use super::Position;
    use super::WithLiveCells;
    use super::GameBoard;
    use super::Generation;

    #[test]
    fn position_should_exist() {
        let pos = Position::new(0, 0);
    }

    #[test]
    fn two_positions_with_same_coords_should_be_equal() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(0, 0);
        let pos3 = Position::new(1, 1);

        assert_eq!(pos1, pos2);
        expect!(pos1).to_not(be_equal_to(pos3));
    }

    #[test]
    fn position_should_have_8_neighbours() {
        let pos = Position::new(0, 0);

        let neighbours: Vec<Position> = pos.get_neighbours();

        expect!(neighbours.len()).to(be_equal_to(8));
    }

    #[test]
    fn new_board_should_have_initialized_live_cells() {
        let game_board = GameBoard::initialize_with(
            Generation::build()
                .add(0, 0)
                .add(0, 1)
                .add(1, 0)
                .add(1, 1)
                .build());

        let current_generation = game_board.get_current_generation();

        let result = current_generation.is_alive(&Position::new(0, 0));

        expect!(result).to(be_true());
    }

    #[test]
    fn lone_cell_should_die() {
        let mut game_board = GameBoard::initialize_with(
            Generation::build()
                .add(0, 0)
                .build());

        game_board.advance_time();

        let current_generation = game_board.get_current_generation();

        let result = current_generation.is_alive(&Position::new(0, 0));

        expect!(result).to_not(be_true());
    }

    #[test]
    fn square_should_survive() {
        let mut game_board = GameBoard::initialize_with(
            Generation::build()
                .add(0, 0)
                .add(0, 1)
                .add(1, 0)
                .add(1, 1)
                .build());

        game_board.advance_time();

        let current_generation = game_board.get_current_generation();

        expect!(current_generation.is_alive(&Position::new(0, 0))).to(be_true());
        expect!(current_generation.is_alive(&Position::new(0, 1))).to(be_true());
        expect!(current_generation.is_alive(&Position::new(1, 0))).to(be_true());
        expect!(current_generation.is_alive(&Position::new(1, 1))).to(be_true());
    }
}