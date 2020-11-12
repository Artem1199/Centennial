use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 2] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zwlr_data_control_manager_v1_requests_create_data_source_types: [*const wl_interface;
    1] = [unsafe { &zwlr_data_control_source_v1_interface as *const wl_interface }];
static mut zwlr_data_control_manager_v1_requests_get_data_device_types: [*const wl_interface; 2] = [
    unsafe { &zwlr_data_control_device_v1_interface as *const wl_interface },
    unsafe { &wl_seat_interface as *const wl_interface },
];
pub static mut zwlr_data_control_manager_v1_requests: [wl_message; 3] = [
    wl_message {
        name: b"create_data_source\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe {
            &zwlr_data_control_manager_v1_requests_create_data_source_types as *const _
        },
    },
    wl_message {
        name: b"get_data_device\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_data_control_manager_v1_requests_get_data_device_types as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_data_control_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_data_control_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &zwlr_data_control_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
static mut zwlr_data_control_device_v1_requests_set_selection_types: [*const wl_interface; 1] =
    [unsafe { &zwlr_data_control_source_v1_interface as *const wl_interface }];
pub static mut zwlr_data_control_device_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"set_selection\0" as *const u8 as *const c_char,
        signature: b"?o\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_data_control_device_v1_requests_set_selection_types as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
static mut zwlr_data_control_device_v1_events_data_offer_types: [*const wl_interface; 1] =
    [unsafe { &zwlr_data_control_offer_v1_interface as *const wl_interface }];
static mut zwlr_data_control_device_v1_events_selection_types: [*const wl_interface; 1] =
    [unsafe { &zwlr_data_control_offer_v1_interface as *const wl_interface }];
pub static mut zwlr_data_control_device_v1_events: [wl_message; 3] = [
    wl_message {
        name: b"data_offer\0" as *const u8 as *const c_char,
        signature: b"n\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_data_control_device_v1_events_data_offer_types as *const _ },
    },
    wl_message {
        name: b"selection\0" as *const u8 as *const c_char,
        signature: b"?o\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_data_control_device_v1_events_selection_types as *const _ },
    },
    wl_message {
        name: b"finished\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_data_control_device_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_data_control_device_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwlr_data_control_device_v1_requests as *const _ },
    event_count: 3,
    events: unsafe { &zwlr_data_control_device_v1_events as *const _ },
};
pub static mut zwlr_data_control_source_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"offer\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_data_control_source_v1_events: [wl_message; 2] = [
    wl_message {
        name: b"send\0" as *const u8 as *const c_char,
        signature: b"sh\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"cancelled\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_data_control_source_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_data_control_source_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwlr_data_control_source_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwlr_data_control_source_v1_events as *const _ },
};
pub static mut zwlr_data_control_offer_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"receive\0" as *const u8 as *const c_char,
        signature: b"sh\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_data_control_offer_v1_events: [wl_message; 1] = [wl_message {
    name: b"offer\0" as *const u8 as *const c_char,
    signature: b"s\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zwlr_data_control_offer_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_data_control_offer_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwlr_data_control_offer_v1_requests as *const _ },
    event_count: 1,
    events: unsafe { &zwlr_data_control_offer_v1_events as *const _ },
};
