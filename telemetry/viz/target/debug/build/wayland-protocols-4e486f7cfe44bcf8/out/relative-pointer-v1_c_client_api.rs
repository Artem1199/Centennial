#[doc = "get relative pointer objects\n\nA global interface used for getting the relative pointer object for a\ngiven pointer."]
pub mod zwp_relative_pointer_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the relative pointer manager object\n\nUsed by the client to notify the server that it will no longer use this\nrelative pointer manager object.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "get a relative pointer object\n\nCreate a relative pointer interface given a wl_pointer object. See the\nwp_relative_pointer interface for more details."]
        GetRelativePointer {
            id: Proxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>,
            pointer: Proxy<super::wl_pointer::WlPointer>,
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
                name: "get_relative_pointer",
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
                Request::GetRelativePointer { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zwp_relative_pointer_v1::ZwpRelativePointerV1,
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
                Request::GetRelativePointer { id, pointer } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::NewId(id.id()), Argument::Object(pointer.id())],
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
                Request::GetRelativePointer { id, pointer } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = pointer.c_ptr() as *mut _;
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
    pub struct ZwpRelativePointerManagerV1;
    impl Interface for ZwpRelativePointerManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_relative_pointer_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_relative_pointer_manager_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the relative pointer manager object\n\nUsed by the client to notify the server that it will no longer use this\nrelative pointer manager object.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "get a relative pointer object\n\nCreate a relative pointer interface given a wl_pointer object. See the\nwp_relative_pointer interface for more details."]
        fn get_relative_pointer<F>(
            &self,
            pointer: &Proxy<super::wl_pointer::WlPointer>,
            implementor: F,
        ) -> Result<Proxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>,
            ) -> Proxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>;
    }
    impl RequestsTrait for Proxy<ZwpRelativePointerManagerV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn get_relative_pointer<F>(
            &self,
            pointer: &Proxy<super::wl_pointer::WlPointer>,
            implementor: F,
        ) -> Result<Proxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>,
            ) -> Proxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>,
        {
            let msg = Request::GetRelativePointer {
                id: self.child_placeholder(),
                pointer: pointer.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_RELATIVE_POINTER_SINCE: u16 = 1u16;
}
#[doc = "relative pointer object\n\nA wp_relative_pointer object is an extension to the wl_pointer interface\nused for emitting relative pointer events. It shares the same focus as\nwl_pointer objects of the same seat and will only emit events when it has\nfocus."]
pub mod zwp_relative_pointer_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "release the relative pointer object\n\n\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
        #[doc = "relative pointer motion\n\nRelative x/y pointer motion from the pointer of the seat associated with\nthis object.\n\nA relative motion is in the same dimension as regular wl_pointer motion\nevents, except they do not represent an absolute position. For example,\nmoving a pointer from (x, y) to (x', y') would have the equivalent\nrelative motion (x' - x, y' - y). If a pointer motion caused the\nabsolute pointer position to be clipped by for example the edge of the\nmonitor, the relative motion is unaffected by the clipping and will\nrepresent the unclipped motion.\n\nThis event also contains non-accelerated motion deltas. The\nnon-accelerated delta is, when applicable, the regular pointer motion\ndelta as it was before having applied motion acceleration and other\ntransformations such as normalization.\n\nNote that the non-accelerated delta does not represent 'raw' events as\nthey were read from some device. Pointer motion acceleration is device-\nand configuration-specific and non-accelerated deltas and accelerated\ndeltas may have the same value on some devices.\n\nRelative motions are not coupled to wl_pointer.motion events, and can be\nsent in combination with such events, but also independently. There may\nalso be scenarios where wl_pointer.motion is sent, but there is no\nrelative motion. The order of an absolute and relative motion event\noriginating from the same physical motion is not guaranteed.\n\nIf the client needs button events or focus state, it can receive them\nfrom a wl_pointer object of the same seat that the wp_relative_pointer\nobject is associated with."]
        RelativeMotion {
            utime_hi: u32,
            utime_lo: u32,
            dx: f64,
            dy: f64,
            dx_unaccel: f64,
            dy_unaccel: f64,
        },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "relative_motion",
            since: 1,
            signature: &[
                super::ArgumentType::Uint,
                super::ArgumentType::Uint,
                super::ArgumentType::Fixed,
                super::ArgumentType::Fixed,
                super::ArgumentType::Fixed,
                super::ArgumentType::Fixed,
            ],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::RelativeMotion { .. } => 0,
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
                    Ok(Event::RelativeMotion {
                        utime_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        utime_lo: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        dx: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        dy: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        dx_unaccel: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
                            } else {
                                return Err(());
                            }
                        },
                        dy_unaccel: {
                            if let Some(Argument::Fixed(val)) = args.next() {
                                (val as f64) / 256.
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
                    let _args = ::std::slice::from_raw_parts(args, 6);
                    Ok(Event::RelativeMotion {
                        utime_hi: _args[0].u,
                        utime_lo: _args[1].u,
                        dx: (_args[2].f as f64) / 256.,
                        dy: (_args[3].f as f64) / 256.,
                        dx_unaccel: (_args[4].f as f64) / 256.,
                        dy_unaccel: (_args[5].f as f64) / 256.,
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
    pub struct ZwpRelativePointerV1;
    impl Interface for ZwpRelativePointerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_relative_pointer_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_relative_pointer_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "release the relative pointer object\n\n\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
    }
    impl RequestsTrait for Proxy<ZwpRelativePointerV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_RELATIVE_MOTION_SINCE: u16 = 1u16;
}
