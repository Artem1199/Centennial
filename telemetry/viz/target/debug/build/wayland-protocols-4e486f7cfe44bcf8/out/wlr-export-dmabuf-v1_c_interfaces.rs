use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 10] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut zwlr_export_dmabuf_manager_v1_requests_capture_output_types: [*const wl_interface; 3] = [
    unsafe { &zwlr_export_dmabuf_frame_v1_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    unsafe { &wl_output_interface as *const wl_interface },
];
pub static mut zwlr_export_dmabuf_manager_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"capture_output\0" as *const u8 as *const c_char,
        signature: b"nio\0" as *const u8 as *const c_char,
        types: unsafe { &zwlr_export_dmabuf_manager_v1_requests_capture_output_types as *const _ },
    },
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_export_dmabuf_manager_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_export_dmabuf_manager_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zwlr_export_dmabuf_manager_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zwlr_export_dmabuf_frame_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zwlr_export_dmabuf_frame_v1_events: [wl_message; 4] = [
    wl_message {
        name: b"frame\0" as *const u8 as *const c_char,
        signature: b"uuuuuuuuuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"object\0" as *const u8 as *const c_char,
        signature: b"uhuuuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"ready\0" as *const u8 as *const c_char,
        signature: b"uuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"cancel\0" as *const u8 as *const c_char,
        signature: b"u\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
];
pub static mut zwlr_export_dmabuf_frame_v1_interface: wl_interface = wl_interface {
    name: b"zwlr_export_dmabuf_frame_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zwlr_export_dmabuf_frame_v1_requests as *const _ },
    event_count: 4,
    events: unsafe { &zwlr_export_dmabuf_frame_v1_events as *const _ },
};
