use super::board::BoardPossibilities;
use rand::Rng;

const NUMBER_OF_BRICKS: i32 = 7;

#[derive(Clone)]
pub struct Brick {
    m_coord: Vec<bool>,
    m_color: BoardPossibilities,
    m_rotation: i8,
    m_number_of_rotations: i8,
}
impl Brick {
    pub fn new() -> Self {
        let random = rand::thread_rng().gen_range(0..NUMBER_OF_BRICKS);
        match random {
            0 => Self {
                m_coord: Self::i_shape(),
                m_color: BoardPossibilities::Cyan,
                m_rotation: 0,
                m_number_of_rotations: 2,
            },
            1 => Self {
                m_coord: Self::j_shape(),
                m_color: BoardPossibilities::Blue,
                m_number_of_rotations: 4,
                m_rotation: 0,
            },
            2 => Self {
                m_coord: Self::l_shape(),
                m_color: BoardPossibilities::Orange,
                m_rotation: 0,
                m_number_of_rotations: 4,
            },
            3 => Self {
                m_coord: Self::o_shape(),
                m_color: BoardPossibilities::Yellow,
                m_rotation: 0,
                m_number_of_rotations: 1,
            },
            4 => Self {
                m_coord: Self::s_shape(),
                m_color: BoardPossibilities::Green,
                m_rotation: 0,
                m_number_of_rotations: 4,
            },
            5 => Self {
                m_coord: Self::t_shape(),
                m_color: BoardPossibilities::Pink,
                m_rotation: 0,
                m_number_of_rotations: 4,
            },
            _ => Self {
                m_coord: Self::z_shape(),
                m_color: BoardPossibilities::Red,
                m_rotation: 0,
                m_number_of_rotations: 4,
            },
        }
    }

    pub fn rotate(&mut self) {
        self.m_rotation += 1;
        self.m_rotation %= self.m_number_of_rotations;
    }
    pub fn unrotate(&mut self) {
        if self.m_rotation == 0 {
            self.m_rotation = 3;
        } else {
            self.m_rotation -= 1;
        }
    }

    pub fn consult(&self, line: usize, column: usize) -> bool {
        match self.m_rotation {
            0 => return self.m_coord[line * 4 + column],
            1 => return self.m_coord[12 + line - column * 4],
            2 => return self.m_coord[15 - line * 4 - column],
            3 => return self.m_coord[3 - line + column * 4],
            _ => return false, // Should not reach this.
        }
    }

    pub fn consult_color(&self) -> &BoardPossibilities {
        &self.m_color
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
