#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindow(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindow,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppWindow {
    pub fn Create() -> windows_core::Result<Self> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateWithPresenter<P0>(appwindowpresenter: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<AppWindowPresenter>,
    {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithPresenter)(
                windows_core::Interface::as_raw(this),
                appwindowpresenter.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateWithPresenterAndOwner<P0>(
        appwindowpresenter: P0,
        ownerwindowid: WindowId,
    ) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<AppWindowPresenter>,
    {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithPresenterAndOwner)(
                windows_core::Interface::as_raw(this),
                appwindowpresenter.param().abi(),
                ownerwindowid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn GetFromWindowId(windowid: WindowId) -> windows_core::Result<Self> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFromWindowId)(
                windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateWithDispatcherQueue<P0, P2>(
        appwindowpresenter: P0,
        ownerwindowid: WindowId,
        dispatcherqueue: P2,
    ) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<AppWindowPresenter>,
        P2: windows_core::Param<DispatcherQueue>,
    {
        Self::IAppWindowStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithDispatcherQueue)(
                windows_core::Interface::as_raw(this),
                appwindowpresenter.param().abi(),
                ownerwindowid,
                dispatcherqueue.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppWindow, IAppWindowStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IAppWindowStatics2<R, F: FnOnce(&IAppWindowStatics2) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppWindow, IAppWindowStatics2> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindow>();
}
unsafe impl windows_core::Interface for AppWindow {
    type Vtable = <IAppWindow as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindow as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindow {
    type Target = IAppWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindow {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindow";
}
unsafe impl Send for AppWindow {}
unsafe impl Sync for AppWindow {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindowChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindowChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppWindowChangedEventArgs {
    type Vtable = <IAppWindowChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindowChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindowChangedEventArgs {
    type Target = IAppWindowChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowChangedEventArgs";
}
unsafe impl Send for AppWindowChangedEventArgs {}
unsafe impl Sync for AppWindowChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindowClosingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindowClosingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for AppWindowClosingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowClosingEventArgs>();
}
unsafe impl windows_core::Interface for AppWindowClosingEventArgs {
    type Vtable = <IAppWindowClosingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindowClosingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindowClosingEventArgs {
    type Target = IAppWindowClosingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindowClosingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowClosingEventArgs";
}
unsafe impl Send for AppWindowClosingEventArgs {}
unsafe impl Sync for AppWindowClosingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindowPresenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindowPresenter,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowPresenter>();
}
unsafe impl windows_core::Interface for AppWindowPresenter {
    type Vtable = <IAppWindowPresenter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindowPresenter as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindowPresenter {
    type Target = IAppWindowPresenter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowPresenter";
}
unsafe impl Send for AppWindowPresenter {}
unsafe impl Sync for AppWindowPresenter {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppWindowPresenterKind(pub i32);
impl AppWindowPresenterKind {
    pub const Default: Self = Self(0);
    pub const CompactOverlay: Self = Self(1);
    pub const FullScreen: Self = Self(2);
    pub const Overlapped: Self = Self(3);
}
impl windows_core::imp::TypeKind for AppWindowPresenterKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for AppWindowPresenterKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.AppWindowPresenterKind;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindowTitleBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindowTitleBar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppWindowTitleBar {
    pub fn IsCustomizationSupported() -> windows_core::Result<bool> {
        Self::IAppWindowTitleBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCustomizationSupported)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    fn IAppWindowTitleBarStatics<
        R,
        F: FnOnce(&IAppWindowTitleBarStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            AppWindowTitleBar,
            IAppWindowTitleBarStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowTitleBar>();
}
unsafe impl windows_core::Interface for AppWindowTitleBar {
    type Vtable = <IAppWindowTitleBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppWindowTitleBar as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppWindowTitleBar {
    type Target = IAppWindowTitleBar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowTitleBar";
}
unsafe impl Send for AppWindowTitleBar {}
unsafe impl Sync for AppWindowTitleBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharacterReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CharacterReceivedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for CharacterReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICharacterReceivedEventArgs>();
}
unsafe impl windows_core::Interface for CharacterReceivedEventArgs {
    type Vtable = <ICharacterReceivedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICharacterReceivedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for CharacterReceivedEventArgs {
    type Target = ICharacterReceivedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CharacterReceivedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.CharacterReceivedEventArgs";
}
unsafe impl Send for CharacterReceivedEventArgs {}
unsafe impl Sync for CharacterReceivedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompactOverlayPresenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompactOverlayPresenter,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CompactOverlayPresenter, AppWindowPresenter);
impl CompactOverlayPresenter {
    pub fn Create() -> windows_core::Result<Self> {
        Self::ICompactOverlayPresenterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn ICompactOverlayPresenterStatics<
        R,
        F: FnOnce(&ICompactOverlayPresenterStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            CompactOverlayPresenter,
            ICompactOverlayPresenterStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompactOverlayPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompactOverlayPresenter>();
}
unsafe impl windows_core::Interface for CompactOverlayPresenter {
    type Vtable = <ICompactOverlayPresenter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompactOverlayPresenter as windows_core::Interface>::IID;
}
impl core::ops::Deref for CompactOverlayPresenter {
    type Target = ICompactOverlayPresenter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CompactOverlayPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.CompactOverlayPresenter";
}
unsafe impl Send for CompactOverlayPresenter {}
unsafe impl Sync for CompactOverlayPresenter {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompactOverlaySize(pub i32);
impl CompactOverlaySize {
    pub const Small: Self = Self(0);
    pub const Medium: Self = Self(1);
    pub const Large: Self = Self(2);
}
impl windows_core::imp::TypeKind for CompactOverlaySize {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for CompactOverlaySize {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.CompactOverlaySize;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextMenuKeyEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContextMenuKeyEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ContextMenuKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContextMenuKeyEventArgs>();
}
unsafe impl windows_core::Interface for ContextMenuKeyEventArgs {
    type Vtable = <IContextMenuKeyEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContextMenuKeyEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContextMenuKeyEventArgs {
    type Target = IContextMenuKeyEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContextMenuKeyEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ContextMenuKeyEventArgs";
}
unsafe impl Send for ContextMenuKeyEventArgs {}
unsafe impl Sync for ContextMenuKeyEventArgs {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CrossSlideThresholds {
    pub selection_start: f32,
    pub speed_bump_start: f32,
    pub speed_bump_end: f32,
    pub rearrange_start: f32,
}
impl windows_core::imp::TypeKind for CrossSlideThresholds {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossSlidingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CrossSlidingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICrossSlidingEventArgs>();
}
unsafe impl windows_core::Interface for CrossSlidingEventArgs {
    type Vtable = <ICrossSlidingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICrossSlidingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for CrossSlidingEventArgs {
    type Target = ICrossSlidingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.CrossSlidingEventArgs";
}
unsafe impl Send for CrossSlidingEventArgs {}
unsafe impl Sync for CrossSlidingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0);
    pub const Dragging: Self = Self(1);
    pub const Selecting: Self = Self(2);
    pub const SelectSpeedBumping: Self = Self(3);
    pub const SpeedBumping: Self = Self(4);
    pub const Rearranging: Self = Self(5);
    pub const Completed: Self = Self(6);
}
impl windows_core::imp::TypeKind for CrossSlidingState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for CrossSlidingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.CrossSlidingState;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DirectXAlphaMode(pub i32);
impl DirectXAlphaMode {
    pub const Unspecified: Self = Self(0);
    pub const Premultiplied: Self = Self(1);
    pub const Straight: Self = Self(2);
    pub const Ignore: Self = Self(3);
}
impl windows_core::imp::TypeKind for DirectXAlphaMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DirectXAlphaMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.DirectX.DirectXAlphaMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DirectXColorSpace(pub i32);
impl DirectXColorSpace {
    pub const RgbFullG22NoneP709: Self = Self(0);
    pub const RgbFullG10NoneP709: Self = Self(1);
    pub const RgbStudioG22NoneP709: Self = Self(2);
    pub const RgbStudioG22NoneP2020: Self = Self(3);
    pub const Reserved: Self = Self(4);
    pub const YccFullG22NoneP709X601: Self = Self(5);
    pub const YccStudioG22LeftP601: Self = Self(6);
    pub const YccFullG22LeftP601: Self = Self(7);
    pub const YccStudioG22LeftP709: Self = Self(8);
    pub const YccFullG22LeftP709: Self = Self(9);
    pub const YccStudioG22LeftP2020: Self = Self(10);
    pub const YccFullG22LeftP2020: Self = Self(11);
    pub const RgbFullG2084NoneP2020: Self = Self(12);
    pub const YccStudioG2084LeftP2020: Self = Self(13);
    pub const RgbStudioG2084NoneP2020: Self = Self(14);
    pub const YccStudioG22TopLeftP2020: Self = Self(15);
    pub const YccStudioG2084TopLeftP2020: Self = Self(16);
    pub const RgbFullG22NoneP2020: Self = Self(17);
    pub const YccStudioGHlgTopLeftP2020: Self = Self(18);
    pub const YccFullGHlgTopLeftP2020: Self = Self(19);
    pub const RgbStudioG24NoneP709: Self = Self(20);
    pub const RgbStudioG24NoneP2020: Self = Self(21);
    pub const YccStudioG24LeftP709: Self = Self(22);
    pub const YccStudioG24LeftP2020: Self = Self(23);
    pub const YccStudioG24TopLeftP2020: Self = Self(24);
    pub const RgbFullG10NoneP2020: Self = Self(25);
    pub const RgbFullG22ExtNoneP709: Self = Self(26);
    pub const RgbFullG22NoneP3: Self = Self(27);
}
impl windows_core::imp::TypeKind for DirectXColorSpace {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DirectXColorSpace {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.DirectX.DirectXColorSpace;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DirectXPixelFormat(pub i32);
impl DirectXPixelFormat {
    pub const Unknown: Self = Self(0);
    pub const R32G32B32A32Typeless: Self = Self(1);
    pub const R32G32B32A32Float: Self = Self(2);
    pub const R32G32B32A32UInt: Self = Self(3);
    pub const R32G32B32A32Int: Self = Self(4);
    pub const R32G32B32Typeless: Self = Self(5);
    pub const R32G32B32Float: Self = Self(6);
    pub const R32G32B32UInt: Self = Self(7);
    pub const R32G32B32Int: Self = Self(8);
    pub const R16G16B16A16Typeless: Self = Self(9);
    pub const R16G16B16A16Float: Self = Self(10);
    pub const R16G16B16A16UIntNormalized: Self = Self(11);
    pub const R16G16B16A16UInt: Self = Self(12);
    pub const R16G16B16A16IntNormalized: Self = Self(13);
    pub const R16G16B16A16Int: Self = Self(14);
    pub const R32G32Typeless: Self = Self(15);
    pub const R32G32Float: Self = Self(16);
    pub const R32G32UInt: Self = Self(17);
    pub const R32G32Int: Self = Self(18);
    pub const R32G8X24Typeless: Self = Self(19);
    pub const D32FloatS8X24UInt: Self = Self(20);
    pub const R32FloatX8X24Typeless: Self = Self(21);
    pub const X32TypelessG8X24UInt: Self = Self(22);
    pub const R10G10B10A2Typeless: Self = Self(23);
    pub const R10G10B10A2UIntNormalized: Self = Self(24);
    pub const R10G10B10A2UInt: Self = Self(25);
    pub const R11G11B10Float: Self = Self(26);
    pub const R8G8B8A8Typeless: Self = Self(27);
    pub const R8G8B8A8UIntNormalized: Self = Self(28);
    pub const R8G8B8A8UIntNormalizedSrgb: Self = Self(29);
    pub const R8G8B8A8UInt: Self = Self(30);
    pub const R8G8B8A8IntNormalized: Self = Self(31);
    pub const R8G8B8A8Int: Self = Self(32);
    pub const R16G16Typeless: Self = Self(33);
    pub const R16G16Float: Self = Self(34);
    pub const R16G16UIntNormalized: Self = Self(35);
    pub const R16G16UInt: Self = Self(36);
    pub const R16G16IntNormalized: Self = Self(37);
    pub const R16G16Int: Self = Self(38);
    pub const R32Typeless: Self = Self(39);
    pub const D32Float: Self = Self(40);
    pub const R32Float: Self = Self(41);
    pub const R32UInt: Self = Self(42);
    pub const R32Int: Self = Self(43);
    pub const R24G8Typeless: Self = Self(44);
    pub const D24UIntNormalizedS8UInt: Self = Self(45);
    pub const R24UIntNormalizedX8Typeless: Self = Self(46);
    pub const X24TypelessG8UInt: Self = Self(47);
    pub const R8G8Typeless: Self = Self(48);
    pub const R8G8UIntNormalized: Self = Self(49);
    pub const R8G8UInt: Self = Self(50);
    pub const R8G8IntNormalized: Self = Self(51);
    pub const R8G8Int: Self = Self(52);
    pub const R16Typeless: Self = Self(53);
    pub const R16Float: Self = Self(54);
    pub const D16UIntNormalized: Self = Self(55);
    pub const R16UIntNormalized: Self = Self(56);
    pub const R16UInt: Self = Self(57);
    pub const R16IntNormalized: Self = Self(58);
    pub const R16Int: Self = Self(59);
    pub const R8Typeless: Self = Self(60);
    pub const R8UIntNormalized: Self = Self(61);
    pub const R8UInt: Self = Self(62);
    pub const R8IntNormalized: Self = Self(63);
    pub const R8Int: Self = Self(64);
    pub const A8UIntNormalized: Self = Self(65);
    pub const R1UIntNormalized: Self = Self(66);
    pub const R9G9B9E5SharedExponent: Self = Self(67);
    pub const R8G8B8G8UIntNormalized: Self = Self(68);
    pub const G8R8G8B8UIntNormalized: Self = Self(69);
    pub const BC1Typeless: Self = Self(70);
    pub const BC1UIntNormalized: Self = Self(71);
    pub const BC1UIntNormalizedSrgb: Self = Self(72);
    pub const BC2Typeless: Self = Self(73);
    pub const BC2UIntNormalized: Self = Self(74);
    pub const BC2UIntNormalizedSrgb: Self = Self(75);
    pub const BC3Typeless: Self = Self(76);
    pub const BC3UIntNormalized: Self = Self(77);
    pub const BC3UIntNormalizedSrgb: Self = Self(78);
    pub const BC4Typeless: Self = Self(79);
    pub const BC4UIntNormalized: Self = Self(80);
    pub const BC4IntNormalized: Self = Self(81);
    pub const BC5Typeless: Self = Self(82);
    pub const BC5UIntNormalized: Self = Self(83);
    pub const BC5IntNormalized: Self = Self(84);
    pub const B5G6R5UIntNormalized: Self = Self(85);
    pub const B5G5R5A1UIntNormalized: Self = Self(86);
    pub const B8G8R8A8UIntNormalized: Self = Self(87);
    pub const B8G8R8X8UIntNormalized: Self = Self(88);
    pub const R10G10B10XRBiasA2UIntNormalized: Self = Self(89);
    pub const B8G8R8A8Typeless: Self = Self(90);
    pub const B8G8R8A8UIntNormalizedSrgb: Self = Self(91);
    pub const B8G8R8X8Typeless: Self = Self(92);
    pub const B8G8R8X8UIntNormalizedSrgb: Self = Self(93);
    pub const BC6HTypeless: Self = Self(94);
    pub const BC6H16UnsignedFloat: Self = Self(95);
    pub const BC6H16Float: Self = Self(96);
    pub const BC7Typeless: Self = Self(97);
    pub const BC7UIntNormalized: Self = Self(98);
    pub const BC7UIntNormalizedSrgb: Self = Self(99);
    pub const Ayuv: Self = Self(100);
    pub const Y410: Self = Self(101);
    pub const Y416: Self = Self(102);
    pub const NV12: Self = Self(103);
    pub const P010: Self = Self(104);
    pub const P016: Self = Self(105);
    pub const Opaque420: Self = Self(106);
    pub const Yuy2: Self = Self(107);
    pub const Y210: Self = Self(108);
    pub const Y216: Self = Self(109);
    pub const NV11: Self = Self(110);
    pub const AI44: Self = Self(111);
    pub const IA44: Self = Self(112);
    pub const P8: Self = Self(113);
    pub const A8P8: Self = Self(114);
    pub const B4G4R4A4UIntNormalized: Self = Self(115);
    pub const P208: Self = Self(130);
    pub const V208: Self = Self(131);
    pub const V408: Self = Self(132);
    pub const SamplerFeedbackMinMipOpaque: Self = Self(189);
    pub const SamplerFeedbackMipRegionUsedOpaque: Self = Self(190);
    pub const A4B4G4R4: Self = Self(191);
}
impl windows_core::imp::TypeKind for DirectXPixelFormat {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DirectXPixelFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.DirectX.DirectXPixelFormat;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DirectXPrimitiveTopology(pub i32);
impl DirectXPrimitiveTopology {
    pub const Undefined: Self = Self(0);
    pub const PointList: Self = Self(1);
    pub const LineList: Self = Self(2);
    pub const LineStrip: Self = Self(3);
    pub const TriangleList: Self = Self(4);
    pub const TriangleStrip: Self = Self(5);
}
impl windows_core::imp::TypeKind for DirectXPrimitiveTopology {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DirectXPrimitiveTopology {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.DirectX.DirectXPrimitiveTopology;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherExitDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherExitDeferral,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DispatcherExitDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherExitDeferral>();
}
unsafe impl windows_core::Interface for DispatcherExitDeferral {
    type Vtable = <IDispatcherExitDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherExitDeferral as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherExitDeferral {
    type Target = IDispatcherExitDeferral;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherExitDeferral {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherExitDeferral";
}
unsafe impl Send for DispatcherExitDeferral {}
unsafe impl Sync for DispatcherExitDeferral {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherQueue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherQueue,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DispatcherQueue {
    pub fn GetForCurrentThread() -> windows_core::Result<Self> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentThread)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDispatcherQueueStatics<
        R,
        F: FnOnce(&IDispatcherQueueStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DispatcherQueue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherQueue>();
}
unsafe impl windows_core::Interface for DispatcherQueue {
    type Vtable = <IDispatcherQueue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherQueue as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherQueue {
    type Target = IDispatcherQueue;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueue";
}
unsafe impl Send for DispatcherQueue {}
unsafe impl Sync for DispatcherQueue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherQueueController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherQueueController,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DispatcherQueueController {
    pub fn CreateOnDedicatedThread() -> windows_core::Result<Self> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOnDedicatedThread)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateOnCurrentThread() -> windows_core::Result<Self> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOnCurrentThread)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDispatcherQueueControllerStatics<
        R,
        F: FnOnce(&IDispatcherQueueControllerStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            DispatcherQueueController,
            IDispatcherQueueControllerStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DispatcherQueueController {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherQueueController>();
}
unsafe impl windows_core::Interface for DispatcherQueueController {
    type Vtable = <IDispatcherQueueController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherQueueController as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherQueueController {
    type Target = IDispatcherQueueController;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueController";
}
unsafe impl Send for DispatcherQueueController {}
unsafe impl Sync for DispatcherQueueController {}
windows_core::imp::define_interface!(
    DispatcherQueueHandler,
    DispatcherQueueHandler_Vtbl,
    0x2e0872a9_4e29_5f14_b688_fb96d5f9d5f8
);
impl windows_core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl DispatcherQueueHandler {
    pub fn new<F: Fn() + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &DispatcherQueueHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct DispatcherQueueHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct DispatcherQueueHandlerBox<F: Fn() + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn() + 'static> DispatcherQueueHandlerBox<F> {
    const VTABLE: DispatcherQueueHandler_Vtbl = DispatcherQueueHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<DispatcherQueueHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<DispatcherQueueHandler, F>);
            (this.invoke)();
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10);
    pub const Normal: Self = Self(0);
    pub const High: Self = Self(10);
}
impl windows_core::imp::TypeKind for DispatcherQueuePriority {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Dispatching.DispatcherQueuePriority;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherQueueShutdownStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherQueueShutdownStartingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IDispatcherQueueShutdownStartingEventArgs,
    >();
}
unsafe impl windows_core::Interface for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = <IDispatcherQueueShutdownStartingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IDispatcherQueueShutdownStartingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherQueueShutdownStartingEventArgs {
    type Target = IDispatcherQueueShutdownStartingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueShutdownStartingEventArgs";
}
unsafe impl Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl Sync for DispatcherQueueShutdownStartingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatcherQueueTimer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DispatcherQueueTimer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDispatcherQueueTimer>();
}
unsafe impl windows_core::Interface for DispatcherQueueTimer {
    type Vtable = <IDispatcherQueueTimer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDispatcherQueueTimer as windows_core::Interface>::IID;
}
impl core::ops::Deref for DispatcherQueueTimer {
    type Target = IDispatcherQueueTimer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueTimer";
}
unsafe impl Send for DispatcherQueueTimer {}
unsafe impl Sync for DispatcherQueueTimer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DispatcherRunOptions(pub u32);
impl DispatcherRunOptions {
    pub const None: Self = Self(0);
    pub const ContinueOnQuit: Self = Self(1);
    pub const QuitOnlyLocalLoop: Self = Self(2);
}
impl windows_core::imp::TypeKind for DispatcherRunOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DispatcherRunOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Dispatching.DispatcherRunOptions;u4)",
    );
}
impl DispatcherRunOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DispatcherRunOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DispatcherRunOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DispatcherRunOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for DispatcherRunOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for DispatcherRunOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayAdvancedColorInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DisplayAdvancedColorInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DisplayAdvancedColorInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDisplayAdvancedColorInfo>();
}
unsafe impl windows_core::Interface for DisplayAdvancedColorInfo {
    type Vtable = <IDisplayAdvancedColorInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDisplayAdvancedColorInfo as windows_core::Interface>::IID;
}
impl core::ops::Deref for DisplayAdvancedColorInfo {
    type Target = IDisplayAdvancedColorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DisplayAdvancedColorInfo {
    const NAME: &'static str = "Microsoft.Graphics.Display.DisplayAdvancedColorInfo";
}
unsafe impl Send for DisplayAdvancedColorInfo {}
unsafe impl Sync for DisplayAdvancedColorInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DisplayAdvancedColorKind(pub i32);
impl DisplayAdvancedColorKind {
    pub const StandardDynamicRange: Self = Self(0);
    pub const WideColorGamut: Self = Self(1);
    pub const HighDynamicRange: Self = Self(2);
}
impl windows_core::imp::TypeKind for DisplayAdvancedColorKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DisplayAdvancedColorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Display.DisplayAdvancedColorKind;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayArea(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DisplayArea,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DisplayArea {
    pub fn Primary() -> windows_core::Result<Self> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Primary)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateWatcher() -> windows_core::Result<DisplayAreaWatcher> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcher)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn FindAll() -> windows_core::Result<windows_collections::IVectorView<Self>> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAll)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn GetFromWindowId(
        windowid: WindowId,
        displayareafallback: DisplayAreaFallback,
    ) -> windows_core::Result<Self> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFromWindowId)(
                windows_core::Interface::as_raw(this),
                windowid,
                displayareafallback,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDisplayAreaStatics<R, F: FnOnce(&IDisplayAreaStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayArea, IDisplayAreaStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayArea {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDisplayArea>();
}
unsafe impl windows_core::Interface for DisplayArea {
    type Vtable = <IDisplayArea as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDisplayArea as windows_core::Interface>::IID;
}
impl core::ops::Deref for DisplayArea {
    type Target = IDisplayArea;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DisplayArea {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayArea";
}
unsafe impl Send for DisplayArea {}
unsafe impl Sync for DisplayArea {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DisplayAreaFallback(pub i32);
impl DisplayAreaFallback {
    pub const None: Self = Self(0);
    pub const Primary: Self = Self(1);
    pub const Nearest: Self = Self(2);
}
impl windows_core::imp::TypeKind for DisplayAreaFallback {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DisplayAreaFallback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaFallback;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayAreaWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DisplayAreaWatcher,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DisplayAreaWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDisplayAreaWatcher>();
}
unsafe impl windows_core::Interface for DisplayAreaWatcher {
    type Vtable = <IDisplayAreaWatcher as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDisplayAreaWatcher as windows_core::Interface>::IID;
}
impl core::ops::Deref for DisplayAreaWatcher {
    type Target = IDisplayAreaWatcher;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DisplayAreaWatcher {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayAreaWatcher";
}
unsafe impl Send for DisplayAreaWatcher {}
unsafe impl Sync for DisplayAreaWatcher {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DisplayAreaWatcherStatus(pub i32);
impl DisplayAreaWatcherStatus {
    pub const Created: Self = Self(0);
    pub const Started: Self = Self(1);
    pub const EnumerationCompleted: Self = Self(2);
    pub const Stopping: Self = Self(3);
    pub const Stopped: Self = Self(4);
    pub const Aborted: Self = Self(5);
}
impl windows_core::imp::TypeKind for DisplayAreaWatcherStatus {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DisplayAreaWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaWatcherStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DisplayHdrMetadataFormat(pub i32);
impl DisplayHdrMetadataFormat {
    pub const Hdr10: Self = Self(0);
    pub const Hdr10Plus: Self = Self(1);
}
impl windows_core::imp::TypeKind for DisplayHdrMetadataFormat {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DisplayHdrMetadataFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Display.DisplayHdrMetadataFormat;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DisplayInformation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DisplayInformation, IClosable);
impl DisplayInformation {
    pub fn CreateForWindowId(windowid: WindowId) -> windows_core::Result<Self> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForWindowId)(
                windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDisplayInformationStatics<
        R,
        F: FnOnce(&IDisplayInformationStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            DisplayInformation,
            IDisplayInformationStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDisplayInformation>();
}
unsafe impl windows_core::Interface for DisplayInformation {
    type Vtable = <IDisplayInformation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDisplayInformation as windows_core::Interface>::IID;
}
impl core::ops::Deref for DisplayInformation {
    type Target = IDisplayInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DisplayInformation {
    const NAME: &'static str = "Microsoft.Graphics.Display.DisplayInformation";
}
unsafe impl Send for DisplayInformation {}
unsafe impl Sync for DisplayInformation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DragDropManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DragDropManager,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DragDropManager, IClosable);
impl windows_core::RuntimeType for DragDropManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDragDropManager>();
}
unsafe impl windows_core::Interface for DragDropManager {
    type Vtable = <IDragDropManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragDropManager as windows_core::Interface>::IID;
}
impl core::ops::Deref for DragDropManager {
    type Target = IDragDropManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DragDropManager {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DragDropManager";
}
unsafe impl Send for DragDropManager {}
unsafe impl Sync for DragDropManager {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DragDropModifiers(pub u32);
impl DragDropModifiers {
    pub const None: Self = Self(0);
    pub const Shift: Self = Self(1);
    pub const Control: Self = Self(2);
    pub const Alt: Self = Self(4);
    pub const LeftButton: Self = Self(8);
    pub const MiddleButton: Self = Self(16);
    pub const RightButton: Self = Self(32);
}
impl windows_core::imp::TypeKind for DragDropModifiers {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DragDropModifiers {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.DragDrop.DragDropModifiers;u4)",
    );
}
impl DragDropModifiers {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DragDropModifiers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DragDropModifiers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DragDropModifiers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for DragDropModifiers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for DragDropModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DragInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DragInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DragInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDragInfo>();
}
unsafe impl windows_core::Interface for DragInfo {
    type Vtable = <IDragInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragInfo as windows_core::Interface>::IID;
}
impl core::ops::Deref for DragInfo {
    type Target = IDragInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DragInfo {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DragInfo";
}
unsafe impl Send for DragInfo {}
unsafe impl Sync for DragInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DragOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DragOperation,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(DragOperation, IClosable);
impl windows_core::RuntimeType for DragOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDragOperation>();
}
unsafe impl windows_core::Interface for DragOperation {
    type Vtable = <IDragOperation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragOperation as windows_core::Interface>::IID;
}
impl core::ops::Deref for DragOperation {
    type Target = IDragOperation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DragOperation {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DragOperation";
}
unsafe impl Send for DragOperation {}
unsafe impl Sync for DragOperation {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DragUIContentMode(pub i32);
impl DragUIContentMode {
    pub const Auto: Self = Self(0);
    pub const Deferred: Self = Self(1);
}
impl windows_core::imp::TypeKind for DragUIContentMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DragUIContentMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.DragDrop.DragUIContentMode;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DragUIOverride(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DragUIOverride,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DragUIOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDragUIOverride>();
}
unsafe impl windows_core::Interface for DragUIOverride {
    type Vtable = <IDragUIOverride as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragUIOverride as windows_core::Interface>::IID;
}
impl core::ops::Deref for DragUIOverride {
    type Target = IDragUIOverride;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DragUIOverride {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DragUIOverride";
}
unsafe impl Send for DragUIOverride {}
unsafe impl Sync for DragUIOverride {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraggingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DraggingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DraggingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDraggingEventArgs>();
}
unsafe impl windows_core::Interface for DraggingEventArgs {
    type Vtable = <IDraggingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDraggingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for DraggingEventArgs {
    type Target = IDraggingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.DraggingEventArgs";
}
unsafe impl Send for DraggingEventArgs {}
unsafe impl Sync for DraggingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0);
    pub const Continuing: Self = Self(1);
    pub const Completed: Self = Self(2);
}
impl windows_core::imp::TypeKind for DraggingState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DraggingState {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.DraggingState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropOperationTargetRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DropOperationTargetRequestedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DropOperationTargetRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDropOperationTargetRequestedEventArgs>();
}
unsafe impl windows_core::Interface for DropOperationTargetRequestedEventArgs {
    type Vtable = <IDropOperationTargetRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IDropOperationTargetRequestedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for DropOperationTargetRequestedEventArgs {
    type Target = IDropOperationTargetRequestedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DropOperationTargetRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DropOperationTargetRequestedEventArgs";
}
unsafe impl Send for DropOperationTargetRequestedEventArgs {}
unsafe impl Sync for DropOperationTargetRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnteredMoveSizeEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    EnteredMoveSizeEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for EnteredMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IEnteredMoveSizeEventArgs>();
}
unsafe impl windows_core::Interface for EnteredMoveSizeEventArgs {
    type Vtable = <IEnteredMoveSizeEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IEnteredMoveSizeEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for EnteredMoveSizeEventArgs {
    type Target = IEnteredMoveSizeEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for EnteredMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.EnteredMoveSizeEventArgs";
}
unsafe impl Send for EnteredMoveSizeEventArgs {}
unsafe impl Sync for EnteredMoveSizeEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnteringMoveSizeEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    EnteringMoveSizeEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for EnteringMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IEnteringMoveSizeEventArgs>();
}
unsafe impl windows_core::Interface for EnteringMoveSizeEventArgs {
    type Vtable = <IEnteringMoveSizeEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IEnteringMoveSizeEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for EnteringMoveSizeEventArgs {
    type Target = IEnteringMoveSizeEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for EnteringMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.EnteringMoveSizeEventArgs";
}
unsafe impl Send for EnteringMoveSizeEventArgs {}
unsafe impl Sync for EnteringMoveSizeEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExitedMoveSizeEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ExitedMoveSizeEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ExitedMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IExitedMoveSizeEventArgs>();
}
unsafe impl windows_core::Interface for ExitedMoveSizeEventArgs {
    type Vtable = <IExitedMoveSizeEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExitedMoveSizeEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ExitedMoveSizeEventArgs {
    type Target = IExitedMoveSizeEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ExitedMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ExitedMoveSizeEventArgs";
}
unsafe impl Send for ExitedMoveSizeEventArgs {}
unsafe impl Sync for ExitedMoveSizeEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FocusChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for FocusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFocusChangedEventArgs>();
}
unsafe impl windows_core::Interface for FocusChangedEventArgs {
    type Vtable = <IFocusChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for FocusChangedEventArgs {
    type Target = IFocusChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FocusChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.FocusChangedEventArgs";
}
unsafe impl Send for FocusChangedEventArgs {}
unsafe impl Sync for FocusChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusNavigationReason(pub i32);
impl FocusNavigationReason {
    pub const Programmatic: Self = Self(0);
    pub const Restore: Self = Self(1);
    pub const First: Self = Self(2);
    pub const Last: Self = Self(3);
    pub const Left: Self = Self(4);
    pub const Up: Self = Self(5);
    pub const Right: Self = Self(6);
    pub const Down: Self = Self(7);
}
impl windows_core::imp::TypeKind for FocusNavigationReason {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FocusNavigationReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.FocusNavigationReason;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusNavigationRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FocusNavigationRequest,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FocusNavigationRequest {
    pub fn Create(reason: FocusNavigationReason) -> windows_core::Result<Self> {
        Self::IFocusNavigationRequestStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                reason,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFocusNavigationRequestStatics<
        R,
        F: FnOnce(&IFocusNavigationRequestStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FocusNavigationRequest,
            IFocusNavigationRequestStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FocusNavigationRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFocusNavigationRequest>();
}
unsafe impl windows_core::Interface for FocusNavigationRequest {
    type Vtable = <IFocusNavigationRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusNavigationRequest as windows_core::Interface>::IID;
}
impl core::ops::Deref for FocusNavigationRequest {
    type Target = IFocusNavigationRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FocusNavigationRequest {
    const NAME: &'static str = "Microsoft.UI.Input.FocusNavigationRequest";
}
unsafe impl Send for FocusNavigationRequest {}
unsafe impl Sync for FocusNavigationRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusNavigationRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FocusNavigationRequestEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for FocusNavigationRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFocusNavigationRequestEventArgs>();
}
unsafe impl windows_core::Interface for FocusNavigationRequestEventArgs {
    type Vtable = <IFocusNavigationRequestEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IFocusNavigationRequestEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for FocusNavigationRequestEventArgs {
    type Target = IFocusNavigationRequestEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FocusNavigationRequestEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.FocusNavigationRequestEventArgs";
}
unsafe impl Send for FocusNavigationRequestEventArgs {}
unsafe impl Sync for FocusNavigationRequestEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusNavigationResult(pub i32);
impl FocusNavigationResult {
    pub const NotMoved: Self = Self(0);
    pub const Moved: Self = Self(1);
    pub const NoFocusableElements: Self = Self(2);
}
impl windows_core::imp::TypeKind for FocusNavigationResult {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FocusNavigationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.FocusNavigationResult;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FullScreenPresenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FullScreenPresenter,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(FullScreenPresenter, AppWindowPresenter);
impl FullScreenPresenter {
    pub fn Create() -> windows_core::Result<Self> {
        Self::IFullScreenPresenterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFullScreenPresenterStatics<
        R,
        F: FnOnce(&IFullScreenPresenterStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            FullScreenPresenter,
            IFullScreenPresenterStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FullScreenPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFullScreenPresenter>();
}
unsafe impl windows_core::Interface for FullScreenPresenter {
    type Vtable = <IFullScreenPresenter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFullScreenPresenter as windows_core::Interface>::IID;
}
impl core::ops::Deref for FullScreenPresenter {
    type Target = IFullScreenPresenter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FullScreenPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.FullScreenPresenter";
}
unsafe impl Send for FullScreenPresenter {}
unsafe impl Sync for FullScreenPresenter {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GestureRecognizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    GestureRecognizer,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for GestureRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IGestureRecognizer>();
}
unsafe impl windows_core::Interface for GestureRecognizer {
    type Vtable = <IGestureRecognizer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGestureRecognizer as windows_core::Interface>::IID;
}
impl core::ops::Deref for GestureRecognizer {
    type Target = IGestureRecognizer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Microsoft.UI.Input.GestureRecognizer";
}
unsafe impl Send for GestureRecognizer {}
unsafe impl Sync for GestureRecognizer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0);
    pub const Tap: Self = Self(1);
    pub const DoubleTap: Self = Self(2);
    pub const Hold: Self = Self(4);
    pub const HoldWithMouse: Self = Self(8);
    pub const RightTap: Self = Self(16);
    pub const Drag: Self = Self(32);
    pub const ManipulationTranslateX: Self = Self(64);
    pub const ManipulationTranslateY: Self = Self(128);
    pub const ManipulationTranslateRailsX: Self = Self(256);
    pub const ManipulationTranslateRailsY: Self = Self(512);
    pub const ManipulationRotate: Self = Self(1024);
    pub const ManipulationScale: Self = Self(2048);
    pub const ManipulationTranslateInertia: Self = Self(4096);
    pub const ManipulationRotateInertia: Self = Self(8192);
    pub const ManipulationScaleInertia: Self = Self(16384);
    pub const CrossSlide: Self = Self(32768);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536);
}
impl windows_core::imp::TypeKind for GestureSettings {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for GestureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.GestureSettings;u4)");
}
impl GestureSettings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for GestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoldingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    HoldingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for HoldingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IHoldingEventArgs>();
}
unsafe impl windows_core::Interface for HoldingEventArgs {
    type Vtable = <IHoldingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHoldingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for HoldingEventArgs {
    type Target = IHoldingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.HoldingEventArgs";
}
unsafe impl Send for HoldingEventArgs {}
unsafe impl Sync for HoldingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0);
    pub const Completed: Self = Self(1);
    pub const Canceled: Self = Self(2);
}
impl windows_core::imp::TypeKind for HoldingState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for HoldingState {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.HoldingState;i4)");
}
windows_core::imp::define_interface!(
    IAppWindow,
    IAppWindow_Vtbl,
    0xcfa788b3_643b_5c5e_ad4e_321d48a82acd
);
impl windows_core::RuntimeType for IAppWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindow {
    pub fn Id(&self) -> windows_core::Result<WindowId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsShownInSwitchers(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsShownInSwitchers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsShownInSwitchers(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsShownInSwitchers)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsVisible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn OwnerWindowId(&self) -> windows_core::Result<WindowId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OwnerWindowId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Presenter(&self) -> windows_core::Result<AppWindowPresenter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Presenter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetTitle(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn TitleBar(&self) -> windows_core::Result<AppWindowTitleBar> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TitleBar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Destroy(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn Hide(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Hide)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub fn SetIcon(&self, iconpath: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIcon)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(iconpath)),
            )
            .ok()
        }
    }
    pub fn SetPresenter<P0>(&self, appwindowpresenter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppWindowPresenter>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPresenter)(
                windows_core::Interface::as_raw(self),
                appwindowpresenter.param().abi(),
            )
            .ok()
        }
    }
    pub fn SetPresenterByKind(
        &self,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPresenterByKind)(
                windows_core::Interface::as_raw(self),
                appwindowpresenterkind,
            )
            .ok()
        }
    }
    pub fn Show(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub fn ShowWithActivation(&self, activatewindow: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ShowWithActivation)(
                windows_core::Interface::as_raw(self),
                activatewindow,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowId) -> windows_core::HRESULT,
    pub IsShownInSwitchers:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsShownInSwitchers:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub OwnerWindowId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowId) -> windows_core::HRESULT,
    Position: usize,
    pub Presenter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Size: usize,
    pub Title: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    Move: usize,
    MoveAndResize: usize,
    MoveAndResizeRelativeToDisplayArea: usize,
    Resize: usize,
    pub SetIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SetIconWithIconId: usize,
    pub SetPresenter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPresenterByKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        AppWindowPresenterKind,
    ) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowWithActivation:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Changed: usize,
    pub RemoveChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Closing: usize,
    pub RemoveClosing:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Destroying: usize,
    pub RemoveDestroying:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindow2,
    IAppWindow2_Vtbl,
    0x6cd41292_794c_5cac_8961_210d012c6ebc
);
impl windows_core::RuntimeType for IAppWindow2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindow2 {
    pub fn MoveInZOrderAtBottom(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).MoveInZOrderAtBottom)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderAtTop(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).MoveInZOrderAtTop)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderBelow(&self, windowid: WindowId) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).MoveInZOrderBelow)(
                windows_core::Interface::as_raw(self),
                windowid,
            )
            .ok()
        }
    }
    pub fn ShowOnceWithRequestedStartupState(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ShowOnceWithRequestedStartupState)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ClientSize: usize,
    pub MoveInZOrderAtBottom:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveInZOrderAtTop:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveInZOrderBelow:
        unsafe extern "system" fn(*mut core::ffi::c_void, WindowId) -> windows_core::HRESULT,
    ResizeClient: usize,
    pub ShowOnceWithRequestedStartupState:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindow3,
    IAppWindow3_Vtbl,
    0x2f260cea_193d_5dd6_a904_d7649a608d2f
);
impl windows_core::RuntimeType for IAppWindow3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindow3 {
    pub fn AssociateWithDispatcherQueue<P0>(&self, dispatcherqueue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DispatcherQueue>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AssociateWithDispatcherQueue)(
                windows_core::Interface::as_raw(self),
                dispatcherqueue.param().abi(),
            )
            .ok()
        }
    }
    pub fn DispatcherQueue(&self) -> windows_core::Result<DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IAppWindow3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AssociateWithDispatcherQueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DispatcherQueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindow4,
    IAppWindow4_Vtbl,
    0x383bfb91_ea29_5414_80cd_6c76d981eb31
);
impl windows_core::RuntimeType for IAppWindow4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindow4 {
    pub fn SetTaskbarIcon(&self, iconpath: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTaskbarIcon)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(iconpath)),
            )
            .ok()
        }
    }
    pub fn SetTitleBarIcon(&self, iconpath: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitleBarIcon)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(iconpath)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindow4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTaskbarIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SetTaskbarIconWithIconId: usize,
    pub SetTitleBarIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowChangedEventArgs,
    IAppWindowChangedEventArgs_Vtbl,
    0x2182bc5d_fdac_5c3e_bf37_7d8d684e9d1d
);
impl windows_core::RuntimeType for IAppWindowChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowChangedEventArgs {
    pub fn DidPositionChange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DidPositionChange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn DidPresenterChange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DidPresenterChange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn DidSizeChange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DidSizeChange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn DidVisibilityChange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DidVisibilityChange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IAppWindowChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DidPositionChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidPresenterChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidSizeChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidVisibilityChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowChangedEventArgs2,
    IAppWindowChangedEventArgs2_Vtbl,
    0xa773ab4c_a5ec_50e8_98ac_247fe6cd4227
);
impl windows_core::RuntimeType for IAppWindowChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowChangedEventArgs2 {
    pub fn DidZOrderChange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DidZOrderChange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsZOrderAtBottom(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsZOrderAtBottom)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsZOrderAtTop(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsZOrderAtTop)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ZOrderBelowWindowId(&self) -> windows_core::Result<WindowId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ZOrderBelowWindowId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IAppWindowChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DidZOrderChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsZOrderAtBottom:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsZOrderAtTop:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ZOrderBelowWindowId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowClosingEventArgs,
    IAppWindowClosingEventArgs_Vtbl,
    0x0e09d90b_2261_590b_9ad1_8504991d8754
);
impl windows_core::RuntimeType for IAppWindowClosingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowClosingEventArgs {
    pub fn Cancel(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cancel)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCancel)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindowClosingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cancel:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowPresenter,
    IAppWindowPresenter_Vtbl,
    0xbc3042c2_c6c6_5632_8989_ff0ec6d3b40d
);
impl windows_core::RuntimeType for IAppWindowPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowPresenter {
    pub fn Kind(&self) -> windows_core::Result<AppWindowPresenterKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Kind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IAppWindowPresenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AppWindowPresenterKind,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowPresenterFactory,
    IAppWindowPresenterFactory_Vtbl,
    0x62082e3c_1368_5238_90d1_e932dc718a82
);
impl windows_core::RuntimeType for IAppWindowPresenterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowPresenterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAppWindowStatics,
    IAppWindowStatics_Vtbl,
    0x3c315c24_d540_5d72_b518_b226b83627cb
);
impl windows_core::RuntimeType for IAppWindowStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateWithPresenter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateWithPresenterAndOwner: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        WindowId,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFromWindowId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        WindowId,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowStatics2,
    IAppWindowStatics2_Vtbl,
    0xcabc23db_4606_5d6e_89a5_06de1d8bd3e2
);
impl windows_core::RuntimeType for IAppWindowStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithDispatcherQueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        WindowId,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowTitleBar,
    IAppWindowTitleBar_Vtbl,
    0x5574efa2_c91c_5700_a363_539c71a7aaf4
);
impl windows_core::RuntimeType for IAppWindowTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowTitleBar {
    pub fn BackgroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetBackgroundColor(&self, value: Option<Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetBackgroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ButtonBackgroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ButtonBackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetButtonBackgroundColor(&self, value: Option<Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetButtonBackgroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ButtonForegroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ButtonForegroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetButtonForegroundColor(&self, value: Option<Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetButtonForegroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ButtonHoverBackgroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ButtonHoverBackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetButtonHoverBackgroundColor(&self, value: Option<Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetButtonHoverBackgroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ButtonHoverForegroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ButtonHoverForegroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetButtonHoverForegroundColor(&self, value: Option<Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetButtonHoverForegroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ButtonInactiveBackgroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ButtonInactiveBackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetButtonInactiveBackgroundColor(
        &self,
        value: Option<Color>,
    ) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetButtonInactiveBackgroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ButtonInactiveForegroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ButtonInactiveForegroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetButtonInactiveForegroundColor(
        &self,
        value: Option<Color>,
    ) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetButtonInactiveForegroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ButtonPressedBackgroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ButtonPressedBackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetButtonPressedBackgroundColor(
        &self,
        value: Option<Color>,
    ) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetButtonPressedBackgroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ButtonPressedForegroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ButtonPressedForegroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetButtonPressedForegroundColor(
        &self,
        value: Option<Color>,
    ) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetButtonPressedForegroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendsContentIntoTitleBar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetExtendsContentIntoTitleBar)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ForegroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForegroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetForegroundColor(&self, value: Option<Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetForegroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn Height(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Height)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IconShowOptions(&self) -> windows_core::Result<IconShowOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IconShowOptions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIconShowOptions(&self, value: IconShowOptions) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIconShowOptions)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InactiveBackgroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InactiveBackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetInactiveBackgroundColor(&self, value: Option<Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetInactiveBackgroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn InactiveForegroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InactiveForegroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Color>| r__.Value())
        }
    }
    pub fn SetInactiveForegroundColor(&self, value: Option<Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Color> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetInactiveForegroundColor)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn LeftInset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LeftInset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RightInset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RightInset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ResetToDefault(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ResetToDefault)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ButtonBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetButtonBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ButtonForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetButtonForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ExtendsContentIntoTitleBar:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetExtendsContentIntoTitleBar:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Height:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IconShowOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut IconShowOptions,
    ) -> windows_core::HRESULT,
    pub SetIconShowOptions:
        unsafe extern "system" fn(*mut core::ffi::c_void, IconShowOptions) -> windows_core::HRESULT,
    pub InactiveBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InactiveForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetInactiveForegroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LeftInset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RightInset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ResetToDefault: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowTitleBar2,
    IAppWindowTitleBar2_Vtbl,
    0x86faed38_748a_5b4b_9ccf_3ba0496c9041
);
impl windows_core::RuntimeType for IAppWindowTitleBar2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowTitleBar2 {
    pub fn PreferredHeightOption(&self) -> windows_core::Result<TitleBarHeightOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredHeightOption)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPreferredHeightOption(
        &self,
        value: TitleBarHeightOption,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredHeightOption)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindowTitleBar2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreferredHeightOption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut TitleBarHeightOption,
    ) -> windows_core::HRESULT,
    pub SetPreferredHeightOption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TitleBarHeightOption,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowTitleBar3,
    IAppWindowTitleBar3_Vtbl,
    0x07146e74_0410_5597_aba7_1af276d2ae07
);
impl windows_core::RuntimeType for IAppWindowTitleBar3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppWindowTitleBar3 {
    pub fn PreferredTheme(&self) -> windows_core::Result<TitleBarTheme> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredTheme)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPreferredTheme(&self, value: TitleBarTheme) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredTheme)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppWindowTitleBar3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreferredTheme: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut TitleBarTheme,
    ) -> windows_core::HRESULT,
    pub SetPreferredTheme:
        unsafe extern "system" fn(*mut core::ffi::c_void, TitleBarTheme) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppWindowTitleBarStatics,
    IAppWindowTitleBarStatics_Vtbl,
    0x9e1da52e_8b15_54d6_a886_f7b9f9d930b2
);
impl windows_core::RuntimeType for IAppWindowTitleBarStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowTitleBarStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCustomizationSupported:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICharacterReceivedEventArgs,
    ICharacterReceivedEventArgs_Vtbl,
    0x36122718_9263_592b_8d87_8f86543ffc95
);
impl windows_core::RuntimeType for ICharacterReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICharacterReceivedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn KeyCode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyCode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<PhysicalKeyStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyStatus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICharacterReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeyCode:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PhysicalKeyStatus,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IClosable,
    IClosable_Vtbl,
    0x30d5a829_7fa4_4026_83bb_d75bae4ea99e
);
impl windows_core::RuntimeType for IClosable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IClosable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IClosable {
    pub fn Close(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
impl windows_core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
pub trait IClosable_Impl: windows_core::IUnknownImpl {
    fn Close(&self) -> windows_core::Result<()>;
}
impl IClosable_Vtbl {
    pub const fn new<Identity: IClosable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IClosable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClosable_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClosable, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClosable as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IClosable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompactOverlayPresenter,
    ICompactOverlayPresenter_Vtbl,
    0xefeb0812_6fc7_5b7d_bd92_cc8f9a6454c9
);
impl windows_core::RuntimeType for ICompactOverlayPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICompactOverlayPresenter {
    pub fn InitialSize(&self) -> windows_core::Result<CompactOverlaySize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInitialSize(&self, value: CompactOverlaySize) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInitialSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICompactOverlayPresenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InitialSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut CompactOverlaySize,
    ) -> windows_core::HRESULT,
    pub SetInitialSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        CompactOverlaySize,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompactOverlayPresenterStatics,
    ICompactOverlayPresenterStatics_Vtbl,
    0xeab93186_4f6a_52f9_8c03_da57a1522f6e
);
impl windows_core::RuntimeType for ICompactOverlayPresenterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompactOverlayPresenterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContextMenuKeyEventArgs,
    IContextMenuKeyEventArgs_Vtbl,
    0xf6025762_9426_541a_b647_037abdbecefc
);
impl windows_core::RuntimeType for IContextMenuKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContextMenuKeyEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IContextMenuKeyEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICrossSlidingEventArgs,
    ICrossSlidingEventArgs_Vtbl,
    0x7679641f_ba9f_543c_a7c8_6229a98f89ef
);
impl windows_core::RuntimeType for ICrossSlidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICrossSlidingEventArgs {
    pub fn CrossSlidingState(&self) -> windows_core::Result<CrossSlidingState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CrossSlidingState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ICrossSlidingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CrossSlidingState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut CrossSlidingState,
    ) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherExitDeferral,
    IDispatcherExitDeferral_Vtbl,
    0x910b5aac_3310_563e_8418_f3005579729e
);
impl windows_core::RuntimeType for IDispatcherExitDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherExitDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IDispatcherExitDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueue,
    IDispatcherQueue_Vtbl,
    0xf6ebf8fa_be1c_5bf6_a467_73da28738ae8
);
impl windows_core::RuntimeType for IDispatcherQueue {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueue {
    pub fn CreateTimer(&self) -> windows_core::Result<DispatcherQueueTimer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTimer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn TryEnqueue<P0>(&self, callback: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DispatcherQueueHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryEnqueue)(
                windows_core::Interface::as_raw(self),
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TryEnqueueWithPriority<P1>(
        &self,
        priority: DispatcherQueuePriority,
        callback: P1,
    ) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<DispatcherQueueHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryEnqueueWithPriority)(
                windows_core::Interface::as_raw(self),
                priority,
                callback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDispatcherQueue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TryEnqueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DispatcherQueuePriority,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    ShutdownStarting: usize,
    pub RemoveShutdownStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ShutdownCompleted: usize,
    pub RemoveShutdownCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueue2,
    IDispatcherQueue2_Vtbl,
    0x0cf48751_f1ac_59b8_ba52_6ce7a1444d6f
);
impl windows_core::RuntimeType for IDispatcherQueue2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueue2 {
    pub fn HasThreadAccess(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasThreadAccess)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDispatcherQueue2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasThreadAccess:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueue3,
    IDispatcherQueue3_Vtbl,
    0x14a7a175_5c27_5a35_b079_21960cf764a8
);
impl windows_core::RuntimeType for IDispatcherQueue3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueue3 {
    pub fn EnqueueEventLoopExit(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).EnqueueEventLoopExit)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn EnsureSystemDispatcherQueue(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).EnsureSystemDispatcherQueue)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn RunEventLoop(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RunEventLoop)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub fn RunEventLoopWithOptions<P1>(
        &self,
        options: DispatcherRunOptions,
        deferral: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<DispatcherExitDeferral>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RunEventLoopWithOptions)(
                windows_core::Interface::as_raw(self),
                options,
                deferral.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDispatcherQueue3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnqueueEventLoopExit:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnsureSystemDispatcherQueue:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunEventLoop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunEventLoopWithOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DispatcherRunOptions,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FrameworkShutdownStarting: usize,
    pub RemoveFrameworkShutdownStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    FrameworkShutdownCompleted: usize,
    pub RemoveFrameworkShutdownCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueueController,
    IDispatcherQueueController_Vtbl,
    0xbce8178d_2183_584c_9e5b_f9366f6ae484
);
impl windows_core::RuntimeType for IDispatcherQueueController {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueueController {
    pub fn DispatcherQueue(&self) -> windows_core::Result<DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ShutdownQueueAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShutdownQueueAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDispatcherQueueController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DispatcherQueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ShutdownQueueAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueueController2,
    IDispatcherQueueController2_Vtbl,
    0x4c68ee2a_1cb1_5591_a3a2_9b590b8f8b9a
);
impl windows_core::RuntimeType for IDispatcherQueueController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueueController2 {
    pub fn ShutdownQueue(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ShutdownQueue)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDispatcherQueueController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShutdownQueue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueueControllerStatics,
    IDispatcherQueueControllerStatics_Vtbl,
    0xf18d6145_722b_593d_bcf2_a61e713f0037
);
impl windows_core::RuntimeType for IDispatcherQueueControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDispatcherQueueControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateOnDedicatedThread: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateOnCurrentThread: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueueShutdownStartingEventArgs,
    IDispatcherQueueShutdownStartingEventArgs_Vtbl,
    0x32519be5_072b_5660_a70e_8835c9b8157d
);
impl windows_core::RuntimeType for IDispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDispatcherQueueShutdownStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDispatcherQueueStatics,
    IDispatcherQueueStatics_Vtbl,
    0xcd3382ea_a455_5124_b63a_ca40d34ca23c
);
impl windows_core::RuntimeType for IDispatcherQueueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDispatcherQueueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDispatcherQueueTimer,
    IDispatcherQueueTimer_Vtbl,
    0xad4d63fd_88fe_541f_ac11_bf2dc1ed2ce5
);
impl windows_core::RuntimeType for IDispatcherQueueTimer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDispatcherQueueTimer {
    pub fn Interval(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Interval)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInterval(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInterval)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsRunning(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRunning)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsRepeating(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRepeating)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsRepeating(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsRepeating)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
        }
    }
}
#[repr(C)]
pub struct IDispatcherQueueTimer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Interval: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    pub IsRunning:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRepeating:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRepeating:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    Tick: usize,
    pub RemoveTick: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDisplayAdvancedColorInfo,
    IDisplayAdvancedColorInfo_Vtbl,
    0xb44f0f47_7065_5175_ba3e_714489c85a3e
);
impl windows_core::RuntimeType for IDisplayAdvancedColorInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDisplayAdvancedColorInfo {
    pub fn CurrentAdvancedColorKind(&self) -> windows_core::Result<DisplayAdvancedColorKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAdvancedColorKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RedPrimary(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedPrimary)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GreenPrimary(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GreenPrimary)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn BluePrimary(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BluePrimary)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn WhitePoint(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WhitePoint)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MaxLuminanceInNits(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxLuminanceInNits)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MinLuminanceInNits(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinLuminanceInNits)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxAverageFullFrameLuminanceInNits)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SdrWhiteLevelInNits(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SdrWhiteLevelInNits)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsHdrMetadataFormatCurrentlySupported(
        &self,
        format: DisplayHdrMetadataFormat,
    ) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHdrMetadataFormatCurrentlySupported)(
                windows_core::Interface::as_raw(self),
                format,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsAdvancedColorKindAvailable(
        &self,
        kind: DisplayAdvancedColorKind,
    ) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAdvancedColorKindAvailable)(
                windows_core::Interface::as_raw(self),
                kind,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDisplayAdvancedColorInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentAdvancedColorKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DisplayAdvancedColorKind,
    ) -> windows_core::HRESULT,
    pub RedPrimary:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub GreenPrimary:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub BluePrimary:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub WhitePoint:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub MaxLuminanceInNits:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MinLuminanceInNits:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SdrWhiteLevelInNits:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsHdrMetadataFormatCurrentlySupported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DisplayHdrMetadataFormat,
        *mut bool,
    )
        -> windows_core::HRESULT,
    pub IsAdvancedColorKindAvailable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DisplayAdvancedColorKind,
        *mut bool,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDisplayArea,
    IDisplayArea_Vtbl,
    0x5c7e0537_b621_5579_bcae_a84aa8746167
);
impl windows_core::RuntimeType for IDisplayArea {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDisplayArea {
    pub fn IsPrimary(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPrimary)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDisplayArea_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    DisplayId: usize,
    pub IsPrimary:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDisplayAreaStatics,
    IDisplayAreaStatics_Vtbl,
    0x02ab4926_211e_5d49_8e4b_2af193daed09
);
impl windows_core::RuntimeType for IDisplayAreaStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayAreaStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Primary: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FindAll: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFromWindowId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        WindowId,
        DisplayAreaFallback,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDisplayAreaStatics2,
    IDisplayAreaStatics2_Vtbl,
    0x7207ad4b_890d_5dd7_bc18_78ffd9544d8f
);
impl windows_core::RuntimeType for IDisplayAreaStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayAreaStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDisplayAreaWatcher,
    IDisplayAreaWatcher_Vtbl,
    0x83f6562f_d3a0_548b_8e4f_a99be3d95c9c
);
impl windows_core::RuntimeType for IDisplayAreaWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDisplayAreaWatcher {
    pub fn Status(&self) -> windows_core::Result<DisplayAreaWatcherStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
        }
    }
}
#[repr(C)]
pub struct IDisplayAreaWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DisplayAreaWatcherStatus,
    ) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    Added: usize,
    pub RemoveAdded:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    EnumerationCompleted: usize,
    pub RemoveEnumerationCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Removed: usize,
    pub RemoveRemoved:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Stopped: usize,
    pub RemoveStopped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Updated: usize,
    pub RemoveUpdated:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDisplayInformation,
    IDisplayInformation_Vtbl,
    0xf0d58d4f_84ce_5b27_b222_4f8f7dc0aaeb
);
impl windows_core::RuntimeType for IDisplayInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDisplayInformation {
    pub fn DispatcherQueue(&self) -> windows_core::Result<DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsStereoEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStereoEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetColorProfileAsync(
        &self,
    ) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStream>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorProfileAsync)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetAdvancedColorInfo(&self) -> windows_core::Result<DisplayAdvancedColorInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdvancedColorInfo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDisplayInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DispatcherQueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsStereoEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    IsStereoEnabledChanged: usize,
    pub RemoveIsStereoEnabledChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetColorProfileAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetColorProfile: usize,
    ColorProfileChanged: usize,
    pub RemoveColorProfileChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetAdvancedColorInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    AdvancedColorInfoChanged: usize,
    pub RemoveAdvancedColorInfoChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Destroyed: usize,
    pub RemoveDestroyed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDisplayInformationStatics,
    IDisplayInformationStatics_Vtbl,
    0x2de85048_37fa_56c0_ac30_47e2044d7ea8
);
impl windows_core::RuntimeType for IDisplayInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForWindowId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        WindowId,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDragDropManager,
    IDragDropManager_Vtbl,
    0x4fea9efc_b073_5fbe_9c95_a4113ef6393f
);
impl windows_core::RuntimeType for IDragDropManager {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDragDropManager {
    pub fn AreConcurrentOperationsEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreConcurrentOperationsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreConcurrentOperationsEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDragDropManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AreConcurrentOperationsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAreConcurrentOperationsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    TargetRequested: usize,
    pub RemoveTargetRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDragDropManagerStatics,
    IDragDropManagerStatics_Vtbl,
    0x5587c863_57d7_5d0f_8ea9_e5dcf06a0f83
);
impl windows_core::RuntimeType for IDragDropManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDragDropManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDragInfo,
    IDragInfo_Vtbl,
    0x7507d891_62a8_5a79_a880_ac7353d001ec
);
impl windows_core::RuntimeType for IDragInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDragInfo {
    pub fn Modifiers(&self) -> windows_core::Result<DragDropModifiers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modifiers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDragInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AllowedOperations: usize,
    Data: usize,
    pub Modifiers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DragDropModifiers,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDragOperation,
    IDragOperation_Vtbl,
    0xef122288_7984_53d3_8488_133dcd3de793
);
impl windows_core::RuntimeType for IDragOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDragOperation {
    pub fn DragUIContentMode(&self) -> windows_core::Result<DragUIContentMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DragUIContentMode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDragUIContentMode(&self, value: DragUIContentMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDragUIContentMode)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn StartAsync<P0, P1>(
        &self,
        initialtarget: P0,
        initialpointerpoint: P1,
    ) -> windows_core::Result<windows_future::IAsyncOperation<DataPackageOperation>>
    where
        P0: windows_core::Param<DragDropManager>,
        P1: windows_core::Param<PointerPoint>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartAsync)(
                windows_core::Interface::as_raw(self),
                initialtarget.param().abi(),
                initialpointerpoint.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDragOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AllowedOperations: usize,
    SetAllowedOperations: usize,
    Data: usize,
    pub DragUIContentMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DragUIContentMode,
    ) -> windows_core::HRESULT,
    pub SetDragUIContentMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DragUIContentMode,
    ) -> windows_core::HRESULT,
    SetDragUIContentFromSoftwareBitmap: usize,
    SetDragUIContentFromSoftwareBitmap2: usize,
    pub StartAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDragUIOverride,
    IDragUIOverride_Vtbl,
    0x8432fbac_a17f_5a95_8f56_fb432280b54d
);
impl windows_core::RuntimeType for IDragUIOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDragUIOverride {
    pub fn Caption(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Caption)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetCaption(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCaption)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn IsCaptionVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCaptionVisible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsCaptionVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsCaptionVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsContentVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsContentVisible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsContentVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsContentVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsGlyphVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsGlyphVisible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsGlyphVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsGlyphVisible)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IDragUIOverride_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Caption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsCaptionVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCaptionVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsContentVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsContentVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsGlyphVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsGlyphVisible:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDraggingEventArgs,
    IDraggingEventArgs_Vtbl,
    0x3efb1b75_3d3b_550e_963d_0828ca76128a
);
impl windows_core::RuntimeType for IDraggingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDraggingEventArgs {
    pub fn DraggingState(&self) -> windows_core::Result<DraggingState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DraggingState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDraggingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DraggingState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DraggingState,
    ) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDropOperationTarget,
    IDropOperationTarget_Vtbl,
    0x1c2707d9_0065_53c7_bbfb_50850378caf3
);
impl windows_core::RuntimeType for IDropOperationTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IDropOperationTarget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IDropOperationTarget {
    pub fn DropAsync<P0>(
        &self,
        draginfo: P0,
    ) -> windows_core::Result<windows_future::IAsyncOperation<DataPackageOperation>>
    where
        P0: windows_core::Param<DragInfo>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DropAsync)(
                windows_core::Interface::as_raw(self),
                draginfo.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn EnterAsync<P0, P1>(
        &self,
        draginfo: P0,
        draguioverride: P1,
    ) -> windows_core::Result<windows_future::IAsyncOperation<DataPackageOperation>>
    where
        P0: windows_core::Param<DragInfo>,
        P1: windows_core::Param<DragUIOverride>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnterAsync)(
                windows_core::Interface::as_raw(self),
                draginfo.param().abi(),
                draguioverride.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn LeaveAsync<P0>(&self, draginfo: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<DragInfo>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LeaveAsync)(
                windows_core::Interface::as_raw(self),
                draginfo.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OverAsync<P0, P1>(
        &self,
        draginfo: P0,
        draguioverride: P1,
    ) -> windows_core::Result<windows_future::IAsyncOperation<DataPackageOperation>>
    where
        P0: windows_core::Param<DragInfo>,
        P1: windows_core::Param<DragUIOverride>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OverAsync)(
                windows_core::Interface::as_raw(self),
                draginfo.param().abi(),
                draguioverride.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IDropOperationTarget {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.IDropOperationTarget";
}
pub trait IDropOperationTarget_Impl: windows_core::IUnknownImpl {
    fn DropAsync(
        &self,
        dragInfo: windows_core::Ref<DragInfo>,
    ) -> windows_core::Result<windows_future::IAsyncOperation<DataPackageOperation>>;
    fn EnterAsync(
        &self,
        dragInfo: windows_core::Ref<DragInfo>,
        dragUIOverride: windows_core::Ref<DragUIOverride>,
    ) -> windows_core::Result<windows_future::IAsyncOperation<DataPackageOperation>>;
    fn LeaveAsync(
        &self,
        dragInfo: windows_core::Ref<DragInfo>,
    ) -> windows_core::Result<windows_future::IAsyncAction>;
    fn OverAsync(
        &self,
        dragInfo: windows_core::Ref<DragInfo>,
        dragUIOverride: windows_core::Ref<DragUIOverride>,
    ) -> windows_core::Result<windows_future::IAsyncOperation<DataPackageOperation>>;
}
impl IDropOperationTarget_Vtbl {
    pub const fn new<Identity: IDropOperationTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DropAsync<
            Identity: IDropOperationTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            draginfo: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropOperationTarget_Impl::DropAsync(
                    this,
                    core::mem::transmute_copy(&draginfo),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnterAsync<
            Identity: IDropOperationTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            draginfo: *mut core::ffi::c_void,
            draguioverride: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropOperationTarget_Impl::EnterAsync(
                    this,
                    core::mem::transmute_copy(&draginfo),
                    core::mem::transmute_copy(&draguioverride),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LeaveAsync<
            Identity: IDropOperationTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            draginfo: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropOperationTarget_Impl::LeaveAsync(
                    this,
                    core::mem::transmute_copy(&draginfo),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OverAsync<
            Identity: IDropOperationTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            draginfo: *mut core::ffi::c_void,
            draguioverride: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropOperationTarget_Impl::OverAsync(
                    this,
                    core::mem::transmute_copy(&draginfo),
                    core::mem::transmute_copy(&draguioverride),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDropOperationTarget, OFFSET>(
            ),
            DropAsync: DropAsync::<Identity, OFFSET>,
            EnterAsync: EnterAsync::<Identity, OFFSET>,
            LeaveAsync: LeaveAsync::<Identity, OFFSET>,
            OverAsync: OverAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropOperationTarget as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IDropOperationTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DropAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub EnterAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LeaveAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OverAsync: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDropOperationTargetRequestedEventArgs,
    IDropOperationTargetRequestedEventArgs_Vtbl,
    0xf61c5b62_720e_59ff_ad0b_e77fc5b8a4a3
);
impl windows_core::RuntimeType for IDropOperationTargetRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDropOperationTargetRequestedEventArgs {
    pub fn SetTarget<P0>(&self, target: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDropOperationTarget>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTarget)(
                windows_core::Interface::as_raw(self),
                target.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDropOperationTargetRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IEnteredMoveSizeEventArgs,
    IEnteredMoveSizeEventArgs_Vtbl,
    0x698d28fe_d325_59e0_9834_b10fc2f7ba67
);
impl windows_core::RuntimeType for IEnteredMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IEnteredMoveSizeEventArgs {
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IEnteredMoveSizeEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PointerScreenPoint: usize,
    pub MoveSizeOperation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MoveSizeOperation,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IEnteringMoveSizeEventArgs,
    IEnteringMoveSizeEventArgs_Vtbl,
    0x47c083b2_402b_51ec_8836_d48679fea695
);
impl windows_core::RuntimeType for IEnteringMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IEnteringMoveSizeEventArgs {
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveSizeWindowId(&self) -> windows_core::Result<WindowId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeWindowId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMoveSizeWindowId(&self, value: WindowId) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMoveSizeWindowId)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IEnteringMoveSizeEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PointerScreenPoint: usize,
    pub MoveSizeOperation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MoveSizeOperation,
    ) -> windows_core::HRESULT,
    pub MoveSizeWindowId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowId) -> windows_core::HRESULT,
    pub SetMoveSizeWindowId:
        unsafe extern "system" fn(*mut core::ffi::c_void, WindowId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IExitedMoveSizeEventArgs,
    IExitedMoveSizeEventArgs_Vtbl,
    0xdf12a46e_daee_5dac_a678_d7d5e4d0893a
);
impl windows_core::RuntimeType for IExitedMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IExitedMoveSizeEventArgs {
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IExitedMoveSizeEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PointerScreenPoint: usize,
    pub MoveSizeOperation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MoveSizeOperation,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFocusChangedEventArgs,
    IFocusChangedEventArgs_Vtbl,
    0xa039b115_dbdf_594c_9b86_da6aa05c9fa2
);
impl windows_core::RuntimeType for IFocusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFocusChangedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFocusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFocusNavigationRequest,
    IFocusNavigationRequest_Vtbl,
    0x6d84bb83_9c84_5112_85e9_8919acf97262
);
impl windows_core::RuntimeType for IFocusNavigationRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFocusNavigationRequest {
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn HintRect(&self) -> windows_core::Result<Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HintRect)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Rect>| r__.Value())
        }
    }
    pub fn Reason(&self) -> windows_core::Result<FocusNavigationReason> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IFocusNavigationRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub HintRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FocusNavigationReason,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFocusNavigationRequestEventArgs,
    IFocusNavigationRequestEventArgs_Vtbl,
    0x35a63426_e271_59f9_a231_0d190314b415
);
impl windows_core::RuntimeType for IFocusNavigationRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFocusNavigationRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<FocusNavigationRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Request)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Result(&self) -> windows_core::Result<FocusNavigationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetResult(&self, value: FocusNavigationResult) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetResult)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFocusNavigationRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FocusNavigationResult,
    ) -> windows_core::HRESULT,
    pub SetResult: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FocusNavigationResult,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFocusNavigationRequestStatics,
    IFocusNavigationRequestStatics_Vtbl,
    0x8c4d2ed8_3a63_519e_a827_f57e263bd1ff
);
impl windows_core::RuntimeType for IFocusNavigationRequestStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFocusNavigationRequestStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FocusNavigationReason,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFullScreenPresenter,
    IFullScreenPresenter_Vtbl,
    0xfa9141fd_b8dd_5da1_8b2b_7cdadb76f593
);
impl windows_core::RuntimeType for IFullScreenPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFullScreenPresenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IFullScreenPresenterStatics,
    IFullScreenPresenterStatics_Vtbl,
    0x2ec0d2c1_e086_55bb_a3b2_44942e231c67
);
impl windows_core::RuntimeType for IFullScreenPresenterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFullScreenPresenterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGestureRecognizer,
    IGestureRecognizer_Vtbl,
    0xcda89afc_6bd0_595c_ba37_545fce5bf016
);
impl windows_core::RuntimeType for IGestureRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IGestureRecognizer {
    pub fn AutoProcessInertia(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoProcessInertia)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAutoProcessInertia(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAutoProcessInertia)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CrossSlideExact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CrossSlideExact)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCrossSlideExact(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCrossSlideExact)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CrossSlideHorizontally(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CrossSlideHorizontally)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCrossSlideHorizontally)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CrossSlideThresholds(&self) -> windows_core::Result<CrossSlideThresholds> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CrossSlideThresholds)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCrossSlideThresholds(&self, value: CrossSlideThresholds) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCrossSlideThresholds)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn GestureSettings(&self) -> windows_core::Result<GestureSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GestureSettings)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetGestureSettings(&self, value: GestureSettings) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetGestureSettings)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsActive)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsInertial(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInertial)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PivotCenter(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PivotCenter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPivotCenter(&self, value: Point) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPivotCenter)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn PivotRadius(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PivotRadius)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPivotRadius(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPivotRadius)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaExpansionDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaExpansionDeceleration)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInertiaExpansionDeceleration)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaExpansion(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaExpansion)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInertiaExpansion(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInertiaExpansion)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaRotationAngle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaRotationAngle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInertiaRotationAngle(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInertiaRotationAngle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaRotationDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaRotationDeceleration)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInertiaRotationDeceleration)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaTranslationDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaTranslationDeceleration)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInertiaTranslationDeceleration)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaTranslationDisplacement(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaTranslationDisplacement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetInertiaTranslationDisplacement)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ManipulationExact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManipulationExact)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetManipulationExact(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetManipulationExact)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn MouseWheelParameters(&self) -> windows_core::Result<MouseWheelParameters> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MouseWheelParameters)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ShowGestureFeedback(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowGestureFeedback)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetShowGestureFeedback(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetShowGestureFeedback)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CanBeDoubleTap<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanBeDoubleTap)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CompleteGesture(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CompleteGesture)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn ProcessDownEvent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessDownEvent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ProcessMoveEvents<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IVector<PointerPoint>>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessMoveEvents)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ProcessMouseWheelEvent<P0>(
        &self,
        value: P0,
        isshiftkeydown: bool,
        iscontrolkeydown: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessMouseWheelEvent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
                isshiftkeydown,
                iscontrolkeydown,
            )
            .ok()
        }
    }
    pub fn ProcessInertia(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessInertia)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub fn ProcessUpEvent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessUpEvent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IGestureRecognizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoProcessInertia:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoProcessInertia:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrossSlideExact:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCrossSlideExact:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrossSlideHorizontally:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCrossSlideHorizontally:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrossSlideThresholds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut CrossSlideThresholds,
    ) -> windows_core::HRESULT,
    pub SetCrossSlideThresholds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        CrossSlideThresholds,
    ) -> windows_core::HRESULT,
    pub GestureSettings: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut GestureSettings,
    ) -> windows_core::HRESULT,
    pub SetGestureSettings:
        unsafe extern "system" fn(*mut core::ffi::c_void, GestureSettings) -> windows_core::HRESULT,
    pub IsActive:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInertial:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PivotCenter:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub SetPivotCenter:
        unsafe extern "system" fn(*mut core::ffi::c_void, Point) -> windows_core::HRESULT,
    pub PivotRadius:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPivotRadius:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaExpansionDeceleration:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaExpansionDeceleration:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaExpansion:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaExpansion:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaRotationAngle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaRotationAngle:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaRotationDeceleration:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaRotationDeceleration:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaTranslationDeceleration:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaTranslationDeceleration:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaTranslationDisplacement:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaTranslationDisplacement:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ManipulationExact:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetManipulationExact:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MouseWheelParameters: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ShowGestureFeedback:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowGestureFeedback:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanBeDoubleTap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub CompleteGesture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessDownEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ProcessMoveEvents: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ProcessMouseWheelEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
        bool,
    ) -> windows_core::HRESULT,
    pub ProcessInertia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessUpEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Tapped: usize,
    pub RemoveTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    RightTapped: usize,
    pub RemoveRightTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Holding: usize,
    pub RemoveHolding:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Dragging: usize,
    pub RemoveDragging:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationStarted: usize,
    pub RemoveManipulationStarted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationUpdated: usize,
    pub RemoveManipulationUpdated:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationInertiaStarting: usize,
    pub RemoveManipulationInertiaStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ManipulationCompleted: usize,
    pub RemoveManipulationCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    CrossSliding: usize,
    pub RemoveCrossSliding:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IHoldingEventArgs,
    IHoldingEventArgs_Vtbl,
    0x8e449e85_d223_533c_b0b2_bf7c6d10c2db
);
impl windows_core::RuntimeType for IHoldingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IHoldingEventArgs {
    pub fn HoldingState(&self) -> windows_core::Result<HoldingState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HoldingState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IHoldingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HoldingState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut HoldingState,
    ) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputActivationListener,
    IInputActivationListener_Vtbl,
    0x3b818627_6ce7_5e0d_a0f5_6684fd1aec78
);
impl windows_core::RuntimeType for IInputActivationListener {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputActivationListener {
    pub fn State(&self) -> windows_core::Result<InputActivationState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IInputActivationListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut InputActivationState,
    ) -> windows_core::HRESULT,
    InputActivationChanged: usize,
    pub RemoveInputActivationChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputActivationListenerActivationChangedEventArgs,
    IInputActivationListenerActivationChangedEventArgs_Vtbl,
    0x7978526b_00b6_5303_8f7d_55bef36da786
);
impl windows_core::RuntimeType for IInputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputActivationListenerActivationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputActivationListenerStatics,
    IInputActivationListenerStatics_Vtbl,
    0xc4249843_f053_5c99_9d51_720ade94224d
);
impl windows_core::RuntimeType for IInputActivationListenerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputActivationListenerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        WindowId,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputActivationListenerStatics2,
    IInputActivationListenerStatics2_Vtbl,
    0x7ea26120_9636_5292_a7b1_56544ac51a22
);
impl windows_core::RuntimeType for IInputActivationListenerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputActivationListenerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputCursor,
    IInputCursor_Vtbl,
    0x359b15f9_19c2_5714_8432_75176826406b
);
impl windows_core::RuntimeType for IInputCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputCursorFactory,
    IInputCursorFactory_Vtbl,
    0x2f47647b_4be0_53e9_be7e_c38d5459db6b
);
impl windows_core::RuntimeType for IInputCursorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputCursorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputCursorStatics,
    IInputCursorStatics_Vtbl,
    0x92f6a552_099f_55fb_8c31_e450284c9643
);
impl windows_core::RuntimeType for IInputCursorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputCursorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputCustomCursor,
    IInputCustomCursor_Vtbl,
    0x5486f042_7e1a_5dc8_8041_e47b609a5ba1
);
impl windows_core::RuntimeType for IInputCustomCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputCustomCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputCustomCursorFactory,
    IInputCustomCursorFactory_Vtbl,
    0x6f402882_66e0_57d3_89d0_aa5e2ff917bc
);
impl windows_core::RuntimeType for IInputCustomCursorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputCustomCursorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputDesktopNamedResourceCursor,
    IInputDesktopNamedResourceCursor_Vtbl,
    0xf40ea93b_0ed7_5b3a_bfe2_14e2b5ad88a3
);
impl windows_core::RuntimeType for IInputDesktopNamedResourceCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputDesktopNamedResourceCursor {
    pub fn ModuleName(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn ResourceName(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
}
#[repr(C)]
pub struct IInputDesktopNamedResourceCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ModuleName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ResourceName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputDesktopNamedResourceCursorStatics,
    IInputDesktopNamedResourceCursorStatics_Vtbl,
    0xe8b6d5aa_898b_5e69_b01f_383a0943e3e4
);
impl windows_core::RuntimeType for IInputDesktopNamedResourceCursorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputDesktopNamedResourceCursorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFromModule: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputDesktopResourceCursor,
    IInputDesktopResourceCursor_Vtbl,
    0x1df2777f_7c90_58fc_a7a3_d5736c6510fd
);
impl windows_core::RuntimeType for IInputDesktopResourceCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputDesktopResourceCursor {
    pub fn ModuleName(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn ResourceId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IInputDesktopResourceCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ModuleName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ResourceId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputDesktopResourceCursorStatics,
    IInputDesktopResourceCursorStatics_Vtbl,
    0xf440dc37_a0b6_56eb_bcec_b024f2233d47
);
impl windows_core::RuntimeType for IInputDesktopResourceCursorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputDesktopResourceCursorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFromModule: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputFocusController,
    IInputFocusController_Vtbl,
    0x8dfdc26c_8b8d_515d_8ddd_4685b3a540e9
);
impl windows_core::RuntimeType for IInputFocusController {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputFocusController {
    pub fn HasFocus(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasFocus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TrySetFocus(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrySetFocus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IInputFocusController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TrySetFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    GotFocus: usize,
    pub RemoveGotFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    LostFocus: usize,
    pub RemoveLostFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputFocusController2,
    IInputFocusController2_Vtbl,
    0x5165077c_cd4b_501d_b386_b50682360185
);
impl windows_core::RuntimeType for IInputFocusController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputFocusController2 {
    pub fn DepartFocus<P0>(&self, request: P0) -> windows_core::Result<FocusNavigationResult>
    where
        P0: windows_core::Param<FocusNavigationRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DepartFocus)(
                windows_core::Interface::as_raw(self),
                request.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IInputFocusController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DepartFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut FocusNavigationResult,
    ) -> windows_core::HRESULT,
    NavigateFocusRequested: usize,
    pub RemoveNavigateFocusRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputFocusController3,
    IInputFocusController3_Vtbl,
    0xe8e82853_46ce_53b5_9029_a74dabea9d32
);
impl windows_core::RuntimeType for IInputFocusController3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputFocusController3 {
    pub fn ShouldShowKeyboardCues(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShouldShowKeyboardCues)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IInputFocusController3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShouldShowKeyboardCues:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputFocusControllerStatics,
    IInputFocusControllerStatics_Vtbl,
    0xaeb311da_da9b_5a1b_92f4_83ddde933e00
);
impl windows_core::RuntimeType for IInputFocusControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputFocusControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputFocusNavigationHost,
    IInputFocusNavigationHost_Vtbl,
    0x53c2a147_932c_5486_a9c6_f6c5a9c65956
);
impl windows_core::RuntimeType for IInputFocusNavigationHost {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputFocusNavigationHost {
    pub fn ContainsFocus(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContainsFocus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NavigateFocus<P0>(&self, request: P0) -> windows_core::Result<FocusNavigationResult>
    where
        P0: windows_core::Param<FocusNavigationRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NavigateFocus)(
                windows_core::Interface::as_raw(self),
                request.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IInputFocusNavigationHost_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContainsFocus:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub NavigateFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut FocusNavigationResult,
    ) -> windows_core::HRESULT,
    DepartFocusRequested: usize,
    pub RemoveDepartFocusRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputFocusNavigationHostStatics,
    IInputFocusNavigationHostStatics_Vtbl,
    0xc9c62cd1_73db_5aa9_b89d_143509db8f37
);
impl windows_core::RuntimeType for IInputFocusNavigationHostStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputFocusNavigationHostStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputFocusNavigationHostStatics2,
    IInputFocusNavigationHostStatics2_Vtbl,
    0x82505f60_ef7b_55d8_8362_8cc2840266a1
);
impl windows_core::RuntimeType for IInputFocusNavigationHostStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputFocusNavigationHostStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputKeyboardSource,
    IInputKeyboardSource_Vtbl,
    0xed61b906_16ad_5df7_a550_5e6f7d2229f7
);
impl windows_core::RuntimeType for IInputKeyboardSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputKeyboardSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputKeyboardSource2,
    IInputKeyboardSource2_Vtbl,
    0x79d1c9b6_b3c9_5ec2_8a5b_707088787f78
);
impl windows_core::RuntimeType for IInputKeyboardSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputKeyboardSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetCurrentKeyState: usize,
    GetKeyState: usize,
    CharacterReceived: usize,
    pub RemoveCharacterReceived:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ContextMenuKey: usize,
    pub RemoveContextMenuKey:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    KeyDown: usize,
    pub RemoveKeyDown:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    KeyUp: usize,
    pub RemoveKeyUp:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    SystemKeyDown: usize,
    pub RemoveSystemKeyDown:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    SystemKeyUp: usize,
    pub RemoveSystemKeyUp:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputKeyboardSourceStatics,
    IInputKeyboardSourceStatics_Vtbl,
    0xf4e1563d_8c2e_5bcd_b784_47adeaa3cd7e
);
impl windows_core::RuntimeType for IInputKeyboardSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputKeyboardSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputKeyboardSourceStatics2,
    IInputKeyboardSourceStatics2_Vtbl,
    0x8857518c_2899_5f11_9b64_0ad83234824b
);
impl windows_core::RuntimeType for IInputKeyboardSourceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputKeyboardSourceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputLightDismissAction,
    IInputLightDismissAction_Vtbl,
    0xe8a39502_a860_502f_8c10_3646d43aecf1
);
impl windows_core::RuntimeType for IInputLightDismissAction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputLightDismissAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Dismissed: usize,
    pub RemoveDismissed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputLightDismissActionStatics,
    IInputLightDismissActionStatics_Vtbl,
    0xed9b8def_6496_5169_984d_d44b4e690623
);
impl windows_core::RuntimeType for IInputLightDismissActionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputLightDismissActionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        WindowId,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputLightDismissEventArgs,
    IInputLightDismissEventArgs_Vtbl,
    0x078660ee_07ca_5808_b982_e6e899cf098c
);
impl windows_core::RuntimeType for IInputLightDismissEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputLightDismissEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputNonClientPointerSource,
    IInputNonClientPointerSource_Vtbl,
    0x471732b4_3d07_5104_b192_ebacf71e86df
);
impl windows_core::RuntimeType for IInputNonClientPointerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputNonClientPointerSource {
    pub fn DispatcherQueue(&self) -> windows_core::Result<DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ClearAllRegionRects(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearAllRegionRects)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn ClearRegionRects(&self, region: NonClientRegionKind) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearRegionRects)(
                windows_core::Interface::as_raw(self),
                region,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IInputNonClientPointerSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DispatcherQueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ClearAllRegionRects:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearRegionRects: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        NonClientRegionKind,
    ) -> windows_core::HRESULT,
    GetRegionRects: usize,
    SetRegionRects: usize,
    CaptionTapped: usize,
    pub RemoveCaptionTapped:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerEntered: usize,
    pub RemovePointerEntered:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerExited: usize,
    pub RemovePointerExited:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerMoved: usize,
    pub RemovePointerMoved:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerPressed: usize,
    pub RemovePointerPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerReleased: usize,
    pub RemovePointerReleased:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    RegionsChanged: usize,
    pub RemoveRegionsChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputNonClientPointerSource2,
    IInputNonClientPointerSource2_Vtbl,
    0xdd2b10c4_7de6_5c1d_b438_06ddc994058f
);
impl windows_core::RuntimeType for IInputNonClientPointerSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputNonClientPointerSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    EnteringMoveSize: usize,
    pub RemoveEnteringMoveSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    EnteredMoveSize: usize,
    pub RemoveEnteredMoveSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    WindowRectChanging: usize,
    pub RemoveWindowRectChanging:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    WindowRectChanged: usize,
    pub RemoveWindowRectChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ExitedMoveSize: usize,
    pub RemoveExitedMoveSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputNonClientPointerSourceStatics,
    IInputNonClientPointerSourceStatics_Vtbl,
    0x7d0b775c_1903_5dc7_bd2f_7a4b31f0cff2
);
impl windows_core::RuntimeType for IInputNonClientPointerSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputNonClientPointerSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        WindowId,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputObject,
    IInputObject_Vtbl,
    0x42edbc88_d386_544d_b1b8_68617fe68282
);
impl windows_core::RuntimeType for IInputObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputObject {
    pub fn DispatcherQueue(&self) -> windows_core::Result<DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IInputObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DispatcherQueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputObjectFactory,
    IInputObjectFactory_Vtbl,
    0xf7786bc2_b0b8_5961_9a57_ae199d452106
);
impl windows_core::RuntimeType for IInputObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputPointerSource,
    IInputPointerSource_Vtbl,
    0x6a6c2764_c3f4_5be5_8447_c9a98766c240
);
impl windows_core::RuntimeType for IInputPointerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputPointerSource {
    pub fn Cursor(&self) -> windows_core::Result<InputCursor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cursor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InputCursor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCursor)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn DeviceKinds(&self) -> windows_core::Result<InputPointerSourceDeviceKinds> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceKinds)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IInputPointerSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cursor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCursor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DeviceKinds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut InputPointerSourceDeviceKinds,
    ) -> windows_core::HRESULT,
    PointerCaptureLost: usize,
    pub RemovePointerCaptureLost:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerEntered: usize,
    pub RemovePointerEntered:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerExited: usize,
    pub RemovePointerExited:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerMoved: usize,
    pub RemovePointerMoved:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerPressed: usize,
    pub RemovePointerPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerReleased: usize,
    pub RemovePointerReleased:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerRoutedAway: usize,
    pub RemovePointerRoutedAway:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerRoutedReleased: usize,
    pub RemovePointerRoutedReleased:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerRoutedTo: usize,
    pub RemovePointerRoutedTo:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PointerWheelChanged: usize,
    pub RemovePointerWheelChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputPointerSourceStatics,
    IInputPointerSourceStatics_Vtbl,
    0xe8a19fd1_a914_533f_9b0f_6bf0065e6781
);
impl windows_core::RuntimeType for IInputPointerSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPointerSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputPreTranslateKeyboardSource,
    IInputPreTranslateKeyboardSource_Vtbl,
    0x2f327feb_b7e7_5e37_a0cc_37dcabe76588
);
impl windows_core::RuntimeType for IInputPreTranslateKeyboardSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPreTranslateKeyboardSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputPreTranslateKeyboardSourceStatics,
    IInputPreTranslateKeyboardSourceStatics_Vtbl,
    0x23d584d2_af8c_5a8a_806f_2ba9c5b1a5ec
);
impl windows_core::RuntimeType for IInputPreTranslateKeyboardSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPreTranslateKeyboardSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IInputSystemCursor,
    IInputSystemCursor_Vtbl,
    0x59f538e7_c500_59ab_8b54_0bc6100fd49e
);
impl windows_core::RuntimeType for IInputSystemCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IInputSystemCursor {
    pub fn CursorShape(&self) -> windows_core::Result<InputSystemCursorShape> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CursorShape)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IInputSystemCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CursorShape: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut InputSystemCursorShape,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IInputSystemCursorStatics,
    IInputSystemCursorStatics_Vtbl,
    0xd3860bb6_698a_5814_aedd_c2fa8bba5a02
);
impl windows_core::RuntimeType for IInputSystemCursorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputSystemCursorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        InputSystemCursorShape,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IKeyEventArgs,
    IKeyEventArgs_Vtbl,
    0x40d5bb74_977e_5194_8039_9f6c44427bbb
);
impl windows_core::RuntimeType for IKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IKeyEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<PhysicalKeyStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyStatus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IKeyEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PhysicalKeyStatus,
    ) -> windows_core::HRESULT,
    pub Timestamp:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IManipulationCompletedEventArgs,
    IManipulationCompletedEventArgs_Vtbl,
    0x0e0249d4_46e4_5559_aee3_fa45ce2a7f56
);
impl windows_core::RuntimeType for IManipulationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IManipulationCompletedEventArgs {
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IManipulationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cumulative: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationDelta,
    ) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationVelocities,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IManipulationInertiaStartingEventArgs,
    IManipulationInertiaStartingEventArgs_Vtbl,
    0xacf9ef71_6e15_56ab_9260_f0d3ce5f66e8
);
impl windows_core::RuntimeType for IManipulationInertiaStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IManipulationInertiaStartingEventArgs {
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Delta(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delta)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IManipulationInertiaStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cumulative: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationDelta,
    ) -> windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationDelta,
    ) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationVelocities,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IManipulationStartedEventArgs,
    IManipulationStartedEventArgs_Vtbl,
    0x4a616613_eef1_5f1b_a768_0775478d49d4
);
impl windows_core::RuntimeType for IManipulationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IManipulationStartedEventArgs {
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IManipulationStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cumulative: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationDelta,
    ) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IManipulationUpdatedEventArgs,
    IManipulationUpdatedEventArgs_Vtbl,
    0x406e1961_0c98_5fc0_b3d8_116492ef0053
);
impl windows_core::RuntimeType for IManipulationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IManipulationUpdatedEventArgs {
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Delta(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delta)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IManipulationUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cumulative: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationDelta,
    ) -> windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationDelta,
    ) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ManipulationVelocities,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMouseWheelParameters,
    IMouseWheelParameters_Vtbl,
    0x6d98be40_1d56_51d1_aa0d_f325439cd009
);
impl windows_core::RuntimeType for IMouseWheelParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMouseWheelParameters {
    pub fn CharTranslation(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CharTranslation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharTranslation(&self, value: Point) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCharTranslation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn DeltaScale(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeltaScale)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDeltaScale(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDeltaScale)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn DeltaRotationAngle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeltaRotationAngle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDeltaRotationAngle(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDeltaRotationAngle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn PageTranslation(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PageTranslation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPageTranslation(&self, value: Point) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPageTranslation)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IMouseWheelParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CharTranslation:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub SetCharTranslation:
        unsafe extern "system" fn(*mut core::ffi::c_void, Point) -> windows_core::HRESULT,
    pub DeltaScale:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDeltaScale:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DeltaRotationAngle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDeltaRotationAngle:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub PageTranslation:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub SetPageTranslation:
        unsafe extern "system" fn(*mut core::ffi::c_void, Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INonClientCaptionTappedEventArgs,
    INonClientCaptionTappedEventArgs_Vtbl,
    0x3d173531_991f_5753_b7e0_14a121c3cd2d
);
impl windows_core::RuntimeType for INonClientCaptionTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INonClientCaptionTappedEventArgs {
    pub fn Point(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Point)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct INonClientCaptionTappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Point:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INonClientPointerEventArgs,
    INonClientPointerEventArgs_Vtbl,
    0xa5b44aec_b797_505a_a129_ae4e5271c73c
);
impl windows_core::RuntimeType for INonClientPointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INonClientPointerEventArgs {
    pub fn Point(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Point)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RegionKind(&self) -> windows_core::Result<NonClientRegionKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegionKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsPointInRegion(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPointInRegion)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct INonClientPointerEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Point:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub RegionKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut NonClientRegionKind,
    ) -> windows_core::HRESULT,
    pub IsPointInRegion:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    INonClientRegionsChangedEventArgs,
    INonClientRegionsChangedEventArgs_Vtbl,
    0xfe97ee95_1824_51b2_b8eb_10ff0665ce23
);
impl windows_core::RuntimeType for INonClientRegionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl INonClientRegionsChangedEventArgs {
    pub fn ChangedRegions(&self) -> windows_core::Result<windows_core::Array<NonClientRegionKind>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).ChangedRegions)(
                windows_core::Interface::as_raw(self),
                windows_core::Array::<NonClientRegionKind>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
}
#[repr(C)]
pub struct INonClientRegionsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangedRegions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut NonClientRegionKind,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IOverlappedPresenter,
    IOverlappedPresenter_Vtbl,
    0x21693970_4f4c_5172_9e9d_682a2d174884
);
impl windows_core::RuntimeType for IOverlappedPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IOverlappedPresenter {
    pub fn HasBorder(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasBorder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn HasTitleBar(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasTitleBar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsAlwaysOnTop(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAlwaysOnTop)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsAlwaysOnTop(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsAlwaysOnTop)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsMaximizable(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMaximizable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsMaximizable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsMaximizable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsMinimizable(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMinimizable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsMinimizable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsMinimizable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsModal(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsModal)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsModal(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsModal)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsResizable(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsResizable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsResizable(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsResizable)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn State(&self) -> windows_core::Result<OverlappedPresenterState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Maximize(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Maximize)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn Minimize(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Minimize)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn Restore(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn SetBorderAndTitleBar(
        &self,
        hasborder: bool,
        hastitlebar: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBorderAndTitleBar)(
                windows_core::Interface::as_raw(self),
                hasborder,
                hastitlebar,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IOverlappedPresenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasBorder:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasTitleBar:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAlwaysOnTop:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAlwaysOnTop:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsMaximizable:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsMaximizable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsMinimizable:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsMinimizable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsModal:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsModal:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsResizable:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsResizable:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut OverlappedPresenterState,
    ) -> windows_core::HRESULT,
    pub Maximize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Minimize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBorderAndTitleBar:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IOverlappedPresenter2,
    IOverlappedPresenter2_Vtbl,
    0x5c6ccd93_4244_5cd2_b355_ed5ea34df730
);
impl windows_core::RuntimeType for IOverlappedPresenter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IOverlappedPresenter2 {
    pub fn MinimizeWithActivation(&self, activatewindow: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).MinimizeWithActivation)(
                windows_core::Interface::as_raw(self),
                activatewindow,
            )
            .ok()
        }
    }
    pub fn RestoreWithActivation(&self, activatewindow: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RestoreWithActivation)(
                windows_core::Interface::as_raw(self),
                activatewindow,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IOverlappedPresenter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MinimizeWithActivation:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RestoreWithActivation:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IOverlappedPresenter3,
    IOverlappedPresenter3_Vtbl,
    0x55d26138_4c38_57e7_a0c1_d467b774db8c
);
impl windows_core::RuntimeType for IOverlappedPresenter3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IOverlappedPresenter3 {
    pub fn PreferredMinimumHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredMinimumHeight)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<i32>| r__.Value())
        }
    }
    pub fn SetPreferredMinimumHeight(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredMinimumHeight)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn PreferredMinimumWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredMinimumWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<i32>| r__.Value())
        }
    }
    pub fn SetPreferredMinimumWidth(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredMinimumWidth)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn PreferredMaximumWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredMaximumWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<i32>| r__.Value())
        }
    }
    pub fn SetPreferredMaximumWidth(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredMaximumWidth)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn PreferredMaximumHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredMaximumHeight)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<i32>| r__.Value())
        }
    }
    pub fn SetPreferredMaximumHeight(&self, value: Option<i32>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<i32> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetPreferredMaximumHeight)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IOverlappedPresenter3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreferredMinimumHeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPreferredMinimumHeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PreferredMinimumWidth: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPreferredMinimumWidth: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PreferredMaximumWidth: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPreferredMaximumWidth: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PreferredMaximumHeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPreferredMaximumHeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IOverlappedPresenterStatics,
    IOverlappedPresenterStatics_Vtbl,
    0x997225e4_7b00_5aee_a4be_d4068d1999e2
);
impl windows_core::RuntimeType for IOverlappedPresenterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOverlappedPresenterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateForContextMenu: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateForDialog: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateForToolWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IOverlappedPresenterStatics2,
    IOverlappedPresenterStatics2_Vtbl,
    0xed5c4f92_32f4_5d15_80d0_b2a5efa04d39
);
impl windows_core::RuntimeType for IOverlappedPresenterStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOverlappedPresenterStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestedStartupState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut OverlappedPresenterState,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerEventArgs,
    IPointerEventArgs_Vtbl,
    0x865b188c_2ed5_5df8_829f_ac0701d5c51a
);
impl windows_core::RuntimeType for IPointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPointerEventArgs {
    pub fn CurrentPoint(&self) -> windows_core::Result<PointerPoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPoint)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHandled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn GetIntermediatePoints(
        &self,
    ) -> windows_core::Result<windows_collections::IVector<PointerPoint>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntermediatePoints)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetIntermediateTransformedPoints<P0>(
        &self,
        transform: P0,
    ) -> windows_core::Result<windows_collections::IVector<PointerPoint>>
    where
        P0: windows_core::Param<IPointerPointTransform>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntermediateTransformedPoints)(
                windows_core::Interface::as_raw(self),
                transform.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IPointerEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    KeyModifiers: usize,
    pub GetIntermediatePoints: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetIntermediateTransformedPoints: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPoint,
    IPointerPoint_Vtbl,
    0x0d430ee6_252c_59a4_b2a2_d44264dc6a40
);
impl windows_core::RuntimeType for IPointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPointerPoint {
    pub fn FrameId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FrameId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsInContact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInContact)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Properties(&self) -> windows_core::Result<PointerPointProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetTransformedPoint<P0>(&self, transform: P0) -> windows_core::Result<PointerPoint>
    where
        P0: windows_core::Param<IPointerPointTransform>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformedPoint)(
                windows_core::Interface::as_raw(self),
                transform.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IPointerPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsInContact:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub PointerId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Timestamp:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetTransformedPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPointProperties,
    IPointerPointProperties_Vtbl,
    0xd760ed77_4b10_57a5_b3cc_d9bf3413e996
);
impl windows_core::RuntimeType for IPointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPointerPointProperties {
    pub fn IsBarrelButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBarrelButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCanceled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsEraser(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEraser)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsHorizontalMouseWheel(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHorizontalMouseWheel)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsInRange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInRange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsInverted(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInverted)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsLeftButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLeftButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsMiddleButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMiddleButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsPrimary(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPrimary)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsRightButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRightButtonPressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsXButton1Pressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsXButton1Pressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsXButton2Pressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsXButton2Pressed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MouseWheelDelta(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MouseWheelDelta)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Orientation(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Orientation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn PointerUpdateKind(&self) -> windows_core::Result<PointerUpdateKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerUpdateKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Pressure(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pressure)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TouchConfidence(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TouchConfidence)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Twist(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Twist)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn XTilt(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XTilt)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn YTilt(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).YTilt)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ContactRect: usize,
    pub IsBarrelButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCanceled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEraser:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHorizontalMouseWheel:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInRange:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInverted:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLeftButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMiddleButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPrimary:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRightButtonPressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsXButton1Pressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsXButton2Pressed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MouseWheelDelta:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Orientation:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerUpdateKind,
    ) -> windows_core::HRESULT,
    pub Pressure:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub TouchConfidence:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Twist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub XTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub YTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPointStatics,
    IPointerPointStatics_Vtbl,
    0x880d34c2_acfd_51d9_b60a_b408ce17590c
);
impl windows_core::RuntimeType for IPointerPointStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerPointStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPointTransform,
    IPointerPointTransform_Vtbl,
    0xdb4791bc_994d_54c7_92ef_66ea1de9b43c
);
impl windows_core::RuntimeType for IPointerPointTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IPointerPointTransform,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IPointerPointTransform {
    pub fn Inverse(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Inverse)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn TryTransform(&self, inpoint: Point, outpoint: &mut Point) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryTransform)(
                windows_core::Interface::as_raw(self),
                inpoint,
                outpoint,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IPointerPointTransform {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPointTransform";
}
#[repr(C)]
pub struct IPointerPointTransform_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Inverse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TryTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Point,
        *mut Point,
        *mut bool,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPredictor,
    IPointerPredictor_Vtbl,
    0x12c100ec_2100_565f_a60c_f1187f438828
);
impl windows_core::RuntimeType for IPointerPredictor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPointerPredictor {
    pub fn PredictionTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PredictionTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPredictionTime(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPredictionTime)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn GetPredictedPoints<P0>(
        &self,
        point: P0,
    ) -> windows_core::Result<windows_core::Array<PointerPoint>>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetPredictedPoints)(
                windows_core::Interface::as_raw(self),
                point.param().abi(),
                windows_core::Array::<PointerPoint>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
}
#[repr(C)]
pub struct IPointerPredictor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PredictionTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    pub SetPredictionTime: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_time::TimeSpan,
    ) -> windows_core::HRESULT,
    pub GetPredictedPoints: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerPredictorStatics,
    IPointerPredictorStatics_Vtbl,
    0x78a8ef30_3e5c_55cd_8f85_65ac09b1a987
);
impl windows_core::RuntimeType for IPointerPredictorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerPredictorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForInputPointerSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRightTappedEventArgs,
    IRightTappedEventArgs_Vtbl,
    0x8ff73b39_887e_50a4_8500_77953039dcb4
);
impl windows_core::RuntimeType for IRightTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRightTappedEventArgs {
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IRightTappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITappedEventArgs,
    ITappedEventArgs_Vtbl,
    0xc3a01bb5_6076_5e0f_871a_9d94a6a8f82b
);
impl windows_core::RuntimeType for ITappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITappedEventArgs {
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TapCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TapCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ITappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut PointerDeviceType,
    ) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub TapCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWindowRectChangedEventArgs,
    IWindowRectChangedEventArgs_Vtbl,
    0x8a885d28_d2d9_5dda_9848_cdf247771037
);
impl windows_core::RuntimeType for IWindowRectChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IWindowRectChangedEventArgs {
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IWindowRectChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PointerScreenPoint: usize,
    pub MoveSizeOperation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MoveSizeOperation,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWindowRectChangingEventArgs,
    IWindowRectChangingEventArgs_Vtbl,
    0xdb13ed3c_debc_5855_8d70_5936fd813457
);
impl windows_core::RuntimeType for IWindowRectChangingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IWindowRectChangingEventArgs {
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AllowRectChange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowRectChange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllowRectChange(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAllowRectChange)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ShowWindow(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowWindow)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetShowWindow(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetShowWindow)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWindowRectChangingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PointerScreenPoint: usize,
    pub MoveSizeOperation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MoveSizeOperation,
    ) -> windows_core::HRESULT,
    OldWindowRect: usize,
    NewWindowRect: usize,
    SetNewWindowRect: usize,
    pub AllowRectChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowRectChange:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShowWindow:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowWindow:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IconShowOptions(pub i32);
