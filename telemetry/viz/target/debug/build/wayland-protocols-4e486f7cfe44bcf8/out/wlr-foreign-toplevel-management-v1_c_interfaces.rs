use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 1] = [NULLPTR as *const wl_interface];
pub static mut zwlr_foreign_toplevel_manager_v1_requests: [wl_message; 1] = [wl_message {
    name: b"stop\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
static mut zwlr_foreign_toplevel_manager_v1_events_toplevel_types: [*const wl_interface; 1] =
    [unsafe { &zwlr_foreign_toplevel_handle_v1_interface as *const wl_interface }];
pub static mut zwlr_foreign_toplevel_manager_v1_events: [wl_message; 2] = [
    wl_message {
        name: b"toplevel\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_foreign_toplevel_manager_v1_events_toplevel_types as *const _ },
    },
    wl_message {
        name: b"finished\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_foreign_toplevel_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_foreign_toplevel_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwlr_foreign_toplevel_manager_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwlr_foreign_toplevel_manager_v1_events as *const _ },
};
static mut zwlr_foreign_toplevel_handle_v1_requests_activate_types: [*const wl_interface; 1] =
    [unsafe { &wl_seat_interface as *const wl_interface }];
static mut zwlr_foreign_toplevel_handle_v1_requests_set_rectangle_types: [*const wl_interface; 5] = [
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut zwlr_foreign_toplevel_handle_v1_requests: [wl_message; 8] = [
    wl_message {
        name: b"set_maximized\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"unset_maximized\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_minimized\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"unset_minimized\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"activate\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_foreign_toplevel_handle_v1_requests_activate_types as *const _ },
    },
    wl_message {
        name: b"close\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_rectangle\0" as *const u8 as *const c_char,
        signature: b"oiiii\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_foreign_toplevel_handle_v1_requests_set_rectangle_types as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
static mut zwlr_foreign_toplevel_handle_v1_events_output_enter_types: [*const wl_interface; 1] =
    [unsafe { &wl_output_interface as *const wl_interface }];
static mut zwlr_foreign_toplevel_handle_v1_events_output_leave_types: [*const wl_interface; 1] =
    [unsafe { &wl_output_interface as *const wl_interface }];
pub static mut zwlr_foreign_toplevel_handle_v1_events: [wl_message; 7] = [
    wl_message {
        name: b"title\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"app_id\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"output_enter\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_foreign_toplevel_handle_v1_events_output_enter_types as *const _ },
    },
    wl_message {
        name: b"output_leave\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_foreign_toplevel_handle_v1_events_output_leave_types as *const _ },
    },
    wl_message {
        name: b"state\0" as *const u8 as *const c_char,
        signature: b"a\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"done\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"closed\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_foreign_toplevel_handle_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_foreign_toplevel_handle_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 8,
    requests: unsafe { &zwlr_foreign_toplevel_handle_v1_requests as *const _ },
    event_count: 7,
    events: unsafe { &zwlr_foreign_toplevel_handle_v1_events as *const _ },
};
