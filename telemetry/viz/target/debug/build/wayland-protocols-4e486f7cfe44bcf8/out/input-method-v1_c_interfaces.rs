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
static mut zwp_input_method_context_v1_requests_grab_keyboard_types: [*const wl_interface; 1] =
    [unsafe { &wl_keyboard_interface as *const wl_interface }];
pub static mut zwp_input_method_context_v1_requests: [wl_message; 14] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"commit_string\0" as *const u8 as *const c_char,
        signature: b"us\0" as *const u8 as *const c_char,
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
        name: b"delete_surrounding_text\0" as *const u8 as *const c_char,
        signature: b"iu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"cursor_position\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"modifiers_map\0" as *const u8 as *const c_char,
        signature: b"a\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"keysym\0" as *const u8 as *const c_char,
        signature: b"uuuuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"grab_keyboard\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_input_method_context_v1_requests_grab_keyboard_types as *const _ },
    },
    wl_message {
        name: b"key\0" as *const u8 as *const c_char,
        signature: b"uuuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"modifiers\0" as *const u8 as *const c_char,
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
pub static mut zwp_input_method_context_v1_events: [wl_message; 6] = [
    wl_message {
        name: b"surrounding_text\0" as *const u8 as *const c_char,
        signature: b"suu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"reset\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"content_type\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"invoke_action\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"commit_state\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"preferred_language\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_input_method_context_v1_interface: wl_interface = wl_interface {
    name: b"zwp_input_method_context_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 14,
    requests: unsafe { &zwp_input_method_context_v1_requests as *const _ },
    event_count: 6,
    events: unsafe { &zwp_input_method_context_v1_events as *const _ },
};
static mut zwp_input_method_v1_events_activate_types: [*const wl_interface; 1] =
    [unsafe { &zwp_input_method_context_v1_interface as *const wl_interface }];
static mut zwp_input_method_v1_events_deactivate_types: [*const wl_interface; 1] =
    [unsafe { &zwp_input_method_context_v1_interface as *const wl_interface }];
pub static mut zwp_input_method_v1_events: [wl_message; 2] = [
    wl_message {
        name: b"activate\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_input_method_v1_events_activate_types as *const _ },
    },
    wl_message {
        name: b"deactivate\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_input_method_v1_events_deactivate_types as *const _ },
    },
];
pub static mut zwp_input_method_v1_interface: wl_interface = wl_interface {
    name: b"zwp_input_method_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 0,
    requests: NULLPTR as *const wl_message,
    event_count: 2,
    events: unsafe { &zwp_input_method_v1_events as *const _ },
};
static mut zwp_input_panel_v1_requests_get_input_panel_surface_types: [*const wl_interface; 2] = [
    unsafe { &zwp_input_panel_surface_v1_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zwp_input_panel_v1_requests: [wl_message; 1] = [wl_message {
    name: b"get_input_panel_surface\0" as *const u8 as *const c_char,
    signature: b"no\0" as *const u8 as *const c_char,
    types: unsafe { &zwp_input_panel_v1_requests_get_input_panel_surface_types as *const _ },
}];
pub static mut zwp_input_panel_v1_interface: wl_interface = wl_interface {
    name: b"zwp_input_panel_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_input_panel_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
static mut zwp_input_panel_surface_v1_requests_set_toplevel_types: [*const wl_interface; 2] = [
    unsafe { &wl_output_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut zwp_input_panel_surface_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"set_toplevel\0" as *const u8 as *const c_char,
        signature: b"ou\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_input_panel_surface_v1_requests_set_toplevel_types as *const _ },
    },
    wl_message {
        name: b"set_overlay_panel\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_input_panel_surface_v1_interface: wl_interface = wl_interface {
    name: b"zwp_input_panel_surface_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_input_panel_surface_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
