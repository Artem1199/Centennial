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
static mut zwp_pointer_gestures_v1_requests_get_swipe_gesture_types: [*const wl_interface; 2] = [
    unsafe { &zwp_pointer_gesture_swipe_v1_interface as *const wl_interface },
    unsafe { &wl_pointer_interface as *const wl_interface },
];
static mut zwp_pointer_gestures_v1_requests_get_pinch_gesture_types: [*const wl_interface; 2] = [
    unsafe { &zwp_pointer_gesture_pinch_v1_interface as *const wl_interface },
    unsafe { &wl_pointer_interface as *const wl_interface },
];
pub static mut zwp_pointer_gestures_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"get_swipe_gesture\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_pointer_gestures_v1_requests_get_swipe_gesture_types as *const _ },
    },
    wl_message {
        name: b"get_pinch_gesture\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_pointer_gestures_v1_requests_get_pinch_gesture_types as *const _ },
    },
];
pub static mut zwp_pointer_gestures_v1_interface: wl_interface = wl_interface {
    name: b"zwp_pointer_gestures_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwp_pointer_gestures_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zwp_pointer_gesture_swipe_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
static mut zwp_pointer_gesture_swipe_v1_events_begin_types: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut zwp_pointer_gesture_swipe_v1_events: [wl_message; 3] = [
    wl_message {
        name: b"begin\0" as *const u8 as *const c_char,
        signature: b"uuou\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_pointer_gesture_swipe_v1_events_begin_types as *const _ },
    },
    wl_message {
        name: b"update\0" as *const u8 as *const c_char,
        signature: b"uff\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"end\0" as *const u8 as *const c_char,
        signature: b"uui\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_pointer_gesture_swipe_v1_interface: wl_interface = wl_interface {
    name: b"zwp_pointer_gesture_swipe_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_pointer_gesture_swipe_v1_requests as *const _ },
    event_count: 3,
    events: unsafe { &zwp_pointer_gesture_swipe_v1_events as *const _ },
};
pub static mut zwp_pointer_gesture_pinch_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
static mut zwp_pointer_gesture_pinch_v1_events_begin_types: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut zwp_pointer_gesture_pinch_v1_events: [wl_message; 3] = [
    wl_message {
        name: b"begin\0" as *const u8 as *const c_char,
        signature: b"uuou\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_pointer_gesture_pinch_v1_events_begin_types as *const _ },
    },
    wl_message {
        name: b"update\0" as *const u8 as *const c_char,
        signature: b"uffff\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"end\0" as *const u8 as *const c_char,
        signature: b"uui\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_pointer_gesture_pinch_v1_interface: wl_interface = wl_interface {
    name: b"zwp_pointer_gesture_pinch_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwp_pointer_gesture_pinch_v1_requests as *const _ },
    event_count: 3,
    events: unsafe { &zwp_pointer_gesture_pinch_v1_events as *const _ },
};
