use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 1] = [NULLPTR as *const wl_interface];
static mut zxdg_decoration_manager_v1_requests_get_toplevel_decoration_types:
    [*const wl_interface; 2] = [
    unsafe { &zxdg_toplevel_decoration_v1_interface as *const wl_interface },
    unsafe { &xdg_toplevel_interface as *const wl_interface },
];
pub static mut zxdg_decoration_manager_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_toplevel_decoration\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe {
            &zxdg_decoration_manager_v1_requests_get_toplevel_decoration_types as *const _
        },
    },
];
pub static mut zxdg_decoration_manager_v1_interface: wl_interface = wl_interface {
    name: b"zxdg_decoration_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zxdg_decoration_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zxdg_toplevel_decoration_v1_requests: [wl_message; 3] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_mode\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"unset_mode\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zxdg_toplevel_decoration_v1_events: [wl_message; 1] = [wl_message {
    name: b"configure\0" as *const u8 as *const c_char,
    signature: b"u\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zxdg_toplevel_decoration_v1_interface: wl_interface = wl_interface {
    name: b"zxdg_toplevel_decoration_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &zxdg_toplevel_decoration_v1_requests as *const _ },
    event_count: 1,
    events: unsafe { &zxdg_toplevel_decoration_v1_events as *const _ },
};
