use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 7] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut wp_presentation_requests_feedback_types: [*const wl_interface; 2] = [
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wp_presentation_feedback_interface as *const wl_interface },
];
pub static mut wp_presentation_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"feedback\0" as *const u8 as *const c_char,
        signature: b"on\0" as *const u8 as *const c_char,
        types: unsafe { &wp_presentation_requests_feedback_types as *const _ },
    },
];
pub static mut wp_presentation_events: [wl_message; 1] = [wl_message {
    name: b"clock_id\0" as *const u8 as *const c_char,
    signature: b"u\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut wp_presentation_interface: wl_interface = wl_interface {
    name: b"wp_presentation\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &wp_presentation_requests as *const _ },
    event_count: 1,
    events: unsafe { &wp_presentation_events as *const _ },
};
static mut wp_presentation_feedback_events_sync_output_types: [*const wl_interface; 1] =
    [unsafe { &wl_output_interface as *const wl_interface }];
pub static mut wp_presentation_feedback_events: [wl_message; 3] = [
    wl_message {
        name: b"sync_output\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &wp_presentation_feedback_events_sync_output_types as *const _ },
    },
    wl_message {
        name: b"presented\0" as *const u8 as *const c_char,
        signature: b"uuuuuuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"discarded\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut wp_presentation_feedback_interface: wl_interface = wl_interface {
    name: b"wp_presentation_feedback\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 0,
    requests: NULLPTR as *const wl_message,
    event_count: 3,
    events: unsafe { &wp_presentation_feedback_events as *const _ },
};
