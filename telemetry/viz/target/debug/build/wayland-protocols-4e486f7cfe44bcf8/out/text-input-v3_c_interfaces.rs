use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut zwp_text_input_v3_requests: [wl_message; 8] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"enable\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"disable\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_surrounding_text\0" as *const u8 as *const c_char,
        signature: b"sii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_text_change_cause\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_content_type\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_cursor_rectangle\0" as *const u8 as *const c_char,
        signature: b"iiii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"commit\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
static mut zwp_text_input_v3_events_enter_types: [*const wl_interface; 1] =
    [unsafe { &wl_surface_interface as *const wl_interface }];
static mut zwp_text_input_v3_events_leave_types: [*const wl_interface; 1] =
    [unsafe { &wl_surface_interface as *const wl_interface }];
pub static mut zwp_text_input_v3_events: [wl_message; 6] = [
    wl_message {
        name: b"enter\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_text_input_v3_events_enter_types as *const _ },
    },
    wl_message {
        name: b"leave\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_text_input_v3_events_leave_types as *const _ },
    },
    wl_message {
        name: b"preedit_string\0" as *const u8 as *const c_char,
        signature: b"?sii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"commit_string\0" as *const u8 as *const c_char,
        signature: b"?s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"delete_surrounding_text\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"done\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_text_input_v3_interface: wl_interface = wl_interface {
    name: b"zwp_text_input_v3\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 8,
    requests: unsafe { &zwp_text_input_v3_requests as *const _ },
    event_count: 6,
    events: unsafe { &zwp_text_input_v3_events as *const _ },
};
static mut zwp_text_input_manager_v3_requests_get_text_input_types: [*const wl_interface; 2] = [
    unsafe { &zwp_text_input_v3_interface as *const wl_interface },
    unsafe { &wl_seat_interface as *const wl_interface },
];
pub static mut zwp_text_input_manager_v3_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_text_input\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_text_input_manager_v3_requests_get_text_input_types as *const _ },
    },
];
pub static mut zwp_text_input_manager_v3_interface: wl_interface = wl_interface {
    name: b"zwp_text_input_manager_v3\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_text_input_manager_v3_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
