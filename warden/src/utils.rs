use std::ffi::CString;
use winapi::um::winuser::MessageBoxA;

pub fn message_box<A, B>(caption: A, text: B)
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    let caption = CString::new(caption.as_ref()).unwrap();
    let text = CString::new(text.as_ref()).unwrap();

    unsafe {
        MessageBoxA(0 as _, text.as_ptr(), caption.as_ptr(), 0);
    }
}
