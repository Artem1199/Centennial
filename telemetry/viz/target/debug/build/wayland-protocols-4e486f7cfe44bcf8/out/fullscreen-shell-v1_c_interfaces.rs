use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 1] = [NULLPTR as *const wl_interface];
static mut zwp_fullscreen_shell_v1_requests_present_surface_types: [*const wl_interface; 3] = [
    unsafe { &wl_surface_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    unsafe { &wl_output_interface as *const wl_interface },
];
static mut zwp_fullscreen_shell_v1_requests_present_surface_for_mode_types: [*const wl_interface;
    4] = [
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wl_output_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    unsafe { &zwp_fullscreen_shell_mode_feedback_v1_interface as *const wl_interface },
];
pub static mut zwp_fullscreen_shell_v1_requests: [wl_message; 3] = [
    wl_message {
        name: b"release\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"present_surface\0" as *const u8 as *const c_char,
        signature: b"?ou?o\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_fullscreen_shell_v1_requests_present_surface_types as *const _ },
    },
    wl_message {
        name: b"present_surface_for_mode\0" as *const u8 as *const c_char,
        signature: b"ooin\0" as *const u8 as *const c_char,
        types: unsafe {
            &zwp_fullscreen_shell_v1_requests_present_surface_for_mode_types as *const _
        },
    },
];
pub static mut zwp_fullscreen_shell_v1_events: [wl_message; 1] = [wl_message {
    name: b"capability\0" as *const u8 as *const c_char,
    signature: b"u\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zwp_fullscreen_shell_v1_interface: wl_interface = wl_interface {
    name: b"zwp_fullscreen_shell_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &zwp_fullscreen_shell_v1_requests as *const _ },
    event_count: 1,
    events: unsafe { &zwp_fullscreen_shell_v1_events as *const _ },
};
pub static mut zwp_fullscreen_shell_mode_feedback_v1_events: [wl_message; 3] = [
    wl_message {
        name: b"mode_successful\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"mode_failed\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"present_cancelled\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwp_fullscreen_shell_mode_feedback_v1_interface: wl_interface = wl_interface {
    name: b"zwp_fullscreen_shell_mode_feedback_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 0,
    requests: NULLPTR as *const wl_message,
    event_count: 3,
    events: unsafe { &zwp_fullscreen_shell_mode_feedback_v1_events as *const _ },
};
