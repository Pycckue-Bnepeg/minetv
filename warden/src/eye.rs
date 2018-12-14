use crate::utils::message_box;

pub static mut EYE: Option<Eye> = None;

pub struct Eye;

impl Eye {
    fn new() -> Eye {
        Eye {}
    }

    pub fn game_start(&mut self) {
        message_box("MineTV", "Game has been started!");
    }

    pub fn game_over(&mut self, _result: u32) {}
    pub fn push_box_up(&mut self, _x: u32, _y: u32) {}
    pub fn push_box_down(&mut self, _x: u32, _y: u32) {}
    pub fn display_cell(&mut self, _x: u32, _y: u32) {}
    pub fn win_proc(&mut self, _: u32, _: u32, _: u32, _: u32) {}

    #[allow(dead_code)]
    pub fn get() -> &'static Eye {
        unsafe { EYE.as_ref().unwrap() }
    }

    pub fn get_mut() -> &'static mut Eye {
        unsafe { EYE.as_mut().unwrap() }
    }
}

pub fn init() {
    unsafe {
        EYE = Some(Eye::new());
    }
}
