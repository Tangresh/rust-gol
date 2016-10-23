struct Position {
    x: int,
    y: int
}
impl Position {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_should_exist() {
        let pos = Position::new(0, 0);
    }

    #[test]
    fn two_positions_with_same_coords_should_be_equal() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(0, 0);

        assert_equal!(pos1, pos2);
    }
}
