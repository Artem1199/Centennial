use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut wp_viewporter_requests_get_viewport_types: [*const wl_interface; 2] = [
    unsafe { &wp_viewport_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut wp_viewporter_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_viewport\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &wp_viewporter_requests_get_viewport_types as *const _ },
    },
];
pub static mut wp_viewporter_interface: wl_interface = wl_interface {
    name: b"wp_viewporter\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &wp_viewporter_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut wp_viewport_requests: [wl_message; 3] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_source\0" as *const u8 as *const c_char,
        signature: b"ffff\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_destination\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut wp_viewport_interface: wl_interface = wl_interface {
    name: b"wp_viewport\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &wp_viewport_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
