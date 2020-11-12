#[doc = "window decoration manager\n\nThis interface allows a compositor to announce support for server-side\ndecorations.\n\nA window decoration is a set of window controls as deemed appropriate by\nthe party managing them, such as user interface components used to move,\nresize and change a window's state.\n\nA client can use this protocol to request being decorated by a supporting\ncompositor.\n\nIf compositor and client do not negotiate the use of a server-side\ndecoration using this protocol, clients continue to self-decorate as they\nsee fit.\n\nWarning! The protocol described in this file is experimental and\nbackward incompatible changes may be made. Backward compatible changes\nmay be added together with the corresponding interface version bump.\nBackward incompatible changes are done by bumping the version number in\nthe protocol and interface names and resetting the interface version.\nOnce the protocol is to be declared stable, the 'z' prefix and the\nversion number in the protocol and interface names are removed and the\ninterface version number is reset."]
pub mod zxdg_decoration_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the decoration manager object\n\nDestroy the decoration manager. This doesn't destroy objects created\nwith the manager.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "create a new toplevel decoration object\n\nCreate a new decoration object associated with the given toplevel.\n\nCreating an xdg_toplevel_decoration from an xdg_toplevel which has a\nbuffer attached or committed is a client error, and any attempts by a\nclient to attach or manipulate a buffer prior to the first\nxdg_toplevel_decoration.configure event must also be treated as\nerrors."]
        GetToplevelDecoration {
            id: Proxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>,
            toplevel: Proxy<super::xdg_toplevel::XdgToplevel>,
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
                name: "get_toplevel_decoration",
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
                Request::GetToplevelDecoration { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
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
                Request::GetToplevelDecoration { id, toplevel } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::NewId(id.id()), Argument::Object(toplevel.id())],
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
                Request::GetToplevelDecoration { id, toplevel } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = toplevel.c_ptr() as *mut _;
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
    pub struct ZxdgDecorationManagerV1;
    impl Interface for ZxdgDecorationManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_decoration_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_decoration_manager_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the decoration manager object\n\nDestroy the decoration manager. This doesn't destroy objects created\nwith the manager.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "create a new toplevel decoration object\n\nCreate a new decoration object associated with the given toplevel.\n\nCreating an xdg_toplevel_decoration from an xdg_toplevel which has a\nbuffer attached or committed is a client error, and any attempts by a\nclient to attach or manipulate a buffer prior to the first\nxdg_toplevel_decoration.configure event must also be treated as\nerrors."]
        fn get_toplevel_decoration<F>(
            &self,
            toplevel: &Proxy<super::xdg_toplevel::XdgToplevel>,
            implementor: F,
        ) -> Result<Proxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>,
            )
                -> Proxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>;
    }
    impl RequestsTrait for Proxy<ZxdgDecorationManagerV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn get_toplevel_decoration<F>(
            &self,
            toplevel: &Proxy<super::xdg_toplevel::XdgToplevel>,
            implementor: F,
        ) -> Result<Proxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>,
            )
                -> Proxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>,
        {
            let msg = Request::GetToplevelDecoration {
                id: self.child_placeholder(),
                toplevel: toplevel.clone(),
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_TOPLEVEL_DECORATION_SINCE: u16 = 1u16;
}
#[doc = "decoration object for a toplevel surface\n\nThe decoration object allows the compositor to toggle server-side window\ndecorations for a toplevel surface. The client can request to switch to\nanother mode.\n\nThe xdg_toplevel_decoration object must be destroyed before its\nxdg_toplevel."]
pub mod zxdg_toplevel_decoration_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Error {
        #[doc = "xdg_toplevel has a buffer attached before configure"]
        UnconfiguredBuffer = 0,
        #[doc = "xdg_toplevel already has a decoration object"]
        AlreadyConstructed = 1,
        #[doc = "xdg_toplevel destroyed before the decoration object"]
        Orphaned = 2,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::UnconfiguredBuffer),
                1 => Some(Error::AlreadyConstructed),
                2 => Some(Error::Orphaned),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "window decoration modes\n\nThese values describe window decoration modes."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Mode {
        #[doc = "no server-side window decoration"]
        ClientSide = 1,
        #[doc = "server-side window decoration"]
        ServerSide = 2,
    }
    impl Mode {
        pub fn from_raw(n: u32) -> Option<Mode> {
            match n {
                1 => Some(Mode::ClientSide),
                2 => Some(Mode::ServerSide),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub enum Request {
        #[doc = "destroy the decoration object\n\nSwitch back to a mode without any server-side decorations at the next\ncommit.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the decoration mode\n\nSet the toplevel surface decoration mode. This informs the compositor\nthat the client prefers the provided decoration mode.\n\nAfter requesting a decoration mode, the compositor will respond by\nemitting a xdg_surface.configure event. The client should then update\nits content, drawing it without decorations if the received mode is\nserver-side decorations. The client must also acknowledge the configure\nwhen committing the new content (see xdg_surface.ack_configure).\n\nThe compositor can decide not to use the client's mode and enforce a\ndifferent mode instead.\n\nClients whose decoration mode depend on the xdg_toplevel state may send\na set_mode request in response to a xdg_surface.configure event and wait\nfor the next xdg_surface.configure event to prevent unwanted state.\nSuch clients are responsible for preventing configure loops and must\nmake sure not to send multiple successive set_mode requests with the\nsame decoration mode."]
        SetMode { mode: Mode },
        #[doc = "unset the decoration mode\n\nUnset the toplevel surface decoration mode. This informs the compositor\nthat the client doesn't prefer a particular decoration mode.\n\nThis request has the same semantics as set_mode."]
        UnsetMode,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "set_mode",
                since: 1,
                signature: &[super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "unset_mode",
                since: 1,
                signature: &[],
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
                Request::SetMode { .. } => 1,
                Request::UnsetMode => 2,
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
                Request::SetMode { mode } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::Uint(mode.to_raw())],
                },
                Request::UnsetMode => Message {
                    sender_id: sender_id,
                    opcode: 2,
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
                Request::SetMode { mode } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = mode.to_raw();
                    f(1, &mut _args_array)
                }
                Request::UnsetMode => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "suggest a surface change\n\nThe configure event asks the client to change its decoration mode. The\nconfigured state should not be applied immediately. Clients must send an\nack_configure in response to this event. See xdg_surface.configure and\nxdg_surface.ack_configure for details.\n\nA configure event can be sent at any time. The specified mode must be\nobeyed by the client."]
        Configure { mode: Mode },
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "configure",
            since: 1,
            signature: &[super::ArgumentType::Uint],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Configure { .. } => 0,
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
                    Ok(Event::Configure {
                        mode: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                Mode::from_raw(val).ok_or(())?
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
                    Ok(Event::Configure {
                        mode: Mode::from_raw(_args[0].u).ok_or(())?,
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
    pub struct ZxdgToplevelDecorationV1;
    impl Interface for ZxdgToplevelDecorationV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_toplevel_decoration_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_toplevel_decoration_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the decoration object\n\nSwitch back to a mode without any server-side decorations at the next\ncommit.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "set the decoration mode\n\nSet the toplevel surface decoration mode. This informs the compositor\nthat the client prefers the provided decoration mode.\n\nAfter requesting a decoration mode, the compositor will respond by\nemitting a xdg_surface.configure event. The client should then update\nits content, drawing it without decorations if the received mode is\nserver-side decorations. The client must also acknowledge the configure\nwhen committing the new content (see xdg_surface.ack_configure).\n\nThe compositor can decide not to use the client's mode and enforce a\ndifferent mode instead.\n\nClients whose decoration mode depend on the xdg_toplevel state may send\na set_mode request in response to a xdg_surface.configure event and wait\nfor the next xdg_surface.configure event to prevent unwanted state.\nSuch clients are responsible for preventing configure loops and must\nmake sure not to send multiple successive set_mode requests with the\nsame decoration mode."]
        fn set_mode(&self, mode: Mode) -> ();
        #[doc = "unset the decoration mode\n\nUnset the toplevel surface decoration mode. This informs the compositor\nthat the client doesn't prefer a particular decoration mode.\n\nThis request has the same semantics as set_mode."]
        fn unset_mode(&self) -> ();
    }
    impl RequestsTrait for Proxy<ZxdgToplevelDecorationV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn set_mode(&self, mode: Mode) -> () {
            let msg = Request::SetMode { mode: mode };
            self.send(msg);
        }
        fn unset_mode(&self) -> () {
            let msg = Request::UnsetMode;
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MODE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_UNSET_MODE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CONFIGURE_SINCE: u16 = 1u16;
}
