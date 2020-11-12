#[doc = "create surfaces that are layers of the desktop\n\nClients can use this interface to assign the surface_layer role to\nwl_surfaces. Such surfaces are assigned to a \"layer\" of the output and\nrendered with a defined z-depth respective to each other. They may also be\nanchored to the edges and corners of a screen and specify input handling\nsemantics. This interface should be suitable for the implementation of\nmany desktop shell components, and a broad number of other applications\nthat interact with the desktop."]
pub mod zwlr_layer_shell_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Error {
        #[doc = "wl_surface has another role"]
        Role = 0,
        #[doc = "layer value is invalid"]
        InvalidLayer = 1,
        #[doc = "wl_surface has a buffer attached or committed"]
        AlreadyConstructed = 2,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                1 => Some(Error::InvalidLayer),
                2 => Some(Error::AlreadyConstructed),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[doc = "available layers for surfaces\n\nThese values indicate which layers a surface can be rendered in. They\nare ordered by z depth, bottom-most first. Traditional shell surfaces\nwill typically be rendered between the bottom and top layers.\nFullscreen shell surfaces are typically rendered at the top layer.\nMultiple surfaces can share a single layer, and ordering within a\nsingle layer is undefined."]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Layer {
        Background = 0,
        Bottom = 1,
        Top = 2,
        Overlay = 3,
    }
    impl Layer {
        pub fn from_raw(n: u32) -> Option<Layer> {
            match n {
                0 => Some(Layer::Background),
                1 => Some(Layer::Bottom),
                2 => Some(Layer::Top),
                3 => Some(Layer::Overlay),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub enum Request {
        #[doc = "create a layer_surface from a surface\n\nCreate a layer surface for an existing surface. This assigns the role of\nlayer_surface, or raises a protocol error if another role is already\nassigned.\n\nCreating a layer surface from a wl_surface which has a buffer attached\nor committed is a client error, and any attempts by a client to attach\nor manipulate a buffer prior to the first layer_surface.configure call\nmust also be treated as errors.\n\nYou may pass NULL for output to allow the compositor to decide which\noutput to use. Generally this will be the one that the user most\nrecently interacted with.\n\nClients can specify a namespace that defines the purpose of the layer\nsurface."]
        GetLayerSurface {
            id: Proxy<super::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>,
            surface: Proxy<super::wl_surface::WlSurface>,
            output: Option<Proxy<super::wl_output::WlOutput>>,
            layer: Layer,
            namespace: String,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "get_layer_surface",
            since: 1,
            signature: &[
                super::ArgumentType::NewId,
                super::ArgumentType::Object,
                super::ArgumentType::Object,
                super::ArgumentType::Uint,
                super::ArgumentType::Str,
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
                Request::GetLayerSurface { .. } => 0,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetLayerSurface {
                    id,
                    surface,
                    output,
                    layer,
                    namespace,
                } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![
                        Argument::NewId(id.id()),
                        Argument::Object(surface.id()),
                        Argument::Object(output.map(|o| o.id()).unwrap_or(0)),
                        Argument::Uint(layer.to_raw()),
                        Argument::Str(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(namespace.into())
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
                Request::GetLayerSurface {
                    id,
                    surface,
                    output,
                    layer,
                    namespace,
                } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    _args_array[2].o = output
                        .map(|o| o.c_ptr() as *mut _)
                        .unwrap_or(::std::ptr::null_mut());
                    _args_array[3].u = layer.to_raw();
                    let _arg_4 = ::std::ffi::CString::new(namespace).unwrap();
                    _args_array[4].s = _arg_4.as_ptr();
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
    pub struct ZwlrLayerShellV1;
    impl Interface for ZwlrLayerShellV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_layer_shell_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_layer_shell_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "create a layer_surface from a surface\n\nCreate a layer surface for an existing surface. This assigns the role of\nlayer_surface, or raises a protocol error if another role is already\nassigned.\n\nCreating a layer surface from a wl_surface which has a buffer attached\nor committed is a client error, and any attempts by a client to attach\nor manipulate a buffer prior to the first layer_surface.configure call\nmust also be treated as errors.\n\nYou may pass NULL for output to allow the compositor to decide which\noutput to use. Generally this will be the one that the user most\nrecently interacted with.\n\nClients can specify a namespace that defines the purpose of the layer\nsurface."]
        fn get_layer_surface<F>(
            &self,
            surface: &Proxy<super::wl_surface::WlSurface>,
            output: Option<&Proxy<super::wl_output::WlOutput>>,
            layer: Layer,
            namespace: String,
            implementor: F,
        ) -> Result<Proxy<super::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>,
            ) -> Proxy<super::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>;
    }
    impl RequestsTrait for Proxy<ZwlrLayerShellV1> {
        fn get_layer_surface<F>(
            &self,
            surface: &Proxy<super::wl_surface::WlSurface>,
            output: Option<&Proxy<super::wl_output::WlOutput>>,
            layer: Layer,
            namespace: String,
            implementor: F,
        ) -> Result<Proxy<super::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>, ()>
        where
            F: FnOnce(
                NewProxy<super::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>,
            ) -> Proxy<super::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>,
        {
            let msg = Request::GetLayerSurface {
                id: self.child_placeholder(),
                surface: surface.clone(),
                output: output.map(|o| o.clone()),
                layer: layer,
                namespace: namespace,
            };
            self.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_LAYER_SURFACE_SINCE: u16 = 1u16;
}
#[doc = "layer metadata interface\n\nAn interface that may be implemented by a wl_surface, for surfaces that\nare designed to be rendered as a layer of a stacked desktop-like\nenvironment.\n\nLayer surface state (size, anchor, exclusive zone, margin, interactivity)\nis double-buffered, and will be applied at the time wl_surface.commit of\nthe corresponding wl_surface is called."]
pub mod zwlr_layer_surface_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface};
    use super::{
        AnonymousObject, Argument, ArgumentType, Interface, Message, MessageDesc, MessageGroup,
        NewProxy, Object, ObjectMetadata, Proxy,
    };
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Error {
        #[doc = "provided surface state is invalid"]
        InvalidSurfaceState = 0,
        #[doc = "size is invalid"]
        InvalidSize = 1,
        #[doc = "anchor bitfield is invalid"]
        InvalidAnchor = 2,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidSurfaceState),
                1 => Some(Error::InvalidSize),
                2 => Some(Error::InvalidAnchor),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    bitflags! { pub struct Anchor : u32 { # [ doc = "the top edge of the anchor rectangle" ] const Top = 1 ; # [ doc = "the bottom edge of the anchor rectangle" ] const Bottom = 2 ; # [ doc = "the left edge of the anchor rectangle" ] const Left = 4 ; # [ doc = "the right edge of the anchor rectangle" ] const Right = 8 ; } }
    impl Anchor {
        pub fn from_raw(n: u32) -> Option<Anchor> {
            Some(Anchor::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    pub enum Request {
        #[doc = "sets the size of the surface\n\nSets the size of the surface in surface-local coordinates. The\ncompositor will display the surface centered with respect to its\nanchors.\n\nIf you pass 0 for either value, the compositor will assign it and\ninform you of the assignment in the configure event. You must set your\nanchor to opposite edges in the dimensions you omit; not doing so is a\nprotocol error. Both values are 0 by default.\n\nSize is double-buffered, see wl_surface.commit."]
        SetSize { width: u32, height: u32 },
        #[doc = "configures the anchor point of the surface\n\nRequests that the compositor anchor the surface to the specified edges\nand corners. If two orthogonal edges are specified (e.g. 'top' and\n'left'), then the anchor point will be the intersection of the edges\n(e.g. the top left corner of the output); otherwise the anchor point\nwill be centered on that edge, or in the center if none is specified.\n\nAnchor is double-buffered, see wl_surface.commit."]
        SetAnchor { anchor: Anchor },
        #[doc = "configures the exclusive geometry of this surface\n\nRequests that the compositor avoids occluding an area of the surface\nwith other surfaces. The compositor's use of this information is\nimplementation-dependent - do not assume that this region will not\nactually be occluded.\n\nA positive value is only meaningful if the surface is anchored to an\nedge, rather than a corner. The zone is the number of surface-local\ncoordinates from the edge that is considered exclusive.\n\nSurfaces that do not wish to have an exclusive zone may instead specify\nhow they should interact with surfaces that do. If set to zero, the\nsurface indicates that it would like to be moved to avoid occluding\nsurfaces with a positive exclusive zone. If set to -1, the surface\nindicates that it would not like to be moved to accommodate for other\nsurfaces, and the compositor should extend it all the way to the edges\nit is anchored to.\n\nFor example, a panel might set its exclusive zone to 10, so that\nmaximized shell surfaces are not shown on top of it. A notification\nmight set its exclusive zone to 0, so that it is moved to avoid\noccluding the panel, but shell surfaces are shown underneath it. A\nwallpaper or lock screen might set their exclusive zone to -1, so that\nthey stretch below or over the panel.\n\nThe default value is 0.\n\nExclusive zone is double-buffered, see wl_surface.commit."]
        SetExclusiveZone { zone: i32 },
        #[doc = "sets a margin from the anchor point\n\nRequests that the surface be placed some distance away from the anchor\npoint on the output, in surface-local coordinates. Setting this value\nfor edges you are not anchored to has no effect.\n\nThe exclusive zone includes the margin.\n\nMargin is double-buffered, see wl_surface.commit."]
        SetMargin {
            top: i32,
            right: i32,
            bottom: i32,
            left: i32,
        },
        #[doc = "requests keyboard events\n\nSet to 1 to request that the seat send keyboard events to this layer\nsurface. For layers below the shell surface layer, the seat will use\nnormal focus semantics. For layers above the shell surface layers, the\nseat will always give exclusive keyboard focus to the top-most layer\nwhich has keyboard interactivity set to true.\n\nLayer surfaces receive pointer, touch, and tablet events normally. If\nyou do not want to receive them, set the input region on your surface\nto an empty region.\n\nEvents is double-buffered, see wl_surface.commit."]
        SetKeyboardInteractivity { keyboard_interactivity: u32 },
        #[doc = "assign this layer_surface as an xdg_popup parent\n\nThis assigns an xdg_popup's parent to this layer_surface.  This popup\nshould have been created via xdg_surface::get_popup with the parent set\nto NULL, and this request must be invoked before committing the popup's\ninitial state.\n\nSee the documentation of xdg_popup for more details about what an\nxdg_popup is and how it is used."]
        GetPopup {
            popup: Proxy<super::xdg_popup::XdgPopup>,
        },
        #[doc = "ack a configure event\n\nWhen a configure event is received, if a client commits the\nsurface in response to the configure event, then the client\nmust make an ack_configure request sometime before the commit\nrequest, passing along the serial of the configure event.\n\nIf the client receives multiple configure events before it\ncan respond to one, it only has to ack the last configure event.\n\nA client is not required to commit immediately after sending\nan ack_configure request - it may even ack_configure several times\nbefore its next surface commit.\n\nA client may send multiple ack_configure requests before committing, but\nonly the last request sent before a commit indicates which configure\nevent the client really is responding to."]
        AckConfigure { serial: u32 },
        #[doc = "destroy the layer_surface\n\nThis request destroys the layer surface.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_size",
                since: 1,
                signature: &[super::ArgumentType::Uint, super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "set_anchor",
                since: 1,
                signature: &[super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "set_exclusive_zone",
                since: 1,
                signature: &[super::ArgumentType::Int],
            },
            super::MessageDesc {
                name: "set_margin",
                since: 1,
                signature: &[
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
            },
            super::MessageDesc {
                name: "set_keyboard_interactivity",
                since: 1,
                signature: &[super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "get_popup",
                since: 1,
                signature: &[super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "ack_configure",
                since: 1,
                signature: &[super::ArgumentType::Uint],
            },
            super::MessageDesc {
                name: "destroy",
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
                Request::SetSize { .. } => 0,
                Request::SetAnchor { .. } => 1,
                Request::SetExclusiveZone { .. } => 2,
                Request::SetMargin { .. } => 3,
                Request::SetKeyboardInteractivity { .. } => 4,
                Request::GetPopup { .. } => 5,
                Request::AckConfigure { .. } => 6,
                Request::Destroy => 7,
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
                Request::SetSize { width, height } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![Argument::Uint(width), Argument::Uint(height)],
                },
                Request::SetAnchor { anchor } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::Uint(anchor.to_raw())],
                },
                Request::SetExclusiveZone { zone } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: vec![Argument::Int(zone)],
                },
                Request::SetMargin {
                    top,
                    right,
                    bottom,
                    left,
                } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: vec![
                        Argument::Int(top),
                        Argument::Int(right),
                        Argument::Int(bottom),
                        Argument::Int(left),
                    ],
                },
                Request::SetKeyboardInteractivity {
                    keyboard_interactivity,
                } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: vec![Argument::Uint(keyboard_interactivity)],
                },
                Request::GetPopup { popup } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: vec![Argument::Object(popup.id())],
                },
                Request::AckConfigure { serial } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: vec![Argument::Uint(serial)],
                },
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 7,
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
                Request::SetSize { width, height } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = width;
                    _args_array[1].u = height;
                    f(0, &mut _args_array)
                }
                Request::SetAnchor { anchor } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = anchor.to_raw();
                    f(1, &mut _args_array)
                }
                Request::SetExclusiveZone { zone } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = zone;
                    f(2, &mut _args_array)
                }
                Request::SetMargin {
                    top,
                    right,
                    bottom,
                    left,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = top;
                    _args_array[1].i = right;
                    _args_array[2].i = bottom;
                    _args_array[3].i = left;
                    f(3, &mut _args_array)
                }
                Request::SetKeyboardInteractivity {
                    keyboard_interactivity,
                } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = keyboard_interactivity;
                    f(4, &mut _args_array)
                }
                Request::GetPopup { popup } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = popup.c_ptr() as *mut _;
                    f(5, &mut _args_array)
                }
                Request::AckConfigure { serial } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(6, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(7, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "suggest a surface change\n\nThe configure event asks the client to resize its surface.\n\nClients should arrange their surface for the new states, and then send\nan ack_configure request with the serial sent in this configure event at\nsome point before committing the new surface.\n\nThe client is free to dismiss all but the last configure event it\nreceived.\n\nThe width and height arguments specify the size of the window in\nsurface-local coordinates.\n\nThe size is a hint, in the sense that the client is free to ignore it if\nit doesn't resize, pick a smaller size (to satisfy aspect ratio or\nresize in steps of NxM pixels). If the client picks a smaller size and\nis anchored to two opposite anchors (e.g. 'top' and 'bottom'), the\nsurface will be centered on this axis.\n\nIf the width or height arguments are zero, it means the client should\ndecide its own window dimension."]
        Configure {
            serial: u32,
            width: u32,
            height: u32,
        },
        #[doc = "surface should be closed\n\nThe closed event is sent by the compositor when the surface will no\nlonger be shown. The output may have been destroyed or the user may\nhave asked for it to be removed. Further changes to the surface will be\nignored. The client should destroy the resource after receiving this\nevent, and create a new surface if they so choose."]
        Closed,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "configure",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
            },
            super::MessageDesc {
                name: "closed",
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
                Event::Configure { .. } => 0,
                Event::Closed => 1,
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
                        serial: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        width: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        height: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Event::Closed),
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
                    Ok(Event::Configure {
                        serial: _args[0].u,
                        width: _args[1].u,
                        height: _args[2].u,
                    })
                }
                1 => Ok(Event::Closed),
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
    pub struct ZwlrLayerSurfaceV1;
    impl Interface for ZwlrLayerSurfaceV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_layer_surface_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwlr_layer_surface_v1_interface }
        }
    }
    pub trait RequestsTrait {
        #[doc = "sets the size of the surface\n\nSets the size of the surface in surface-local coordinates. The\ncompositor will display the surface centered with respect to its\nanchors.\n\nIf you pass 0 for either value, the compositor will assign it and\ninform you of the assignment in the configure event. You must set your\nanchor to opposite edges in the dimensions you omit; not doing so is a\nprotocol error. Both values are 0 by default.\n\nSize is double-buffered, see wl_surface.commit."]
        fn set_size(&self, width: u32, height: u32) -> ();
        #[doc = "configures the anchor point of the surface\n\nRequests that the compositor anchor the surface to the specified edges\nand corners. If two orthogonal edges are specified (e.g. 'top' and\n'left'), then the anchor point will be the intersection of the edges\n(e.g. the top left corner of the output); otherwise the anchor point\nwill be centered on that edge, or in the center if none is specified.\n\nAnchor is double-buffered, see wl_surface.commit."]
        fn set_anchor(&self, anchor: Anchor) -> ();
        #[doc = "configures the exclusive geometry of this surface\n\nRequests that the compositor avoids occluding an area of the surface\nwith other surfaces. The compositor's use of this information is\nimplementation-dependent - do not assume that this region will not\nactually be occluded.\n\nA positive value is only meaningful if the surface is anchored to an\nedge, rather than a corner. The zone is the number of surface-local\ncoordinates from the edge that is considered exclusive.\n\nSurfaces that do not wish to have an exclusive zone may instead specify\nhow they should interact with surfaces that do. If set to zero, the\nsurface indicates that it would like to be moved to avoid occluding\nsurfaces with a positive exclusive zone. If set to -1, the surface\nindicates that it would not like to be moved to accommodate for other\nsurfaces, and the compositor should extend it all the way to the edges\nit is anchored to.\n\nFor example, a panel might set its exclusive zone to 10, so that\nmaximized shell surfaces are not shown on top of it. A notification\nmight set its exclusive zone to 0, so that it is moved to avoid\noccluding the panel, but shell surfaces are shown underneath it. A\nwallpaper or lock screen might set their exclusive zone to -1, so that\nthey stretch below or over the panel.\n\nThe default value is 0.\n\nExclusive zone is double-buffered, see wl_surface.commit."]
        fn set_exclusive_zone(&self, zone: i32) -> ();
        #[doc = "sets a margin from the anchor point\n\nRequests that the surface be placed some distance away from the anchor\npoint on the output, in surface-local coordinates. Setting this value\nfor edges you are not anchored to has no effect.\n\nThe exclusive zone includes the margin.\n\nMargin is double-buffered, see wl_surface.commit."]
        fn set_margin(&self, top: i32, right: i32, bottom: i32, left: i32) -> ();
        #[doc = "requests keyboard events\n\nSet to 1 to request that the seat send keyboard events to this layer\nsurface. For layers below the shell surface layer, the seat will use\nnormal focus semantics. For layers above the shell surface layers, the\nseat will always give exclusive keyboard focus to the top-most layer\nwhich has keyboard interactivity set to true.\n\nLayer surfaces receive pointer, touch, and tablet events normally. If\nyou do not want to receive them, set the input region on your surface\nto an empty region.\n\nEvents is double-buffered, see wl_surface.commit."]
        fn set_keyboard_interactivity(&self, keyboard_interactivity: u32) -> ();
        #[doc = "assign this layer_surface as an xdg_popup parent\n\nThis assigns an xdg_popup's parent to this layer_surface.  This popup\nshould have been created via xdg_surface::get_popup with the parent set\nto NULL, and this request must be invoked before committing the popup's\ninitial state.\n\nSee the documentation of xdg_popup for more details about what an\nxdg_popup is and how it is used."]
        fn get_popup(&self, popup: &Proxy<super::xdg_popup::XdgPopup>) -> ();
        #[doc = "ack a configure event\n\nWhen a configure event is received, if a client commits the\nsurface in response to the configure event, then the client\nmust make an ack_configure request sometime before the commit\nrequest, passing along the serial of the configure event.\n\nIf the client receives multiple configure events before it\ncan respond to one, it only has to ack the last configure event.\n\nA client is not required to commit immediately after sending\nan ack_configure request - it may even ack_configure several times\nbefore its next surface commit.\n\nA client may send multiple ack_configure requests before committing, but\nonly the last request sent before a commit indicates which configure\nevent the client really is responding to."]
        fn ack_configure(&self, serial: u32) -> ();
        #[doc = "destroy the layer_surface\n\nThis request destroys the layer surface.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        fn destroy(&self) -> ();
    }
    impl RequestsTrait for Proxy<ZwlrLayerSurfaceV1> {
        fn set_size(&self, width: u32, height: u32) -> () {
            let msg = Request::SetSize {
                width: width,
                height: height,
            };
            self.send(msg);
        }
        fn set_anchor(&self, anchor: Anchor) -> () {
            let msg = Request::SetAnchor { anchor: anchor };
            self.send(msg);
        }
        fn set_exclusive_zone(&self, zone: i32) -> () {
            let msg = Request::SetExclusiveZone { zone: zone };
            self.send(msg);
        }
        fn set_margin(&self, top: i32, right: i32, bottom: i32, left: i32) -> () {
            let msg = Request::SetMargin {
                top: top,
                right: right,
                bottom: bottom,
                left: left,
            };
            self.send(msg);
        }
        fn set_keyboard_interactivity(&self, keyboard_interactivity: u32) -> () {
            let msg = Request::SetKeyboardInteractivity {
                keyboard_interactivity: keyboard_interactivity,
            };
            self.send(msg);
        }
        fn get_popup(&self, popup: &Proxy<super::xdg_popup::XdgPopup>) -> () {
            let msg = Request::GetPopup {
                popup: popup.clone(),
            };
            self.send(msg);
        }
        fn ack_configure(&self, serial: u32) -> () {
            let msg = Request::AckConfigure { serial: serial };
            self.send(msg);
        }
        fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_SIZE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_ANCHOR_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_EXCLUSIVE_ZONE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MARGIN_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_KEYBOARD_INTERACTIVITY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_POPUP_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ACK_CONFIGURE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CONFIGURE_SINCE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CLOSED_SINCE: u16 = 1u16;
}
