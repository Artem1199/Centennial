use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 3] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zwp_tablet_manager_v2_requests_get_tablet_seat_types: [*const wl_interface; 2] = [
    unsafe { &zwp_tablet_seat_v2_interface as *const wl_interface },
    unsafe { &wl_seat_interface as *const wl_interface },
];
pub static mut zwp_tablet_manager_v2_requests: [wl_message; 2] = [
    wl_message {
        name: b"get_tablet_seat\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_manager_v2_requests_get_tablet_seat_types as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_manager_v2_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_manager_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_tablet_manager_v2_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zwp_tablet_seat_v2_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
static mut zwp_tablet_seat_v2_events_tablet_added_types: [*const wl_interface; 1] =
    [unsafe { &zwp_tablet_v2_interface as *const wl_interface }];
static mut zwp_tablet_seat_v2_events_tool_added_types: [*const wl_interface; 1] =
    [unsafe { &zwp_tablet_tool_v2_interface as *const wl_interface }];
static mut zwp_tablet_seat_v2_events_pad_added_types: [*const wl_interface; 1] =
    [unsafe { &zwp_tablet_pad_v2_interface as *const wl_interface }];
pub static mut zwp_tablet_seat_v2_events: [wl_message; 3] = [
    wl_message {
        name: b"tablet_added\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_seat_v2_events_tablet_added_types as *const _ },
    },
    wl_message {
        name: b"tool_added\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_seat_v2_events_tool_added_types as *const _ },
    },
    wl_message {
        name: b"pad_added\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_seat_v2_events_pad_added_types as *const _ },
    },
];
pub static mut zwp_tablet_seat_v2_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_seat_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_tablet_seat_v2_requests as *const _ },
    event_count: 3,
    events: unsafe { &zwp_tablet_seat_v2_events as *const _ },
};
static mut zwp_tablet_tool_v2_requests_set_cursor_types: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut zwp_tablet_tool_v2_requests: [wl_message; 2] = [
    wl_message {
        name: b"set_cursor\0" as *const u8 as *const c_char,
        signature: b"u?oii\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_tool_v2_requests_set_cursor_types as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
static mut zwp_tablet_tool_v2_events_proximity_in_types: [*const wl_interface; 3] = [
    NULLPTR as *const wl_interface,
    unsafe { &zwp_tablet_v2_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zwp_tablet_tool_v2_events: [wl_message; 19] = [
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
        types: unsafe { &zwp_tablet_tool_v2_events_proximity_in_types as *const _ },
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
        signature: b"ff\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"rotation\0" as *const u8 as *const c_char,
        signature: b"f\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"slider\0" as *const u8 as *const c_char,
        signature: b"i\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"wheel\0" as *const u8 as *const c_char,
        signature: b"fi\0" as *const u8 as *const c_char,
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
pub static mut zwp_tablet_tool_v2_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_tool_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_tablet_tool_v2_requests as *const _ },
    event_count: 19,
    events: unsafe { &zwp_tablet_tool_v2_events as *const _ },
};
pub static mut zwp_tablet_v2_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zwp_tablet_v2_events: [wl_message; 5] = [
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
pub static mut zwp_tablet_v2_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_tablet_v2_requests as *const _ },
    event_count: 5,
    events: unsafe { &zwp_tablet_v2_events as *const _ },
};
pub static mut zwp_tablet_pad_ring_v2_requests: [wl_message; 2] = [
    wl_message {
        name: b"set_feedback\0" as *const u8 as *const c_char,
        signature: b"su\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_pad_ring_v2_events: [wl_message; 4] = [
    wl_message {
        name: b"source\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"angle\0" as *const u8 as *const c_char,
        signature: b"f\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"stop\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"frame\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_pad_ring_v2_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_pad_ring_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_tablet_pad_ring_v2_requests as *const _ },
    event_count: 4,
    events: unsafe { &zwp_tablet_pad_ring_v2_events as *const _ },
};
pub static mut zwp_tablet_pad_strip_v2_requests: [wl_message; 2] = [
    wl_message {
        name: b"set_feedback\0" as *const u8 as *const c_char,
        signature: b"su\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_pad_strip_v2_events: [wl_message; 4] = [
    wl_message {
        name: b"source\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"position\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"stop\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"frame\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_pad_strip_v2_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_pad_strip_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_tablet_pad_strip_v2_requests as *const _ },
    event_count: 4,
    events: unsafe { &zwp_tablet_pad_strip_v2_events as *const _ },
};
pub static mut zwp_tablet_pad_group_v2_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
static mut zwp_tablet_pad_group_v2_events_ring_types: [*const wl_interface; 1] =
    [unsafe { &zwp_tablet_pad_ring_v2_interface as *const wl_interface }];
static mut zwp_tablet_pad_group_v2_events_strip_types: [*const wl_interface; 1] =
    [unsafe { &zwp_tablet_pad_strip_v2_interface as *const wl_interface }];
pub static mut zwp_tablet_pad_group_v2_events: [wl_message; 6] = [
    wl_message {
        name: b"buttons\0" as *const u8 as *const c_char,
        signature: b"a\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"ring\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_pad_group_v2_events_ring_types as *const _ },
    },
    wl_message {
        name: b"strip\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_pad_group_v2_events_strip_types as *const _ },
    },
    wl_message {
        name: b"modes\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"done\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"mode_switch\0" as *const u8 as *const c_char,
        signature: b"uuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_pad_group_v2_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_pad_group_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_tablet_pad_group_v2_requests as *const _ },
    event_count: 6,
    events: unsafe { &zwp_tablet_pad_group_v2_events as *const _ },
};
pub static mut zwp_tablet_pad_v2_requests: [wl_message; 2] = [
    wl_message {
        name: b"set_feedback\0" as *const u8 as *const c_char,
        signature: b"usu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
static mut zwp_tablet_pad_v2_events_group_types: [*const wl_interface; 1] =
    [unsafe { &zwp_tablet_pad_group_v2_interface as *const wl_interface }];
static mut zwp_tablet_pad_v2_events_enter_types: [*const wl_interface; 3] = [
    NULLPTR as *const wl_interface,
    unsafe { &zwp_tablet_v2_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
static mut zwp_tablet_pad_v2_events_leave_types: [*const wl_interface; 2] =
    [NULLPTR as *const wl_interface, unsafe {
        &wl_surface_interface as *const wl_interface
    }];
pub static mut zwp_tablet_pad_v2_events: [wl_message; 8] = [
    wl_message {
        name: b"group\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_pad_v2_events_group_types as *const _ },
    },
    wl_message {
        name: b"path\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"buttons\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"done\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"button\0" as *const u8 as *const c_char,
        signature: b"uuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"enter\0" as *const u8 as *const c_char,
        signature: b"uoo\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_pad_v2_events_enter_types as *const _ },
    },
    wl_message {
        name: b"leave\0" as *const u8 as *const c_char,
        signature: b"uo\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_tablet_pad_v2_events_leave_types as *const _ },
    },
    wl_message {
        name: b"removed\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_tablet_pad_v2_interface: wl_interface = wl_interface {
    name: b"zwp_tablet_pad_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_tablet_pad_v2_requests as *const _ },
    event_count: 8,
    events: unsafe { &zwp_tablet_pad_v2_events as *const _ },
};
