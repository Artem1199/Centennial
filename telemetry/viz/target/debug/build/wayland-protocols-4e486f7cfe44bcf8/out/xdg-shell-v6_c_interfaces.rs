use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zxdg_shell_v6_requests_create_positioner_types: [*const wl_interface; 1] =
    [unsafe { &zxdg_positioner_v6_interface as *const wl_interface }];
static mut zxdg_shell_v6_requests_get_xdg_surface_types: [*const wl_interface; 2] = [
    unsafe { &zxdg_surface_v6_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zxdg_shell_v6_requests: [wl_message; 4] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"create_positioner\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_shell_v6_requests_create_positioner_types as *const _ },
    },
    wl_message {
        name: b"get_xdg_surface\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_shell_v6_requests_get_xdg_surface_types as *const _ },
    },
    wl_message {
        name: b"pong\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zxdg_shell_v6_events: [wl_message; 1] = [wl_message {
    name: b"ping\0" as *const u8 as *const c_char,
    signature: b"u\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zxdg_shell_v6_interface: wl_interface = wl_interface {
    name: b"zxdg_shell_v6\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 4,
    requests: unsafe { &zxdg_shell_v6_requests as *const _ },
    event_count: 1,
    events: unsafe { &zxdg_shell_v6_events as *const _ },
};
pub static mut zxdg_positioner_v6_requests: [wl_message; 7] = [
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
pub static mut zxdg_positioner_v6_interface: wl_interface = wl_interface {
    name: b"zxdg_positioner_v6\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 7,
    requests: unsafe { &zxdg_positioner_v6_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
static mut zxdg_surface_v6_requests_get_toplevel_types: [*const wl_interface; 1] =
    [unsafe { &zxdg_toplevel_v6_interface as *const wl_interface }];
static mut zxdg_surface_v6_requests_get_popup_types: [*const wl_interface; 3] = [
    unsafe { &zxdg_popup_v6_interface as *const wl_interface },
    unsafe { &zxdg_surface_v6_interface as *const wl_interface },
    unsafe { &zxdg_positioner_v6_interface as *const wl_interface },
];
pub static mut zxdg_surface_v6_requests: [wl_message; 5] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"get_toplevel\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_surface_v6_requests_get_toplevel_types as *const _ },
    },
    wl_message {
        name: b"get_popup\0" as *const u8 as *const c_char,
        signature: b"noo\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_surface_v6_requests_get_popup_types as *const _ },
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
pub static mut zxdg_surface_v6_events: [wl_message; 1] = [wl_message {
    name: b"configure\0" as *const u8 as *const c_char,
    signature: b"u\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zxdg_surface_v6_interface: wl_interface = wl_interface {
    name: b"zxdg_surface_v6\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 5,
    requests: unsafe { &zxdg_surface_v6_requests as *const _ },
    event_count: 1,
    events: unsafe { &zxdg_surface_v6_events as *const _ },
};
static mut zxdg_toplevel_v6_requests_set_parent_types: [*const wl_interface; 1] =
    [unsafe { &zxdg_toplevel_v6_interface as *const wl_interface }];
static mut zxdg_toplevel_v6_requests_show_window_menu_types: [*const wl_interface; 4] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zxdg_toplevel_v6_requests_move_types: [*const wl_interface; 2] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
static mut zxdg_toplevel_v6_requests_resize_types: [*const wl_interface; 3] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zxdg_toplevel_v6_requests_set_fullscreen_types: [*const wl_interface; 1] =
    [unsafe { &wl_output_interface as *const wl_interface }];
pub static mut zxdg_toplevel_v6_requests: [wl_message; 14] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_parent\0" as *const u8 as *const c_char,
        signature: b"?o\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_toplevel_v6_requests_set_parent_types as *const _ },
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
        types: unsafe { &zxdg_toplevel_v6_requests_show_window_menu_types as *const _ },
    },
    wl_message {
        name: b"move\0" as *const u8 as *const c_char,
        signature: b"ou\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_toplevel_v6_requests_move_types as *const _ },
    },
    wl_message {
        name: b"resize\0" as *const u8 as *const c_char,
        signature: b"ouu\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_toplevel_v6_requests_resize_types as *const _ },
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
        types: unsafe { &zxdg_toplevel_v6_requests_set_fullscreen_types as *const _ },
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
pub static mut zxdg_toplevel_v6_events: [wl_message; 2] = [
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
pub static mut zxdg_toplevel_v6_interface: wl_interface = wl_interface {
    name: b"zxdg_toplevel_v6\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 14,
    requests: unsafe { &zxdg_toplevel_v6_requests as *const _ },
    event_count: 2,
    events: unsafe { &zxdg_toplevel_v6_events as *const _ },
};
static mut zxdg_popup_v6_requests_grab_types: [*const wl_interface; 2] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut zxdg_popup_v6_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"grab\0" as *const u8 as *const c_char,
        signature: b"ou\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_popup_v6_requests_grab_types as *const _ },
    },
];
pub static mut zxdg_popup_v6_events: [wl_message; 2] = [
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
pub static mut zxdg_popup_v6_interface: wl_interface = wl_interface {
    name: b"zxdg_popup_v6\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zxdg_popup_v6_requests as *const _ },
    event_count: 2,
    events: unsafe { &zxdg_popup_v6_events as *const _ },
};
