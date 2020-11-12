use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 2] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zwp_pointer_constraints_v1_requests_lock_pointer_types: [*const wl_interface; 5] = [
    unsafe { &zwp_locked_pointer_v1_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wl_pointer_interface as *const wl_interface },
    unsafe { &wl_region_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
static mut zwp_pointer_constraints_v1_requests_confine_pointer_types: [*const wl_interface; 5] = [
    unsafe { &zwp_confined_pointer_v1_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wl_pointer_interface as *const wl_interface },
    unsafe { &wl_region_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut zwp_pointer_constraints_v1_requests: [wl_message; 3] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"lock_pointer\0" as *const u8 as *const c_char,
        signature: b"noo?ou\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_pointer_constraints_v1_requests_lock_pointer_types as *const _ },
    },
    wl_message {
        name: b"confine_pointer\0" as *const u8 as *const c_char,
        signature: b"noo?ou\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_pointer_constraints_v1_requests_confine_pointer_types as *const _ },
    },
];
pub static mut zwp_pointer_constraints_v1_interface: wl_interface = wl_interface {
    name: b"zwp_pointer_constraints_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &zwp_pointer_constraints_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
static mut zwp_locked_pointer_v1_requests_set_region_types: [*const wl_interface; 1] =
    [unsafe { &wl_region_interface as *const wl_interface }];
pub static mut zwp_locked_pointer_v1_requests: [wl_message; 3] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_cursor_position_hint\0" as *const u8 as *const c_char,
        signature: b"ff\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_region\0" as *const u8 as *const c_char,
        signature: b"?o\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_locked_pointer_v1_requests_set_region_types as *const _ },
    },
];
pub static mut zwp_locked_pointer_v1_events: [wl_message; 2] = [
    wl_message {
        name: b"locked\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"unlocked\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_locked_pointer_v1_interface: wl_interface = wl_interface {
    name: b"zwp_locked_pointer_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &zwp_locked_pointer_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwp_locked_pointer_v1_events as *const _ },
};
static mut zwp_confined_pointer_v1_requests_set_region_types: [*const wl_interface; 1] =
    [unsafe { &wl_region_interface as *const wl_interface }];
pub static mut zwp_confined_pointer_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_region\0" as *const u8 as *const c_char,
        signature: b"?o\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_confined_pointer_v1_requests_set_region_types as *const _ },
    },
];
pub static mut zwp_confined_pointer_v1_events: [wl_message; 2] = [
    wl_message {
        name: b"confined\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"unconfined\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_confined_pointer_v1_interface: wl_interface = wl_interface {
    name: b"zwp_confined_pointer_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_confined_pointer_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwp_confined_pointer_v1_events as *const _ },
};
