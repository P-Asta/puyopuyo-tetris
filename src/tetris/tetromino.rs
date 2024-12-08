use crate::contracts::Block;
use crate::utils::OverFlowOps;
use crate::GameField;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct I {
    pub position: (i32, i32),
    pub rotate_index: i8,
}
/// I미노에 대한 블럭 설정입니다.
impl Block for I {
    const ID: u8 = 0;
    /// I미노를 초기화합니다.
    fn new(x: i32, y: i32) -> Self {
        I {
            position: (x, y),
            rotate_index: 0,
        }
    }
    /// 시계방향 회전CW)을 하는 함수입니다.
    fn rotate_clockwise(&mut self) {
        /// CW시에 srs)+ 월킥 테스트를 하는 변수입니다 
        let kick_tests = [(0, 0), (-2, 0), (2, 0), (-1, 1), (2, -1)];

        self.rotate_index.add(1);
    }
    /// 반시계방향 회전(CCW)을 하는 함수입니다.
    fn rotate_counter_clockwise(&mut self) {
        // CCW시에 srs+ 월킥을 테스트를 하는 변수입니다
        let kick_tests = [(0, 0), (-2, 0), (2, 0), (-1, 1), (2, -1)];
        self.rotate_index.sub(1);
    }
    /// 180도 회전을 하는 함수입니다.
    fn rotate_180(&mut self) {
        // 180회전시에 srs+ 월킥을 테스트를 하는 변수입니다
        let kick_tests = [
            (0, 0),
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (-2, 0),
            (2, 0),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ];
        self.rotate_index.add(2);
    }

    /// 왼쪽으로 블럭을 옴김니다.
    fn move_left(&mut self) {
        self.position.0 -= 1;
    }
    
    /// 오른쪽으로 블럭을 옴김니다.
    fn move_right(&mut self) {
        self.position.0 += 1;
    }
    
    /// 소프트 드랍을 합니다.
    fn move_down(&mut self) {
        self.position.1 += 1;
    }

    /// 소프트 드랍을 합니다.
    fn test_collision(&self, dx: i32, dy: i32, field: &GameField) -> bool {
        let shapes = self.get_shapes();
        let (x, y) = self.position;
        for i in 0..4 {
            for j in 0..4 {
                if shapes[self.rotate_index as usize][i][j] == 1 {
                    let x = x + j as i32 + dx;
                    let y = y + i as i32 + dy;
                    if x < 0 || x >= field.size.0 as i32 || y >= field.size.1 as i32 {
                        return true;
                    }
                    if y >= 0 && field.blocks.contains(&(x as u8, y as u8)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }
}
