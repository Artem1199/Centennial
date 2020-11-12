#[doc = "inhibits input events to other clients\n\nClients can use this interface to prevent input events from being sent to\nany surfaces but its own, which is useful for example in lock screen\nsoftware. It is assumed that access to this interface will be locked down\nto whitelisted clients by the compositor."]
pub mod zwlr_input_inhibit_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Error {
        #[doc = "an input inhibitor is already in use on the compositor"]
        AlreadyInhibited = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyInhibited),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub enum Request {
        #[doc = "inhibit input to other clients\n\nActivates the input inhibitor. As long as the inhibitor is active, the\ncompositor will not send input events to other clients."]
        GetInhibitor {
            id: Proxy<super::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1>,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "get_inhibitor",
            since: 1,
            signature: &[super::ArgumentType::NewId],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::GetInhibitor { .. } => 0,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetInhibitor { id } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![Argument::NewId(id.id())],
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
                Request::GetInhibitor { id } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(0, &mut _args_array)
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
    pub struct ZwlrInputInhibitManagerV1;
    impl Interface for ZwlrInputInhibitManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_input_inhibit_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_input_inhibit_manager_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "inhibit input to other clients\n\nActivates the input inhibitor. As long as the inhibitor is active, the\ncompositor will not send input events to other clients."]
        fn get_inhibitor<F>(
            &self,
            implementor: F,
        ) -> Result<Proxy<super::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1>,
            ) -> Proxy<super::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1>;
    }
    impl RequestsTrait for Proxy<ZwlrInputInhibitManagerV1> {
        fn get_inhibitor<F>(
            &self,
            implementor: F,
        ) -> Result<Proxy<super::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1>,
            ) -> Proxy<super::zwlr_input_inhibitor_v1::ZwlrInputInhibitorV1>,
        {
            let msg = Request::GetInhibitor {
                id: self.child_placeholder(),
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_INHIBITOR_SINCE: u16 = 1u16;
}
#[doc = "inhibits input to other clients\n\nWhile this resource exists, input to clients other than the owner of the\ninhibitor resource will not receive input events. Any client which\npreviously had focus will receive a leave event and will not be given\nfocus again. The client that owns this resource will receive all input\nevents normally. The compositor will also disable all of its own input\nprocessing (such as keyboard shortcuts) while the inhibitor is active.\n\nThe compositor may continue to send input events to selected clients,\nsuch as an on-screen keyboard (via the input-method protocol)."]
pub mod zwlr_input_inhibitor_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the input inhibitor object\n\nDestroy the inhibitor and allow other clients to receive input.\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
    pub struct ZwlrInputInhibitorV1;
    impl Interface for ZwlrInputInhibitorV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_input_inhibitor_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_input_inhibitor_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the input inhibitor object\n\nDestroy the inhibitor and allow other clients to receive input.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
    }
    impl RequestsTrait for Proxy<ZwlrInputInhibitorV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
}