impl IconShowOptions {
    pub const ShowIconAndSystemMenu: Self = Self(0);
    pub const HideIconAndSystemMenu: Self = Self(1);
}
impl windows_core::imp::TypeKind for IconShowOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for IconShowOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.IconShowOptions;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputActivationListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputActivationListener,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputActivationListener, InputObject);
impl InputActivationListener {
    pub fn GetForWindowId(windowid: WindowId) -> windows_core::Result<Self> {
        Self::IInputActivationListenerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindowId)(
                windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputActivationListenerStatics<
        R,
        F: FnOnce(&IInputActivationListenerStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            InputActivationListener,
            IInputActivationListenerStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputActivationListener {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputActivationListener>();
}
unsafe impl windows_core::Interface for InputActivationListener {
    type Vtable = <IInputActivationListener as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputActivationListener as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputActivationListener {
    type Target = IInputActivationListener;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputActivationListener {
    const NAME: &'static str = "Microsoft.UI.Input.InputActivationListener";
}
unsafe impl Send for InputActivationListener {}
unsafe impl Sync for InputActivationListener {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputActivationListenerActivationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputActivationListenerActivationChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for InputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IInputActivationListenerActivationChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for InputActivationListenerActivationChangedEventArgs {
    type Vtable =
        <IInputActivationListenerActivationChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IInputActivationListenerActivationChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputActivationListenerActivationChangedEventArgs {
    type Target = IInputActivationListenerActivationChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str =
        "Microsoft.UI.Input.InputActivationListenerActivationChangedEventArgs";
}
unsafe impl Send for InputActivationListenerActivationChangedEventArgs {}
unsafe impl Sync for InputActivationListenerActivationChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0);
    pub const Deactivated: Self = Self(1);
    pub const Activated: Self = Self(2);
}
impl windows_core::imp::TypeKind for InputActivationState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputActivationState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.InputActivationState;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputCursor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputCursor, IClosable);
impl windows_core::RuntimeType for InputCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputCursor>();
}
unsafe impl windows_core::Interface for InputCursor {
    type Vtable = <IInputCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputCursor as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputCursor {
    type Target = IInputCursor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputCursor";
}
unsafe impl Send for InputCursor {}
unsafe impl Sync for InputCursor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputCustomCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputCustomCursor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputCustomCursor, IClosable, InputCursor);
impl windows_core::RuntimeType for InputCustomCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputCustomCursor>();
}
unsafe impl windows_core::Interface for InputCustomCursor {
    type Vtable = <IInputCustomCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputCustomCursor as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputCustomCursor {
    type Target = IInputCustomCursor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputCustomCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputCustomCursor";
}
unsafe impl Send for InputCustomCursor {}
unsafe impl Sync for InputCustomCursor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputDesktopNamedResourceCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputDesktopNamedResourceCursor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputDesktopNamedResourceCursor, IClosable, InputCursor);
impl InputDesktopNamedResourceCursor {
    pub fn Create(resourcename: &str) -> windows_core::Result<Self> {
        Self::IInputDesktopNamedResourceCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(resourcename)),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromModule(modulename: &str, resourcename: &str) -> windows_core::Result<Self> {
        Self::IInputDesktopNamedResourceCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModule)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(modulename)),
                core::mem::transmute_copy(&windows_core::HSTRING::from(resourcename)),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputDesktopNamedResourceCursorStatics<
        R,
        F: FnOnce(&IInputDesktopNamedResourceCursorStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            InputDesktopNamedResourceCursor,
            IInputDesktopNamedResourceCursorStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputDesktopNamedResourceCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputDesktopNamedResourceCursor>();
}
unsafe impl windows_core::Interface for InputDesktopNamedResourceCursor {
    type Vtable = <IInputDesktopNamedResourceCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IInputDesktopNamedResourceCursor as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputDesktopNamedResourceCursor {
    type Target = IInputDesktopNamedResourceCursor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputDesktopNamedResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputDesktopNamedResourceCursor";
}
unsafe impl Send for InputDesktopNamedResourceCursor {}
unsafe impl Sync for InputDesktopNamedResourceCursor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputDesktopResourceCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputDesktopResourceCursor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputDesktopResourceCursor, IClosable, InputCursor);
impl InputDesktopResourceCursor {
    pub fn Create(resourceid: u32) -> windows_core::Result<Self> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                resourceid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromModule(modulename: &str, resourceid: u32) -> windows_core::Result<Self> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModule)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(modulename)),
                resourceid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputDesktopResourceCursorStatics<
        R,
        F: FnOnce(&IInputDesktopResourceCursorStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            InputDesktopResourceCursor,
            IInputDesktopResourceCursorStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputDesktopResourceCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputDesktopResourceCursor>();
}
unsafe impl windows_core::Interface for InputDesktopResourceCursor {
    type Vtable = <IInputDesktopResourceCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputDesktopResourceCursor as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputDesktopResourceCursor {
    type Target = IInputDesktopResourceCursor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputDesktopResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputDesktopResourceCursor";
}
unsafe impl Send for InputDesktopResourceCursor {}
unsafe impl Sync for InputDesktopResourceCursor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputFocusController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputFocusController,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputFocusController, InputObject);
impl windows_core::RuntimeType for InputFocusController {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputFocusController>();
}
unsafe impl windows_core::Interface for InputFocusController {
    type Vtable = <IInputFocusController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputFocusController as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputFocusController {
    type Target = IInputFocusController;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputFocusController {
    const NAME: &'static str = "Microsoft.UI.Input.InputFocusController";
}
unsafe impl Send for InputFocusController {}
unsafe impl Sync for InputFocusController {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputFocusNavigationHost(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputFocusNavigationHost,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputFocusNavigationHost, InputObject);
impl windows_core::RuntimeType for InputFocusNavigationHost {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputFocusNavigationHost>();
}
unsafe impl windows_core::Interface for InputFocusNavigationHost {
    type Vtable = <IInputFocusNavigationHost as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputFocusNavigationHost as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputFocusNavigationHost {
    type Target = IInputFocusNavigationHost;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputFocusNavigationHost {
    const NAME: &'static str = "Microsoft.UI.Input.InputFocusNavigationHost";
}
unsafe impl Send for InputFocusNavigationHost {}
unsafe impl Sync for InputFocusNavigationHost {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputKeyboardSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputKeyboardSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputKeyboardSource, InputObject);
impl windows_core::RuntimeType for InputKeyboardSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputKeyboardSource>();
}
unsafe impl windows_core::Interface for InputKeyboardSource {
    type Vtable = <IInputKeyboardSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputKeyboardSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputKeyboardSource {
    type Target = IInputKeyboardSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputKeyboardSource";
}
unsafe impl Send for InputKeyboardSource {}
unsafe impl Sync for InputKeyboardSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputLightDismissAction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputLightDismissAction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputLightDismissAction, InputObject);
impl InputLightDismissAction {
    pub fn GetForWindowId(windowid: WindowId) -> windows_core::Result<Self> {
        Self::IInputLightDismissActionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindowId)(
                windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputLightDismissActionStatics<
        R,
        F: FnOnce(&IInputLightDismissActionStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            InputLightDismissAction,
            IInputLightDismissActionStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputLightDismissAction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputLightDismissAction>();
}
unsafe impl windows_core::Interface for InputLightDismissAction {
    type Vtable = <IInputLightDismissAction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputLightDismissAction as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputLightDismissAction {
    type Target = IInputLightDismissAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputLightDismissAction {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissAction";
}
unsafe impl Send for InputLightDismissAction {}
unsafe impl Sync for InputLightDismissAction {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputLightDismissEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputLightDismissEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for InputLightDismissEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputLightDismissEventArgs>();
}
unsafe impl windows_core::Interface for InputLightDismissEventArgs {
    type Vtable = <IInputLightDismissEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputLightDismissEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputLightDismissEventArgs {
    type Target = IInputLightDismissEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputLightDismissEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissEventArgs";
}
unsafe impl Send for InputLightDismissEventArgs {}
unsafe impl Sync for InputLightDismissEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputNonClientPointerSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputNonClientPointerSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl InputNonClientPointerSource {
    pub fn GetForWindowId(windowid: WindowId) -> windows_core::Result<Self> {
        Self::IInputNonClientPointerSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindowId)(
                windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputNonClientPointerSourceStatics<
        R,
        F: FnOnce(&IInputNonClientPointerSourceStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            InputNonClientPointerSource,
            IInputNonClientPointerSourceStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputNonClientPointerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputNonClientPointerSource>();
}
unsafe impl windows_core::Interface for InputNonClientPointerSource {
    type Vtable = <IInputNonClientPointerSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputNonClientPointerSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputNonClientPointerSource {
    type Target = IInputNonClientPointerSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputNonClientPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputNonClientPointerSource";
}
unsafe impl Send for InputNonClientPointerSource {}
unsafe impl Sync for InputNonClientPointerSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for InputObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputObject>();
}
unsafe impl windows_core::Interface for InputObject {
    type Vtable = <IInputObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputObject as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputObject {
    type Target = IInputObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputObject {
    const NAME: &'static str = "Microsoft.UI.Input.InputObject";
}
unsafe impl Send for InputObject {}
unsafe impl Sync for InputObject {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputPointerSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputPointerSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputPointerSource, InputObject);
impl windows_core::RuntimeType for InputPointerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputPointerSource>();
}
unsafe impl windows_core::Interface for InputPointerSource {
    type Vtable = <IInputPointerSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputPointerSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputPointerSource {
    type Target = IInputPointerSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputPointerSource";
}
unsafe impl Send for InputPointerSource {}
unsafe impl Sync for InputPointerSource {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputPointerSourceDeviceKinds(pub u32);
impl InputPointerSourceDeviceKinds {
    pub const None: Self = Self(0);
    pub const Touch: Self = Self(1);
    pub const Pen: Self = Self(2);
    pub const Mouse: Self = Self(4);
}
impl windows_core::imp::TypeKind for InputPointerSourceDeviceKinds {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputPointerSourceDeviceKinds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.InputPointerSourceDeviceKinds;u4)",
    );
}
impl InputPointerSourceDeviceKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InputPointerSourceDeviceKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for InputPointerSourceDeviceKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputPreTranslateKeyboardSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputPreTranslateKeyboardSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputPreTranslateKeyboardSource, InputObject);
impl windows_core::RuntimeType for InputPreTranslateKeyboardSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputPreTranslateKeyboardSource>();
}
unsafe impl windows_core::Interface for InputPreTranslateKeyboardSource {
    type Vtable = <IInputPreTranslateKeyboardSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IInputPreTranslateKeyboardSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputPreTranslateKeyboardSource {
    type Target = IInputPreTranslateKeyboardSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputPreTranslateKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputPreTranslateKeyboardSource";
}
unsafe impl Send for InputPreTranslateKeyboardSource {}
unsafe impl Sync for InputPreTranslateKeyboardSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputSystemCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    InputSystemCursor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(InputSystemCursor, IClosable, InputCursor);
impl InputSystemCursor {
    pub fn Create(r#type: InputSystemCursorShape) -> windows_core::Result<Self> {
        Self::IInputSystemCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                r#type,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputSystemCursorStatics<
        R,
        F: FnOnce(&IInputSystemCursorStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            InputSystemCursor,
            IInputSystemCursorStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputSystemCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInputSystemCursor>();
}
unsafe impl windows_core::Interface for InputSystemCursor {
    type Vtable = <IInputSystemCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputSystemCursor as windows_core::Interface>::IID;
}
impl core::ops::Deref for InputSystemCursor {
    type Target = IInputSystemCursor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for InputSystemCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputSystemCursor";
}
unsafe impl Send for InputSystemCursor {}
unsafe impl Sync for InputSystemCursor {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputSystemCursorShape(pub i32);
impl InputSystemCursorShape {
    pub const Arrow: Self = Self(0);
    pub const Cross: Self = Self(1);
    pub const Hand: Self = Self(3);
    pub const Help: Self = Self(4);
    pub const IBeam: Self = Self(5);
    pub const SizeAll: Self = Self(6);
    pub const SizeNortheastSouthwest: Self = Self(7);
    pub const SizeNorthSouth: Self = Self(8);
    pub const SizeNorthwestSoutheast: Self = Self(9);
    pub const SizeWestEast: Self = Self(10);
    pub const UniversalNo: Self = Self(11);
    pub const UpArrow: Self = Self(12);
    pub const Wait: Self = Self(13);
    pub const Pin: Self = Self(14);
    pub const Person: Self = Self(15);
    pub const AppStarting: Self = Self(16);
}
impl windows_core::imp::TypeKind for InputSystemCursorShape {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputSystemCursorShape {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.InputSystemCursorShape;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    KeyEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for KeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IKeyEventArgs>();
}
unsafe impl windows_core::Interface for KeyEventArgs {
    type Vtable = <IKeyEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for KeyEventArgs {
    type Target = IKeyEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for KeyEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.KeyEventArgs";
}
unsafe impl Send for KeyEventArgs {}
unsafe impl Sync for KeyEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ManipulationCompletedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IManipulationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationCompletedEventArgs {
    type Vtable = <IManipulationCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IManipulationCompletedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ManipulationCompletedEventArgs {
    type Target = IManipulationCompletedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationCompletedEventArgs";
}
unsafe impl Send for ManipulationCompletedEventArgs {}
unsafe impl Sync for ManipulationCompletedEventArgs {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ManipulationDelta {
    pub translation: Point,
    pub scale: f32,
    pub rotation: f32,
    pub expansion: f32,
}
impl windows_core::imp::TypeKind for ManipulationDelta {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ManipulationDelta {
    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"struct(Microsoft.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)") ;
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationInertiaStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ManipulationInertiaStartingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IManipulationInertiaStartingEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = <IManipulationInertiaStartingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IManipulationInertiaStartingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ManipulationInertiaStartingEventArgs {
    type Target = IManipulationInertiaStartingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationInertiaStartingEventArgs";
}
unsafe impl Send for ManipulationInertiaStartingEventArgs {}
unsafe impl Sync for ManipulationInertiaStartingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ManipulationStartedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IManipulationStartedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationStartedEventArgs {
    type Vtable = <IManipulationStartedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationStartedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ManipulationStartedEventArgs {
    type Target = IManipulationStartedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationStartedEventArgs";
}
unsafe impl Send for ManipulationStartedEventArgs {}
unsafe impl Sync for ManipulationStartedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ManipulationUpdatedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IManipulationUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationUpdatedEventArgs {
    type Vtable = <IManipulationUpdatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationUpdatedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for ManipulationUpdatedEventArgs {
    type Target = IManipulationUpdatedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationUpdatedEventArgs";
}
unsafe impl Send for ManipulationUpdatedEventArgs {}
unsafe impl Sync for ManipulationUpdatedEventArgs {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ManipulationVelocities {
    pub linear: Point,
    pub angular: f32,
    pub expansion: f32,
}
impl windows_core::imp::TypeKind for ManipulationVelocities {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ManipulationVelocities {
    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"struct(Microsoft.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)") ;
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MouseWheelParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MouseWheelParameters,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for MouseWheelParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMouseWheelParameters>();
}
unsafe impl windows_core::Interface for MouseWheelParameters {
    type Vtable = <IMouseWheelParameters as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMouseWheelParameters as windows_core::Interface>::IID;
}
impl core::ops::Deref for MouseWheelParameters {
    type Target = IMouseWheelParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Microsoft.UI.Input.MouseWheelParameters";
}
unsafe impl Send for MouseWheelParameters {}
unsafe impl Sync for MouseWheelParameters {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MoveSizeOperation(pub i32);
impl MoveSizeOperation {
    pub const Move: Self = Self(0);
    pub const SizeBottom: Self = Self(1);
    pub const SizeBottomLeft: Self = Self(2);
    pub const SizeBottomRight: Self = Self(3);
    pub const SizeLeft: Self = Self(4);
    pub const SizeRight: Self = Self(5);
    pub const SizeTop: Self = Self(6);
    pub const SizeTopLeft: Self = Self(7);
    pub const SizeTopRight: Self = Self(8);
}
impl windows_core::imp::TypeKind for MoveSizeOperation {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MoveSizeOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.MoveSizeOperation;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonClientCaptionTappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NonClientCaptionTappedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for NonClientCaptionTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INonClientCaptionTappedEventArgs>();
}
unsafe impl windows_core::Interface for NonClientCaptionTappedEventArgs {
    type Vtable = <INonClientCaptionTappedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <INonClientCaptionTappedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for NonClientCaptionTappedEventArgs {
    type Target = INonClientCaptionTappedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NonClientCaptionTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientCaptionTappedEventArgs";
}
unsafe impl Send for NonClientCaptionTappedEventArgs {}
unsafe impl Sync for NonClientCaptionTappedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonClientPointerEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NonClientPointerEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for NonClientPointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INonClientPointerEventArgs>();
}
unsafe impl windows_core::Interface for NonClientPointerEventArgs {
    type Vtable = <INonClientPointerEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INonClientPointerEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for NonClientPointerEventArgs {
    type Target = INonClientPointerEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NonClientPointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientPointerEventArgs";
}
unsafe impl Send for NonClientPointerEventArgs {}
unsafe impl Sync for NonClientPointerEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NonClientRegionKind(pub i32);
impl NonClientRegionKind {
    pub const Close: Self = Self(0);
    pub const Maximize: Self = Self(1);
    pub const Minimize: Self = Self(2);
    pub const Icon: Self = Self(3);
    pub const Caption: Self = Self(4);
    pub const TopBorder: Self = Self(5);
    pub const LeftBorder: Self = Self(6);
    pub const BottomBorder: Self = Self(7);
    pub const RightBorder: Self = Self(8);
    pub const Passthrough: Self = Self(9);
}
impl windows_core::imp::TypeKind for NonClientRegionKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for NonClientRegionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.NonClientRegionKind;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonClientRegionsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    NonClientRegionsChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for NonClientRegionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, INonClientRegionsChangedEventArgs>();
}
unsafe impl windows_core::Interface for NonClientRegionsChangedEventArgs {
    type Vtable = <INonClientRegionsChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <INonClientRegionsChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for NonClientRegionsChangedEventArgs {
    type Target = INonClientRegionsChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for NonClientRegionsChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientRegionsChangedEventArgs";
}
unsafe impl Send for NonClientRegionsChangedEventArgs {}
unsafe impl Sync for NonClientRegionsChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlappedPresenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    OverlappedPresenter,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(OverlappedPresenter, AppWindowPresenter);
impl OverlappedPresenter {
    pub fn Create() -> windows_core::Result<Self> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateForContextMenu() -> windows_core::Result<Self> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForContextMenu)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateForDialog() -> windows_core::Result<Self> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForDialog)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateForToolWindow() -> windows_core::Result<Self> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForToolWindow)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn RequestedStartupState() -> windows_core::Result<OverlappedPresenterState> {
        Self::IOverlappedPresenterStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedStartupState)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    fn IOverlappedPresenterStatics<
        R,
        F: FnOnce(&IOverlappedPresenterStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IOverlappedPresenterStatics2<
        R,
        F: FnOnce(&IOverlappedPresenterStatics2) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics2,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for OverlappedPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IOverlappedPresenter>();
}
unsafe impl windows_core::Interface for OverlappedPresenter {
    type Vtable = <IOverlappedPresenter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOverlappedPresenter as windows_core::Interface>::IID;
}
impl core::ops::Deref for OverlappedPresenter {
    type Target = IOverlappedPresenter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for OverlappedPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.OverlappedPresenter";
}
unsafe impl Send for OverlappedPresenter {}
unsafe impl Sync for OverlappedPresenter {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverlappedPresenterState(pub i32);
impl OverlappedPresenterState {
    pub const Maximized: Self = Self(0);
    pub const Minimized: Self = Self(1);
    pub const Restored: Self = Self(2);
}
impl windows_core::imp::TypeKind for OverlappedPresenterState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for OverlappedPresenterState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.OverlappedPresenterState;i4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhysicalKeyStatus {
    pub repeat_count: u32,
    pub scan_code: u32,
    pub is_extended_key: bool,
    pub is_menu_key_down: bool,
    pub was_key_down: bool,
    pub is_key_released: bool,
}
impl windows_core::imp::TypeKind for PhysicalKeyStatus {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PhysicalKeyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Input.PhysicalKeyStatus;u4;u4;b1;b1;b1;b1)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl windows_core::imp::TypeKind for Point {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Point {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0);
    pub const Pen: Self = Self(1);
    pub const Mouse: Self = Self(2);
    pub const Touchpad: Self = Self(3);
}
impl windows_core::imp::TypeKind for PointerDeviceType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PointerDeviceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.PointerDeviceType;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for PointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerEventArgs>();
}
unsafe impl windows_core::Interface for PointerEventArgs {
    type Vtable = <IPointerEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for PointerEventArgs {
    type Target = IPointerEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.PointerEventArgs";
}
unsafe impl Send for PointerEventArgs {}
unsafe impl Sync for PointerEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerPoint,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl PointerPoint {
    pub fn GetCurrentPoint(pointerid: u32) -> windows_core::Result<Self> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPoint)(
                windows_core::Interface::as_raw(this),
                pointerid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IPointerPointStatics<R, F: FnOnce(&IPointerPointStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerPoint, IPointerPointStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerPoint>();
}
unsafe impl windows_core::Interface for PointerPoint {
    type Vtable = <IPointerPoint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPoint as windows_core::Interface>::IID;
}
impl core::ops::Deref for PointerPoint {
    type Target = IPointerPoint;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPoint";
}
unsafe impl Send for PointerPoint {}
unsafe impl Sync for PointerPoint {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPointProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerPointProperties,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for PointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerPointProperties>();
}
unsafe impl windows_core::Interface for PointerPointProperties {
    type Vtable = <IPointerPointProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPointProperties as windows_core::Interface>::IID;
}
impl core::ops::Deref for PointerPointProperties {
    type Target = IPointerPointProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPointProperties";
}
unsafe impl Send for PointerPointProperties {}
unsafe impl Sync for PointerPointProperties {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPredictor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerPredictor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(PointerPredictor, IClosable);
impl PointerPredictor {
    pub fn CreateForInputPointerSource<P0>(inputpointersource: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<InputPointerSource>,
    {
        Self::IPointerPredictorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForInputPointerSource)(
                windows_core::Interface::as_raw(this),
                inputpointersource.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IPointerPredictorStatics<
        R,
        F: FnOnce(&IPointerPredictorStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerPredictor, IPointerPredictorStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerPredictor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerPredictor>();
}
unsafe impl windows_core::Interface for PointerPredictor {
    type Vtable = <IPointerPredictor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPredictor as windows_core::Interface>::IID;
}
impl core::ops::Deref for PointerPredictor {
    type Target = IPointerPredictor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PointerPredictor {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPredictor";
}
unsafe impl Send for PointerPredictor {}
unsafe impl Sync for PointerPredictor {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0);
    pub const LeftButtonPressed: Self = Self(1);
    pub const LeftButtonReleased: Self = Self(2);
    pub const RightButtonPressed: Self = Self(3);
    pub const RightButtonReleased: Self = Self(4);
    pub const MiddleButtonPressed: Self = Self(5);
    pub const MiddleButtonReleased: Self = Self(6);
    pub const XButton1Pressed: Self = Self(7);
    pub const XButton1Released: Self = Self(8);
    pub const XButton2Pressed: Self = Self(9);
    pub const XButton2Released: Self = Self(10);
}
impl windows_core::imp::TypeKind for PointerUpdateKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PointerUpdateKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.PointerUpdateKind;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RightTappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RightTappedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRightTappedEventArgs>();
}
unsafe impl windows_core::Interface for RightTappedEventArgs {
    type Vtable = <IRightTappedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRightTappedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for RightTappedEventArgs {
    type Target = IRightTappedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.RightTappedEventArgs";
}
unsafe impl Send for RightTappedEventArgs {}
unsafe impl Sync for RightTappedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TappedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for TappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITappedEventArgs>();
}
unsafe impl windows_core::Interface for TappedEventArgs {
    type Vtable = <ITappedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITappedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TappedEventArgs {
    type Target = ITappedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.TappedEventArgs";
}
unsafe impl Send for TappedEventArgs {}
unsafe impl Sync for TappedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TitleBarHeightOption(pub i32);
impl TitleBarHeightOption {
    pub const Standard: Self = Self(0);
    pub const Tall: Self = Self(1);
    pub const Collapsed: Self = Self(2);
}
impl windows_core::imp::TypeKind for TitleBarHeightOption {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TitleBarHeightOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.TitleBarHeightOption;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TitleBarTheme(pub i32);
impl TitleBarTheme {
    pub const Legacy: Self = Self(0);
    pub const UseDefaultAppMode: Self = Self(1);
    pub const Light: Self = Self(2);
    pub const Dark: Self = Self(3);
}
impl windows_core::imp::TypeKind for TitleBarTheme {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TitleBarTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.TitleBarTheme;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VirtualKeyStates(pub u32);
impl VirtualKeyStates {
    pub const None: Self = Self(0);
    pub const Down: Self = Self(1);
    pub const Locked: Self = Self(2);
}
impl windows_core::imp::TypeKind for VirtualKeyStates {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for VirtualKeyStates {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.VirtualKeyStates;u4)");
}
impl VirtualKeyStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VirtualKeyStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VirtualKeyStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VirtualKeyStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for VirtualKeyStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for VirtualKeyStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowId {
    pub value: u64,
}
impl windows_core::imp::TypeKind for WindowId {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for WindowId {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.WindowId;u8)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowRectChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WindowRectChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for WindowRectChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWindowRectChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowRectChangedEventArgs {
    type Vtable = <IWindowRectChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowRectChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for WindowRectChangedEventArgs {
    type Target = IWindowRectChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for WindowRectChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.WindowRectChangedEventArgs";
}
unsafe impl Send for WindowRectChangedEventArgs {}
unsafe impl Sync for WindowRectChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowRectChangingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WindowRectChangingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for WindowRectChangingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWindowRectChangingEventArgs>();
}
unsafe impl windows_core::Interface for WindowRectChangingEventArgs {
    type Vtable = <IWindowRectChangingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowRectChangingEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for WindowRectChangingEventArgs {
    type Target = IWindowRectChangingEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for WindowRectChangingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.WindowRectChangingEventArgs";
}
unsafe impl Send for WindowRectChangingEventArgs {}
unsafe impl Sync for WindowRectChangingEventArgs {}
