use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zwlr_layer_shell_v1_requests_get_layer_surface_types: [*const wl_interface; 5] = [
    unsafe { &zwlr_layer_surface_v1_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wl_output_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut zwlr_layer_shell_v1_requests: [wl_message; 1] = [wl_message {
    name: b"get_layer_surface\0" as *const u8 as *const c_char,
    signature: b"no?ous\0" as *const u8 as *const c_char,
    types: unsafe { &zwlr_layer_shell_v1_requests_get_layer_surface_types as *const _ },
}];
pub static mut zwlr_layer_shell_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_layer_shell_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwlr_layer_shell_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
static mut zwlr_layer_surface_v1_requests_get_popup_types: [*const wl_interface; 1] =
    [unsafe { &xdg_popup_interface as *const wl_interface }];
pub static mut zwlr_layer_surface_v1_requests: [wl_message; 8] = [
    wl_message {
        name: b"set_size\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_anchor\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_exclusive_zone\0" as *const u8 as *const c_char,
        signature: b"i\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_margin\0" as *const u8 as *const c_char,
        signature: b"iiii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_keyboard_interactivity\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_popup\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_layer_surface_v1_requests_get_popup_types as *const _ },
    },
    wl_message {
        name: b"ack_configure\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_layer_surface_v1_events: [wl_message; 2] = [
    wl_message {
        name: b"configure\0" as *const u8 as *const c_char,
        signature: b"uuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"closed\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_layer_surface_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_layer_surface_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 8,
    requests: unsafe { &zwlr_layer_surface_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwlr_layer_surface_v1_events as *const _ },
};
