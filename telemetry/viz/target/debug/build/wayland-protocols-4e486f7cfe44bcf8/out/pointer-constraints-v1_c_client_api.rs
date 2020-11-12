#[doc = "constrain the movement of a pointer\n\nThe global interface exposing pointer constraining functionality. It\nexposes two requests: lock_pointer for locking the pointer to its\nposition, and confine_pointer for locking the pointer to a region.\n\nThe lock_pointer and confine_pointer requests create the objects\nwp_locked_pointer and wp_confined_pointer respectively, and the client can\nuse these objects to interact with the lock.\n\nFor any surface, only one lock or confinement may be active across all\nwl_pointer objects of the same seat. If a lock or confinement is requested\nwhen another lock or confinement is active or requested on the same surface\nand with any of the wl_pointer objects of the same seat, an\n'already_constrained' error will be raised."]
pub mod zwp_pointer_constraints_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    #[doc = "wp_pointer_constraints error values\n\nThese errors can be emitted in response to wp_pointer_constraints\nrequests."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Error {
        #[doc = "pointer constraint already requested on that surface"]
        AlreadyConstrained = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                1 => Some(Error::AlreadyConstrained),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "constraint lifetime\n\nThese values represent different lifetime semantics. They are passed\nas arguments to the factory requests to specify how the constraint\nlifetimes should be managed."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Lifetime {
        #[doc = "the pointer constraint is defunct once deactivated\n\nA oneshot pointer constraint will never reactivate once it has been\ndeactivated. See the corresponding deactivation event\n(wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for\ndetails."]
        Oneshot = 1,
        #[doc = "the pointer constraint may reactivate\n\nA persistent pointer constraint may again reactivate once it has\nbeen deactivated. See the corresponding deactivation event\n(wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for\ndetails."]
        Persistent = 2,
    }
    impl Lifetime {
        pub fn from_raw(n: u32) -> Option<Lifetime> {
            match n {
                1 => Some(Lifetime::Oneshot),
                2 => Some(Lifetime::Persistent),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub enum Request {
        #[doc = "destroy the pointer constraints manager object\n\nUsed by the client to notify the server that it will no longer use this\npointer constraints object.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "lock pointer to a position\n\nThe lock_pointer request lets the client request to disable movements of\nthe virtual pointer (i.e. the cursor), effectively locking the pointer\nto a position. This request may not take effect immediately; in the\nfuture, when the compositor deems implementation-specific constraints\nare satisfied, the pointer lock will be activated and the compositor\nsends a locked event.\n\nThe protocol provides no guarantee that the constraints are ever\nsatisfied, and does not require the compositor to send an error if the\nconstraints cannot ever be satisfied. It is thus possible to request a\nlock that will never activate.\n\nThere may not be another pointer constraint of any kind requested or\nactive on the surface for any of the wl_pointer objects of the seat of\nthe passed pointer when requesting a lock. If there is, an error will be\nraised. See general pointer lock documentation for more details.\n\nThe intersection of the region passed with this request and the input\nregion of the surface is used to determine where the pointer must be\nin order for the lock to activate. It is up to the compositor whether to\nwarp the pointer or require some kind of user interaction for the lock\nto activate. If the region is null the surface input region is used.\n\nA surface may receive pointer focus without the lock being activated.\n\nThe request creates a new object wp_locked_pointer which is used to\ninteract with the lock as well as receive updates about its state. See\nthe the description of wp_locked_pointer for further information.\n\nNote that while a pointer is locked, the wl_pointer objects of the\ncorresponding seat will not emit any wl_pointer.motion events, but\nrelative motion events will still be emitted via wp_relative_pointer\nobjects of the same seat. wl_pointer.axis and wl_pointer.button events\nare unaffected."]
        LockPointer {
            id: Proxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>,
            surface: Proxy<super::wl_surface::WlSurface>,
            pointer: Proxy<super::wl_pointer::WlPointer>,
            region: Option<Proxy<super::wl_region::WlRegion>>,
            lifetime: u32,
        },
        #[doc = "confine pointer to a region\n\nThe confine_pointer request lets the client request to confine the\npointer cursor to a given region. This request may not take effect\nimmediately; in the future, when the compositor deems implementation-\nspecific constraints are satisfied, the pointer confinement will be\nactivated and the compositor sends a confined event.\n\nThe intersection of the region passed with this request and the input\nregion of the surface is used to determine where the pointer must be\nin order for the confinement to activate. It is up to the compositor\nwhether to warp the pointer or require some kind of user interaction for\nthe confinement to activate. If the region is null the surface input\nregion is used.\n\nThe request will create a new object wp_confined_pointer which is used\nto interact with the confinement as well as receive updates about its\nstate. See the the description of wp_confined_pointer for further\ninformation."]
        ConfinePointer {
            id: Proxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>,
            surface: Proxy<super::wl_surface::WlSurface>,
            pointer: Proxy<super::wl_pointer::WlPointer>,
            region: Option<Proxy<super::wl_region::WlRegion>>,
            lifetime: u32,
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
                name: "lock_pointer",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                ],
            },
            super::MessageDesc {
                name: "confine_pointer",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Object,
                    super::ArgumentType::Uint,
                ],
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
                Request::LockPointer { .. } => 1,
                Request::ConfinePointer { .. } => 2,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zwp_locked_pointer_v1::ZwpLockedPointerV1,
                >(version, meta.child())),
                2 => Some(Object::from_interface::<
                    super::zwp_confined_pointer_v1::ZwpConfinedPointerV1,
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
                Request::LockPointer {
                    id,
                    surface,
                    pointer,
                    region,
                    lifetime,
                } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::NewId(id.id()),
                        Argument::Object(surface.id()),
                        Argument::Object(pointer.id()),
                        Argument::Object(region.map(|o| o.id()).unwrap_or(0)),
                        Argument::Uint(lifetime),
                    ],
                },
                Request::ConfinePointer {
                    id,
                    surface,
                    pointer,
                    region,
                    lifetime,
                } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: vec![
                        Argument::NewId(id.id()),
                        Argument::Object(surface.id()),
                        Argument::Object(pointer.id()),
                        Argument::Object(region.map(|o| o.id()).unwrap_or(0)),
                        Argument::Uint(lifetime),
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
                Request::LockPointer {
                    id,
                    surface,
                    pointer,
                    region,
                    lifetime,
                } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    _args_array[2].o = pointer.c_ptr() as *mut _;
                    _args_array[3].o = region
                        .map(|o| o.c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[4].u = lifetime;
                    f(1, &mut _args_array)
                }
                Request::ConfinePointer {
                    id,
                    surface,
                    pointer,
                    region,
                    lifetime,
                } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    _args_array[2].o = pointer.c_ptr() as *mut _;
                    _args_array[3].o = region
                        .map(|o| o.c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[4].u = lifetime;
                    f(2, &mut _args_array)
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
    pub struct ZwpPointerConstraintsV1;
    impl Interface for ZwpPointerConstraintsV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_pointer_constraints_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_pointer_constraints_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the pointer constraints manager object\n\nUsed by the client to notify the server that it will no longer use this\npointer constraints object.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "lock pointer to a position\n\nThe lock_pointer request lets the client request to disable movements of\nthe virtual pointer (i.e. the cursor), effectively locking the pointer\nto a position. This request may not take effect immediately; in the\nfuture, when the compositor deems implementation-specific constraints\nare satisfied, the pointer lock will be activated and the compositor\nsends a locked event.\n\nThe protocol provides no guarantee that the constraints are ever\nsatisfied, and does not require the compositor to send an error if the\nconstraints cannot ever be satisfied. It is thus possible to request a\nlock that will never activate.\n\nThere may not be another pointer constraint of any kind requested or\nactive on the surface for any of the wl_pointer objects of the seat of\nthe passed pointer when requesting a lock. If there is, an error will be\nraised. See general pointer lock documentation for more details.\n\nThe intersection of the region passed with this request and the input\nregion of the surface is used to determine where the pointer must be\nin order for the lock to activate. It is up to the compositor whether to\nwarp the pointer or require some kind of user interaction for the lock\nto activate. If the region is null the surface input region is used.\n\nA surface may receive pointer focus without the lock being activated.\n\nThe request creates a new object wp_locked_pointer which is used to\ninteract with the lock as well as receive updates about its state. See\nthe the description of wp_locked_pointer for further information.\n\nNote that while a pointer is locked, the wl_pointer objects of the\ncorresponding seat will not emit any wl_pointer.motion events, but\nrelative motion events will still be emitted via wp_relative_pointer\nobjects of the same seat. wl_pointer.axis and wl_pointer.button events\nare unaffected."]
        fn lock_pointer<F>(
            &self,
            surface: &Proxy<super::wl_surface::WlSurface>,
            pointer: &Proxy<super::wl_pointer::WlPointer>,
            region: Option<&Proxy<super::wl_region::WlRegion>>,
            lifetime: u32,
            implementor: F,
        ) -> Result<Proxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>,
            ) -> Proxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>;
        #[doc = "confine pointer to a region\n\nThe confine_pointer request lets the client request to confine the\npointer cursor to a given region. This request may not take effect\nimmediately; in the future, when the compositor deems implementation-\nspecific constraints are satisfied, the pointer confinement will be\nactivated and the compositor sends a confined event.\n\nThe intersection of the region passed with this request and the input\nregion of the surface is used to determine where the pointer must be\nin order for the confinement to activate. It is up to the compositor\nwhether to warp the pointer or require some kind of user interaction for\nthe confinement to activate. If the region is null the surface input\nregion is used.\n\nThe request will create a new object wp_confined_pointer which is used\nto interact with the confinement as well as receive updates about its\nstate. See the the description of wp_confined_pointer for further\ninformation."]
        fn confine_pointer<F>(
            &self,
            surface: &Proxy<super::wl_surface::WlSurface>,
            pointer: &Proxy<super::wl_pointer::WlPointer>,
            region: Option<&Proxy<super::wl_region::WlRegion>>,
            lifetime: u32,
            implementor: F,
        ) -> Result<Proxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>,
            ) -> Proxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>;
    }
    impl RequestsTrait for Proxy<ZwpPointerConstraintsV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn lock_pointer<F>(
            &self,
            surface: &Proxy<super::wl_surface::WlSurface>,
            pointer: &Proxy<super::wl_pointer::WlPointer>,
            region: Option<&Proxy<super::wl_region::WlRegion>>,
            lifetime: u32,
            implementor: F,
        ) -> Result<Proxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>,
            ) -> Proxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>,
        {
            let msg = Request::LockPointer {
                id: self.child_placeholder(),
                surface: surface.clone(),
                pointer: pointer.clone(),
                region: region.map(|o| o.clone()),
                lifetime: lifetime,
            };
            self.send_constructor(msg, implementor, None)
        }
        fn confine_pointer<F>(
            &self,
            surface: &Proxy<super::wl_surface::WlSurface>,
            pointer: &Proxy<super::wl_pointer::WlPointer>,
            region: Option<&Proxy<super::wl_region::WlRegion>>,
            lifetime: u32,
            implementor: F,
        ) -> Result<Proxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>,
            ) -> Proxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>,
        {
            let msg = Request::ConfinePointer {
                id: self.child_placeholder(),
                surface: surface.clone(),
                pointer: pointer.clone(),
                region: region.map(|o| o.clone()),
                lifetime: lifetime,
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_LOCK_POINTER_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CONFINE_POINTER_SINCE: u16 = 1u16;
}
#[doc = "receive relative pointer motion events\n\nThe wp_locked_pointer interface represents a locked pointer state.\n\nWhile the lock of this object is active, the wl_pointer objects of the\nassociated seat will not emit any wl_pointer.motion events.\n\nThis object will send the event 'locked' when the lock is activated.\nWhenever the lock is activated, it is guaranteed that the locked surface\nwill already have received pointer focus and that the pointer will be\nwithin the region passed to the request creating this object.\n\nTo unlock the pointer, send the destroy request. This will also destroy\nthe wp_locked_pointer object.\n\nIf the compositor decides to unlock the pointer the unlocked event is\nsent. See wp_locked_pointer.unlock for details.\n\nWhen unlocking, the compositor may warp the cursor position to the set\ncursor position hint. If it does, it will not result in any relative\nmotion events emitted via wp_relative_pointer.\n\nIf the surface the lock was requested on is destroyed and the lock is not\nyet activated, the wp_locked_pointer object is now defunct and must be\ndestroyed."]
pub mod zwp_locked_pointer_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the locked pointer object\n\nDestroy the locked pointer object. If applicable, the compositor will\nunlock the pointer.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set the pointer cursor position hint\n\nSet the cursor position hint relative to the top left corner of the\nsurface.\n\nIf the client is drawing its own cursor, it should update the position\nhint to the position of its own cursor. A compositor may use this\ninformation to warp the pointer upon unlock in order to avoid pointer\njumps.\n\nThe cursor position hint is double buffered. The new hint will only take\neffect when the associated surface gets it pending state applied. See\nwl_surface.commit for details."]
        SetCursorPositionHint { surface_x: f64, surface_y: f64 },
        #[doc = "set a new lock region\n\nSet a new region used to lock the pointer.\n\nThe new lock region is double-buffered. The new lock region will\nonly take effect when the associated surface gets its pending state\napplied. See wl_surface.commit for details.\n\nFor details about the lock region, see wp_locked_pointer."]
        SetRegion {
            region: Option<Proxy<super::wl_region::WlRegion>>,
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
                name: "set_cursor_position_hint",
                since: 1,
                signature: &[super::ArgumentType::Fixed, super::ArgumentType::Fixed],
            },
            super::MessageDesc {
                name: "set_region",
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
                Request::SetCursorPositionHint { .. } => 1,
                Request::SetRegion { .. } => 2,
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
                Request::SetCursorPositionHint {
                    surface_x,
                    surface_y,
                } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::Fixed((surface_x * 256.) as i32),
                        Argument::Fixed((surface_y * 256.) as i32),
                    ],
                },
                Request::SetRegion { region } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: vec![Argument::Object(region.map(|o| o.id()).unwrap_or(0))],
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
                Request::SetCursorPositionHint {
                    surface_x,
                    surface_y,
                } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].f = (surface_x * 256.) as i32;
                    _args_array[1].f = (surface_y * 256.) as i32;
                    f(1, &mut _args_array)
                }
                Request::SetRegion { region } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = region
                        .map(|o| o.c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(2, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "lock activation event\n\nNotification that the pointer lock of the seat's pointer is activated."]
        Locked,
        #[doc = "lock deactivation event\n\nNotification that the pointer lock of the seat's pointer is no longer\nactive. If this is a oneshot pointer lock (see\nwp_pointer_constraints.lifetime) this object is now defunct and should\nbe destroyed. If this is a persistent pointer lock (see\nwp_pointer_constraints.lifetime) this pointer lock may again\nreactivate in the future."]
        Unlocked,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "locked",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "unlocked",
                since: 1,
                signature: &[],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Locked => 0,
                Event::Unlocked => 1,
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
                0 => Ok(Event::Locked),
                1 => Ok(Event::Unlocked),
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
                0 => Ok(Event::Locked),
                1 => Ok(Event::Unlocked),
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
    pub struct ZwpLockedPointerV1;
    impl Interface for ZwpLockedPointerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_locked_pointer_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_locked_pointer_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the locked pointer object\n\nDestroy the locked pointer object. If applicable, the compositor will\nunlock the pointer.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "set the pointer cursor position hint\n\nSet the cursor position hint relative to the top left corner of the\nsurface.\n\nIf the client is drawing its own cursor, it should update the position\nhint to the position of its own cursor. A compositor may use this\ninformation to warp the pointer upon unlock in order to avoid pointer\njumps.\n\nThe cursor position hint is double buffered. The new hint will only take\neffect when the associated surface gets it pending state applied. See\nwl_surface.commit for details."]
        fn set_cursor_position_hint(&self, surface_x: f64, surface_y: f64) -> ();
        #[doc = "set a new lock region\n\nSet a new region used to lock the pointer.\n\nThe new lock region is double-buffered. The new lock region will\nonly take effect when the associated surface gets its pending state\napplied. See wl_surface.commit for details.\n\nFor details about the lock region, see wp_locked_pointer."]
        fn set_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) -> ();
    }
    impl RequestsTrait for Proxy<ZwpLockedPointerV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn set_cursor_position_hint(&self, surface_x: f64, surface_y: f64) -> () {
            let msg = Request::SetCursorPositionHint {
                surface_x: surface_x,
                surface_y: surface_y,
            };
            self.send(msg);
        }
        fn set_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) -> () {
            let msg = Request::SetRegion {
                region: region.map(|o| o.clone()),
            };
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_CURSOR_POSITION_HINT_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_REGION_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LOCKED_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UNLOCKED_SINCE: u16 = 1u16;
}
#[doc = "confined pointer object\n\nThe wp_confined_pointer interface represents a confined pointer state.\n\nThis object will send the event 'confined' when the confinement is\nactivated. Whenever the confinement is activated, it is guaranteed that\nthe surface the pointer is confined to will already have received pointer\nfocus and that the pointer will be within the region passed to the request\ncreating this object. It is up to the compositor to decide whether this\nrequires some user interaction and if the pointer will warp to within the\npassed region if outside.\n\nTo unconfine the pointer, send the destroy request. This will also destroy\nthe wp_confined_pointer object.\n\nIf the compositor decides to unconfine the pointer the unconfined event is\nsent. The wp_confined_pointer object is at this point defunct and should\nbe destroyed."]
pub mod zwp_confined_pointer_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    pub enum Request {
        #[doc = "destroy the confined pointer object\n\nDestroy the confined pointer object. If applicable, the compositor will\nunconfine the pointer.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set a new confine region\n\nSet a new region used to confine the pointer.\n\nThe new confine region is double-buffered. The new confine region will\nonly take effect when the associated surface gets its pending state\napplied. See wl_surface.commit for details.\n\nIf the confinement is active when the new confinement region is applied\nand the pointer ends up outside of newly applied region, the pointer may\nwarped to a position within the new confinement region. If warped, a\nwl_pointer.motion event will be emitted, but no\nwp_relative_pointer.relative_motion event.\n\nThe compositor may also, instead of using the new region, unconfine the\npointer.\n\nFor details about the confine region, see wp_confined_pointer."]
        SetRegion {
            region: Option<Proxy<super::wl_region::WlRegion>>,
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
                name: "set_region",
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
                Request::SetRegion { .. } => 1,
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
                Request::SetRegion { region } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::Object(region.map(|o| o.id()).unwrap_or(0))],
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
                Request::SetRegion { region } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = region
                        .map(|o| o.c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    f(1, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "pointer confined\n\nNotification that the pointer confinement of the seat's pointer is\nactivated."]
        Confined,
        #[doc = "pointer unconfined\n\nNotification that the pointer confinement of the seat's pointer is no\nlonger active. If this is a oneshot pointer confinement (see\nwp_pointer_constraints.lifetime) this object is now defunct and should\nbe destroyed. If this is a persistent pointer confinement (see\nwp_pointer_constraints.lifetime) this pointer confinement may again\nreactivate in the future."]
        Unconfined,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "confined",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "unconfined",
                since: 1,
                signature: &[],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Confined => 0,
                Event::Unconfined => 1,
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
                0 => Ok(Event::Confined),
                1 => Ok(Event::Unconfined),
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
                0 => Ok(Event::Confined),
                1 => Ok(Event::Unconfined),
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
    pub struct ZwpConfinedPointerV1;
    impl Interface for ZwpConfinedPointerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_confined_pointer_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_confined_pointer_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "destroy the confined pointer object\n\nDestroy the confined pointer object. If applicable, the compositor will\nunconfine the pointer.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
        #[doc = "set a new confine region\n\nSet a new region used to confine the pointer.\n\nThe new confine region is double-buffered. The new confine region will\nonly take effect when the associated surface gets its pending state\napplied. See wl_surface.commit for details.\n\nIf the confinement is active when the new confinement region is applied\nand the pointer ends up outside of newly applied region, the pointer may\nwarped to a position within the new confinement region. If warped, a\nwl_pointer.motion event will be emitted, but no\nwp_relative_pointer.relative_motion event.\n\nThe compositor may also, instead of using the new region, unconfine the\npointer.\n\nFor details about the confine region, see wp_confined_pointer."]
        fn set_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) -> ();
    }
    impl RequestsTrait for Proxy<ZwpConfinedPointerV1> {
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
        fn set_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) -> () {
            let msg = Request::SetRegion {
                region: region.map(|o| o.clone()),
            };
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_REGION_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CONFINED_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_UNCONFINED_SINCE: u16 = 1u16;
}
