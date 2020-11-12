use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 3] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zwp_tablet_manager_v1_requests_get_tablet_seat_types: [*const wl_interface; 2] = [
    unsafe { &zwp_tablet_seat_v1_interface as *const wl_interface },
    unsafe { &wl_seat_interface as *const wl_interface },
];
pub static mut zwp_tablet_manager_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"get_tablet_seat\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_manager_v1_requests_get_tablet_seat_types as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_tablet_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zwp_tablet_seat_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
static mut zwp_tablet_seat_v1_events_tablet_added_types: [*const wl_interface; 1] =
    [unsafe { &zwp_tablet_v1_interface as *const wl_interface }];
static mut zwp_tablet_seat_v1_events_tool_added_types: [*const wl_interface; 1] =
    [unsafe { &zwp_tablet_tool_v1_interface as *const wl_interface }];
pub static mut zwp_tablet_seat_v1_events: [wl_message; 2] = [
    wl_message {
        name: b"tablet_added\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_seat_v1_events_tablet_added_types as *const _ },
    },
    wl_message {
        name: b"tool_added\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_seat_v1_events_tool_added_types as *const _ },
    },
];
pub static mut zwp_tablet_seat_v1_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_seat_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_tablet_seat_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwp_tablet_seat_v1_events as *const _ },
};
static mut zwp_tablet_tool_v1_requests_set_cursor_types: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut zwp_tablet_tool_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"set_cursor\0" as *const u8 as *const c_char,
        signature: b"u?oii\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_tool_v1_requests_set_cursor_types as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
static mut zwp_tablet_tool_v1_events_proximity_in_types: [*const wl_interface; 3] = [
    NULLPTR as *const wl_interface,
    unsafe { &zwp_tablet_v1_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zwp_tablet_tool_v1_events: [wl_message; 19] = [
    wl_message {
        name: b"type\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"hardware_serial\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"hardware_id_wacom\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"capability\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"done\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"removed\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"proximity_in\0" as *const u8 as *const c_char,
        signature: b"uoo\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_tool_v1_events_proximity_in_types as *const _ },
    },
    wl_message {
        name: b"proximity_out\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"down\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"up\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"motion\0" as *const u8 as *const c_char,
        signature: b"ff\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"pressure\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"distance\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"tilt\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"rotation\0" as *const u8 as *const c_char,
        signature: b"i\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"slider\0" as *const u8 as *const c_char,
        signature: b"i\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"wheel\0" as *const u8 as *const c_char,
        signature: b"ii\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"button\0" as *const u8 as *const c_char,
        signature: b"uuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"frame\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_tool_v1_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_tool_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_tablet_tool_v1_requests as *const _ },
    event_count: 19,
    events: unsafe { &zwp_tablet_tool_v1_events as *const _ },
};
pub static mut zwp_tablet_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zwp_tablet_v1_events: [wl_message; 5] = [
    wl_message {
        name: b"name\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"id\0" as *const u8 as *const c_char,
        signature: b"uu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"path\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"done\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"removed\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_v1_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_tablet_v1_requests as *const _ },
    event_count: 5,
    events: unsafe { &zwp_tablet_v1_events as *const _ },
};
