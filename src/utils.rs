use std::ops::Range;
use std::time::{Duration, Instant};

/// # OverFlowOps
/// 각 블록의 각도를 정할때 숫자가 범위를 넘어가지 않게 만들기 위한 트레이트
/// - `add` : 더하기 연산을 수행하고 범위를 넘어가면 반대편으로 이동
/// - `sub` : 빼기 연산을 수행하고 범위를 넘어가면 반대편으로 이동
pub trait OverFlowOps {
    const RANGE: Range<i8> = 0..4;
    fn add(&self, other: u8) -> Self;
    fn sub(&self, other: u8) -> Self;
}
impl OverFlowOps for i8 {
    //
    fn add(&self, rhs: u8) -> Self {
        let n = self - rhs as i8;
        if n < Self::RANGE.start {
            Self::RANGE.end - (Self::RANGE.start - n)
        } else if n >= Self::RANGE.end {
            Self::RANGE.start + (n - Self::RANGE.end)
        } else {
            n
        }
    }
    fn sub(&self, rhs: u8) -> Self {
        let n = self - rhs as i8;
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

/// # 타이머
/// das를 만들기 위해 만든 타이머
struct Timer {
    start: Option<Instant>,
    duration: Duration,
}

impl Timer {
    fn new() -> Self {
        Timer {
            start: None,
            duration: Duration::new(0, 0),
        }
    }
    fn start_timer(&mut self, duration: u64) {
        self.start = Some(Instant::now());
        self.duration = Duration::new(duration, 0);
    }

    fn stop_timer(&mut self) {
        self.start = None;
    }

    fn reset_timer(&mut self) {
        self.start = Some(Instant::now());
    }

    fn is_timer_expired(&self) -> bool {
        if let Some(start) = self.start {
            start.elapsed() >= self.duration
        } else {
            false
        }
    }
}
