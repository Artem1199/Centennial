use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 1] = [NULLPTR as *const wl_interface];
static mut zwp_linux_explicit_synchronization_v1_requests_get_synchronization_types:
    [*const wl_interface; 2] = [
    unsafe { &zwp_linux_surface_synchronization_v1_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zwp_linux_explicit_synchronization_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_synchronization\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe {
            &zwp_linux_explicit_synchronization_v1_requests_get_synchronization_types as *const _
        },
    },
];
pub static mut zwp_linux_explicit_synchronization_v1_interface: wl_interface = wl_interface {
    name: b"zwp_linux_explicit_synchronization_v1\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 2,
    requests: unsafe { &zwp_linux_explicit_synchronization_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
static mut zwp_linux_surface_synchronization_v1_requests_get_release_types: [*const wl_interface;
    1] = [unsafe { &zwp_linux_buffer_release_v1_interface as *const wl_interface }];
pub static mut zwp_linux_surface_synchronization_v1_requests: [wl_message; 3] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_acquire_fence\0" as *const u8 as *const c_char,
        signature: b"h\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_release\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe {
            &zwp_linux_surface_synchronization_v1_requests_get_release_types as *const _
        },
    },
];
pub static mut zwp_linux_surface_synchronization_v1_interface: wl_interface = wl_interface {
    name: b"zwp_linux_surface_synchronization_v1\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 3,
    requests: unsafe { &zwp_linux_surface_synchronization_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zwp_linux_buffer_release_v1_events: [wl_message; 2] = [
    wl_message {
        name: b"fenced_release\0" as *const u8 as *const c_char,
        signature: b"h\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"immediate_release\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_linux_buffer_release_v1_interface: wl_interface = wl_interface {
    name: b"zwp_linux_buffer_release_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 0,
    requests: NULLPTR as *const wl_message,
    event_count: 2,
    events: unsafe { &zwp_linux_buffer_release_v1_events as *const _ },
};
