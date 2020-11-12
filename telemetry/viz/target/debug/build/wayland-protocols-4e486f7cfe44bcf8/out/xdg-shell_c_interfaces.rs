use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut xdg_wm_base_requests_create_positioner_types: [*const wl_interface; 1] =
    [unsafe { &xdg_positioner_interface as *const wl_interface }];
static mut xdg_wm_base_requests_get_xdg_surface_types: [*const wl_interface; 2] = [
    unsafe { &xdg_surface_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut xdg_wm_base_requests: [wl_message; 4] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"create_positioner\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_wm_base_requests_create_positioner_types as *const _ },
    },
    wl_message {
        name: b"get_xdg_surface\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_wm_base_requests_get_xdg_surface_types as *const _ },
    },
    wl_message {
        name: b"pong\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut xdg_wm_base_events: [wl_message; 1] = [wl_message {
    name: b"ping\0" as *const u8 as *const c_char,
    signature: b"u\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut xdg_wm_base_interface: wl_interface = wl_interface {
    name: b"xdg_wm_base\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 4,
    requests: unsafe { &xdg_wm_base_requests as *const _ },
    event_count: 1,
    events: unsafe { &xdg_wm_base_events as *const _ },
};
pub static mut xdg_positioner_requests: [wl_message; 7] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_size\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_anchor_rect\0" as *const u8 as *const c_char,
        signature: b"iiii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_anchor\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_gravity\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_constraint_adjustment\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_offset\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut xdg_positioner_interface: wl_interface = wl_interface {
    name: b"xdg_positioner\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 7,
    requests: unsafe { &xdg_positioner_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
static mut xdg_surface_requests_get_toplevel_types: [*const wl_interface; 1] =
    [unsafe { &xdg_toplevel_interface as *const wl_interface }];
static mut xdg_surface_requests_get_popup_types: [*const wl_interface; 3] = [
    unsafe { &xdg_popup_interface as *const wl_interface },
    unsafe { &xdg_surface_interface as *const wl_interface },
    unsafe { &xdg_positioner_interface as *const wl_interface },
];
pub static mut xdg_surface_requests: [wl_message; 5] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_toplevel\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_surface_requests_get_toplevel_types as *const _ },
    },
    wl_message {
        name: b"get_popup\0" as *const u8 as *const c_char,
        signature: b"n?oo\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_surface_requests_get_popup_types as *const _ },
    },
    wl_message {
        name: b"set_window_geometry\0" as *const u8 as *const c_char,
        signature: b"iiii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"ack_configure\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut xdg_surface_events: [wl_message; 1] = [wl_message {
    name: b"configure\0" as *const u8 as *const c_char,
    signature: b"u\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut xdg_surface_interface: wl_interface = wl_interface {
    name: b"xdg_surface\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 5,
    requests: unsafe { &xdg_surface_requests as *const _ },
    event_count: 1,
    events: unsafe { &xdg_surface_events as *const _ },
};
static mut xdg_toplevel_requests_set_parent_types: [*const wl_interface; 1] =
    [unsafe { &xdg_toplevel_interface as *const wl_interface }];
static mut xdg_toplevel_requests_show_window_menu_types: [*const wl_interface; 4] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut xdg_toplevel_requests_move_types: [*const wl_interface; 2] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
static mut xdg_toplevel_requests_resize_types: [*const wl_interface; 3] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut xdg_toplevel_requests_set_fullscreen_types: [*const wl_interface; 1] =
    [unsafe { &wl_output_interface as *const wl_interface }];
pub static mut xdg_toplevel_requests: [wl_message; 14] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_parent\0" as *const u8 as *const c_char,
        signature: b"?o\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_toplevel_requests_set_parent_types as *const _ },
    },
    wl_message {
        name: b"set_title\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_app_id\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"show_window_menu\0" as *const u8 as *const c_char,
        signature: b"ouii\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_toplevel_requests_show_window_menu_types as *const _ },
    },
    wl_message {
        name: b"move\0" as *const u8 as *const c_char,
        signature: b"ou\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_toplevel_requests_move_types as *const _ },
    },
    wl_message {
        name: b"resize\0" as *const u8 as *const c_char,
        signature: b"ouu\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_toplevel_requests_resize_types as *const _ },
    },
    wl_message {
        name: b"set_max_size\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_min_size\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
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
        name: b"set_fullscreen\0" as *const u8 as *const c_char,
        signature: b"?o\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_toplevel_requests_set_fullscreen_types as *const _ },
    },
    wl_message {
        name: b"unset_fullscreen\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_minimized\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut xdg_toplevel_events: [wl_message; 2] = [
    wl_message {
        name: b"configure\0" as *const u8 as *const c_char,
        signature: b"iia\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"close\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut xdg_toplevel_interface: wl_interface = wl_interface {
    name: b"xdg_toplevel\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 14,
    requests: unsafe { &xdg_toplevel_requests as *const _ },
    event_count: 2,
    events: unsafe { &xdg_toplevel_events as *const _ },
};
static mut xdg_popup_requests_grab_types: [*const wl_interface; 2] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut xdg_popup_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"grab\0" as *const u8 as *const c_char,
        signature: b"ou\0" as *const u8 as *const c_char,
        types: unsafe { &xdg_popup_requests_grab_types as *const _ },
    },
];
pub static mut xdg_popup_events: [wl_message; 2] = [
    wl_message {
        name: b"configure\0" as *const u8 as *const c_char,
        signature: b"iiii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"popup_done\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut xdg_popup_interface: wl_interface = wl_interface {
    name: b"xdg_popup\0" as *const u8 as *const c_char,
    version: 2,
    request_count: 2,
    requests: unsafe { &xdg_popup_requests as *const _ },
    event_count: 2,
    events: unsafe { &xdg_popup_events as *const _ },
};
