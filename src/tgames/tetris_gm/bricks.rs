use super::board::BoardPossibilities;

const NUMBER_OF_BRICKS: i32 = 7;

#[derive(Clone)]
pub struct Brick {
    coord: Vec<bool>,
    color: BoardPossibilities,
    rotation: i8,
    number_of_rotations: i8,
}
impl Brick {
    pub fn new() -> Self {
        let random = rand::random_range(0..NUMBER_OF_BRICKS);
        match random {
            0 => Self {
                coord: Self::i_shape(),
                color: BoardPossibilities::Cyan,
                rotation: 0,
                number_of_rotations: 2,
            },
            1 => Self {
                coord: Self::j_shape(),
                color: BoardPossibilities::Blue,
                number_of_rotations: 4,
                rotation: 0,
            },
            2 => Self {
                coord: Self::l_shape(),
                color: BoardPossibilities::Orange,
                rotation: 0,
                number_of_rotations: 4,
            },
            3 => Self {
                coord: Self::o_shape(),
                color: BoardPossibilities::Yellow,
                rotation: 0,
                number_of_rotations: 1,
            },
            4 => Self {
                coord: Self::s_shape(),
                color: BoardPossibilities::Green,
                rotation: 0,
                number_of_rotations: 4,
            },
            5 => Self {
                coord: Self::t_shape(),
                color: BoardPossibilities::Pink,
                rotation: 0,
                number_of_rotations: 4,
            },
            _ => Self {
                coord: Self::z_shape(),
                color: BoardPossibilities::Red,
                rotation: 0,
                number_of_rotations: 4,
            },
        }
    }

    pub fn rotate(&mut self) {
        self.rotation += 1;
        self.rotation %= self.number_of_rotations;
    }
    pub fn unrotate(&mut self) {
        if self.rotation == 0 {
            self.rotation = 3;
        } else {
            self.rotation -= 1;
        }
    }

    pub fn consult(&self, line: usize, column: usize) -> bool {
        match self.rotation {
            0 => self.coord[line * 4 + column],
            1 => self.coord[12 + line - column * 4],
            2 => self.coord[15 - line * 4 - column],
            3 => self.coord[3 - line + column * 4],
            _ => false, // Should not reach this.
        }
    }

    pub fn consult_color(&self) -> &BoardPossibilities {
        &self.color
    }

    fn i_shape() -> Vec<bool> {
        vec![
            false, false, false, false, true, true, true, true, false, false, false, false, false,
            false, false, false,
        ]
    }

    fn j_shape() -> Vec<bool> {
        vec![
            false, true, false, false, false, true, true, true, false, false, false, false, false,
            false, false, false,
        ]
    }

    fn l_shape() -> Vec<bool> {
        vec![
            false, false, false, true, false, true, true, true, false, false, false, false, false,
            false, false, false,
        ]
    }

    fn o_shape() -> Vec<bool> {
        vec![
            false, true, true, false, false, true, true, false, false, false, false, false, false,
            false, false, false,
        ]
    }

    fn s_shape() -> Vec<bool> {
        vec![
            false, false, true, true, false, true, true, false, false, false, false, false, false,
            false, false, false,
        ]
    }

    fn t_shape() -> Vec<bool> {
        vec![
            false, false, true, false, false, true, true, true, false, false, false, false, false,
            false, false, false,
        ]
    }

    fn z_shape() -> Vec<bool> {
        vec![
            false, true, true, false, false, false, true, true, false, false, false, false, false,
            false, false, false,
        ]
    }
}
