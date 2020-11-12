#[doc = "interface for exporting surfaces\n\nA global interface used for exporting surfaces that can later be imported\nusing xdg_importer."]
pub mod zxdg_exporter_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the xdg_exporter object\n\nNotify the compositor that the xdg_exporter object will no longer be\nused.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "export a toplevel surface\n\nThe export_toplevel request exports the passed surface so that it can later be\nimported via xdg_importer. When called, a new xdg_exported object will\nbe created and xdg_exported.handle will be sent immediately. See the\ncorresponding interface and event for details.\n\nA surface may be exported multiple times, and each exported handle may\nbe used to create a xdg_imported multiple times. Only xdg_toplevel\nequivalent surfaces may be exported."]
        ExportToplevel {
            id: Proxy<super::zxdg_exported_v2::ZxdgExportedV2>,
            surface: Proxy<super::wl_surface::WlSurface>,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "export_toplevel",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
                Request::ExportToplevel { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zxdg_exported_v2::ZxdgExportedV2,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![],
                },
                Request::ExportToplevel { id, surface } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::NewId(id.id()), Argument::Object(surface.id())],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::ExportToplevel { id, surface } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    pub struct ZxdgExporterV2;
    impl Interface for ZxdgExporterV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_exporter_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_exporter_v2_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the xdg_exporter object\n\nNotify the compositor that the xdg_exporter object will no longer be\nused.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "export a toplevel surface\n\nThe export_toplevel request exports the passed surface so that it can later be\nimported via xdg_importer. When called, a new xdg_exported object will\nbe created and xdg_exported.handle will be sent immediately. See the\ncorresponding interface and event for details.\n\nA surface may be exported multiple times, and each exported handle may\nbe used to create a xdg_imported multiple times. Only xdg_toplevel\nequivalent surfaces may be exported."]
        fn export_toplevel<F>(
            &self,
            surface: &Proxy<super::wl_surface::WlSurface>,
            implementor: F,
        ) -> Result<Proxy<super::zxdg_exported_v2::ZxdgExportedV2>, ()>
        where
            F: FnOnce(
                NewProxy<super::zxdg_exported_v2::ZxdgExportedV2>,
            ) -> Proxy<super::zxdg_exported_v2::ZxdgExportedV2>;
    }
    impl RequestsTrait for Proxy<ZxdgExporterV2> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn export_toplevel<F>(
            &self,
            surface: &Proxy<super::wl_surface::WlSurface>,
            implementor: F,
        ) -> Result<Proxy<super::zxdg_exported_v2::ZxdgExportedV2>, ()>
        where
            F: FnOnce(
                NewProxy<super::zxdg_exported_v2::ZxdgExportedV2>,
            ) -> Proxy<super::zxdg_exported_v2::ZxdgExportedV2>,
        {
            let msg = Request::ExportToplevel {
                id: self.child_placeholder(),
                surface: surface.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_EXPORT_TOPLEVEL_SINCE: u16 = 1u16;
}
#[doc = "interface for importing surfaces\n\nA global interface used for importing surfaces exported by xdg_exporter.\nWith this interface, a client can create a reference to a surface of\nanother client."]
pub mod zxdg_importer_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the xdg_importer object\n\nNotify the compositor that the xdg_importer object will no longer be\nused.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "import a toplevel surface\n\nThe import_toplevel request imports a surface from any client given a handle\nretrieved by exporting said surface using xdg_exporter.export_toplevel.\nWhen called, a new xdg_imported object will be created. This new object\nrepresents the imported surface, and the importing client can\nmanipulate its relationship using it. See xdg_imported for details."]
        ImportToplevel {
            id: Proxy<super::zxdg_imported_v2::ZxdgImportedV2>,
            handle: String,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "import_toplevel",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Str],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
                Request::ImportToplevel { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zxdg_imported_v2::ZxdgImportedV2,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![],
                },
                Request::ImportToplevel { id, handle } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::NewId(id.id()),
                        Argument::Str(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(handle.into())
                        }),
                    ],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::ImportToplevel { id, handle } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    let _arg_1 = ::std::ffi::CString::new(handle).unwrap();
                    _args_array[1].s = _arg_1.as_ptr();
                    f(1, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    pub struct ZxdgImporterV2;
    impl Interface for ZxdgImporterV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_importer_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_importer_v2_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the xdg_importer object\n\nNotify the compositor that the xdg_importer object will no longer be\nused.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "import a toplevel surface\n\nThe import_toplevel request imports a surface from any client given a handle\nretrieved by exporting said surface using xdg_exporter.export_toplevel.\nWhen called, a new xdg_imported object will be created. This new object\nrepresents the imported surface, and the importing client can\nmanipulate its relationship using it. See xdg_imported for details."]
        fn import_toplevel<F>(
            &self,
            handle: String,
            implementor: F,
        ) -> Result<Proxy<super::zxdg_imported_v2::ZxdgImportedV2>, ()>
        where
            F: FnOnce(
                NewProxy<super::zxdg_imported_v2::ZxdgImportedV2>,
            ) -> Proxy<super::zxdg_imported_v2::ZxdgImportedV2>;
    }
    impl RequestsTrait for Proxy<ZxdgImporterV2> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn import_toplevel<F>(
            &self,
            handle: String,
            implementor: F,
        ) -> Result<Proxy<super::zxdg_imported_v2::ZxdgImportedV2>, ()>
        where
            F: FnOnce(
                NewProxy<super::zxdg_imported_v2::ZxdgImportedV2>,
            ) -> Proxy<super::zxdg_imported_v2::ZxdgImportedV2>,
        {
            let msg = Request::ImportToplevel {
                id: self.child_placeholder(),
                handle: handle,
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_IMPORT_TOPLEVEL_SINCE: u16 = 1u16;
}
#[doc = "an exported surface handle\n\nA xdg_exported object represents an exported reference to a surface. The\nexported surface may be referenced as long as the xdg_exported object not\ndestroyed. Destroying the xdg_exported invalidates any relationship the\nimporter may have established using xdg_imported."]
pub mod zxdg_exported_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "unexport the exported surface\n\nRevoke the previously exported surface. This invalidates any\nrelationship the importer may have set up using the xdg_imported created\ngiven the handle sent via xdg_exported.handle.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "destroy",
            since: 1,
            signature: &[],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "the exported surface handle\n\nThe handle event contains the unique handle of this exported surface\nreference. It may be shared with any client, which then can use it to\nimport the surface by calling xdg_importer.import_toplevel. A handle\nmay be used to import the surface multiple times."]
        Handle { handle: String },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "handle",
            since: 1,
            signature: &[super::ArgumentType::Str],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Handle { .. } => 0,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Handle {
                        handle: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Handle {
                        handle: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    pub struct ZxdgExportedV2;
    impl Interface for ZxdgExportedV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_exported_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_exported_v2_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "unexport the exported surface\n\nRevoke the previously exported surface. This invalidates any\nrelationship the importer may have set up using the xdg_imported created\ngiven the handle sent via xdg_exported.handle.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
    }
    impl RequestsTrait for Proxy<ZxdgExportedV2> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_HANDLE_SINCE: u16 = 1u16;
}
#[doc = "an imported surface handle\n\nA xdg_imported object represents an imported reference to surface exported\nby some client. A client can use this interface to manipulate\nrelationships between its own surfaces and the imported surface."]
pub mod zxdg_imported_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the xdg_imported object\n\nNotify the compositor that it will no longer use the xdg_imported\nobject. Any relationship that may have been set up will at this point\nbe invalidated.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set as the parent of some surface\n\nSet the imported surface as the parent of some surface of the client.\nThe passed surface must be a xdg_toplevel equivalent. Calling this\nfunction sets up a surface to surface relation with the same stacking\nand positioning semantics as xdg_toplevel.set_parent."]
        SetParentOf {
            surface: Proxy<super::wl_surface::WlSurface>,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "set_parent_of",
                since: 1,
                signature: &[super::ArgumentType::Object],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Destroy => 0,
                Request::SetParentOf { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![],
                },
                Request::SetParentOf { surface } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::Object(surface.id())],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::SetParentOf { surface } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "the imported surface handle has been destroyed\n\nThe imported surface handle has been destroyed and any relationship set\nup has been invalidated. This may happen for various reasons, for\nexample if the exported surface or the exported surface handle has been\ndestroyed, if the handle used for importing was invalid."]
        Destroyed,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "destroyed",
            since: 1,
            signature: &[],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Destroyed => 0,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => Ok(Event::Destroyed),
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                0 => Ok(Event::Destroyed),
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    pub struct ZxdgImportedV2;
    impl Interface for ZxdgImportedV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_imported_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_imported_v2_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the xdg_imported object\n\nNotify the compositor that it will no longer use the xdg_imported\nobject. Any relationship that may have been set up will at this point\nbe invalidated.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "set as the parent of some surface\n\nSet the imported surface as the parent of some surface of the client.\nThe passed surface must be a xdg_toplevel equivalent. Calling this\nfunction sets up a surface to surface relation with the same stacking\nand positioning semantics as xdg_toplevel.set_parent."]
        fn set_parent_of(&self, surface: &Proxy<super::wl_surface::WlSurface>) -> ();
    }
    impl RequestsTrait for Proxy<ZxdgImportedV2> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn set_parent_of(&self, surface: &Proxy<super::wl_surface::WlSurface>) -> () {
            let msg = Request::SetParentOf {
                surface: surface.clone(),
            };
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_PARENT_OF_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DESTROYED_SINCE: u16 = 1u16;
}
