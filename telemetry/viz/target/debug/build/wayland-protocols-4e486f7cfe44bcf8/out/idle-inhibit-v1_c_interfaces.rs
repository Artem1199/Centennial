use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 0] = [];
static mut zwp_idle_inhibit_manager_v1_requests_create_inhibitor_types: [*const wl_interface; 2] = [
    unsafe { &zwp_idle_inhibitor_v1_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zwp_idle_inhibit_manager_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"create_inhibitor\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_idle_inhibit_manager_v1_requests_create_inhibitor_types as *const _ },
    },
];
pub static mut zwp_idle_inhibit_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwp_idle_inhibit_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_idle_inhibit_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zwp_idle_inhibitor_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zwp_idle_inhibitor_v1_interface: wl_interface = wl_interface {
    name: b"zwp_idle_inhibitor_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_idle_inhibitor_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
