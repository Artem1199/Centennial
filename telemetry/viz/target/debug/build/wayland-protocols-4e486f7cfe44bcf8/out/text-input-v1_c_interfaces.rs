use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 5] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zwp_text_input_v1_requests_activate_types: [*const wl_interface; 2] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
static mut zwp_text_input_v1_requests_deactivate_types: [*const wl_interface; 1] =
    [unsafe { &wl_seat_interface as *const wl_interface }];
pub static mut zwp_text_input_v1_requests: [wl_message; 11] = [
    wl_message {
        name: b"activate\0" as *const u8 as *const c_char,
        signature: b"oo\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_text_input_v1_requests_activate_types as *const _ },
    },
    wl_message {
        name: b"deactivate\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_text_input_v1_requests_deactivate_types as *const _ },
    },
    wl_message {
        name: b"show_input_panel\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"hide_input_panel\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"reset\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_surrounding_text\0" as *const u8 as *const c_char,
        signature: b"suu\0" as *const u8 as *const c_char,
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
        name: b"set_preferred_language\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"commit_state\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"invoke_action\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
static mut zwp_text_input_v1_events_enter_types: [*const wl_interface; 1] =
    [unsafe { &wl_surface_interface as *const wl_interface }];
pub static mut zwp_text_input_v1_events: [wl_message; 13] = [
    wl_message {
        name: b"enter\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_text_input_v1_events_enter_types as *const _ },
    },
    wl_message {
        name: b"leave\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"modifiers_map\0" as *const u8 as *const c_char,
        signature: b"a\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"input_panel_state\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"preedit_string\0" as *const u8 as *const c_char,
        signature: b"uss\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"preedit_styling\0" as *const u8 as *const c_char,
        signature: b"uuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"preedit_cursor\0" as *const u8 as *const c_char,
        signature: b"i\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"commit_string\0" as *const u8 as *const c_char,
        signature: b"us\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"cursor_position\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"delete_surrounding_text\0" as *const u8 as *const c_char,
        signature: b"iu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"keysym\0" as *const u8 as *const c_char,
        signature: b"uuuuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"language\0" as *const u8 as *const c_char,
        signature: b"us\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"text_direction\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_text_input_v1_interface: wl_interface = wl_interface {
    name: b"zwp_text_input_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 11,
    requests: unsafe { &zwp_text_input_v1_requests as *const _ },
    event_count: 13,
    events: unsafe { &zwp_text_input_v1_events as *const _ },
};
static mut zwp_text_input_manager_v1_requests_create_text_input_types: [*const wl_interface; 1] =
    [unsafe { &zwp_text_input_v1_interface as *const wl_interface }];
pub static mut zwp_text_input_manager_v1_requests: [wl_message; 1] = [wl_message {
    name: b"create_text_input\0" as *const u8 as *const c_char,
    signature: b"n\0" as *const u8 as *const c_char,
    types: unsafe { &zwp_text_input_manager_v1_requests_create_text_input_types as *const _ },
}];
pub static mut zwp_text_input_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwp_text_input_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_text_input_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
