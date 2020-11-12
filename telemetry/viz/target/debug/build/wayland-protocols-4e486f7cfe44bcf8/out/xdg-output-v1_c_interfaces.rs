use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 2] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zxdg_output_manager_v1_requests_get_xdg_output_types: [*const wl_interface; 2] = [
    unsafe { &zxdg_output_v1_interface as *const wl_interface },
    unsafe { &wl_output_interface as *const wl_interface },
];
pub static mut zxdg_output_manager_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_xdg_output\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_output_manager_v1_requests_get_xdg_output_types as *const _ },
    },
];
pub static mut zxdg_output_manager_v1_interface: wl_interface = wl_interface {
    name: b"zxdg_output_manager_v1\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 2,
    requests: unsafe { &zxdg_output_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zxdg_output_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zxdg_output_v1_events: [wl_message; 5] = [
    wl_message {
        name: b"logical_position\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"logical_size\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"done\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"name\0" as *const u8 as *const c_char,
        signature: b"2s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"description\0" as *const u8 as *const c_char,
        signature: b"2s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zxdg_output_v1_interface: wl_interface = wl_interface {
    name: b"zxdg_output_v1\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 1,
    requests: unsafe { &zxdg_output_v1_requests as *const _ },
    event_count: 5,
    events: unsafe { &zxdg_output_v1_events as *const _ },
};
