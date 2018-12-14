pub mod functions {
    pub const WIN_PROC: usize = 0x01001BC9;
    pub const GAME_START: usize = 0x0100367A;
    pub const GAME_OVER: usize = 0x0100347C;
    pub const PUSH_BOX_UP: usize = 0x010031A0;
    pub const PUSH_BOX_DOWN: usize = 0x0100316B;
    pub const DISPLAY_CELL: usize = 0x01002646;
}

pub const FIELD: usize = 0x01005340;
pub const BOMBS_COUNT: usize = 0x010056A4;
pub const FIELD_HEIGHT: usize = 0x010056A8;
pub const FIELD_WIDTH: usize = 0x010056AC;
