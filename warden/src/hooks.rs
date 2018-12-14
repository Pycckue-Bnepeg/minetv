use crate::eye::Eye;
use detour::GenericDetour;

type WinProc = extern "stdcall" fn(window: u32, message: u32, w_param: u32, l_param: u32) -> i32;
type GameStart = extern "stdcall" fn();
type GameOver = extern "stdcall" fn(result: u32) -> u32;
type PushBoxUp = extern "stdcall" fn(x: u32, y: u32);
type PushBoxDown = extern "stdcall" fn(x: u32, y: u32);
type DisplayCell = extern "stdcall" fn(x: u32, y: u32);

macro_rules! hook {
    ($fn_type:ty, $addr:expr, $fn_name:ident) => {
        unsafe { detour::GenericDetour::new(addr::<$fn_type>($addr), $fn_name).unwrap() }
    };
}

macro_rules! call_origin {
    ($func_name:ident) => {
        unsafe {
            HOOKS.as_mut().map(|hooks| hooks.$func_name.call())
        }
    };

    ($func_name:ident, $($arg:expr),*) => {
        unsafe {
            HOOKS.as_mut().map(|hooks| hooks.$func_name.call($($arg),*))
        }
    }
}

struct Hooks {
    win_proc: GenericDetour<WinProc>,
    game_start: GenericDetour<GameStart>,
    game_over: GenericDetour<GameOver>,
    push_box_up: GenericDetour<PushBoxUp>,
    push_box_down: GenericDetour<PushBoxDown>,
    display_cell: GenericDetour<DisplayCell>,
}

impl Hooks {
    fn new() -> Hooks {
        Hooks {
            win_proc: hook!(WinProc, 0x01001BC9, win_proc),
            game_start: hook!(GameStart, 0x0100367A, game_start),
            game_over: hook!(GameOver, 0x0100347C, game_over),
            push_box_up: hook!(PushBoxUp, 0x010031A0, push_box_up),
            push_box_down: hook!(PushBoxDown, 0x0100316B, push_box_down),
            display_cell: hook!(DisplayCell, 0x01002646, display_cell),
        }
    }
}

static mut HOOKS: Option<Hooks> = None;

pub fn install() {
    unsafe {
        HOOKS = Some(Hooks::new());

        HOOKS.as_mut().map(|hooks| {
            let install_result = hooks
                .win_proc
                .enable()
                .and(hooks.game_start.enable())
                .and(hooks.game_over.enable())
                .and(hooks.push_box_up.enable())
                .and(hooks.push_box_down.enable())
                .and(hooks.display_cell.enable());

            if install_result.is_err() {
                crate::utils::message_box("MineTV", "Can't initialize MineTV core.");
                panic!("Unable to enable hooks");
            }
        });
    }
}

fn addr<T: Sized>(address: usize) -> T {
    unsafe { std::mem::transmute_copy(&address) }
}

extern "stdcall" fn game_start() {
    Eye::get_mut().game_start();
    call_origin!(game_start);
}

extern "stdcall" fn game_over(result: u32) -> u32 {
    Eye::get_mut().game_over(result);
    call_origin!(game_over, result).unwrap_or(0)
}

extern "stdcall" fn push_box_up(x: u32, y: u32) {
    Eye::get_mut().push_box_up(x, y);
    call_origin!(push_box_up, x, y);
}

extern "stdcall" fn push_box_down(x: u32, y: u32) {
    Eye::get_mut().push_box_down(x, y);
    call_origin!(push_box_down, x, y);
}

extern "stdcall" fn display_cell(x: u32, y: u32) {
    Eye::get_mut().display_cell(x, y);
    call_origin!(display_cell, x, y);
}

extern "stdcall" fn win_proc(window: u32, message: u32, w_param: u32, l_param: u32) -> i32 {
    Eye::get_mut().win_proc(window, message, w_param, l_param);
    call_origin!(win_proc, window, message, w_param, l_param).unwrap_or(0)
}
