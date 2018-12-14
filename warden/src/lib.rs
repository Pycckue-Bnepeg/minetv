mod eye;
mod game_api;
mod hooks;
mod utils;

#[no_mangle]
pub extern "stdcall" fn DllMain(_module: u32, reason_for_call: u32, _reserved: u32) -> bool {
    if reason_for_call == 1 {
        crate::eye::init();
        crate::hooks::install();
    }

    return true;
}
