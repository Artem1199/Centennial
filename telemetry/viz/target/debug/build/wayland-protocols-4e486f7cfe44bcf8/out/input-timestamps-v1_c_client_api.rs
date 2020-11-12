#[doc = "context object for high-resolution input timestamps\n\nA global interface used for requesting high-resolution timestamps\nfor input events."]
pub mod zwp_input_timestamps_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the input timestamps manager object\n\nInforms the server that the client will no longer be using this\nprotocol object. Existing objects created by this object are not\naffected.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "subscribe to high-resolution keyboard timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_keyboard events that\ncarry a timestamp.\n\nIf the associated wl_keyboard object is invalidated, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        GetKeyboardTimestamps {
            id: Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            keyboard: Proxy<super::wl_keyboard::WlKeyboard>,
        },
        #[doc = "subscribe to high-resolution pointer timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_pointer events that\ncarry a timestamp.\n\nIf the associated wl_pointer object is invalidated, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        GetPointerTimestamps {
            id: Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            pointer: Proxy<super::wl_pointer::WlPointer>,
        },
        #[doc = "subscribe to high-resolution touch timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_touch events that\ncarry a timestamp.\n\nIf the associated wl_touch object becomes invalid, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        GetTouchTimestamps {
            id: Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            touch: Proxy<super::wl_touch::WlTouch>,
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
                name: "get_keyboard_timestamps",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "get_pointer_timestamps",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "get_touch_timestamps",
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
                Request::GetKeyboardTimestamps { .. } => 1,
                Request::GetPointerTimestamps { .. } => 2,
                Request::GetTouchTimestamps { .. } => 3,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
                >(version, meta.child())),
                2 => Some(Object::from_interface::<
                    super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
                >(version, meta.child())),
                3 => Some(Object::from_interface::<
                    super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
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
                Request::GetKeyboardTimestamps { id, keyboard } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::NewId(id.id()), Argument::Object(keyboard.id())],
                },
                Request::GetPointerTimestamps { id, pointer } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: vec![Argument::NewId(id.id()), Argument::Object(pointer.id())],
                },
                Request::GetTouchTimestamps { id, touch } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: vec![Argument::NewId(id.id()), Argument::Object(touch.id())],
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
                Request::GetKeyboardTimestamps { id, keyboard } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = keyboard.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
                Request::GetPointerTimestamps { id, pointer } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = pointer.c_ptr() as *mut _;
                    f(2, &mut _args_array)
                }
                Request::GetTouchTimestamps { id, touch } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = touch.c_ptr() as *mut _;
                    f(3, &mut _args_array)
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
    pub struct ZwpInputTimestampsManagerV1;
    impl Interface for ZwpInputTimestampsManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_input_timestamps_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_input_timestamps_manager_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the input timestamps manager object\n\nInforms the server that the client will no longer be using this\nprotocol object. Existing objects created by this object are not\naffected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "subscribe to high-resolution keyboard timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_keyboard events that\ncarry a timestamp.\n\nIf the associated wl_keyboard object is invalidated, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        fn get_keyboard_timestamps<F>(
            &self,
            keyboard: &Proxy<super::wl_keyboard::WlKeyboard>,
            implementor: F,
        ) -> Result<Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>;
        #[doc = "subscribe to high-resolution pointer timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_pointer events that\ncarry a timestamp.\n\nIf the associated wl_pointer object is invalidated, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        fn get_pointer_timestamps<F>(
            &self,
            pointer: &Proxy<super::wl_pointer::WlPointer>,
            implementor: F,
        ) -> Result<Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>;
        #[doc = "subscribe to high-resolution touch timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_touch events that\ncarry a timestamp.\n\nIf the associated wl_touch object becomes invalid, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        fn get_touch_timestamps<F>(
            &self,
            touch: &Proxy<super::wl_touch::WlTouch>,
            implementor: F,
        ) -> Result<Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>;
    }
    impl RequestsTrait for Proxy<ZwpInputTimestampsManagerV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn get_keyboard_timestamps<F>(
            &self,
            keyboard: &Proxy<super::wl_keyboard::WlKeyboard>,
            implementor: F,
        ) -> Result<Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
        {
            let msg = Request::GetKeyboardTimestamps {
                id: self.child_placeholder(),
                keyboard: keyboard.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }
        fn get_pointer_timestamps<F>(
            &self,
            pointer: &Proxy<super::wl_pointer::WlPointer>,
            implementor: F,
        ) -> Result<Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
        {
            let msg = Request::GetPointerTimestamps {
                id: self.child_placeholder(),
                pointer: pointer.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }
        fn get_touch_timestamps<F>(
            &self,
            touch: &Proxy<super::wl_touch::WlTouch>,
            implementor: F,
        ) -> Result<Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> Proxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
        {
            let msg = Request::GetTouchTimestamps {
                id: self.child_placeholder(),
                touch: touch.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_KEYBOARD_TIMESTAMPS_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_POINTER_TIMESTAMPS_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_TOUCH_TIMESTAMPS_SINCE: u16 = 1u16;
}
#[doc = "context object for input timestamps\n\nProvides high-resolution timestamp events for a set of subscribed input\nevents. The set of subscribed input events is determined by the\nzwp_input_timestamps_manager_v1 request used to create this object."]
pub mod zwp_input_timestamps_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the input timestamps object\n\nInforms the server that the client will no longer be using this\nprotocol object. After the server processes the request, no more\ntimestamp events will be emitted.\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
        #[doc = "high-resolution timestamp event\n\nThe timestamp event is associated with the first subsequent input event\ncarrying a timestamp which belongs to the set of input events this\nobject is subscribed to.\n\nThe timestamp provided by this event is a high-resolution version of\nthe timestamp argument of the associated input event. The provided\ntimestamp is in the same clock domain and is at least as accurate as\nthe associated input event timestamp.\n\nThe timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,\neach component being an unsigned 32-bit value. Whole seconds are in\ntv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,\nand the additional fractional part in tv_nsec as nanoseconds. Hence,\nfor valid timestamps tv_nsec must be in [0, 999999999]."]
        Timestamp {
            tv_sec_hi: u32,
            tv_sec_lo: u32,
            tv_nsec: u32,
        },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "timestamp",
            since: 1,
            signature: &[
                super::ArgumentType::Uint,
                super::ArgumentType::Uint,
                super::ArgumentType::Uint,
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
                Event::Timestamp { .. } => 0,
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
                    Ok(Event::Timestamp {
                        tv_sec_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        tv_sec_lo: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        tv_nsec: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
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
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Timestamp {
                        tv_sec_hi: _args[0].u,
                        tv_sec_lo: _args[1].u,
                        tv_nsec: _args[2].u,
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
    pub struct ZwpInputTimestampsV1;
    impl Interface for ZwpInputTimestampsV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_input_timestamps_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_input_timestamps_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the input timestamps object\n\nInforms the server that the client will no longer be using this\nprotocol object. After the server processes the request, no more\ntimestamp events will be emitted.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
    }
    impl RequestsTrait for Proxy<ZwpInputTimestampsV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_TIMESTAMP_SINCE: u16 = 1u16;
}
