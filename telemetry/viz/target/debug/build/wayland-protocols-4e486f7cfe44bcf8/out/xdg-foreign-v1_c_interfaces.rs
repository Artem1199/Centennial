use std::os::raw::{c_char, c_void};
use wayland_sys::common::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const wl_interface; 1] = [NULLPTR as *const wl_interface];
static mut zxdg_exporter_v1_requests_export_types: [*const wl_interface; 2] = [
    unsafe { &zxdg_exported_v1_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zxdg_exporter_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"export\0" as *const u8 as *const c_char,
        signature: b"no\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_exporter_v1_requests_export_types as *const _ },
    },
];
pub static mut zxdg_exporter_v1_interface: wl_interface = wl_interface {
    name: b"zxdg_exporter_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zxdg_exporter_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
static mut zxdg_importer_v1_requests_import_types: [*const wl_interface; 2] = [
    unsafe { &zxdg_imported_v1_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut zxdg_importer_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"import\0" as *const u8 as *const c_char,
        signature: b"ns\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_importer_v1_requests_import_types as *const _ },
    },
];
pub static mut zxdg_importer_v1_interface: wl_interface = wl_interface {
    name: b"zxdg_importer_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zxdg_importer_v1_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};
pub static mut zxdg_exported_v1_requests: [wl_message; 1] = [wl_message {
    name: b"destroy\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zxdg_exported_v1_events: [wl_message; 1] = [wl_message {
    name: b"handle\0" as *const u8 as *const c_char,
    signature: b"s\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zxdg_exported_v1_interface: wl_interface = wl_interface {
    name: b"zxdg_exported_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zxdg_exported_v1_requests as *const _ },
    event_count: 1,
    events: unsafe { &zxdg_exported_v1_events as *const _ },
};
static mut zxdg_imported_v1_requests_set_parent_of_types: [*const wl_interface; 1] =
    [unsafe { &wl_surface_interface as *const wl_interface }];
pub static mut zxdg_imported_v1_requests: [wl_message; 2] = [
    wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    },
    wl_message {
        name: b"set_parent_of\0" as *const u8 as *const c_char,
        signature: b"o\0" as *const u8 as *const c_char,
        types: unsafe { &zxdg_imported_v1_requests_set_parent_of_types as *const _ },
    },
];
pub static mut zxdg_imported_v1_events: [wl_message; 1] = [wl_message {
    name: b"destroyed\0" as *const u8 as *const c_char,
    signature: b"\0" as *const u8 as *const c_char,
    types: unsafe { &types_null as *const _ },
}];
pub static mut zxdg_imported_v1_interface: wl_interface = wl_interface {
    name: b"zxdg_imported_v1\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zxdg_imported_v1_requests as *const _ },
    event_count: 1,
    events: unsafe { &zxdg_imported_v1_events as *const _ },
};
