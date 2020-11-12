use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 3] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zwp_input_timestamps_manager_v1_requests_get_keyboard_timestamps_types:
    [*const wl_interface; 2] = [
    unsafe { &zwp_input_timestamps_v1_interface as *const wl_interface },
    unsafe { &wl_keyboard_interface as *const wl_interface },
];
static mut zwp_input_timestamps_manager_v1_requests_get_pointer_timestamps_types:
    [*const wl_interface; 2] = [
    unsafe { &zwp_input_timestamps_v1_interface as *const wl_interface },
    unsafe { &wl_pointer_interface as *const wl_interface },
];
static mut zwp_input_timestamps_manager_v1_requests_get_touch_timestamps_types:
    [*const wl_interface; 2] = [
    unsafe { &zwp_input_timestamps_v1_interface as *const wl_interface },
    unsafe { &wl_touch_interface as *const wl_interface },
];
pub static mut zwp_input_timestamps_manager_v1_requests: [wl_message; 4] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_keyboard_timestamps\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe {
            &zwp_input_timestamps_manager_v1_requests_get_keyboard_timestamps_types as *const _
        },
    },
    wl_message {
        name: b"get_pointer_timestamps\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe {
            &zwp_input_timestamps_manager_v1_requests_get_pointer_timestamps_types as *const _
        },
    },
    wl_message {
        name: b"get_touch_timestamps\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe {
            &zwp_input_timestamps_manager_v1_requests_get_touch_timestamps_types as *const _
        },
    },
];
pub static mut zwp_input_timestamps_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwp_input_timestamps_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 4,
    requests: unsafe { &zwp_input_timestamps_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zwp_input_timestamps_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zwp_input_timestamps_v1_events: [wl_message; 1] = [wl_message {
    name: b"timestamp\0" as *const u8 as *const c_char,
    signature: b"uuu\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zwp_input_timestamps_v1_interface: wl_interface = wl_interface {
    name: b"zwp_input_timestamps_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_input_timestamps_v1_requests as *const _ },
    event_count: 1,
    events: unsafe { &zwp_input_timestamps_v1_events as *const _ },
};
