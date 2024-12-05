use std::ops::Range;

// 각 블록의 각도를 정할때 숫자가 범위를 넘어가지 않게 만들기 위한 트레이트
pub trait OverFlowOps {
    const RANGE: Range<i8> = 0..4;
    fn add(&self, other: Self) -> Self;
    fn sub(&self, other: Self) -> Self;
}
impl OverFlowOps for i8 {
    fn add(&self, rhs: Self) -> Self {
        let n = *self + rhs;
        if n < Self::RANGE.start {
            Self::RANGE.end - (Self::RANGE.start - n)
        } else if n >= Self::RANGE.end {
            Self::RANGE.start + (n - Self::RANGE.end)
        } else {
            n
        }
    }
    fn sub(&self, rhs: Self) -> Self {
        let n = *self - rhs;
        if n < Self::RANGE.start {
            Self::RANGE.end - (Self::RANGE.start - n)
        } else if n >= Self::RANGE.end {
            Self::RANGE.start + (n - Self::RANGE.end)
        } else {
            n
        }
    }
}
/// # 블록의 모양을 반환하는 함수
/// - 블록은 4가지 회전 상태를 가지고 있다.
/// - 각 회전 상태는 4x4 2차원 배열로 표현된다.
/// - 각 요소는 블록 내 셀의 좌표를 포함한다.
///
/// 1. `I`
/// 2. `T`
/// 3. `O`
/// 4. `J`
/// 5. `L`
/// 6. `Z`
/// 7. `S`
pub fn get_shape_by_id(id: u8) -> Vec<Vec<Vec<u8>>> {
    match id {
        // ⬜⬜⬜⬜
        1 => {
            vec![
                vec![vec![0, 1], vec![1, 1], vec![2, 1], vec![3, 1]],
                vec![vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3]],
                vec![vec![0, 2], vec![1, 2], vec![2, 2], vec![3, 2]],
                vec![vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3]],
            ]
        }
        // ⬛🟪
        // 🟪🟪🟪
        2 => {
            vec![
                vec![vec![1, 0], vec![1, 1], vec![0, 1], vec![2, 1]],
                vec![vec![1, 0], vec![1, 1], vec![2, 1], vec![1, 2]],
                vec![vec![0, 1], vec![1, 1], vec![2, 1], vec![1, 2]],
                vec![vec![1, 0], vec![1, 1], vec![0, 1], vec![1, 2]],
            ]
        }
        // 🟨🟨
        // 🟨🟨
        3 => {
            vec![
                vec![vec![1, 0], vec![2, 0], vec![1, 1], vec![2, 1]],
                vec![vec![1, 0], vec![2, 0], vec![1, 1], vec![2, 1]],
                vec![vec![1, 0], vec![2, 0], vec![1, 1], vec![2, 1]],
                vec![vec![1, 0], vec![2, 0], vec![1, 1], vec![2, 1]],
            ]
        }
        // 🟦
        // 🟦🟦🟦
        4 => {
            vec![
                vec![vec![0, 1], vec![1, 1], vec![2, 1], vec![0, 0]],
                vec![vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 0]],
                vec![vec![0, 1], vec![1, 1], vec![2, 1], vec![2, 2]],
                vec![vec![1, 0], vec![1, 1], vec![1, 2], vec![0, 2]],
            ]
        }
        // ⬛⬛🟧
        // 🟧🟧🟧
        5 => {
            vec![
                vec![vec![0, 1], vec![1, 1], vec![2, 1], vec![2, 0]],
                vec![vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 2]],
                vec![vec![0, 1], vec![1, 1], vec![2, 1], vec![0, 2]],
                vec![vec![1, 0], vec![1, 1], vec![1, 2], vec![0, 0]],
            ]
        }
        // 🟥🟥
        // ⬛🟥🟥
        6 => {
            vec![
                vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![2, 1]],
                vec![vec![1, 2], vec![1, 1], vec![2, 1], vec![2, 0]],
                vec![vec![0, 1], vec![1, 1], vec![1, 2], vec![2, 2]],
                vec![vec![1, 0], vec![1, 1], vec![0, 1], vec![0, 2]],
            ]
        }
        // ⬛🟩🟩
        // 🟩🟩
        7 => {
            vec![
                vec![vec![0, 1], vec![1, 1], vec![1, 0], vec![2, 0]],
                vec![vec![1, 0], vec![1, 1], vec![2, 1], vec![2, 2]],
                vec![vec![0, 2], vec![1, 1], vec![1, 2], vec![2, 1]],
                vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![1, 2]],
            ]
        }
        _ => {
            vec![vec![vec![0; 2]; 4]; 4]
        }
    }
}
