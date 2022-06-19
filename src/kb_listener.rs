use winapi::um::winuser::{GetMessageW, MSG, WH_KEYBOARD_LL};
use std::{mem::{MaybeUninit}, ptr::null_mut, sync::atomic::AtomicPtr, thread};
use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::winuser::*,
};
use std::sync::atomic::Ordering;
use winapi::shared::minwindef::{WPARAM, LPARAM, LRESULT};
use std::sync::mpsc::channel;
use std::sync::mpsc::*;


#[derive(Debug)]
pub enum EventType {
    UP,
    DOWN,
    SYSUP,
    SYSDOWN
}


#[derive(Debug)]
pub struct Event {
    pub value: u64,
    pub event_type: EventType,
}

static mut CHANNEL: Option<Sender<Event>> = None;

fn set_hook(
    hook_id: i32,
    hook_ptr: &AtomicPtr<HHOOK__>,
    hook_proc: unsafe extern "system" fn(c_int, WPARAM, LPARAM) -> LRESULT,
) {
    hook_ptr.store(
        unsafe { SetWindowsHookExW(hook_id, Some(hook_proc), 0 as HINSTANCE, 0) },
        Ordering::Relaxed,
    );
}
/*
fn unset_hook(hook_ptr: &AtomicPtr<HHOOK__>) {
    if !hook_ptr.load(Ordering::Relaxed).is_null() {
        unsafe { UnhookWindowsHookEx(hook_ptr.load(Ordering::Relaxed)) };
        hook_ptr.store(null_mut(), Ordering::Relaxed);
    }
}
*/

pub fn handle_input_events() -> Receiver<Event> {
    // if !MOUSE_BINDS.lock().unwrap().is_empty() {
    //     set_hook(WH_MOUSE_LL, &*MOUSE_HHOOK, mouse_proc);
    // };
    // if !KEYBD_BINDS.lock().unwrap().is_empty() {
    let (sx, rx) = channel();
    unsafe {
        // CHANNEL = Some(ChannelWrapper::new());
        CHANNEL = Some(sx);
    }
    thread::spawn(||{
        let keybd_hhook: AtomicPtr<HHOOK__> = AtomicPtr::default();

        set_hook(WH_KEYBOARD_LL, &keybd_hhook, keybd_proc);
        // };
        let mut msg: MSG = unsafe { MaybeUninit::zeroed().assume_init() };
            unsafe { GetMessageW(&mut msg, 0 as HWND, 0, 0) };
    });

    return rx;
}

unsafe extern "system" fn keybd_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {

    let event_type = match w_param as u32 {
        WM_KEYDOWN => EventType::DOWN,
        WM_KEYUP => EventType::UP,
        WM_SYSKEYDOWN => EventType::SYSDOWN,
        WM_SYSKEYUP => EventType::SYSUP,
        _ => panic!("unknown event type"),
    };

    let vkcode = u64::from(
        (*(l_param as *const KBDLLHOOKSTRUCT)).vkCode);


    match &CHANNEL {
        None => panic!("asdfasdf"),
        Some(sender) => {
            // println!("got input {}", vkcode);
            sender.send(Event {
                value: vkcode,
                event_type,
            }).expect("Error sending key code");
        }
    }

    CallNextHookEx(null_mut(), code, w_param, l_param)
}