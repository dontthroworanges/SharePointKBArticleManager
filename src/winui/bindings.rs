windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
windows_core::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *mut core::ffi::c_void));
windows_core::link!("kernel32.dll" "system" fn GetUserDefaultLocaleName(lplocalename : windows_core::PWSTR, cchlocalename : i32) -> i32);
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppBarButton(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppBarButton,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    AppBarButton,
    IAnimationObject,
    ICommandBarElement,
    IVisualElement,
    IVisualElement2,
    Button,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl AppBarButton {
    pub fn new() -> windows_core::Result<Self> {
        Self::IAppBarButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IAppBarButtonFactory<R, F: FnOnce(&IAppBarButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBarButton, IAppBarButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppBarButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppBarButton>();
}
unsafe impl windows_core::Interface for AppBarButton {
    type Vtable = <IAppBarButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppBarButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppBarButton {
    type Target = IAppBarButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppBarButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AppBarButton";
}
unsafe impl Send for AppBarButton {}
unsafe impl Sync for AppBarButton {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppBarSeparator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppBarSeparator,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    AppBarSeparator,
    IAnimationObject,
    ICommandBarElement,
    IVisualElement,
    IVisualElement2,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl AppBarSeparator {
    pub fn new() -> windows_core::Result<Self> {
        Self::IAppBarSeparatorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IAppBarSeparatorFactory<
        R,
        F: FnOnce(&IAppBarSeparatorFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBarSeparator, IAppBarSeparatorFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppBarSeparator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IAppBarSeparator>();
}
unsafe impl windows_core::Interface for AppBarSeparator {
    type Vtable = <IAppBarSeparator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppBarSeparator as windows_core::Interface>::IID;
}
impl core::ops::Deref for AppBarSeparator {
    type Target = IAppBarSeparator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for AppBarSeparator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.AppBarSeparator";
}
unsafe impl Send for AppBarSeparator {}
unsafe impl Sync for AppBarSeparator {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppWindow(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    AppWindow,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl AppWindow {
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
    fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppWindow, IAppWindowStatics> =
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
pub struct BringIntoViewOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BringIntoViewOptions,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for BringIntoViewOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBringIntoViewOptions>();
}
unsafe impl windows_core::Interface for BringIntoViewOptions {
    type Vtable = <IBringIntoViewOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBringIntoViewOptions as windows_core::Interface>::IID;
}
impl core::ops::Deref for BringIntoViewOptions {
    type Target = IBringIntoViewOptions;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BringIntoViewOptions {
    const NAME: &'static str = "Microsoft.UI.Xaml.BringIntoViewOptions";
}
unsafe impl Send for BringIntoViewOptions {}
unsafe impl Sync for BringIntoViewOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BringIntoViewRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    BringIntoViewRequestedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(BringIntoViewRequestedEventArgs, RoutedEventArgs);
impl windows_core::RuntimeType for BringIntoViewRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IBringIntoViewRequestedEventArgs>();
}
unsafe impl windows_core::Interface for BringIntoViewRequestedEventArgs {
    type Vtable = <IBringIntoViewRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IBringIntoViewRequestedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for BringIntoViewRequestedEventArgs {
    type Target = IBringIntoViewRequestedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for BringIntoViewRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.BringIntoViewRequestedEventArgs";
}
unsafe impl Send for BringIntoViewRequestedEventArgs {}
unsafe impl Sync for BringIntoViewRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Button(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Button, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(
    Button,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    ButtonBase,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Button {
    pub fn new() -> windows_core::Result<Self> {
        Self::IButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IButtonFactory<R, F: FnOnce(&IButtonFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Button, IButtonFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Button {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IButton>();
}
unsafe impl windows_core::Interface for Button {
    type Vtable = <IButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IButton as windows_core::Interface>::IID;
}
impl core::ops::Deref for Button {
    type Target = IButton;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Button {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Button";
}
unsafe impl Send for Button {}
unsafe impl Sync for Button {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ButtonBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ButtonBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ButtonBase,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ButtonBase {
    pub fn new() -> windows_core::Result<Self> {
        Self::IButtonBaseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IButtonBaseFactory<R, F: FnOnce(&IButtonBaseFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ButtonBase, IButtonBaseFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IButtonBase>();
}
unsafe impl windows_core::Interface for ButtonBase {
    type Vtable = <IButtonBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IButtonBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for ButtonBase {
    type Target = IButtonBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ButtonBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ButtonBase";
}
unsafe impl Send for ButtonBase {}
unsafe impl Sync for ButtonBase {}
pub type CLSCTX = u32;
pub type CORRECTIVE_ACTION = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CaretType(pub i32);
impl CaretType {
    pub const Normal: Self = Self(0);
    pub const Null: Self = Self(1);
}
impl windows_core::imp::TypeKind for CaretType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for CaretType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.CaretType;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl windows_core::imp::TypeKind for Color {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Color {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandBarFlyout(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CommandBarFlyout,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(CommandBarFlyout, FlyoutBase, DependencyObject);
impl CommandBarFlyout {
    pub fn new() -> windows_core::Result<Self> {
        Self::ICommandBarFlyoutFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn ICommandBarFlyoutFactory<
        R,
        F: FnOnce(&ICommandBarFlyoutFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CommandBarFlyout, ICommandBarFlyoutFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CommandBarFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICommandBarFlyout>();
}
unsafe impl windows_core::Interface for CommandBarFlyout {
    type Vtable = <ICommandBarFlyout as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICommandBarFlyout as windows_core::Interface>::IID;
}
impl core::ops::Deref for CommandBarFlyout {
    type Target = ICommandBarFlyout;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CommandBarFlyout {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.CommandBarFlyout";
}
unsafe impl Send for CommandBarFlyout {}
unsafe impl Sync for CommandBarFlyout {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContentControl,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ContentControl,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ContentControl {
    pub fn new() -> windows_core::Result<Self> {
        Self::IContentControlFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IContentControlFactory<R, F: FnOnce(&IContentControlFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentControl, IContentControlFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ContentControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContentControl>();
}
unsafe impl windows_core::Interface for ContentControl {
    type Vtable = <IContentControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContentControl {
    type Target = IContentControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContentControl {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ContentControl";
}
unsafe impl Send for ContentControl {}
unsafe impl Sync for ContentControl {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentIslandEnvironment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContentIslandEnvironment,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for ContentIslandEnvironment {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContentIslandEnvironment>();
}
unsafe impl windows_core::Interface for ContentIslandEnvironment {
    type Vtable = <IContentIslandEnvironment as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentIslandEnvironment as windows_core::Interface>::IID;
}
impl core::ops::Deref for ContentIslandEnvironment {
    type Target = IContentIslandEnvironment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ContentIslandEnvironment {
    const NAME: &'static str = "Microsoft.UI.Content.ContentIslandEnvironment";
}
unsafe impl Send for ContentIslandEnvironment {}
unsafe impl Sync for ContentIslandEnvironment {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Control(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Control,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    Control,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl Control {
    pub fn new() -> windows_core::Result<Self> {
        Self::IControlFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn GetIsTemplateFocusTarget<P0>(element: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsTemplateFocusTarget)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SetIsTemplateFocusTarget<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IControlStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetIsTemplateFocusTarget)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn GetIsTemplateKeyTipTarget<P0>(element: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsTemplateKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SetIsTemplateKeyTipTarget<P0>(element: P0, value: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IControlStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetIsTemplateKeyTipTarget)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value,
            )
            .ok()
        })
    }
    fn IControlFactory<R, F: FnOnce(&IControlFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IControlStatics<R, F: FnOnce(&IControlStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Control {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IControl>();
}
unsafe impl windows_core::Interface for Control {
    type Vtable = <IControl as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IControl as windows_core::Interface>::IID;
}
impl core::ops::Deref for Control {
    type Target = IControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Control {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Control";
}
unsafe impl Send for Control {}
unsafe impl Sync for Control {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DependencyObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl DependencyObject {
    pub fn new() -> windows_core::Result<Self> {
        Self::IDependencyObjectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDependencyObjectFactory<
        R,
        F: FnOnce(&IDependencyObjectFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DependencyObject, IDependencyObjectFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
}
unsafe impl windows_core::Interface for DependencyObject {
    type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
}
impl core::ops::Deref for DependencyObject {
    type Target = IDependencyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DependencyObject {
    const NAME: &'static str = "Microsoft.UI.Xaml.DependencyObject";
}
unsafe impl Send for DependencyObject {}
unsafe impl Sync for DependencyObject {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DisabledFormattingAccelerators(pub u32);
impl DisabledFormattingAccelerators {
    pub const None: Self = Self(0);
    pub const Bold: Self = Self(1);
    pub const Italic: Self = Self(2);
    pub const Underline: Self = Self(4);
    pub const All: Self = Self(4294967295);
}
impl windows_core::imp::TypeKind for DisabledFormattingAccelerators {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DisabledFormattingAccelerators {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.DisabledFormattingAccelerators;u4)",
    );
}
impl DisabledFormattingAccelerators {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisabledFormattingAccelerators {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisabledFormattingAccelerators {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisabledFormattingAccelerators {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for DisabledFormattingAccelerators {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for DisabledFormattingAccelerators {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ElementTheme(pub i32);
impl ElementTheme {
    pub const Default: Self = Self(0);
    pub const Light: Self = Self(1);
    pub const Dark: Self = Self(2);
}
impl windows_core::imp::TypeKind for ElementTheme {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ElementTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.ElementTheme;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for EventHandler<T> {
    type Vtable = EventHandler_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for EventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> EventHandler<T> {
    pub fn new<
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &EventHandlerBox::<T, F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct EventHandler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
struct EventHandlerBox<
    T,
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) + 'static,
>(core::marker::PhantomData<(T, fn() -> F)>)
where
    T: windows_core::RuntimeType + 'static;
impl<
    T: windows_core::RuntimeType + 'static,
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) + 'static,
> EventHandlerBox<T, F>
{
    const VTABLE: EventHandler_Vtbl<T> = EventHandler_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<EventHandler<T>, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<EventHandler<T>, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<EventHandler<T>, F>::Release,
        },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<EventHandler<T>, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&args),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FindOptions(pub u32);
impl FindOptions {
    pub const None: Self = Self(0);
    pub const Word: Self = Self(2);
    pub const Case: Self = Self(4);
}
impl windows_core::imp::TypeKind for FindOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FindOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.FindOptions;u4)");
}
impl FindOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FindOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FindOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FindOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for FindOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for FindOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlyoutBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FlyoutBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(FlyoutBase, DependencyObject);
impl FlyoutBase {
    pub fn new() -> windows_core::Result<Self> {
        Self::IFlyoutBaseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn GetAttachedFlyout<P0>(element: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAttachedFlyout)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn SetAttachedFlyout<P0, P1>(element: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
        P1: windows_core::Param<Self>,
    {
        Self::IFlyoutBaseStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetAttachedFlyout)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value.param().abi(),
            )
            .ok()
        })
    }
    pub fn ShowAttachedFlyout<P0>(flyoutowner: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FrameworkElement>,
    {
        Self::IFlyoutBaseStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).ShowAttachedFlyout)(
                windows_core::Interface::as_raw(this),
                flyoutowner.param().abi(),
            )
            .ok()
        })
    }
    fn IFlyoutBaseFactory<R, F: FnOnce(&IFlyoutBaseFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FlyoutBase, IFlyoutBaseFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFlyoutBaseStatics<R, F: FnOnce(&IFlyoutBaseStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FlyoutBase, IFlyoutBaseStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FlyoutBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFlyoutBase>();
}
unsafe impl windows_core::Interface for FlyoutBase {
    type Vtable = <IFlyoutBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFlyoutBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for FlyoutBase {
    type Target = IFlyoutBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FlyoutBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.FlyoutBase";
}
unsafe impl Send for FlyoutBase {}
unsafe impl Sync for FlyoutBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontIcon(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FontIcon,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    FontIcon,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    IconElement,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl FontIcon {
    pub fn new() -> windows_core::Result<Self> {
        Self::IFontIconFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFontIconFactory<R, F: FnOnce(&IFontIconFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FontIcon, IFontIconFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FontIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFontIcon>();
}
unsafe impl windows_core::Interface for FontIcon {
    type Vtable = <IFontIcon as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFontIcon as windows_core::Interface>::IID;
}
impl core::ops::Deref for FontIcon {
    type Target = IFontIcon;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FontIcon {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.FontIcon";
}
unsafe impl Send for FontIcon {}
unsafe impl Sync for FontIcon {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontWeight {
    pub weight: u16,
}
impl windows_core::imp::TypeKind for FontWeight {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FontWeight {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Text.FontWeight;u2)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontWeights(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FontWeights,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl FontWeights {
    pub fn Black() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Black)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn Bold() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bold)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn ExtraBlack() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtraBlack)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn ExtraBold() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtraBold)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn ExtraLight() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtraLight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn Light() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Light)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn Medium() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Medium)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn Normal() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Normal)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SemiBold() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SemiBold)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn SemiLight() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SemiLight)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn Thin() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thin)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    fn IFontWeightsStatics<R, F: FnOnce(&IFontWeightsStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FontWeights, IFontWeightsStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FontWeights {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFontWeights>();
}
unsafe impl windows_core::Interface for FontWeights {
    type Vtable = <IFontWeights as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFontWeights as windows_core::Interface>::IID;
}
impl core::ops::Deref for FontWeights {
    type Target = IFontWeights;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FontWeights {
    const NAME: &'static str = "Microsoft.UI.Text.FontWeights";
}
unsafe impl Send for FontWeights {}
unsafe impl Sync for FontWeights {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FormatEffect(pub i32);
impl FormatEffect {
    pub const Off: Self = Self(0);
    pub const On: Self = Self(1);
    pub const Toggle: Self = Self(2);
    pub const Undefined: Self = Self(3);
}
impl windows_core::imp::TypeKind for FormatEffect {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FormatEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.FormatEffect;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameworkElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    FrameworkElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    FrameworkElement,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    UIElement,
    DependencyObject
);
impl FrameworkElement {
    pub fn new() -> windows_core::Result<Self> {
        Self::IFrameworkElementFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn DeferTree<P0>(element: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IFrameworkElementStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).DeferTree)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
            )
            .ok()
        })
    }
    fn IFrameworkElementFactory<
        R,
        F: FnOnce(&IFrameworkElementFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementStatics<
        R,
        F: FnOnce(&IFrameworkElementStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkElement>();
}
unsafe impl windows_core::Interface for FrameworkElement {
    type Vtable = <IFrameworkElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFrameworkElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for FrameworkElement {
    type Target = IFrameworkElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for FrameworkElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.FrameworkElement";
}
unsafe impl Send for FrameworkElement {}
unsafe impl Sync for FrameworkElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneralTransform(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    GeneralTransform,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(GeneralTransform, DependencyObject);
impl GeneralTransform {
    pub fn new() -> windows_core::Result<Self> {
        Self::IGeneralTransformFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IGeneralTransformFactory<
        R,
        F: FnOnce(&IGeneralTransformFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GeneralTransform, IGeneralTransformFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GeneralTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IGeneralTransform>();
}
unsafe impl windows_core::Interface for GeneralTransform {
    type Vtable = <IGeneralTransform as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGeneralTransform as windows_core::Interface>::IID;
}
impl core::ops::Deref for GeneralTransform {
    type Target = IGeneralTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for GeneralTransform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.GeneralTransform";
}
unsafe impl Send for GeneralTransform {}
unsafe impl Sync for GeneralTransform {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HorizontalCharacterAlignment(pub i32);
impl HorizontalCharacterAlignment {
    pub const Left: Self = Self(0);
    pub const Right: Self = Self(1);
    pub const Center: Self = Self(2);
}
impl windows_core::imp::TypeKind for HorizontalCharacterAlignment {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for HorizontalCharacterAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Text.HorizontalCharacterAlignment;i4)",
    );
}
windows_core::imp::define_interface!(
    IAnimationObject,
    IAnimationObject_Vtbl,
    0x8f56119d_b96d_58d0_9916_d1c5e390f890
);
impl windows_core::RuntimeType for IAnimationObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IAnimationObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Microsoft.UI.Composition.IAnimationObject";
}
#[repr(C)]
pub struct IAnimationObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAppBarButton,
    IAppBarButton_Vtbl,
    0x8ab0e278_b6ae_569e_8dcd_d293552fe4d5
);
impl windows_core::RuntimeType for IAppBarButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppBarButton {
    pub fn Label(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetLabel(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLabel)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn Icon(&self) -> windows_core::Result<IconElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Icon)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IconElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetIcon)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppBarButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Icon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppBarButtonFactory,
    IAppBarButtonFactory_Vtbl,
    0x4168a40a_d11f_5aeb_974e_bb43a6e7f9b2
);
impl windows_core::RuntimeType for IAppBarButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBarButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppBarButtonStatics,
    IAppBarButtonStatics_Vtbl,
    0x74327a2a_18fa_59c3_8268_ec3732a4d272
);
impl windows_core::RuntimeType for IAppBarButtonStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBarButtonStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAppBarSeparator,
    IAppBarSeparator_Vtbl,
    0x57bb94a3_1e56_5ebe_8a57_3a243c491d67
);
impl windows_core::RuntimeType for IAppBarSeparator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBarSeparator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAppBarSeparatorFactory,
    IAppBarSeparatorFactory_Vtbl,
    0x6497d326_fb55_5cf5_8cc4_c556b1a958fb
);
impl windows_core::RuntimeType for IAppBarSeparatorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBarSeparatorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IAppBarSeparatorStatics,
    IAppBarSeparatorStatics_Vtbl,
    0xdda9a3d2_d3b9_5be1_abf7_dba1eaafe0f7
);
impl windows_core::RuntimeType for IAppBarSeparatorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBarSeparatorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IAppBarToggleButton,
    IAppBarToggleButton_Vtbl,
    0x9687c0b1_c390_59be_acdc_4fc48f552823
);
impl windows_core::RuntimeType for IAppBarToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IAppBarToggleButton {
    pub fn Label(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetLabel(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLabel)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IAppBarToggleButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
}
#[repr(C)]
pub struct IAppWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Id: usize,
    IsShownInSwitchers: usize,
    SetIsShownInSwitchers: usize,
    IsVisible: usize,
    OwnerWindowId: usize,
    Position: usize,
    Presenter: usize,
    Size: usize,
    Title: usize,
    SetTitle: usize,
    pub TitleBar: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    Create: usize,
    CreateWithPresenter: usize,
    CreateWithPresenterAndOwner: usize,
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
}
#[repr(C)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BackgroundColor: usize,
    SetBackgroundColor: usize,
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
#[repr(C)]
pub struct IAppWindowTitleBar2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
#[repr(C)]
pub struct IAppWindowTitleBar3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    IBringIntoViewOptions,
    IBringIntoViewOptions_Vtbl,
    0xeeb4a447_eb9e_5003_a479_b9e3a886b708
);
impl windows_core::RuntimeType for IBringIntoViewOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBringIntoViewOptions {
    pub fn AnimationDesired(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnimationDesired)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAnimationDesired(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAnimationDesired)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn TargetRect(&self) -> windows_core::Result<Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetRect)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
            .and_then(|r__: windows_reference::IReference<Rect>| r__.Value())
        }
    }
    pub fn SetTargetRect(&self, value: Option<Rect>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<Rect> as From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetTargetRect)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn HorizontalAlignmentRatio(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontalAlignmentRatio)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalAlignmentRatio(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHorizontalAlignmentRatio)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignmentRatio(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalAlignmentRatio)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalAlignmentRatio(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetVerticalAlignmentRatio)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalOffset(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontalOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHorizontalOffset)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalOffset(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetVerticalOffset)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IBringIntoViewOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AnimationDesired:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAnimationDesired:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TargetRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTargetRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub HorizontalAlignmentRatio:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHorizontalAlignmentRatio:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub VerticalAlignmentRatio:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetVerticalAlignmentRatio:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub HorizontalOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHorizontalOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub VerticalOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetVerticalOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IBringIntoViewRequestedEventArgs,
    IBringIntoViewRequestedEventArgs_Vtbl,
    0x807de8f9_b1dc_5a63_8101_5ee966841a27
);
impl windows_core::RuntimeType for IBringIntoViewRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IBringIntoViewRequestedEventArgs {
    pub fn TargetElement(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTargetElement<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTargetElement)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn TargetRect(&self) -> windows_core::Result<Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetRect)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTargetRect(&self, value: Rect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTargetRect)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignmentRatio(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalAlignmentRatio)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
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
}
#[repr(C)]
pub struct IBringIntoViewRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TargetElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTargetElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    AnimationDesired: usize,
    SetAnimationDesired: usize,
    pub TargetRect:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Rect) -> windows_core::HRESULT,
    pub SetTargetRect:
        unsafe extern "system" fn(*mut core::ffi::c_void, Rect) -> windows_core::HRESULT,
    HorizontalAlignmentRatio: usize,
    pub VerticalAlignmentRatio:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    HorizontalOffset: usize,
    SetHorizontalOffset: usize,
    VerticalOffset: usize,
    SetVerticalOffset: usize,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButton,
    IButton_Vtbl,
    0x216c183d_d07a_5aa5_b8a4_0300a2683e87
);
impl windows_core::RuntimeType for IButton {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IButton {
    pub fn Flyout(&self) -> windows_core::Result<FlyoutBase> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flyout)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetFlyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FlyoutBase>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFlyout)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Flyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFlyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonBase,
    IButtonBase_Vtbl,
    0x65714269_2473_5327_a652_0ea6bce7f403
);
impl windows_core::RuntimeType for IButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IButtonBase {
    pub fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClick,
            ))
        }
    }
}
#[repr(C)]
pub struct IButtonBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ClickMode: usize,
    SetClickMode: usize,
    IsPointerOver: usize,
    IsPressed: usize,
    Command: usize,
    SetCommand: usize,
    CommandParameter: usize,
    SetCommandParameter: usize,
    pub Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonBaseFactory,
    IButtonBaseFactory_Vtbl,
    0x21251aa9_6fd1_5e51_ab3b_e6fcaf3395ed
);
impl windows_core::RuntimeType for IButtonBaseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IButtonBaseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonBaseStatics,
    IButtonBaseStatics_Vtbl,
    0xdbe812f6_adf8_51d3_8137_a8fbf6445b3c
);
impl windows_core::RuntimeType for IButtonBaseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IButtonBaseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IButtonFactory,
    IButtonFactory_Vtbl,
    0xfe393422_d91c_57b1_9a9c_2c7e3f41f77c
);
impl windows_core::RuntimeType for IButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IButtonStatics,
    IButtonStatics_Vtbl,
    0x57823d25_b26a_5e0f_94f6_bbae70683dc5
);
impl windows_core::RuntimeType for IButtonStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IButtonStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
    IComboBox,
    IComboBox_Vtbl,
    0xc77da58b_4fd7_51e0_a431_f84658a83e9e
);
impl windows_core::RuntimeType for IComboBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IComboBox {
    pub fn DropDownOpened<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + 'static,
    {
        let handler: EventHandler<windows_core::IInspectable> = {
            let com =
                windows_core::imp::DelegateBox::<EventHandler<windows_core::IInspectable>, F>::new(
                    &EventHandlerBox::<windows_core::IInspectable, F>::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DropDownOpened)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveDropDownOpened,
            ))
        }
    }
}
#[repr(C)]
pub struct IComboBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsDropDownOpen: usize,
    SetIsDropDownOpen: usize,
    IsEditable: usize,
    SetIsEditable: usize,
    IsSelectionBoxHighlighted: usize,
    MaxDropDownHeight: usize,
    SetMaxDropDownHeight: usize,
    SelectionBoxItem: usize,
    SelectionBoxItemTemplate: usize,
    TemplateSettings: usize,
    Header: usize,
    SetHeader: usize,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    PlaceholderText: usize,
    SetPlaceholderText: usize,
    LightDismissOverlayMode: usize,
    SetLightDismissOverlayMode: usize,
    IsTextSearchEnabled: usize,
    SetIsTextSearchEnabled: usize,
    SelectionChangedTrigger: usize,
    SetSelectionChangedTrigger: usize,
    PlaceholderForeground: usize,
    SetPlaceholderForeground: usize,
    Text: usize,
    SetText: usize,
    TextBoxStyle: usize,
    SetTextBoxStyle: usize,
    Description: usize,
    SetDescription: usize,
    DropDownClosed: usize,
    RemoveDropDownClosed: usize,
    pub DropDownOpened: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveDropDownOpened:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICommandBarElement,
    ICommandBarElement_Vtbl,
    0xf8eb20b4_373e_5327_9942_66a1ea21f5f9
);
impl windows_core::RuntimeType for ICommandBarElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ICommandBarElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ICommandBarElement {
    pub fn IsCompact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCompact)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsCompact(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsCompact)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsInOverflow(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInOverflow)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn DynamicOverflowOrder(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DynamicOverflowOrder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDynamicOverflowOrder(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDynamicOverflowOrder)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for ICommandBarElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ICommandBarElement";
}
pub trait ICommandBarElement_Impl: windows_core::IUnknownImpl {
    fn IsCompact(&self) -> windows_core::Result<bool>;
    fn SetIsCompact(&self, value: bool) -> windows_core::Result<()>;
    fn IsInOverflow(&self) -> windows_core::Result<bool>;
    fn DynamicOverflowOrder(&self) -> windows_core::Result<i32>;
    fn SetDynamicOverflowOrder(&self, value: i32) -> windows_core::Result<()>;
}
impl ICommandBarElement_Vtbl {
    pub const fn new<Identity: ICommandBarElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsCompact<
            Identity: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICommandBarElement_Impl::IsCompact(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsCompact<
            Identity: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandBarElement_Impl::SetIsCompact(this, value).into()
            }
        }
        unsafe extern "system" fn IsInOverflow<
            Identity: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICommandBarElement_Impl::IsInOverflow(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DynamicOverflowOrder<
            Identity: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICommandBarElement_Impl::DynamicOverflowOrder(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDynamicOverflowOrder<
            Identity: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommandBarElement_Impl::SetDynamicOverflowOrder(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommandBarElement, OFFSET>(),
            IsCompact: IsCompact::<Identity, OFFSET>,
            SetIsCompact: SetIsCompact::<Identity, OFFSET>,
            IsInOverflow: IsInOverflow::<Identity, OFFSET>,
            DynamicOverflowOrder: DynamicOverflowOrder::<Identity, OFFSET>,
            SetDynamicOverflowOrder: SetDynamicOverflowOrder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandBarElement as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ICommandBarElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCompact:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCompact:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsInOverflow:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DynamicOverflowOrder:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDynamicOverflowOrder:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICommandBarFlyout,
    ICommandBarFlyout_Vtbl,
    0x18e2cc40_09cb_5f20_a715_f0b2039c5e18
);
impl windows_core::RuntimeType for ICommandBarFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICommandBarFlyout {
    pub fn PrimaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrimaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SecondaryCommands(
        &self,
    ) -> windows_core::Result<windows_collections::IObservableVector<ICommandBarElement>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecondaryCommands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ICommandBarFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrimaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SecondaryCommands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICommandBarFlyout2,
    ICommandBarFlyout2_Vtbl,
    0x5f81ec9e_a9d2_5f04_b5b1_013daef026dd
);
impl windows_core::RuntimeType for ICommandBarFlyout2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICommandBarFlyout2 {
    pub fn AlwaysExpanded(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AlwaysExpanded)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAlwaysExpanded(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAlwaysExpanded)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICommandBarFlyout2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlwaysExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAlwaysExpanded:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICommandBarFlyoutFactory,
    ICommandBarFlyoutFactory_Vtbl,
    0xa194dbe6_4311_5bd2_a8eb_b49c4733a33c
);
impl windows_core::RuntimeType for ICommandBarFlyoutFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommandBarFlyoutFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentControl,
    IContentControl_Vtbl,
    0x07e81761_11b2_52ae_8f8b_4d53d2b5900a
);
impl windows_core::RuntimeType for IContentControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentControl {
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentTemplateRoot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IContentControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ContentTemplate: usize,
    SetContentTemplate: usize,
    ContentTemplateSelector: usize,
    SetContentTemplateSelector: usize,
    ContentTransitions: usize,
    SetContentTransitions: usize,
    pub ContentTemplateRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentControlFactory,
    IContentControlFactory_Vtbl,
    0x3dea958e_5acd_5f80_8938_38634f51493a
);
impl windows_core::RuntimeType for IContentControlFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentControlFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentControlOverrides,
    IContentControlOverrides_Vtbl,
    0x2504174a_017e_5a2d_9c28_d97c66ae9937
);
impl windows_core::RuntimeType for IContentControlOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentControlOverrides {
    pub fn OnContentChanged<P0, P1>(
        &self,
        oldcontent: P0,
        newcontent: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnContentChanged)(
                windows_core::Interface::as_raw(self),
                oldcontent.param().abi(),
                newcontent.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IContentControlOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnContentChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentControlStatics,
    IContentControlStatics_Vtbl,
    0xf25484f4_2fed_5a0a_8864_7d6d4ac43ef8
);
impl windows_core::RuntimeType for IContentControlStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentControlStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IContentIslandEnvironment,
    IContentIslandEnvironment_Vtbl,
    0xc334236d_da88_566d_811d_74aef2eba978
);
impl windows_core::RuntimeType for IContentIslandEnvironment {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentIslandEnvironment {
    pub fn AppWindowId(&self) -> windows_core::Result<WindowId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppWindowId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IContentIslandEnvironment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppWindowId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentIslandEnvironment2,
    IContentIslandEnvironment2_Vtbl,
    0x6bf81a71_c1e4_54d6_ac0d_02bcff5297e7
);
impl windows_core::RuntimeType for IContentIslandEnvironment2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IContentIslandEnvironment2 {
    pub fn DisplayScale(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayScale)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IContentIslandEnvironment2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayScale:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContentIslandEnvironmentFactory,
    IContentIslandEnvironmentFactory_Vtbl,
    0x47a782d6_b177_5c1e_bf87_90437dd809d0
);
impl windows_core::RuntimeType for IContentIslandEnvironmentFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentIslandEnvironmentFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IControl,
    IControl_Vtbl,
    0x857d6e8a_d45a_5c69_a99c_bf6a5c54fb38
);
impl windows_core::RuntimeType for IControl {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IControl {
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> windows_core::Result<FontWeight> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontWeight)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsFocusEngagementEnabled: usize,
    SetIsFocusEngagementEnabled: usize,
    IsFocusEngaged: usize,
    SetIsFocusEngaged: usize,
    RequiresPointer: usize,
    SetRequiresPointer: usize,
    pub FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetFontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    FontFamily: usize,
    SetFontFamily: usize,
    pub FontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub SetFontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlFactory,
    IControlFactory_Vtbl,
    0x25159233_9438_5534_aeb9_00eb059cf73f
);
impl windows_core::RuntimeType for IControlFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IControlFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlOverrides,
    IControlOverrides_Vtbl,
    0x5f644395_2177_5161_9f87_c6c749c73a03
);
impl windows_core::RuntimeType for IControlOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IControlOverrides {
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPointerEntered)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPointerPressed)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPointerMoved)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPointerReleased)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPointerExited)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPointerCaptureLost)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPointerCanceled)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPointerWheelChanged)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<KeyRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnKeyUp)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<KeyRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnKeyDown)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<KeyRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPreviewKeyDown)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<KeyRoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnPreviewKeyUp)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnGotFocus)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnLostFocus)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IControlOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnPointerEntered: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerPressed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerMoved: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerReleased: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerExited: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerCaptureLost: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerCanceled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPointerWheelChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    OnTapped: usize,
    OnDoubleTapped: usize,
    OnHolding: usize,
    OnRightTapped: usize,
    OnManipulationStarting: usize,
    OnManipulationInertiaStarting: usize,
    OnManipulationStarted: usize,
    OnManipulationDelta: usize,
    OnManipulationCompleted: usize,
    pub OnKeyUp: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnKeyDown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPreviewKeyDown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnPreviewKeyUp: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnGotFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OnLostFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlProtected,
    IControlProtected_Vtbl,
    0x5c750339_1789_5095_a214_dd9d0564e579
);
impl windows_core::RuntimeType for IControlProtected {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IControlProtected {
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultStyleKey)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultStyleKey)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetTemplateChild(&self, childname: &str) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTemplateChild)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(childname)),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IControlProtected_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DefaultStyleKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetDefaultStyleKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetTemplateChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IControlStatics,
    IControlStatics_Vtbl,
    0xc3ae388d_aa36_5e10_acac_98415f47bcc7
);
impl windows_core::RuntimeType for IControlStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IControlStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsFocusEngagementEnabledProperty: usize,
    IsFocusEngagedProperty: usize,
    RequiresPointerProperty: usize,
    FontSizeProperty: usize,
    FontFamilyProperty: usize,
    FontWeightProperty: usize,
    FontStyleProperty: usize,
    FontStretchProperty: usize,
    CharacterSpacingProperty: usize,
    ForegroundProperty: usize,
    IsTextScaleFactorEnabledProperty: usize,
    IsEnabledProperty: usize,
    TabNavigationProperty: usize,
    TemplateProperty: usize,
    PaddingProperty: usize,
    HorizontalContentAlignmentProperty: usize,
    VerticalContentAlignmentProperty: usize,
    BackgroundProperty: usize,
    BackgroundSizingProperty: usize,
    BorderThicknessProperty: usize,
    BorderBrushProperty: usize,
    DefaultStyleKeyProperty: usize,
    DefaultStyleResourceUriProperty: usize,
    ElementSoundModeProperty: usize,
    CornerRadiusProperty: usize,
    IsTemplateFocusTargetProperty: usize,
    pub GetIsTemplateFocusTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub SetIsTemplateFocusTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
    IsTemplateKeyTipTargetProperty: usize,
    pub GetIsTemplateKeyTipTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub SetIsTemplateKeyTipTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDependencyObject,
    IDependencyObject_Vtbl,
    0xe7beaee7_160e_50f7_8789_d63463f979fa
);
impl windows_core::RuntimeType for IDependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IDependencyObject {
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
pub struct IDependencyObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetValue: usize,
    SetValue: usize,
    ClearValue: usize,
    ReadLocalValue: usize,
    GetAnimationBaseValue: usize,
    RegisterPropertyChangedCallback: usize,
    UnregisterPropertyChangedCallback: usize,
    Dispatcher: usize,
    pub DispatcherQueue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDependencyObjectFactory,
    IDependencyObjectFactory_Vtbl,
    0x936b614c_475f_5d7d_b3f7_bf1fbea28126
);
impl windows_core::RuntimeType for IDependencyObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDependencyObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    CreateTimer: usize,
    TryEnqueue: usize,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DispatcherQueuePriority,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
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
    pub fn FrameworkShutdownStarting<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<DispatcherQueue>,
                windows_core::Ref<DispatcherQueueShutdownStartingEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs> = {
            let com =
                windows_core::imp::DelegateBox::<
                    TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>,
                    F,
                >::new(
                    &TypedEventHandlerBox::<
                        DispatcherQueue,
                        DispatcherQueueShutdownStartingEventArgs,
                        F,
                    >::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).FrameworkShutdownStarting)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveFrameworkShutdownStarting,
            ))
        }
    }
    pub fn FrameworkShutdownCompleted<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<DispatcherQueue>, windows_core::Ref<windows_core::IInspectable>)
            + 'static,
    {
        let handler: TypedEventHandler<DispatcherQueue, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<DispatcherQueue, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<DispatcherQueue, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).FrameworkShutdownCompleted)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveFrameworkShutdownCompleted,
            ))
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
    RunEventLoopWithOptions: usize,
    pub FrameworkShutdownStarting: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveFrameworkShutdownStarting:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FrameworkShutdownCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveFrameworkShutdownCompleted:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
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
    IEnumSpellingError,
    IEnumSpellingError_Vtbl,
    0x803e3bd4_2828_4410_8290_418d1d73c762
);
windows_core::imp::interface_hierarchy!(IEnumSpellingError, windows_core::IUnknown);
impl IEnumSpellingError {
    pub unsafe fn Next(&self) -> windows_core::Result<ISpellingError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IEnumSpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IEnumSpellingError_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<ISpellingError>;
}
impl IEnumSpellingError_Vtbl {
    pub const fn new<Identity: IEnumSpellingError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSpellingError_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSpellingError_Impl::Next(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSpellingError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSpellingError {}
windows_core::imp::define_interface!(
    IEnumString,
    IEnumString_Vtbl,
    0x00000101_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IEnumString, windows_core::IUnknown);
impl IEnumString {
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut windows_core::PWSTR,
        pceltfetched: Option<*mut u32>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                rgelt as _,
                pceltfetched.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IEnumString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::PWSTR,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IEnumString_Impl: windows_core::IUnknownImpl {
    fn Next(
        &self,
        celt: u32,
        rgelt: *mut windows_core::PWSTR,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumString>;
}
impl IEnumString_Vtbl {
    pub const fn new<Identity: IEnumString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumString_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut windows_core::PWSTR,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumString_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumString_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumString_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumString_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumString {}
windows_core::imp::define_interface!(
    IFlyoutBase,
    IFlyoutBase_Vtbl,
    0xbb6603bf_744d_5c31_a87d_744394634d77
);
impl windows_core::RuntimeType for IFlyoutBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFlyoutBase {
    pub fn Target(&self) -> windows_core::Result<FrameworkElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Target)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Opened<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + 'static,
    {
        let handler: EventHandler<windows_core::IInspectable> = {
            let com =
                windows_core::imp::DelegateBox::<EventHandler<windows_core::IInspectable>, F>::new(
                    &EventHandlerBox::<windows_core::IInspectable, F>::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Opened)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveOpened,
            ))
        }
    }
    pub fn Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + 'static,
    {
        let handler: EventHandler<windows_core::IInspectable> = {
            let com =
                windows_core::imp::DelegateBox::<EventHandler<windows_core::IInspectable>, F>::new(
                    &EventHandlerBox::<windows_core::IInspectable, F>::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Closed)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClosed,
            ))
        }
    }
    pub fn Opening<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<windows_core::IInspectable>,
            ) + 'static,
    {
        let handler: EventHandler<windows_core::IInspectable> = {
            let com =
                windows_core::imp::DelegateBox::<EventHandler<windows_core::IInspectable>, F>::new(
                    &EventHandlerBox::<windows_core::IInspectable, F>::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Opening)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveOpening,
            ))
        }
    }
    pub fn Hide(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Hide)(windows_core::Interface::as_raw(self)).ok()
        }
    }
}
#[repr(C)]
pub struct IFlyoutBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Placement: usize,
    SetPlacement: usize,
    pub Target: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    AllowFocusOnInteraction: usize,
    SetAllowFocusOnInteraction: usize,
    LightDismissOverlayMode: usize,
    SetLightDismissOverlayMode: usize,
    AllowFocusWhenDisabled: usize,
    SetAllowFocusWhenDisabled: usize,
    ShowMode: usize,
    SetShowMode: usize,
    InputDevicePrefersPrimaryCommands: usize,
    AreOpenCloseAnimationsEnabled: usize,
    SetAreOpenCloseAnimationsEnabled: usize,
    ShouldConstrainToRootBounds: usize,
    SetShouldConstrainToRootBounds: usize,
    IsConstrainedToRootBounds: usize,
    ElementSoundMode: usize,
    SetElementSoundMode: usize,
    OverlayInputPassThroughElement: usize,
    SetOverlayInputPassThroughElement: usize,
    IsOpen: usize,
    XamlRoot: usize,
    SetXamlRoot: usize,
    pub Opened: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveOpened:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClosed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Opening: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveOpening:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Closing: usize,
    RemoveClosing: usize,
    ShowAt: usize,
    ShowAtWithOptions: usize,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFlyoutBase2,
    IFlyoutBase2_Vtbl,
    0x54592f97_e40e_5cad_864b_32307d047020
);
impl windows_core::RuntimeType for IFlyoutBase2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlyoutBase2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IFlyoutBaseFactory,
    IFlyoutBaseFactory_Vtbl,
    0x006d738f_7c91_5ef3_8a80_a548108dab8b
);
impl windows_core::RuntimeType for IFlyoutBaseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlyoutBaseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFlyoutBaseOverrides,
    IFlyoutBaseOverrides_Vtbl,
    0x5bb19ed8_08de_5eec_91cb_5fc59974e894
);
impl windows_core::RuntimeType for IFlyoutBaseOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFlyoutBaseOverrides {
    pub fn CreatePresenter(&self) -> windows_core::Result<Control> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePresenter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IFlyoutBaseOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreatePresenter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFlyoutBaseStatics,
    IFlyoutBaseStatics_Vtbl,
    0x63ee628e_d2e3_5515_aea4_e461088c0c4e
);
impl windows_core::RuntimeType for IFlyoutBaseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlyoutBaseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    TargetProperty: usize,
    PlacementProperty: usize,
    AllowFocusOnInteractionProperty: usize,
    LightDismissOverlayModeProperty: usize,
    AllowFocusWhenDisabledProperty: usize,
    ShowModeProperty: usize,
    InputDevicePrefersPrimaryCommandsProperty: usize,
    AreOpenCloseAnimationsEnabledProperty: usize,
    ShouldConstrainToRootBoundsProperty: usize,
    ElementSoundModeProperty: usize,
    OverlayInputPassThroughElementProperty: usize,
    IsOpenProperty: usize,
    AttachedFlyoutProperty: usize,
    pub GetAttachedFlyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetAttachedFlyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ShowAttachedFlyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFlyoutBaseStatics2,
    IFlyoutBaseStatics2_Vtbl,
    0x2403cd2c_a6b8_5dc2_be3b_2a4bdd072ef1
);
impl windows_core::RuntimeType for IFlyoutBaseStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlyoutBaseStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IFontIcon,
    IFontIcon_Vtbl,
    0x6eba5ed9_d233_5f5e_91a8_f5134292658a
);
impl windows_core::RuntimeType for IFontIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFontIcon {
    pub fn Glyph(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Glyph)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetGlyph(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetGlyph)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFontIcon_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Glyph: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetGlyph: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFontIconFactory,
    IFontIconFactory_Vtbl,
    0xaa9a24fe_bef8_564a_b200_694cd6f6ba4e
);
impl windows_core::RuntimeType for IFontIconFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFontIconFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFontIconStatics,
    IFontIconStatics_Vtbl,
    0x55564030_2f80_5be5_8c2a_ebb6ecba07e8
);
impl windows_core::RuntimeType for IFontIconStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFontIconStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IFontWeights,
    IFontWeights_Vtbl,
    0x386cd040_5404_5a8d_8bc7_2ca989f5c065
);
impl windows_core::RuntimeType for IFontWeights {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFontWeights_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IFontWeightsStatics,
    IFontWeightsStatics_Vtbl,
    0xcc390df6_76b0_5807_8b9d_e949a4e623ae
);
impl windows_core::RuntimeType for IFontWeightsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFontWeightsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Black:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Bold:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub ExtraBlack:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub ExtraBold:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub ExtraLight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Light:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Medium:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Normal:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub SemiBold:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub SemiLight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Thin:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElement,
    IFrameworkElement_Vtbl,
    0xfe08f13d_dc6a_5495_ad44_c2d8d21863b0
);
impl windows_core::RuntimeType for IFrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFrameworkElement {
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActualHeight)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RequestedTheme(&self) -> windows_core::Result<ElementTheme> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedTheme)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRequestedTheme(&self, value: ElementTheme) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRequestedTheme)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn SizeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SizeChangedEventArgs>,
            ) + 'static,
    {
        let handler: SizeChangedEventHandler = {
            let com = windows_core::imp::DelegateBox::<SizeChangedEventHandler, F>::new(
                &SizeChangedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SizeChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSizeChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IFrameworkElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Triggers: usize,
    Resources: usize,
    SetResources: usize,
    Tag: usize,
    SetTag: usize,
    Language: usize,
    SetLanguage: usize,
    ActualWidth: usize,
    pub ActualHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    Width: usize,
    SetWidth: usize,
    Height: usize,
    SetHeight: usize,
    MinWidth: usize,
    SetMinWidth: usize,
    MaxWidth: usize,
    SetMaxWidth: usize,
    MinHeight: usize,
    SetMinHeight: usize,
    MaxHeight: usize,
    SetMaxHeight: usize,
    HorizontalAlignment: usize,
    SetHorizontalAlignment: usize,
    VerticalAlignment: usize,
    SetVerticalAlignment: usize,
    Margin: usize,
    SetMargin: usize,
    Name: usize,
    SetName: usize,
    BaseUri: usize,
    DataContext: usize,
    SetDataContext: usize,
    AllowFocusOnInteraction: usize,
    SetAllowFocusOnInteraction: usize,
    FocusVisualMargin: usize,
    SetFocusVisualMargin: usize,
    FocusVisualSecondaryThickness: usize,
    SetFocusVisualSecondaryThickness: usize,
    FocusVisualPrimaryThickness: usize,
    SetFocusVisualPrimaryThickness: usize,
    FocusVisualSecondaryBrush: usize,
    SetFocusVisualSecondaryBrush: usize,
    FocusVisualPrimaryBrush: usize,
    SetFocusVisualPrimaryBrush: usize,
    AllowFocusWhenDisabled: usize,
    SetAllowFocusWhenDisabled: usize,
    Style: usize,
    SetStyle: usize,
    Parent: usize,
    FlowDirection: usize,
    SetFlowDirection: usize,
    pub RequestedTheme: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ElementTheme,
    ) -> windows_core::HRESULT,
    pub SetRequestedTheme:
        unsafe extern "system" fn(*mut core::ffi::c_void, ElementTheme) -> windows_core::HRESULT,
    IsLoaded: usize,
    ActualTheme: usize,
    Loaded: usize,
    RemoveLoaded: usize,
    Unloaded: usize,
    RemoveUnloaded: usize,
    DataContextChanged: usize,
    RemoveDataContextChanged: usize,
    pub SizeChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSizeChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementFactory,
    IFrameworkElementFactory_Vtbl,
    0xbd3f2272_3efa_5f92_b759_90b1cc3e784c
);
impl windows_core::RuntimeType for IFrameworkElementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameworkElementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementOverrides,
    IFrameworkElementOverrides_Vtbl,
    0xffc6fd98_f38c_5904_9ce4_97a3427cf4ba
);
impl windows_core::RuntimeType for IFrameworkElementOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFrameworkElementOverrides {
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).OnApplyTemplate)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &str,
        usetransitions: bool,
    ) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GoToElementStateCore)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(statename)),
                usetransitions,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IFrameworkElementOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    MeasureOverride: usize,
    ArrangeOverride: usize,
    pub OnApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoToElementStateCore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        bool,
        *mut bool,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementProtected,
    IFrameworkElementProtected_Vtbl,
    0xe59a3db0_91e5_5903_9caf_d1bb9f458bf2
);
impl windows_core::RuntimeType for IFrameworkElementProtected {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IFrameworkElementProtected {
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).InvalidateViewport)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFrameworkElementProtected_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvalidateViewport:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFrameworkElementStatics,
    IFrameworkElementStatics_Vtbl,
    0x894e2704_14e7_569a_b21e_afc7df7145a1
);
impl windows_core::RuntimeType for IFrameworkElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameworkElementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    TagProperty: usize,
    LanguageProperty: usize,
    ActualWidthProperty: usize,
    ActualHeightProperty: usize,
    WidthProperty: usize,
    HeightProperty: usize,
    MinWidthProperty: usize,
    MaxWidthProperty: usize,
    MinHeightProperty: usize,
    MaxHeightProperty: usize,
    HorizontalAlignmentProperty: usize,
    VerticalAlignmentProperty: usize,
    MarginProperty: usize,
    NameProperty: usize,
    DataContextProperty: usize,
    AllowFocusOnInteractionProperty: usize,
    FocusVisualMarginProperty: usize,
    FocusVisualSecondaryThicknessProperty: usize,
    FocusVisualPrimaryThicknessProperty: usize,
    FocusVisualSecondaryBrushProperty: usize,
    FocusVisualPrimaryBrushProperty: usize,
    AllowFocusWhenDisabledProperty: usize,
    StyleProperty: usize,
    FlowDirectionProperty: usize,
    RequestedThemeProperty: usize,
    ActualThemeProperty: usize,
    pub DeferTree: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGeneralTransform,
    IGeneralTransform_Vtbl,
    0x04eedeeb_31e5_54c0_ae3f_8bd06645d339
);
impl windows_core::RuntimeType for IGeneralTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IGeneralTransform {
    pub fn TransformPoint(&self, point: Point) -> windows_core::Result<Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransformPoint)(
                windows_core::Interface::as_raw(self),
                point,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IGeneralTransform_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Inverse: usize,
    pub TransformPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Point,
        *mut Point,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGeneralTransformFactory,
    IGeneralTransformFactory_Vtbl,
    0x2f1025a3_5391_5d1b_8382_3caaa1d26a96
);
impl windows_core::RuntimeType for IGeneralTransformFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeneralTransformFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IGeneralTransformOverrides,
    IGeneralTransformOverrides_Vtbl,
    0xce8970f1_83f8_543f_9cf5_439c461601f1
);
impl windows_core::RuntimeType for IGeneralTransformOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IGeneralTransformOverrides {
    pub fn InverseCore(&self) -> windows_core::Result<GeneralTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InverseCore)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn TryTransformCore(
        &self,
        inpoint: Point,
        outpoint: &mut Point,
    ) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryTransformCore)(
                windows_core::Interface::as_raw(self),
                inpoint,
                outpoint,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TransformBoundsCore(&self, rect: Rect) -> windows_core::Result<Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransformBoundsCore)(
                windows_core::Interface::as_raw(self),
                rect,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IGeneralTransformOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InverseCore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TryTransformCore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Point,
        *mut Point,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub TransformBoundsCore:
        unsafe extern "system" fn(*mut core::ffi::c_void, Rect, *mut Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IIconElement,
    IIconElement_Vtbl,
    0x18f69350_279e_50ea_8d23_138e717ed939
);
impl windows_core::RuntimeType for IIconElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIconElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IIconElementFactory,
    IIconElementFactory_Vtbl,
    0x7161eeff_2cc3_59c9_b355_f752ef2030fb
);
impl windows_core::RuntimeType for IIconElementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIconElementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IIconElementStatics,
    IIconElementStatics_Vtbl,
    0x4ccfdc81_9d9d_5a02_b95b_ad1add921750
);
impl windows_core::RuntimeType for IIconElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIconElementStatics_Vtbl {
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
    IItemContainerMapping,
    IItemContainerMapping_Vtbl,
    0xb3e03a32_ea83_5626_bfda_d327a1c7550c
);
impl windows_core::RuntimeType for IItemContainerMapping {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IItemContainerMapping,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IItemContainerMapping {
    pub fn ContainerFromIndex(&self, index: i32) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContainerFromIndex)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IItemContainerMapping {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IItemContainerMapping";
}
#[repr(C)]
pub struct IItemContainerMapping_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ItemFromContainer: usize,
    ContainerFromItem: usize,
    IndexFromContainer: usize,
    pub ContainerFromIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IKeyRoutedEventArgs,
    IKeyRoutedEventArgs_Vtbl,
    0xee357007_a2d6_5c75_9431_05fd66ec7915
);
impl windows_core::RuntimeType for IKeyRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IKeyRoutedEventArgs {
    pub fn Key(&self) -> windows_core::Result<VirtualKey> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
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
}
#[repr(C)]
pub struct IKeyRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Key:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut VirtualKey) -> windows_core::HRESULT,
    KeyStatus: usize,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyout,
    IMenuFlyout_Vtbl,
    0xf4c77c6c_1fa5_5d85_8559_5d02b7d4e5e7
);
impl windows_core::RuntimeType for IMenuFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMenuFlyout {
    pub fn Items(&self) -> windows_core::Result<windows_collections::IVector<MenuFlyoutItemBase>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MenuFlyoutPresenterStyle(&self) -> windows_core::Result<Style> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuFlyoutPresenterStyle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetMenuFlyoutPresenterStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Style>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetMenuFlyoutPresenterStyle)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IMenuFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Items: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MenuFlyoutPresenterStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMenuFlyoutPresenterStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyoutFactory,
    IMenuFlyoutFactory_Vtbl,
    0xa3d225de_6b35_5442_b6c9_06fd24139a63
);
impl windows_core::RuntimeType for IMenuFlyoutFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMenuFlyoutFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyoutItem,
    IMenuFlyoutItem_Vtbl,
    0x4252df5a_44f9_5ee8_b1cc_53de9aaa4095
);
impl windows_core::RuntimeType for IMenuFlyoutItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IMenuFlyoutItem {
    pub fn Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClick,
            ))
        }
    }
}
#[repr(C)]
pub struct IMenuFlyoutItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Command: usize,
    SetCommand: usize,
    CommandParameter: usize,
    SetCommandParameter: usize,
    Icon: usize,
    SetIcon: usize,
    KeyboardAcceleratorTextOverride: usize,
    SetKeyboardAcceleratorTextOverride: usize,
    TemplateSettings: usize,
    pub Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyoutItemBase,
    IMenuFlyoutItemBase_Vtbl,
    0x4bee2715_44a1_5f94_86e8_02ddbe3dc6b9
);
impl windows_core::RuntimeType for IMenuFlyoutItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMenuFlyoutItemBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IMenuFlyoutItemBaseFactory,
    IMenuFlyoutItemBaseFactory_Vtbl,
    0x33f4ead3_ab54_505d_bc5f_45e7c35f20f8
);
impl windows_core::RuntimeType for IMenuFlyoutItemBaseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMenuFlyoutItemBaseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IMenuFlyoutItemFactory,
    IMenuFlyoutItemFactory_Vtbl,
    0x9c3c9a1f_89af_521a_81a5_8a01db7a79af
);
impl windows_core::RuntimeType for IMenuFlyoutItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMenuFlyoutItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMenuFlyoutItemStatics,
    IMenuFlyoutItemStatics_Vtbl,
    0x9d7953d6_e6a6_5154_91f4_67f4d6042a4f
);
impl windows_core::RuntimeType for IMenuFlyoutItemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMenuFlyoutItemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IMenuFlyoutStatics,
    IMenuFlyoutStatics_Vtbl,
    0x6959e683_16f6_5208_84e0_b0be92d9f6dd
);
impl windows_core::RuntimeType for IMenuFlyoutStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMenuFlyoutStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
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
}
#[repr(C)]
pub struct IPointerPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FrameId: usize,
    IsInContact: usize,
    PointerDeviceType: usize,
    PointerId: usize,
    Position: usize,
    pub Properties: unsafe extern "system" fn(
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
}
#[repr(C)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ContactRect: usize,
    IsBarrelButtonPressed: usize,
    IsCanceled: usize,
    IsEraser: usize,
    IsHorizontalMouseWheel: usize,
    IsInRange: usize,
    IsInverted: usize,
    IsLeftButtonPressed: usize,
    IsMiddleButtonPressed: usize,
    IsPrimary: usize,
    IsRightButtonPressed: usize,
    IsXButton1Pressed: usize,
    IsXButton2Pressed: usize,
    pub MouseWheelDelta:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IPointerRoutedEventArgs,
    IPointerRoutedEventArgs_Vtbl,
    0x66e78a9a_1bec_5f92_b1a1_ea6334ee511c
);
impl windows_core::RuntimeType for IPointerRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IPointerRoutedEventArgs {
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
    pub fn GetCurrentPoint<P0>(&self, relativeto: P0) -> windows_core::Result<PointerPoint>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPoint)(
                windows_core::Interface::as_raw(self),
                relativeto.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IPointerRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Pointer: usize,
    KeyModifiers: usize,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsGenerated: usize,
    pub GetCurrentPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRichEditBox,
    IRichEditBox_Vtbl,
    0x699163db_723d_5514_a8c9_2c64d99e1ea6
);
impl windows_core::RuntimeType for IRichEditBox {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRichEditBox {
    pub fn IsSpellCheckEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSpellCheckEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsSpellCheckEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsSpellCheckEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn DisabledFormattingAccelerators(
        &self,
    ) -> windows_core::Result<DisabledFormattingAccelerators> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisabledFormattingAccelerators)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDisabledFormattingAccelerators(
        &self,
        value: DisabledFormattingAccelerators,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDisabledFormattingAccelerators)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn TextDocument(&self) -> windows_core::Result<RichEditTextDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TextDocument)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SelectionFlyout(&self) -> windows_core::Result<FlyoutBase> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectionFlyout)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetSelectionFlyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FlyoutBase>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectionFlyout)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn TextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).TextChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveTextChanged,
            ))
        }
    }
    pub fn SelectionChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    {
        let handler: RoutedEventHandler = {
            let com = windows_core::imp::DelegateBox::<RoutedEventHandler, F>::new(
                &RoutedEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectionChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveSelectionChanged,
            ))
        }
    }
    pub fn Paste<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<TextControlPasteEventArgs>,
            ) + 'static,
    {
        let handler: TextControlPasteEventHandler = {
            let com = windows_core::imp::DelegateBox::<TextControlPasteEventHandler, F>::new(
                &TextControlPasteEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Paste)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePaste,
            ))
        }
    }
    pub fn ClipboardCopyFormat(&self) -> windows_core::Result<RichEditClipboardFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClipboardCopyFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetClipboardCopyFormat(
        &self,
        value: RichEditClipboardFormat,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetClipboardCopyFormat)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IRichEditBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsReadOnly: usize,
    SetIsReadOnly: usize,
    AcceptsReturn: usize,
    SetAcceptsReturn: usize,
    TextAlignment: usize,
    SetTextAlignment: usize,
    TextWrapping: usize,
    SetTextWrapping: usize,
    pub IsSpellCheckEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSpellCheckEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    IsTextPredictionEnabled: usize,
    SetIsTextPredictionEnabled: usize,
    Document: usize,
    InputScope: usize,
    SetInputScope: usize,
    Header: usize,
    SetHeader: usize,
    HeaderTemplate: usize,
    SetHeaderTemplate: usize,
    PlaceholderText: usize,
    SetPlaceholderText: usize,
    SelectionHighlightColor: usize,
    SetSelectionHighlightColor: usize,
    PreventKeyboardDisplayOnProgrammaticFocus: usize,
    SetPreventKeyboardDisplayOnProgrammaticFocus: usize,
    IsColorFontEnabled: usize,
    SetIsColorFontEnabled: usize,
    SelectionHighlightColorWhenNotFocused: usize,
    SetSelectionHighlightColorWhenNotFocused: usize,
    MaxLength: usize,
    SetMaxLength: usize,
    HorizontalTextAlignment: usize,
    SetHorizontalTextAlignment: usize,
    CharacterCasing: usize,
    SetCharacterCasing: usize,
    pub DisabledFormattingAccelerators: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DisabledFormattingAccelerators,
    ) -> windows_core::HRESULT,
    pub SetDisabledFormattingAccelerators: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DisabledFormattingAccelerators,
    ) -> windows_core::HRESULT,
    pub TextDocument: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SelectionFlyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetSelectionFlyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ProofingMenuFlyout: usize,
    Description: usize,
    SetDescription: usize,
    pub TextChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveTextChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SelectionChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveSelectionChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    ContextMenuOpening: usize,
    RemoveContextMenuOpening: usize,
    pub Paste: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePaste:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    TextCompositionStarted: usize,
    RemoveTextCompositionStarted: usize,
    TextCompositionChanged: usize,
    RemoveTextCompositionChanged: usize,
    TextCompositionEnded: usize,
    RemoveTextCompositionEnded: usize,
    CopyingToClipboard: usize,
    RemoveCopyingToClipboard: usize,
    CuttingToClipboard: usize,
    RemoveCuttingToClipboard: usize,
    SelectionChanging: usize,
    RemoveSelectionChanging: usize,
    GetLinguisticAlternativesAsync: usize,
    TextReadingOrder: usize,
    SetTextReadingOrder: usize,
    pub ClipboardCopyFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut RichEditClipboardFormat,
    ) -> windows_core::HRESULT,
    pub SetClipboardCopyFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        RichEditClipboardFormat,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRoutedEventArgs,
    IRoutedEventArgs_Vtbl,
    0x0908c407_1c7d_5de3_9c50_d971c62ec8ec
);
impl windows_core::RuntimeType for IRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IRoutedEventArgs {
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OriginalSource)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OriginalSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IRoutedEventArgsFactory,
    IRoutedEventArgsFactory_Vtbl,
    0x914b02c7_076b_5b89_98e7_6c373379e9af
);
impl windows_core::RuntimeType for IRoutedEventArgsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRoutedEventArgsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IScrollViewerStatics,
    IScrollViewerStatics_Vtbl,
    0xd971fd86_0a96_50c4_a6e1_9975faa2a142
);
impl windows_core::RuntimeType for IScrollViewerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScrollViewerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    HorizontalSnapPointsAlignmentProperty: usize,
    VerticalSnapPointsAlignmentProperty: usize,
    HorizontalSnapPointsTypeProperty: usize,
    VerticalSnapPointsTypeProperty: usize,
    ZoomSnapPointsTypeProperty: usize,
    HorizontalOffsetProperty: usize,
    ViewportWidthProperty: usize,
    ScrollableWidthProperty: usize,
    ComputedHorizontalScrollBarVisibilityProperty: usize,
    ExtentWidthProperty: usize,
    VerticalOffsetProperty: usize,
    ViewportHeightProperty: usize,
    ScrollableHeightProperty: usize,
    ComputedVerticalScrollBarVisibilityProperty: usize,
    ExtentHeightProperty: usize,
    MinZoomFactorProperty: usize,
    MaxZoomFactorProperty: usize,
    ZoomFactorProperty: usize,
    ZoomSnapPointsProperty: usize,
    TopLeftHeaderProperty: usize,
    LeftHeaderProperty: usize,
    TopHeaderProperty: usize,
    ReduceViewportForCoreInputViewOcclusionsProperty: usize,
    HorizontalAnchorRatioProperty: usize,
    VerticalAnchorRatioProperty: usize,
    HorizontalScrollBarVisibilityProperty: usize,
    GetHorizontalScrollBarVisibility: usize,
    SetHorizontalScrollBarVisibility: usize,
    VerticalScrollBarVisibilityProperty: usize,
    GetVerticalScrollBarVisibility: usize,
    pub SetVerticalScrollBarVisibility: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        ScrollBarVisibility,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISizeChangedEventArgs,
    ISizeChangedEventArgs_Vtbl,
    0xfe76324e_6dfb_58b1_9dcd_886ca8f9a2ea
);
impl windows_core::RuntimeType for ISizeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISizeChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISpellChecker,
    ISpellChecker_Vtbl,
    0xb6fd0b71_e2bc_4653_8d05_f197e412770b
);
windows_core::imp::interface_hierarchy!(ISpellChecker, windows_core::IUnknown);
impl ISpellChecker {
    pub unsafe fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LanguageTag)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Check<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Check)(
                windows_core::Interface::as_raw(self),
                text.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Suggest<P0>(&self, word: P0) -> windows_core::Result<IEnumString>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suggest)(
                windows_core::Interface::as_raw(self),
                word.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, word: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Add)(
                windows_core::Interface::as_raw(self),
                word.param().abi(),
            )
        }
    }
    pub unsafe fn Ignore<P0>(&self, word: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Ignore)(
                windows_core::Interface::as_raw(self),
                word.param().abi(),
            )
        }
    }
    pub unsafe fn AutoCorrect<P0, P1>(&self, from: P0, to: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AutoCorrect)(
                windows_core::Interface::as_raw(self),
                from.param().abi(),
                to.param().abi(),
            )
        }
    }
    pub unsafe fn GetOptionValue<P0>(&self, optionid: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionValue)(
                windows_core::Interface::as_raw(self),
                optionid.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn OptionIds(&self) -> windows_core::Result<IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OptionIds)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalizedName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn remove_SpellCheckerChanged(&self, eventcookie: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).remove_SpellCheckerChanged)(
                windows_core::Interface::as_raw(self),
                eventcookie,
            )
        }
    }
    pub unsafe fn ComprehensiveCheck<P0>(
        &self,
        text: P0,
    ) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComprehensiveCheck)(
                windows_core::Interface::as_raw(self),
                text.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ISpellChecker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LanguageTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    pub Check: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Suggest: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub Ignore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub AutoCorrect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub GetOptionValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub OptionIds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    pub LocalizedName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    add_SpellCheckerChanged: usize,
    pub remove_SpellCheckerChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    GetOptionDescription: usize,
    pub ComprehensiveCheck: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for ISpellChecker {}
windows_core::imp::define_interface!(
    ISpellCheckerFactory,
    ISpellCheckerFactory_Vtbl,
    0x8e018a9d_2415_4677_bf08_794ea61f94bb
);
windows_core::imp::interface_hierarchy!(ISpellCheckerFactory, windows_core::IUnknown);
impl ISpellCheckerFactory {
    pub unsafe fn SupportedLanguages(&self) -> windows_core::Result<IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedLanguages)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsSupported<P0>(
        &self,
        languagetag: P0,
    ) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSupported)(
                windows_core::Interface::as_raw(self),
                languagetag.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CreateSpellChecker<P0>(
        &self,
        languagetag: P0,
    ) -> windows_core::Result<ISpellChecker>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSpellChecker)(
                windows_core::Interface::as_raw(self),
                languagetag.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ISpellCheckerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SupportedLanguages: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CreateSpellChecker: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ISpellCheckerFactory_Impl: windows_core::IUnknownImpl {
    fn SupportedLanguages(&self) -> windows_core::Result<IEnumString>;
    fn IsSupported(
        &self,
        languagetag: &windows_core::PCWSTR,
    ) -> windows_core::Result<windows_core::BOOL>;
    fn CreateSpellChecker(
        &self,
        languagetag: &windows_core::PCWSTR,
    ) -> windows_core::Result<ISpellChecker>;
}
impl ISpellCheckerFactory_Vtbl {
    pub const fn new<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportedLanguages<
            Identity: ISpellCheckerFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::SupportedLanguages(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSupported<
            Identity: ISpellCheckerFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            languagetag: windows_core::PCWSTR,
            value: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::IsSupported(
                    this,
                    core::mem::transmute(&languagetag),
                ) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSpellChecker<
            Identity: ISpellCheckerFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            languagetag: windows_core::PCWSTR,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::CreateSpellChecker(
                    this,
                    core::mem::transmute(&languagetag),
                ) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupportedLanguages: SupportedLanguages::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
            CreateSpellChecker: CreateSpellChecker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckerFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpellCheckerFactory {}
windows_core::imp::define_interface!(
    ISpellingError,
    ISpellingError_Vtbl,
    0xb7c82d61_fbe8_4b47_9b27_6c0d2e0de0a3
);
windows_core::imp::interface_hierarchy!(ISpellingError, windows_core::IUnknown);
impl ISpellingError {
    pub unsafe fn StartIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrectiveAction)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Replacement(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Replacement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ISpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CorrectiveAction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut CORRECTIVE_ACTION,
    ) -> windows_core::HRESULT,
    pub Replacement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
}
pub trait ISpellingError_Impl: windows_core::IUnknownImpl {
    fn StartIndex(&self) -> windows_core::Result<u32>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION>;
    fn Replacement(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ISpellingError_Vtbl {
    pub const fn new<Identity: ISpellingError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartIndex<Identity: ISpellingError_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::StartIndex(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Length<Identity: ISpellingError_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::Length(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrectiveAction<
            Identity: ISpellingError_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut CORRECTIVE_ACTION,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::CorrectiveAction(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Replacement<
            Identity: ISpellingError_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: *mut windows_core::PWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::Replacement(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartIndex: StartIndex::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            CorrectiveAction: CorrectiveAction::<Identity, OFFSET>,
            Replacement: Replacement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellingError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpellingError {}
windows_core::imp::define_interface!(IStyle, IStyle_Vtbl, 0x65e1d164_572f_5b0e_a80f_9c02441fac49);
impl windows_core::RuntimeType for IStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IStyle {
    pub fn IsSealed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSealed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn BasedOn(&self) -> windows_core::Result<Style> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BasedOn)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetBasedOn<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Style>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBasedOn)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn Seal(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Seal)(windows_core::Interface::as_raw(self)).ok()
        }
    }
}
#[repr(C)]
pub struct IStyle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSealed:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    Setters: usize,
    TargetType: usize,
    SetTargetType: usize,
    pub BasedOn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetBasedOn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Seal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStyleFactory,
    IStyleFactory_Vtbl,
    0xc2d924a2_3862_517c_b083_9a9120d7302d
);
impl windows_core::RuntimeType for IStyleFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStyleFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ITextBlock,
    ITextBlock_Vtbl,
    0x1ac8d84f_392c_5c7e_83f5_a53e3bf0abb0
);
impl windows_core::RuntimeType for ITextBlock {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextBlock {
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> windows_core::Result<FontWeight> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FontWeight)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: FontWeight) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITextBlock_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetFontSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    FontFamily: usize,
    SetFontFamily: usize,
    pub FontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub SetFontWeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FontWeight) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextCharacterFormat,
    ITextCharacterFormat_Vtbl,
    0xf5710050_98e5_5788_b1e3_32191eebf94d
);
impl windows_core::RuntimeType for ITextCharacterFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ITextCharacterFormat,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ITextCharacterFormat {
    pub fn AllCaps(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllCaps)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAllCaps(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAllCaps)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn BackgroundColor(&self) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackgroundColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Bold(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Bold)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBold(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBold)(
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
            .map(|| result__)
        }
    }
    pub fn SetForegroundColor(&self, value: Color) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetForegroundColor)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Hidden(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hidden)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHidden(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetHidden)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Italic(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Italic)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetItalic(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetItalic)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Kerning(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Kerning)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKerning(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetKerning)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn LanguageTag(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LanguageTag)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetLanguageTag(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLanguageTag)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn LinkType(&self) -> windows_core::Result<LinkType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LinkType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetName(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn Outline(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Outline)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOutline(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOutline)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Position(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPosition)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ProtectedText(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProtectedText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetProtectedText(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetProtectedText)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Size(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSize(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSize)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn SmallCaps(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmallCaps)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSmallCaps(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSmallCaps)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Spacing(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Spacing)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSpacing(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Strikethrough(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Strikethrough)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetStrikethrough(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStrikethrough)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Subscript(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subscript)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSubscript(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSubscript)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Superscript(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Superscript)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSuperscript(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSuperscript)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn TextScript(&self) -> windows_core::Result<TextScript> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TextScript)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetTextScript(&self, value: TextScript) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextScript)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Underline(&self) -> windows_core::Result<UnderlineType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Underline)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUnderline(&self, value: UnderlineType) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnderline)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Weight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Weight)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWeight(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetWeight)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn SetClone<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetClone)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn GetClone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsEqual<P0>(&self, format: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(
                windows_core::Interface::as_raw(self),
                format.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ITextCharacterFormat {
    const NAME: &'static str = "Microsoft.UI.Text.ITextCharacterFormat";
}
#[repr(C)]
pub struct ITextCharacterFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllCaps: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetAllCaps:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub BackgroundColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SetBackgroundColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
    pub Bold: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetBold:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    FontStretch: usize,
    SetFontStretch: usize,
    FontStyle: usize,
    SetFontStyle: usize,
    pub ForegroundColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SetForegroundColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, Color) -> windows_core::HRESULT,
    pub Hidden: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetHidden:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Italic: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetItalic:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Kerning:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub LanguageTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetLanguageTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LinkType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut LinkType) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Outline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetOutline:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Position:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPosition:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ProtectedText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetProtectedText:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SmallCaps: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetSmallCaps:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Spacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetStrikethrough:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Subscript: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetSubscript:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Superscript: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetSuperscript:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub TextScript:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextScript) -> windows_core::HRESULT,
    pub SetTextScript:
        unsafe extern "system" fn(*mut core::ffi::c_void, TextScript) -> windows_core::HRESULT,
    pub Underline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut UnderlineType,
    ) -> windows_core::HRESULT,
    pub SetUnderline:
        unsafe extern "system" fn(*mut core::ffi::c_void, UnderlineType) -> windows_core::HRESULT,
    pub Weight:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetClone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetClone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextConstantsStatics,
    ITextConstantsStatics_Vtbl,
    0xcd353b3c_af63_5cfb_918c_0f9c8931a161
);
impl windows_core::RuntimeType for ITextConstantsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextConstantsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MinUnitCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxUnitCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UndefinedColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub UndefinedFloatValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub UndefinedInt32Value:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextControlPasteEventArgs,
    ITextControlPasteEventArgs_Vtbl,
    0xfc5e567e_3aa9_5c57_88d4_834ad3b62e9e
);
impl windows_core::RuntimeType for ITextControlPasteEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextControlPasteEventArgs {
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
pub struct ITextControlPasteEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextDocument,
    ITextDocument_Vtbl,
    0x1149d57d_86a6_59dd_88d9_196f27bc5c85
);
impl windows_core::RuntimeType for ITextDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextDocument {
    pub fn CaretType(&self) -> windows_core::Result<CaretType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaretType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCaretType(&self, value: CaretType) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCaretType)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn DefaultTabStop(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultTabStop)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultTabStop)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Selection(&self) -> windows_core::Result<ITextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Selection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn UndoLimit(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UndoLimit)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetUndoLimit(&self, value: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUndoLimit)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CanCopy(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanCopy)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanPaste(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPaste)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanRedo(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRedo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CanUndo(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanUndo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ApplyDisplayUpdates(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplyDisplayUpdates)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn BatchDisplayUpdates(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BatchDisplayUpdates)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn BeginUndoGroup(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).BeginUndoGroup)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub fn EndUndoGroup(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).EndUndoGroup)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub fn GetDefaultCharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultCharacterFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultParagraphFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetRange(
        &self,
        startposition: i32,
        endposition: i32,
    ) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRange)(
                windows_core::Interface::as_raw(self),
                startposition,
                endposition,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetRangeFromPoint(
        &self,
        point: Point,
        options: PointOptions,
    ) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRangeFromPoint)(
                windows_core::Interface::as_raw(self),
                point,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetText(
        &self,
        options: TextGetOptions,
        value: &mut windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetText)(
                windows_core::Interface::as_raw(self),
                options,
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn Redo(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Redo)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub fn SetDefaultCharacterFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultCharacterFormat)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn SetDefaultParagraphFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultParagraphFormat)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn SetText(&self, options: TextSetOptions, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                options,
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn Undo(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Undo)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub fn AlignmentIncludesTrailingWhitespace(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AlignmentIncludesTrailingWhitespace)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAlignmentIncludesTrailingWhitespace(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAlignmentIncludesTrailingWhitespace)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IgnoreTrailingCharacterSpacing(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IgnoreTrailingCharacterSpacing)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIgnoreTrailingCharacterSpacing(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIgnoreTrailingCharacterSpacing)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ClearUndoRedoHistory(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearUndoRedoHistory)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITextDocument_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CaretType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut CaretType) -> windows_core::HRESULT,
    pub SetCaretType:
        unsafe extern "system" fn(*mut core::ffi::c_void, CaretType) -> windows_core::HRESULT,
    pub DefaultTabStop:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDefaultTabStop:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub Selection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub UndoLimit:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetUndoLimit:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CanCopy:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanPaste:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanRedo:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanUndo:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ApplyDisplayUpdates:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BatchDisplayUpdates:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BeginUndoGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndUndoGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultCharacterFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetDefaultParagraphFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetRangeFromPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Point,
        PointOptions,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextGetOptions,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    LoadFromStream: usize,
    pub Redo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    SaveToStream: usize,
    pub SetDefaultCharacterFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetDefaultParagraphFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextSetOptions,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Undo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AlignmentIncludesTrailingWhitespace:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAlignmentIncludesTrailingWhitespace:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IgnoreTrailingCharacterSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIgnoreTrailingCharacterSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ClearUndoRedoHistory:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextDocument2,
    ITextDocument2_Vtbl,
    0x21febcf1_2110_5879_b1da_b343097e71e1
);
impl windows_core::RuntimeType for ITextDocument2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ITextDocument2 {
    pub fn GetMathMode(&self) -> windows_core::Result<RichEditMathMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMathMode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMathMode(&self, mode: RichEditMathMode) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMathMode)(
                windows_core::Interface::as_raw(self),
                mode,
            )
            .ok()
        }
    }
    pub fn GetMathML(&self, value: &mut windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetMathML)(
                windows_core::Interface::as_raw(self),
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn SetMathML(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMathML)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ITextDocument2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetMathMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut RichEditMathMode,
    ) -> windows_core::HRESULT,
    pub SetMathMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        RichEditMathMode,
    ) -> windows_core::HRESULT,
    pub GetMathML: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMathML: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextParagraphFormat,
    ITextParagraphFormat_Vtbl,
    0x219b6cdf_0d0b_5701_b8a1_6c906b3ebbe1
);
impl windows_core::RuntimeType for ITextParagraphFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ITextParagraphFormat,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ITextParagraphFormat {
    pub fn Alignment(&self) -> windows_core::Result<ParagraphAlignment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Alignment)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAlignment(&self, value: ParagraphAlignment) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAlignment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn FirstLineIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirstLineIndent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn KeepTogether(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeepTogether)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeepTogether(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetKeepTogether)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn KeepWithNext(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeepWithNext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetKeepWithNext(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetKeepWithNext)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn LeftIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LeftIndent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LineSpacing(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LineSpacing)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LineSpacingRule(&self) -> windows_core::Result<LineSpacingRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LineSpacingRule)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ListAlignment(&self) -> windows_core::Result<MarkerAlignment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListAlignment)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetListAlignment(&self, value: MarkerAlignment) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetListAlignment)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ListLevelIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListLevelIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetListLevelIndex)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ListStart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListStart)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetListStart(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetListStart)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ListStyle(&self) -> windows_core::Result<MarkerStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListStyle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetListStyle(&self, value: MarkerStyle) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetListStyle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ListTab(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListTab)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetListTab(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetListTab)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn ListType(&self) -> windows_core::Result<MarkerType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetListType(&self, value: MarkerType) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetListType)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn NoLineNumber(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NoLineNumber)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetNoLineNumber(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetNoLineNumber)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn PageBreakBefore(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PageBreakBefore)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPageBreakBefore(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPageBreakBefore)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn RightIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RightIndent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRightIndent(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRightIndent)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn RightToLeft(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RightToLeft)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRightToLeft(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRightToLeft)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Style(&self) -> windows_core::Result<ParagraphStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Style)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetStyle(&self, value: ParagraphStyle) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStyle)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn SpaceAfter(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpaceAfter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSpaceAfter)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn SpaceBefore(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpaceBefore)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSpaceBefore)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn WidowControl(&self) -> windows_core::Result<FormatEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WidowControl)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetWidowControl(&self, value: FormatEffect) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetWidowControl)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn TabCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TabCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AddTab(
        &self,
        position: f32,
        align: TabAlignment,
        leader: TabLeader,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).AddTab)(
                windows_core::Interface::as_raw(self),
                position,
                align,
                leader,
            )
            .ok()
        }
    }
    pub fn ClearAllTabs(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearAllTabs)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub fn DeleteTab(&self, position: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteTab)(
                windows_core::Interface::as_raw(self),
                position,
            )
            .ok()
        }
    }
    pub fn GetClone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetTab(
        &self,
        index: i32,
        position: &mut f32,
        align: &mut TabAlignment,
        leader: &mut TabLeader,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetTab)(
                windows_core::Interface::as_raw(self),
                index,
                position,
                align,
                leader,
            )
            .ok()
        }
    }
    pub fn IsEqual<P0>(&self, format: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(
                windows_core::Interface::as_raw(self),
                format.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetClone<P0>(&self, format: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetClone)(
                windows_core::Interface::as_raw(self),
                format.param().abi(),
            )
            .ok()
        }
    }
    pub fn SetIndents(&self, start: f32, left: f32, right: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIndents)(
                windows_core::Interface::as_raw(self),
                start,
                left,
                right,
            )
            .ok()
        }
    }
    pub fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLineSpacing)(
                windows_core::Interface::as_raw(self),
                rule,
                spacing,
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for ITextParagraphFormat {
    const NAME: &'static str = "Microsoft.UI.Text.ITextParagraphFormat";
}
pub trait ITextParagraphFormat_Impl: windows_core::IUnknownImpl {
    fn Alignment(&self) -> windows_core::Result<ParagraphAlignment>;
    fn SetAlignment(&self, value: ParagraphAlignment) -> windows_core::Result<()>;
    fn FirstLineIndent(&self) -> windows_core::Result<f32>;
    fn KeepTogether(&self) -> windows_core::Result<FormatEffect>;
    fn SetKeepTogether(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn KeepWithNext(&self) -> windows_core::Result<FormatEffect>;
    fn SetKeepWithNext(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn LeftIndent(&self) -> windows_core::Result<f32>;
    fn LineSpacing(&self) -> windows_core::Result<f32>;
    fn LineSpacingRule(&self) -> windows_core::Result<LineSpacingRule>;
    fn ListAlignment(&self) -> windows_core::Result<MarkerAlignment>;
    fn SetListAlignment(&self, value: MarkerAlignment) -> windows_core::Result<()>;
    fn ListLevelIndex(&self) -> windows_core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()>;
    fn ListStart(&self) -> windows_core::Result<i32>;
    fn SetListStart(&self, value: i32) -> windows_core::Result<()>;
    fn ListStyle(&self) -> windows_core::Result<MarkerStyle>;
    fn SetListStyle(&self, value: MarkerStyle) -> windows_core::Result<()>;
    fn ListTab(&self) -> windows_core::Result<f32>;
    fn SetListTab(&self, value: f32) -> windows_core::Result<()>;
    fn ListType(&self) -> windows_core::Result<MarkerType>;
    fn SetListType(&self, value: MarkerType) -> windows_core::Result<()>;
    fn NoLineNumber(&self) -> windows_core::Result<FormatEffect>;
    fn SetNoLineNumber(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn PageBreakBefore(&self) -> windows_core::Result<FormatEffect>;
    fn SetPageBreakBefore(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn RightIndent(&self) -> windows_core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> windows_core::Result<()>;
    fn RightToLeft(&self) -> windows_core::Result<FormatEffect>;
    fn SetRightToLeft(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Style(&self) -> windows_core::Result<ParagraphStyle>;
    fn SetStyle(&self, value: ParagraphStyle) -> windows_core::Result<()>;
    fn SpaceAfter(&self) -> windows_core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()>;
    fn SpaceBefore(&self) -> windows_core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()>;
    fn WidowControl(&self) -> windows_core::Result<FormatEffect>;
    fn SetWidowControl(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn TabCount(&self) -> windows_core::Result<i32>;
    fn AddTab(
        &self,
        position: f32,
        align: TabAlignment,
        leader: TabLeader,
    ) -> windows_core::Result<()>;
    fn ClearAllTabs(&self) -> windows_core::Result<()>;
    fn DeleteTab(&self, position: f32) -> windows_core::Result<()>;
    fn GetClone(&self) -> windows_core::Result<ITextParagraphFormat>;
    fn GetTab(
        &self,
        index: i32,
        position: &mut f32,
        align: &mut TabAlignment,
        leader: &mut TabLeader,
    ) -> windows_core::Result<()>;
    fn IsEqual(
        &self,
        format: windows_core::Ref<ITextParagraphFormat>,
    ) -> windows_core::Result<bool>;
    fn SetClone(&self, format: windows_core::Ref<ITextParagraphFormat>)
    -> windows_core::Result<()>;
    fn SetIndents(&self, start: f32, left: f32, right: f32) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> windows_core::Result<()>;
}
impl ITextParagraphFormat_Vtbl {
    pub const fn new<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Alignment<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut ParagraphAlignment,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::Alignment(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlignment<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: ParagraphAlignment,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetAlignment(this, value).into()
            }
        }
        unsafe extern "system" fn FirstLineIndent<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::FirstLineIndent(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeepTogether<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::KeepTogether(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepTogether<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetKeepTogether(this, value).into()
            }
        }
        unsafe extern "system" fn KeepWithNext<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::KeepWithNext(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepWithNext<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetKeepWithNext(this, value).into()
            }
        }
        unsafe extern "system" fn LeftIndent<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::LeftIndent(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LineSpacing<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::LineSpacing(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LineSpacingRule<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut LineSpacingRule,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::LineSpacingRule(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ListAlignment<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut MarkerAlignment,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListAlignment(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListAlignment<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: MarkerAlignment,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListAlignment(this, value).into()
            }
        }
        unsafe extern "system" fn ListLevelIndex<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListLevelIndex(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListLevelIndex<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListLevelIndex(this, value).into()
            }
        }
        unsafe extern "system" fn ListStart<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListStart(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListStart<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListStart(this, value).into()
            }
        }
        unsafe extern "system" fn ListStyle<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut MarkerStyle,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListStyle(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListStyle<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: MarkerStyle,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListStyle(this, value).into()
            }
        }
        unsafe extern "system" fn ListTab<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListTab(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListTab<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListTab(this, value).into()
            }
        }
        unsafe extern "system" fn ListType<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut MarkerType,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListType<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: MarkerType,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListType(this, value).into()
            }
        }
        unsafe extern "system" fn NoLineNumber<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::NoLineNumber(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNoLineNumber<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetNoLineNumber(this, value).into()
            }
        }
        unsafe extern "system" fn PageBreakBefore<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::PageBreakBefore(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetPageBreakBefore(this, value).into()
            }
        }
        unsafe extern "system" fn RightIndent<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::RightIndent(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRightIndent<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetRightIndent(this, value).into()
            }
        }
        unsafe extern "system" fn RightToLeft<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::RightToLeft(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRightToLeft<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetRightToLeft(this, value).into()
            }
        }
        unsafe extern "system" fn Style<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut ParagraphStyle,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::Style(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStyle<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: ParagraphStyle,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetStyle(this, value).into()
            }
        }
        unsafe extern "system" fn SpaceAfter<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::SpaceAfter(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceAfter<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetSpaceAfter(this, value).into()
            }
        }
        unsafe extern "system" fn SpaceBefore<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::SpaceBefore(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceBefore<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetSpaceBefore(this, value).into()
            }
        }
        unsafe extern "system" fn WidowControl<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::WidowControl(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidowControl<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: FormatEffect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetWidowControl(this, value).into()
            }
        }
        unsafe extern "system" fn TabCount<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::TabCount(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTab<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            position: f32,
            align: TabAlignment,
            leader: TabLeader,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::AddTab(this, position, align, leader).into()
            }
        }
        unsafe extern "system" fn ClearAllTabs<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::ClearAllTabs(this).into()
            }
        }
        unsafe extern "system" fn DeleteTab<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            position: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::DeleteTab(this, position).into()
            }
        }
        unsafe extern "system" fn GetClone<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::GetClone(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTab<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: i32,
            position: *mut f32,
            align: *mut TabAlignment,
            leader: *mut TabLeader,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::GetTab(
                    this,
                    index,
                    core::mem::transmute_copy(&position),
                    core::mem::transmute_copy(&align),
                    core::mem::transmute_copy(&leader),
                )
                .into()
            }
        }
        unsafe extern "system" fn IsEqual<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            format: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::IsEqual(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClone<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            format: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetClone(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn SetIndents<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            start: f32,
            left: f32,
            right: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetIndents(this, start, left, right).into()
            }
        }
        unsafe extern "system" fn SetLineSpacing<
            Identity: ITextParagraphFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rule: LineSpacingRule,
            spacing: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetLineSpacing(this, rule, spacing).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextParagraphFormat, OFFSET>(
            ),
            Alignment: Alignment::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            FirstLineIndent: FirstLineIndent::<Identity, OFFSET>,
            KeepTogether: KeepTogether::<Identity, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, OFFSET>,
            KeepWithNext: KeepWithNext::<Identity, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, OFFSET>,
            LeftIndent: LeftIndent::<Identity, OFFSET>,
            LineSpacing: LineSpacing::<Identity, OFFSET>,
            LineSpacingRule: LineSpacingRule::<Identity, OFFSET>,
            ListAlignment: ListAlignment::<Identity, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, OFFSET>,
            ListLevelIndex: ListLevelIndex::<Identity, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, OFFSET>,
            ListStart: ListStart::<Identity, OFFSET>,
            SetListStart: SetListStart::<Identity, OFFSET>,
            ListStyle: ListStyle::<Identity, OFFSET>,
            SetListStyle: SetListStyle::<Identity, OFFSET>,
            ListTab: ListTab::<Identity, OFFSET>,
            SetListTab: SetListTab::<Identity, OFFSET>,
            ListType: ListType::<Identity, OFFSET>,
            SetListType: SetListType::<Identity, OFFSET>,
            NoLineNumber: NoLineNumber::<Identity, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, OFFSET>,
            PageBreakBefore: PageBreakBefore::<Identity, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, OFFSET>,
            RightIndent: RightIndent::<Identity, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, OFFSET>,
            RightToLeft: RightToLeft::<Identity, OFFSET>,
            SetRightToLeft: SetRightToLeft::<Identity, OFFSET>,
            Style: Style::<Identity, OFFSET>,
            SetStyle: SetStyle::<Identity, OFFSET>,
            SpaceAfter: SpaceAfter::<Identity, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, OFFSET>,
            SpaceBefore: SpaceBefore::<Identity, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, OFFSET>,
            WidowControl: WidowControl::<Identity, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, OFFSET>,
            TabCount: TabCount::<Identity, OFFSET>,
            AddTab: AddTab::<Identity, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, OFFSET>,
            DeleteTab: DeleteTab::<Identity, OFFSET>,
            GetClone: GetClone::<Identity, OFFSET>,
            GetTab: GetTab::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            SetClone: SetClone::<Identity, OFFSET>,
            SetIndents: SetIndents::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextParagraphFormat as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ITextParagraphFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Alignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ParagraphAlignment,
    ) -> windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        ParagraphAlignment,
    ) -> windows_core::HRESULT,
    pub FirstLineIndent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub KeepTogether: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetKeepTogether:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub KeepWithNext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetKeepWithNext:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub LeftIndent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub LineSpacing:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub LineSpacingRule: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut LineSpacingRule,
    ) -> windows_core::HRESULT,
    pub ListAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MarkerAlignment,
    ) -> windows_core::HRESULT,
    pub SetListAlignment:
        unsafe extern "system" fn(*mut core::ffi::c_void, MarkerAlignment) -> windows_core::HRESULT,
    pub ListLevelIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListLevelIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ListStart:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListStart:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ListStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MarkerStyle,
    ) -> windows_core::HRESULT,
    pub SetListStyle:
        unsafe extern "system" fn(*mut core::ffi::c_void, MarkerStyle) -> windows_core::HRESULT,
    pub ListTab:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetListTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ListType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut MarkerType) -> windows_core::HRESULT,
    pub SetListType:
        unsafe extern "system" fn(*mut core::ffi::c_void, MarkerType) -> windows_core::HRESULT,
    pub NoLineNumber: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetNoLineNumber:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub PageBreakBefore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetPageBreakBefore:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub RightIndent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRightIndent:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub RightToLeft: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetRightToLeft:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Style: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ParagraphStyle,
    ) -> windows_core::HRESULT,
    pub SetStyle:
        unsafe extern "system" fn(*mut core::ffi::c_void, ParagraphStyle) -> windows_core::HRESULT,
    pub SpaceAfter:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceAfter:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SpaceBefore:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceBefore:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub WidowControl: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FormatEffect,
    ) -> windows_core::HRESULT,
    pub SetWidowControl:
        unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub TabCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddTab: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        TabAlignment,
        TabLeader,
    ) -> windows_core::HRESULT,
    pub ClearAllTabs: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetClone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetTab: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut f32,
        *mut TabAlignment,
        *mut TabLeader,
    ) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub SetClone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetIndents:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        LineSpacingRule,
        f32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextRange,
    ITextRange_Vtbl,
    0x06d4abcf_0c06_5d12_a743_85537efd09ea
);
impl windows_core::RuntimeType for ITextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ITextRange,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ITextRange {
    pub fn Character(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Character)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCharacter(&self, value: u16) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCharacter)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CharacterFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetCharacterFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCharacterFormat)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn FormattedText(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormattedText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetFormattedText<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFormattedText)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn EndPosition(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndPosition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetEndPosition(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetEndPosition)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Gravity(&self) -> windows_core::Result<RangeGravity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Gravity)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetGravity(&self, value: RangeGravity) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetGravity)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Link(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Link)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetLink(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLink)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn ParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParagraphFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetParagraphFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetParagraphFormat)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn StartPosition(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPosition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetStartPosition(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStartPosition)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn StoryLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StoryLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| {
                let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                hstring.to_string_lossy()
            })
        }
    }
    pub fn SetText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn CanPaste(&self, format: i32) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPaste)(
                windows_core::Interface::as_raw(self),
                format,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ChangeCase(&self, value: LetterCase) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ChangeCase)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Collapse(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Collapse)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Copy(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub fn Cut(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Cut)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delete)(
                windows_core::Interface::as_raw(self),
                unit,
                count,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndOf)(
                windows_core::Interface::as_raw(self),
                unit,
                extend,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Expand(&self, unit: TextRangeUnit) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Expand)(
                windows_core::Interface::as_raw(self),
                unit,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn FindText(
        &self,
        value: &str,
        scanlength: i32,
        options: FindOptions,
    ) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
                scanlength,
                options,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetCharacterUtf32)(
                windows_core::Interface::as_raw(self),
                value,
                offset,
            )
            .ok()
        }
    }
    pub fn GetClone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetIndex(&self, unit: TextRangeUnit) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(
                windows_core::Interface::as_raw(self),
                unit,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetPoint(
        &self,
        horizontalalign: HorizontalCharacterAlignment,
        verticalalign: VerticalCharacterAlignment,
        options: PointOptions,
        point: &mut Point,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetPoint)(
                windows_core::Interface::as_raw(self),
                horizontalalign,
                verticalalign,
                options,
                point,
            )
            .ok()
        }
    }
    pub fn GetRect(
        &self,
        options: PointOptions,
        rect: &mut Rect,
        hit: &mut i32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetRect)(
                windows_core::Interface::as_raw(self),
                options,
                rect,
                hit,
            )
            .ok()
        }
    }
    pub fn GetText(
        &self,
        options: TextGetOptions,
        value: &mut windows_core::HSTRING,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetText)(
                windows_core::Interface::as_raw(self),
                options,
                value as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn InRange<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InRange)(
                windows_core::Interface::as_raw(self),
                range.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn InStory<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InStory)(
                windows_core::Interface::as_raw(self),
                range.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IsEqual<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(
                windows_core::Interface::as_raw(self),
                range.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Move)(
                windows_core::Interface::as_raw(self),
                unit,
                count,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEnd)(
                windows_core::Interface::as_raw(self),
                unit,
                count,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveStart)(
                windows_core::Interface::as_raw(self),
                unit,
                count,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Paste(&self, format: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Paste)(
                windows_core::Interface::as_raw(self),
                format,
            )
            .ok()
        }
    }
    pub fn ScrollIntoView(&self, value: PointOptions) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ScrollIntoView)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn MatchSelection(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).MatchSelection)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub fn SetIndex(
        &self,
        unit: TextRangeUnit,
        index: i32,
        extend: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIndex)(
                windows_core::Interface::as_raw(self),
                unit,
                index,
                extend,
            )
            .ok()
        }
    }
    pub fn SetPoint(
        &self,
        point: Point,
        options: PointOptions,
        extend: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPoint)(
                windows_core::Interface::as_raw(self),
                point,
                options,
                extend,
            )
            .ok()
        }
    }
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRange)(
                windows_core::Interface::as_raw(self),
                startposition,
                endposition,
            )
            .ok()
        }
    }
    pub fn SetText2(&self, options: TextSetOptions, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetText2)(
                windows_core::Interface::as_raw(self),
                options,
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOf)(
                windows_core::Interface::as_raw(self),
                unit,
                extend,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ITextRange {
    const NAME: &'static str = "Microsoft.UI.Text.ITextRange";
}
#[repr(C)]
pub struct ITextRange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Character:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetCharacter:
        unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub CharacterFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCharacterFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FormattedText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub EndPosition:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEndPosition:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Gravity: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut RangeGravity,
    ) -> windows_core::HRESULT,
    pub SetGravity:
        unsafe extern "system" fn(*mut core::ffi::c_void, RangeGravity) -> windows_core::HRESULT,
    pub Length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetLink: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ParagraphFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetParagraphFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub StartPosition:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStartPosition:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StoryLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CanPaste:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut bool) -> windows_core::HRESULT,
    pub ChangeCase:
        unsafe extern "system" fn(*mut core::ffi::c_void, LetterCase) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub EndOf: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        bool,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub Expand: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub FindText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
        FindOptions,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub GetCharacterUtf32:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, i32) -> windows_core::HRESULT,
    pub GetClone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HorizontalCharacterAlignment,
        VerticalCharacterAlignment,
        PointOptions,
        *mut Point,
    ) -> windows_core::HRESULT,
    pub GetRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PointOptions,
        *mut Rect,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextGetOptions,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetTextViaStream: usize,
    pub InRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    InsertImage: usize,
    pub InStory: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub MoveEnd: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub MoveStart: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub Paste: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ScrollIntoView:
        unsafe extern "system" fn(*mut core::ffi::c_void, PointOptions) -> windows_core::HRESULT,
    pub MatchSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        bool,
    ) -> windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Point,
        PointOptions,
        bool,
    ) -> windows_core::HRESULT,
    pub SetRange:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetText2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextSetOptions,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SetTextViaStream: usize,
    pub StartOf: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        bool,
        *mut i32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ITextSelection,
    ITextSelection_Vtbl,
    0x8f5e6cb1_2b04_589f_bd24_54e5cd8dd399
);
impl windows_core::RuntimeType for ITextSelection {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    ITextSelection,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ITextSelection, ITextRange);
impl ITextSelection {
    pub fn Options(&self) -> windows_core::Result<SelectionOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Options)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetOptions(&self, value: SelectionOptions) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOptions)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn Type(&self) -> windows_core::Result<SelectionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndKey)(
                windows_core::Interface::as_raw(self),
                unit,
                extend,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HomeKey)(
                windows_core::Interface::as_raw(self),
                unit,
                extend,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveDown(
        &self,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
    ) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveDown)(
                windows_core::Interface::as_raw(self),
                unit,
                count,
                extend,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveLeft(
        &self,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
    ) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveLeft)(
                windows_core::Interface::as_raw(self),
                unit,
                count,
                extend,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveRight(
        &self,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
    ) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveRight)(
                windows_core::Interface::as_raw(self),
                unit,
                count,
                extend,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveUp(
        &self,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
    ) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveUp)(
                windows_core::Interface::as_raw(self),
                unit,
                count,
                extend,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TypeText(&self, value: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).TypeText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(value)),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for ITextSelection {
    const NAME: &'static str = "Microsoft.UI.Text.ITextSelection";
}
#[repr(C)]
pub struct ITextSelection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Options: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut SelectionOptions,
    ) -> windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SelectionOptions,
    ) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut SelectionType,
    ) -> windows_core::HRESULT,
    pub EndKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        bool,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub HomeKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        bool,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub MoveDown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        bool,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub MoveLeft: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        bool,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub MoveRight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        bool,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub MoveUp: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TextRangeUnit,
        i32,
        bool,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub TypeText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToolTip,
    IToolTip_Vtbl,
    0x67e93d74_5e93_59a1_91bf_413efbeb904c
);
impl windows_core::RuntimeType for IToolTip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IToolTip {
    pub fn IsOpen(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsOpen(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsOpen)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IToolTip_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    HorizontalOffset: usize,
    SetHorizontalOffset: usize,
    pub IsOpen:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToolTipFactory,
    IToolTipFactory_Vtbl,
    0xbcbb3720_2db8_54e1_8806_fcbed38949a9
);
impl windows_core::RuntimeType for IToolTipFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToolTipFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToolTipService,
    IToolTipService_Vtbl,
    0x01140768_2727_5f89_80e0_5210326a3431
);
impl windows_core::RuntimeType for IToolTipService {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToolTipService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IToolTipServiceStatics,
    IToolTipServiceStatics_Vtbl,
    0x5aa38adc_9874_5e0a_8d8e_1574efc0b88f
);
impl windows_core::RuntimeType for IToolTipServiceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToolTipServiceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PlacementProperty: usize,
    GetPlacement: usize,
    SetPlacement: usize,
    PlacementTargetProperty: usize,
    GetPlacementTarget: usize,
    SetPlacementTarget: usize,
    ToolTipProperty: usize,
    GetToolTip: usize,
    pub SetToolTip: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IToolTipStatics,
    IToolTipStatics_Vtbl,
    0x88eb60d4_cd54_5e23_bd1a_223702442ba4
);
impl windows_core::RuntimeType for IToolTipStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToolTipStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUIElement,
    IUIElement_Vtbl,
    0xc3c01020_320c_5cf6_9d24_d396bbfa4d8b
);
impl windows_core::RuntimeType for IUIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUIElement {
    pub fn ContextFlyout(&self) -> windows_core::Result<FlyoutBase> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContextFlyout)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetContextFlyout<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FlyoutBase>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContextFlyout)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<XamlRoot> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XamlRoot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<XamlRoot>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetXamlRoot)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn PointerWheelChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<PointerRoutedEventArgs>,
            ) + 'static,
    {
        let handler: PointerEventHandler = {
            let com = windows_core::imp::DelegateBox::<PointerEventHandler, F>::new(
                &PointerEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerWheelChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePointerWheelChanged,
            ))
        }
    }
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<KeyRoutedEventArgs>)
            + 'static,
    {
        let handler: KeyEventHandler = {
            let com = windows_core::imp::DelegateBox::<KeyEventHandler, F>::new(
                &KeyEventHandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PreviewKeyDown)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemovePreviewKeyDown,
            ))
        }
    }
    pub fn BringIntoViewRequested<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<BringIntoViewRequestedEventArgs>)
            + 'static,
    {
        let handler: TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs>,
                F,
            >::new(
                &TypedEventHandlerBox::<UIElement, BringIntoViewRequestedEventArgs, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).BringIntoViewRequested)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveBringIntoViewRequested,
            ))
        }
    }
    pub fn TransformToVisual<P0>(&self, visual: P0) -> windows_core::Result<GeneralTransform>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransformToVisual)(
                windows_core::Interface::as_raw(self),
                visual.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).StartBringIntoView)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn StartBringIntoViewWithOptions<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BringIntoViewOptions>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).StartBringIntoViewWithOptions)(
                windows_core::Interface::as_raw(self),
                options.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IUIElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    DesiredSize: usize,
    AllowDrop: usize,
    SetAllowDrop: usize,
    Opacity: usize,
    SetOpacity: usize,
    Clip: usize,
    SetClip: usize,
    RenderTransform: usize,
    SetRenderTransform: usize,
    Projection: usize,
    SetProjection: usize,
    Transform3D: usize,
    SetTransform3D: usize,
    RenderTransformOrigin: usize,
    SetRenderTransformOrigin: usize,
    IsHitTestVisible: usize,
    SetIsHitTestVisible: usize,
    Visibility: usize,
    SetVisibility: usize,
    RenderSize: usize,
    UseLayoutRounding: usize,
    SetUseLayoutRounding: usize,
    Transitions: usize,
    SetTransitions: usize,
    CacheMode: usize,
    SetCacheMode: usize,
    IsTapEnabled: usize,
    SetIsTapEnabled: usize,
    IsDoubleTapEnabled: usize,
    SetIsDoubleTapEnabled: usize,
    CanDrag: usize,
    SetCanDrag: usize,
    IsRightTapEnabled: usize,
    SetIsRightTapEnabled: usize,
    IsHoldingEnabled: usize,
    SetIsHoldingEnabled: usize,
    ManipulationMode: usize,
    SetManipulationMode: usize,
    PointerCaptures: usize,
    pub ContextFlyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContextFlyout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CompositeMode: usize,
    SetCompositeMode: usize,
    Lights: usize,
    CanBeScrollAnchor: usize,
    SetCanBeScrollAnchor: usize,
    ExitDisplayModeOnAccessKeyInvoked: usize,
    SetExitDisplayModeOnAccessKeyInvoked: usize,
    IsAccessKeyScope: usize,
    SetIsAccessKeyScope: usize,
    AccessKeyScopeOwner: usize,
    SetAccessKeyScopeOwner: usize,
    AccessKey: usize,
    SetAccessKey: usize,
    KeyTipPlacementMode: usize,
    SetKeyTipPlacementMode: usize,
    KeyTipHorizontalOffset: usize,
    SetKeyTipHorizontalOffset: usize,
    KeyTipVerticalOffset: usize,
    SetKeyTipVerticalOffset: usize,
    KeyTipTarget: usize,
    SetKeyTipTarget: usize,
    XYFocusKeyboardNavigation: usize,
    SetXYFocusKeyboardNavigation: usize,
    XYFocusUpNavigationStrategy: usize,
    SetXYFocusUpNavigationStrategy: usize,
    XYFocusDownNavigationStrategy: usize,
    SetXYFocusDownNavigationStrategy: usize,
    XYFocusLeftNavigationStrategy: usize,
    SetXYFocusLeftNavigationStrategy: usize,
    XYFocusRightNavigationStrategy: usize,
    SetXYFocusRightNavigationStrategy: usize,
    KeyboardAccelerators: usize,
    KeyboardAcceleratorPlacementTarget: usize,
    SetKeyboardAcceleratorPlacementTarget: usize,
    KeyboardAcceleratorPlacementMode: usize,
    SetKeyboardAcceleratorPlacementMode: usize,
    HighContrastAdjustment: usize,
    SetHighContrastAdjustment: usize,
    TabFocusNavigation: usize,
    SetTabFocusNavigation: usize,
    OpacityTransition: usize,
    SetOpacityTransition: usize,
    Translation: usize,
    SetTranslation: usize,
    TranslationTransition: usize,
    SetTranslationTransition: usize,
    Rotation: usize,
    SetRotation: usize,
    RotationTransition: usize,
    SetRotationTransition: usize,
    Scale: usize,
    SetScale: usize,
    ScaleTransition: usize,
    SetScaleTransition: usize,
    TransformMatrix: usize,
    SetTransformMatrix: usize,
    CenterPoint: usize,
    SetCenterPoint: usize,
    RotationAxis: usize,
    SetRotationAxis: usize,
    ActualOffset: usize,
    ActualSize: usize,
    pub XamlRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Shadow: usize,
    SetShadow: usize,
    RasterizationScale: usize,
    SetRasterizationScale: usize,
    FocusState: usize,
    UseSystemFocusVisuals: usize,
    SetUseSystemFocusVisuals: usize,
    XYFocusLeft: usize,
    SetXYFocusLeft: usize,
    XYFocusRight: usize,
    SetXYFocusRight: usize,
    XYFocusUp: usize,
    SetXYFocusUp: usize,
    XYFocusDown: usize,
    SetXYFocusDown: usize,
    IsTabStop: usize,
    SetIsTabStop: usize,
    TabIndex: usize,
    SetTabIndex: usize,
    KeyUp: usize,
    RemoveKeyUp: usize,
    KeyDown: usize,
    RemoveKeyDown: usize,
    GotFocus: usize,
    RemoveGotFocus: usize,
    LostFocus: usize,
    RemoveLostFocus: usize,
    DragStarting: usize,
    RemoveDragStarting: usize,
    DropCompleted: usize,
    RemoveDropCompleted: usize,
    CharacterReceived: usize,
    RemoveCharacterReceived: usize,
    DragEnter: usize,
    RemoveDragEnter: usize,
    DragLeave: usize,
    RemoveDragLeave: usize,
    DragOver: usize,
    RemoveDragOver: usize,
    Drop: usize,
    RemoveDrop: usize,
    PointerPressed: usize,
    RemovePointerPressed: usize,
    PointerMoved: usize,
    RemovePointerMoved: usize,
    PointerReleased: usize,
    RemovePointerReleased: usize,
    PointerEntered: usize,
    RemovePointerEntered: usize,
    PointerExited: usize,
    RemovePointerExited: usize,
    PointerCaptureLost: usize,
    RemovePointerCaptureLost: usize,
    PointerCanceled: usize,
    RemovePointerCanceled: usize,
    pub PointerWheelChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePointerWheelChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Tapped: usize,
    RemoveTapped: usize,
    DoubleTapped: usize,
    RemoveDoubleTapped: usize,
    Holding: usize,
    RemoveHolding: usize,
    ContextRequested: usize,
    RemoveContextRequested: usize,
    ContextCanceled: usize,
    RemoveContextCanceled: usize,
    RightTapped: usize,
    RemoveRightTapped: usize,
    ManipulationStarting: usize,
    RemoveManipulationStarting: usize,
    ManipulationInertiaStarting: usize,
    RemoveManipulationInertiaStarting: usize,
    ManipulationStarted: usize,
    RemoveManipulationStarted: usize,
    ManipulationDelta: usize,
    RemoveManipulationDelta: usize,
    ManipulationCompleted: usize,
    RemoveManipulationCompleted: usize,
    AccessKeyDisplayRequested: usize,
    RemoveAccessKeyDisplayRequested: usize,
    AccessKeyDisplayDismissed: usize,
    RemoveAccessKeyDisplayDismissed: usize,
    AccessKeyInvoked: usize,
    RemoveAccessKeyInvoked: usize,
    ProcessKeyboardAccelerators: usize,
    RemoveProcessKeyboardAccelerators: usize,
    GettingFocus: usize,
    RemoveGettingFocus: usize,
    LosingFocus: usize,
    RemoveLosingFocus: usize,
    NoFocusCandidateFound: usize,
    RemoveNoFocusCandidateFound: usize,
    pub PreviewKeyDown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemovePreviewKeyDown:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    PreviewKeyUp: usize,
    RemovePreviewKeyUp: usize,
    pub BringIntoViewRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveBringIntoViewRequested:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Measure: usize,
    Arrange: usize,
    CapturePointer: usize,
    ReleasePointerCapture: usize,
    ReleasePointerCaptures: usize,
    AddHandler: usize,
    RemoveHandler: usize,
    pub TransformToVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    InvalidateMeasure: usize,
    InvalidateArrange: usize,
    UpdateLayout: usize,
    CancelDirectManipulations: usize,
    StartDragAsync: usize,
    pub StartBringIntoView:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartBringIntoViewWithOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElementFactory,
    IUIElementFactory_Vtbl,
    0x14d1d309_add0_5ccb_b946_77488cd70f87
);
impl windows_core::RuntimeType for IUIElementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUIElementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUIElementOverrides,
    IUIElementOverrides_Vtbl,
    0x9034f41e_ab7b_59e7_8168_50de6b689dde
);
impl windows_core::RuntimeType for IUIElementOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUIElementOverrides {
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).OnDisconnectVisualChildren)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub fn FindSubElementsForTouchTargeting(
        &self,
        point: Point,
        boundingrect: Rect,
    ) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<Point>>>
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindSubElementsForTouchTargeting)(
                windows_core::Interface::as_raw(self),
                point,
                boundingrect,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildrenInTabFocusOrder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BringIntoViewRequestedEventArgs>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnBringIntoViewRequested)(
                windows_core::Interface::as_raw(self),
                e.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IUIElementOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OnCreateAutomationPeer: usize,
    pub OnDisconnectVisualChildren:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindSubElementsForTouchTargeting: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        Point,
        Rect,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetChildrenInTabFocusOrder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    OnKeyboardAcceleratorInvoked: usize,
    OnProcessKeyboardAccelerators: usize,
    pub OnBringIntoViewRequested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElementProtected,
    IUIElementProtected_Vtbl,
    0x8f69b9e9_1f00_5834_9bf1_a9257bed39f0
);
impl windows_core::RuntimeType for IUIElementProtected {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUIElementProtected {
    pub fn ProtectedCursor(&self) -> windows_core::Result<InputCursor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProtectedCursor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InputCursor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetProtectedCursor)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IUIElementProtected_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProtectedCursor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetProtectedCursor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUIElementStatics,
    IUIElementStatics_Vtbl,
    0xd2921d87_3584_5e22_8a3a_c2c78dab4f6e
);
impl windows_core::RuntimeType for IUIElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUIElementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    KeyDownEvent: usize,
    KeyUpEvent: usize,
    PointerEnteredEvent: usize,
    PointerPressedEvent: usize,
    PointerMovedEvent: usize,
    PointerReleasedEvent: usize,
    PointerExitedEvent: usize,
    PointerCaptureLostEvent: usize,
    PointerCanceledEvent: usize,
    PointerWheelChangedEvent: usize,
    TappedEvent: usize,
    DoubleTappedEvent: usize,
    HoldingEvent: usize,
    RightTappedEvent: usize,
    ManipulationStartingEvent: usize,
    ManipulationInertiaStartingEvent: usize,
    ManipulationStartedEvent: usize,
    ManipulationDeltaEvent: usize,
    ManipulationCompletedEvent: usize,
    DragEnterEvent: usize,
    DragLeaveEvent: usize,
    DragOverEvent: usize,
    DropEvent: usize,
    GettingFocusEvent: usize,
    LosingFocusEvent: usize,
    NoFocusCandidateFoundEvent: usize,
    PreviewKeyDownEvent: usize,
    CharacterReceivedEvent: usize,
    PreviewKeyUpEvent: usize,
    BringIntoViewRequestedEvent: usize,
    ContextRequestedEvent: usize,
    AllowDropProperty: usize,
    OpacityProperty: usize,
    ClipProperty: usize,
    RenderTransformProperty: usize,
    ProjectionProperty: usize,
    Transform3DProperty: usize,
    RenderTransformOriginProperty: usize,
    IsHitTestVisibleProperty: usize,
    VisibilityProperty: usize,
    UseLayoutRoundingProperty: usize,
    TransitionsProperty: usize,
    CacheModeProperty: usize,
    IsTapEnabledProperty: usize,
    IsDoubleTapEnabledProperty: usize,
    CanDragProperty: usize,
    IsRightTapEnabledProperty: usize,
    IsHoldingEnabledProperty: usize,
    ManipulationModeProperty: usize,
    PointerCapturesProperty: usize,
    ContextFlyoutProperty: usize,
    CompositeModeProperty: usize,
    LightsProperty: usize,
    CanBeScrollAnchorProperty: usize,
    ExitDisplayModeOnAccessKeyInvokedProperty: usize,
    IsAccessKeyScopeProperty: usize,
    AccessKeyScopeOwnerProperty: usize,
    AccessKeyProperty: usize,
    KeyTipPlacementModeProperty: usize,
    KeyTipHorizontalOffsetProperty: usize,
    KeyTipVerticalOffsetProperty: usize,
    KeyTipTargetProperty: usize,
    XYFocusKeyboardNavigationProperty: usize,
    XYFocusUpNavigationStrategyProperty: usize,
    XYFocusDownNavigationStrategyProperty: usize,
    XYFocusLeftNavigationStrategyProperty: usize,
    XYFocusRightNavigationStrategyProperty: usize,
    KeyboardAcceleratorPlacementTargetProperty: usize,
    KeyboardAcceleratorPlacementModeProperty: usize,
    HighContrastAdjustmentProperty: usize,
    TabFocusNavigationProperty: usize,
    ShadowProperty: usize,
    FocusStateProperty: usize,
    UseSystemFocusVisualsProperty: usize,
    XYFocusLeftProperty: usize,
    XYFocusRightProperty: usize,
    XYFocusUpProperty: usize,
    XYFocusDownProperty: usize,
    IsTabStopProperty: usize,
    TabIndexProperty: usize,
    TryStartDirectManipulation: usize,
    pub RegisterAsScrollPort: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUISettings,
    IUISettings_Vtbl,
    0x85361600_1c63_4627_bcb1_3a89e0bc9c55
);
impl windows_core::RuntimeType for IUISettings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUISettings {
    pub fn MessageDuration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageDuration)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AnimationsEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnimationsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CaretBrowsingEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaretBrowsingEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CaretBlinkRate(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaretBlinkRate)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CaretWidth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaretWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn DoubleClickTime(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoubleClickTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MouseHoverTime(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MouseHoverTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IUISettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    HandPreference: usize,
    CursorSize: usize,
    ScrollBarSize: usize,
    ScrollBarArrowSize: usize,
    ScrollBarThumbBoxSize: usize,
    pub MessageDuration:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AnimationsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CaretBrowsingEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CaretBlinkRate:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CaretWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DoubleClickTime:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MouseHoverTime:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUISettings2,
    IUISettings2_Vtbl,
    0xbad82401_2721_44f9_bb91_2bb228be442f
);
impl windows_core::RuntimeType for IUISettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUISettings2 {
    pub fn TextScaleFactor(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TextScaleFactor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn TextScaleFactorChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UISettings>, windows_core::Ref<windows_core::IInspectable>)
            + 'static,
    {
        let handler: TypedEventHandler<UISettings, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<UISettings, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<UISettings, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).TextScaleFactorChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveTextScaleFactorChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IUISettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextScaleFactor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TextScaleFactorChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveTextScaleFactorChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUISettings3,
    IUISettings3_Vtbl,
    0x03021be4_5254_4781_8194_5168f7d06d7b
);
impl windows_core::RuntimeType for IUISettings3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUISettings3 {
    pub fn GetColorValue(&self, desiredcolor: UIColorType) -> windows_core::Result<Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorValue)(
                windows_core::Interface::as_raw(self),
                desiredcolor,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IUISettings3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetColorValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UIColorType,
        *mut Color,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUISettings4,
    IUISettings4_Vtbl,
    0x52bb3002_919b_4d6b_9b78_8dd66ff4b93b
);
impl windows_core::RuntimeType for IUISettings4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUISettings4 {
    pub fn AdvancedEffectsEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdvancedEffectsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AdvancedEffectsEnabledChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UISettings>, windows_core::Ref<windows_core::IInspectable>)
            + 'static,
    {
        let handler: TypedEventHandler<UISettings, windows_core::IInspectable> = {
            let com = windows_core::imp::DelegateBox::<
                TypedEventHandler<UISettings, windows_core::IInspectable>,
                F,
            >::new(
                &TypedEventHandlerBox::<UISettings, windows_core::IInspectable, F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).AdvancedEffectsEnabledChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveAdvancedEffectsEnabledChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IUISettings4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AdvancedEffectsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AdvancedEffectsEnabledChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveAdvancedEffectsEnabledChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUISettings5,
    IUISettings5_Vtbl,
    0x5349d588_0cb5_5f05_bd34_706b3231f0bd
);
impl windows_core::RuntimeType for IUISettings5 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUISettings5 {
    pub fn AutoHideScrollBars(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoHideScrollBars)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn AutoHideScrollBarsChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<UISettings>,
                windows_core::Ref<UISettingsAutoHideScrollBarsChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs> = {
            let com =
                windows_core::imp::DelegateBox::<
                    TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs>,
                    F,
                >::new(
                    &TypedEventHandlerBox::<
                        UISettings,
                        UISettingsAutoHideScrollBarsChangedEventArgs,
                        F,
                    >::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).AutoHideScrollBarsChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveAutoHideScrollBarsChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IUISettings5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoHideScrollBars:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AutoHideScrollBarsChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveAutoHideScrollBarsChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUISettings6,
    IUISettings6_Vtbl,
    0xaef19bd7_fe31_5a04_ada4_469aaec6dfa9
);
impl windows_core::RuntimeType for IUISettings6 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IUISettings6 {
    pub fn AnimationsEnabledChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<UISettings>,
                windows_core::Ref<UISettingsAnimationsEnabledChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs> = {
            let com =
                windows_core::imp::DelegateBox::<
                    TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs>,
                    F,
                >::new(
                    &TypedEventHandlerBox::<
                        UISettings,
                        UISettingsAnimationsEnabledChangedEventArgs,
                        F,
                    >::VTABLE,
                    handler,
                );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).AnimationsEnabledChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveAnimationsEnabledChanged,
            ))
        }
    }
    pub fn MessageDurationChanged<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<UISettings>,
                windows_core::Ref<UISettingsMessageDurationChangedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < UISettings , UISettingsMessageDurationChangedEventArgs > , F >::new (& TypedEventHandlerBox::< UISettings , UISettingsMessageDurationChangedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).MessageDurationChanged)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveMessageDurationChanged,
            ))
        }
    }
}
#[repr(C)]
pub struct IUISettings6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AnimationsEnabledChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveAnimationsEnabledChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub MessageDurationChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveMessageDurationChanged:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUISettingsAnimationsEnabledChangedEventArgs,
    IUISettingsAnimationsEnabledChangedEventArgs_Vtbl,
    0x0c7b4b3d_2ea1_533e_894d_415bc5243c29
);
impl windows_core::RuntimeType for IUISettingsAnimationsEnabledChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettingsAnimationsEnabledChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUISettingsAutoHideScrollBarsChangedEventArgs,
    IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl,
    0x87afd4b2_9146_5f02_8f6b_06d454174c0f
);
impl windows_core::RuntimeType for IUISettingsAutoHideScrollBarsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IUISettingsMessageDurationChangedEventArgs,
    IUISettingsMessageDurationChangedEventArgs_Vtbl,
    0x338aad52_4a5d_5b59_8002_d930f608fd6e
);
impl windows_core::RuntimeType for IUISettingsMessageDurationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettingsMessageDurationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IVisualElement,
    IVisualElement_Vtbl,
    0x2180f1f5_b5d8_4bf6_920a_12006e63efef
);
impl windows_core::RuntimeType for IVisualElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IVisualElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IVisualElement {
    const NAME: &'static str = "Microsoft.UI.Composition.IVisualElement";
}
pub trait IVisualElement_Impl: windows_core::IUnknownImpl {}
impl IVisualElement_Vtbl {
    pub const fn new<Identity: IVisualElement_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualElement, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualElement as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IVisualElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IVisualElement2,
    IVisualElement2_Vtbl,
    0xbc950c8d_1db0_53aa_9dee_34271cd18ce6
);
impl windows_core::RuntimeType for IVisualElement2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IVisualElement2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IVisualElement2 {
    const NAME: &'static str = "Microsoft.UI.Composition.IVisualElement2";
}
#[repr(C)]
pub struct IVisualElement2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IVisualTreeHelper,
    IVisualTreeHelper_Vtbl,
    0x5f69ac1e_6504_5e3f_a11c_87684c1db814
);
impl windows_core::RuntimeType for IVisualTreeHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVisualTreeHelper_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IVisualTreeHelperStatics,
    IVisualTreeHelperStatics_Vtbl,
    0x5aece43c_7651_5bb5_855c_2198496e455e
);
impl windows_core::RuntimeType for IVisualTreeHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVisualTreeHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FindElementsInHostCoordinatesPoint: usize,
    FindElementsInHostCoordinatesRect: usize,
    FindAllElementsInHostCoordinatesPoint: usize,
    FindAllElementsInHostCoordinatesRect: usize,
    pub GetChild: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetChildrenCount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWindowEventArgs,
    IWindowEventArgs_Vtbl,
    0x1140827c_fe0a_5268_bc2b_f4492c2ccb49
);
impl windows_core::RuntimeType for IWindowEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IWindowEventArgs {
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
pub struct IWindowEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlReader,
    IXamlReader_Vtbl,
    0x54ce54c8_38c6_50d9_ac98_4b03eddbde9f
);
impl windows_core::RuntimeType for IXamlReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXamlReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IXamlReaderStatics,
    IXamlReaderStatics_Vtbl,
    0x82a4cd9e_435e_5aeb_8c4f_300cece45cae
);
impl windows_core::RuntimeType for IXamlReaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXamlReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Load: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlRoot,
    IXamlRoot_Vtbl,
    0x60cb215a_ad15_520a_8b01_4416824f0441
);
impl windows_core::RuntimeType for IXamlRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IXamlRoot {
    pub fn Content(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RasterizationScale)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IXamlRoot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Size: usize,
    pub RasterizationScale:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlRoot2,
    IXamlRoot2_Vtbl,
    0xbdee0f42_71cb_50c5_829b_4614d98c5794
);
impl windows_core::RuntimeType for IXamlRoot2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl IXamlRoot2 {
    pub fn ContentIslandEnvironment(&self) -> windows_core::Result<ContentIslandEnvironment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentIslandEnvironment)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IXamlRoot2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentIslandEnvironment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IXamlRoot3,
    IXamlRoot3_Vtbl,
    0xb71dbf3b_2e0f_5de0_ac68_f0c1f65114c8
);
impl windows_core::RuntimeType for IXamlRoot3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXamlRoot3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IXamlRoot4,
    IXamlRoot4_Vtbl,
    0x377bec22_632b_52be_b26f_5edf7838e5ca
);
impl windows_core::RuntimeType for IXamlRoot4 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXamlRoot4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IXamlRootChangedEventArgs,
    IXamlRootChangedEventArgs_Vtbl,
    0x61d2c719_f8a1_515a_902c_cfa498ba7a7f
);
impl windows_core::RuntimeType for IXamlRootChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXamlRootChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    IconElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    IconElement,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for IconElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IIconElement>();
}
unsafe impl windows_core::Interface for IconElement {
    type Vtable = <IIconElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIconElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for IconElement {
    type Target = IIconElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for IconElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IconElement";
}
unsafe impl Send for IconElement {}
unsafe impl Sync for IconElement {}
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
windows_core::imp::define_interface!(
    KeyEventHandler,
    KeyEventHandler_Vtbl,
    0xdb68e7cc_9a2b_527d_9989_25284daccc03
);
impl windows_core::RuntimeType for KeyEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl KeyEventHandler {
    pub fn new<
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<KeyRoutedEventArgs>)
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &KeyEventHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct KeyEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct KeyEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<KeyRoutedEventArgs>)
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<KeyRoutedEventArgs>)
        + 'static,
> KeyEventHandlerBox<F>
{
    const VTABLE: KeyEventHandler_Vtbl = KeyEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<KeyEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<KeyEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<KeyEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<KeyEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    KeyRoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(KeyRoutedEventArgs, RoutedEventArgs);
impl windows_core::RuntimeType for KeyRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IKeyRoutedEventArgs>();
}
unsafe impl windows_core::Interface for KeyRoutedEventArgs {
    type Vtable = <IKeyRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyRoutedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for KeyRoutedEventArgs {
    type Target = IKeyRoutedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for KeyRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyRoutedEventArgs";
}
unsafe impl Send for KeyRoutedEventArgs {}
unsafe impl Sync for KeyRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LetterCase(pub i32);
impl LetterCase {
    pub const Lower: Self = Self(0);
    pub const Upper: Self = Self(1);
}
impl windows_core::imp::TypeKind for LetterCase {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for LetterCase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.LetterCase;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LineSpacingRule(pub i32);
impl LineSpacingRule {
    pub const Undefined: Self = Self(0);
    pub const Single: Self = Self(1);
    pub const OneAndHalf: Self = Self(2);
    pub const Double: Self = Self(3);
    pub const AtLeast: Self = Self(4);
    pub const Exactly: Self = Self(5);
    pub const Multiple: Self = Self(6);
    pub const Percent: Self = Self(7);
}
impl windows_core::imp::TypeKind for LineSpacingRule {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for LineSpacingRule {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.LineSpacingRule;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LinkType(pub i32);
impl LinkType {
    pub const Undefined: Self = Self(0);
    pub const NotALink: Self = Self(1);
    pub const ClientLink: Self = Self(2);
    pub const FriendlyLinkName: Self = Self(3);
    pub const FriendlyLinkAddress: Self = Self(4);
    pub const AutoLink: Self = Self(5);
    pub const AutoLinkEmail: Self = Self(6);
    pub const AutoLinkPhone: Self = Self(7);
    pub const AutoLinkPath: Self = Self(8);
}
impl windows_core::imp::TypeKind for LinkType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for LinkType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.LinkType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MarkerAlignment(pub i32);
impl MarkerAlignment {
    pub const Undefined: Self = Self(0);
    pub const Left: Self = Self(1);
    pub const Center: Self = Self(2);
    pub const Right: Self = Self(3);
}
impl windows_core::imp::TypeKind for MarkerAlignment {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MarkerAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.MarkerAlignment;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MarkerStyle(pub i32);
impl MarkerStyle {
    pub const Undefined: Self = Self(0);
    pub const Parenthesis: Self = Self(1);
    pub const Parentheses: Self = Self(2);
    pub const Period: Self = Self(3);
    pub const Plain: Self = Self(4);
    pub const Minus: Self = Self(5);
    pub const NoNumber: Self = Self(6);
}
impl windows_core::imp::TypeKind for MarkerStyle {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MarkerStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.MarkerStyle;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MarkerType(pub i32);
impl MarkerType {
    pub const Undefined: Self = Self(0);
    pub const None: Self = Self(1);
    pub const Bullet: Self = Self(2);
    pub const Arabic: Self = Self(3);
    pub const LowercaseEnglishLetter: Self = Self(4);
    pub const UppercaseEnglishLetter: Self = Self(5);
    pub const LowercaseRoman: Self = Self(6);
    pub const UppercaseRoman: Self = Self(7);
    pub const UnicodeSequence: Self = Self(8);
    pub const CircledNumber: Self = Self(9);
    pub const BlackCircleWingding: Self = Self(10);
    pub const WhiteCircleWingding: Self = Self(11);
    pub const ArabicWide: Self = Self(12);
    pub const SimplifiedChinese: Self = Self(13);
    pub const TraditionalChinese: Self = Self(14);
    pub const JapanSimplifiedChinese: Self = Self(15);
    pub const JapanKorea: Self = Self(16);
    pub const ArabicDictionary: Self = Self(17);
    pub const ArabicAbjad: Self = Self(18);
    pub const Hebrew: Self = Self(19);
    pub const ThaiAlphabetic: Self = Self(20);
    pub const ThaiNumeric: Self = Self(21);
    pub const DevanagariVowel: Self = Self(22);
    pub const DevanagariConsonant: Self = Self(23);
    pub const DevanagariNumeric: Self = Self(24);
}
impl windows_core::imp::TypeKind for MarkerType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MarkerType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.MarkerType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuFlyout(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuFlyout,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(MenuFlyout, FlyoutBase, DependencyObject);
impl MenuFlyout {
    pub fn new() -> windows_core::Result<Self> {
        Self::IMenuFlyoutFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IMenuFlyoutFactory<R, F: FnOnce(&IMenuFlyoutFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MenuFlyout, IMenuFlyoutFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuFlyout>();
}
unsafe impl windows_core::Interface for MenuFlyout {
    type Vtable = <IMenuFlyout as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuFlyout as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuFlyout {
    type Target = IMenuFlyout;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuFlyout {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuFlyout";
}
unsafe impl Send for MenuFlyout {}
unsafe impl Sync for MenuFlyout {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuFlyoutItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuFlyoutItem,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuFlyoutItem,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    MenuFlyoutItemBase,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl MenuFlyoutItem {
    pub fn new() -> windows_core::Result<Self> {
        Self::IMenuFlyoutItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IMenuFlyoutItemFactory<R, F: FnOnce(&IMenuFlyoutItemFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MenuFlyoutItem, IMenuFlyoutItemFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MenuFlyoutItem {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuFlyoutItem>();
}
unsafe impl windows_core::Interface for MenuFlyoutItem {
    type Vtable = <IMenuFlyoutItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuFlyoutItem as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuFlyoutItem {
    type Target = IMenuFlyoutItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuFlyoutItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuFlyoutItem";
}
unsafe impl Send for MenuFlyoutItem {}
unsafe impl Sync for MenuFlyoutItem {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuFlyoutItemBase(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MenuFlyoutItemBase,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    MenuFlyoutItemBase,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl windows_core::RuntimeType for MenuFlyoutItemBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMenuFlyoutItemBase>();
}
unsafe impl windows_core::Interface for MenuFlyoutItemBase {
    type Vtable = <IMenuFlyoutItemBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMenuFlyoutItemBase as windows_core::Interface>::IID;
}
impl core::ops::Deref for MenuFlyoutItemBase {
    type Target = IMenuFlyoutItemBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MenuFlyoutItemBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.MenuFlyoutItemBase";
}
unsafe impl Send for MenuFlyoutItemBase {}
unsafe impl Sync for MenuFlyoutItemBase {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParagraphAlignment(pub i32);
impl ParagraphAlignment {
    pub const Undefined: Self = Self(0);
    pub const Left: Self = Self(1);
    pub const Center: Self = Self(2);
    pub const Right: Self = Self(3);
    pub const Justify: Self = Self(4);
}
impl windows_core::imp::TypeKind for ParagraphAlignment {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ParagraphAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Text.ParagraphAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParagraphStyle(pub i32);
impl ParagraphStyle {
    pub const Undefined: Self = Self(0);
    pub const None: Self = Self(1);
    pub const Normal: Self = Self(2);
    pub const Heading1: Self = Self(3);
    pub const Heading2: Self = Self(4);
    pub const Heading3: Self = Self(5);
    pub const Heading4: Self = Self(6);
    pub const Heading5: Self = Self(7);
    pub const Heading6: Self = Self(8);
    pub const Heading7: Self = Self(9);
    pub const Heading8: Self = Self(10);
    pub const Heading9: Self = Self(11);
}
impl windows_core::imp::TypeKind for ParagraphStyle {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ParagraphStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.ParagraphStyle;i4)");
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
pub struct PointOptions(pub u32);
impl PointOptions {
    pub const None: Self = Self(0);
    pub const IncludeInset: Self = Self(1);
    pub const Start: Self = Self(32);
    pub const ClientCoordinates: Self = Self(256);
    pub const AllowOffClient: Self = Self(512);
    pub const Transform: Self = Self(1024);
    pub const NoHorizontalScroll: Self = Self(65536);
    pub const NoVerticalScroll: Self = Self(262144);
}
impl windows_core::imp::TypeKind for PointOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PointOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.PointOptions;u4)");
}
impl PointOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PointOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PointOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PointOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for PointOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for PointOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
windows_core::imp::define_interface!(
    PointerEventHandler,
    PointerEventHandler_Vtbl,
    0xa48a71e1_8bb4_5597_9e31_903a3f6a04fb
);
impl windows_core::RuntimeType for PointerEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl PointerEventHandler {
    pub fn new<
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<PointerRoutedEventArgs>,
            ) + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &PointerEventHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct PointerEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct PointerEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<PointerRoutedEventArgs>)
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<PointerRoutedEventArgs>)
        + 'static,
> PointerEventHandlerBox<F>
{
    const VTABLE: PointerEventHandler_Vtbl = PointerEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<PointerEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<PointerEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<PointerEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<PointerEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerPoint,
    windows_core::IUnknown,
    windows_core::IInspectable
);
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
pub struct PointerRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    PointerRoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(PointerRoutedEventArgs, RoutedEventArgs);
impl windows_core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IPointerRoutedEventArgs>();
}
unsafe impl windows_core::Interface for PointerRoutedEventArgs {
    type Vtable = <IPointerRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerRoutedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for PointerRoutedEventArgs {
    type Target = IPointerRoutedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.PointerRoutedEventArgs";
}
unsafe impl Send for PointerRoutedEventArgs {}
unsafe impl Sync for PointerRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RangeGravity(pub i32);
impl RangeGravity {
    pub const UIBehavior: Self = Self(0);
    pub const Backward: Self = Self(1);
    pub const Forward: Self = Self(2);
    pub const Inward: Self = Self(3);
    pub const Outward: Self = Self(4);
}
impl windows_core::imp::TypeKind for RangeGravity {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for RangeGravity {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.RangeGravity;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl windows_core::imp::TypeKind for Rect {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Rect {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RichEditClipboardFormat(pub i32);
impl RichEditClipboardFormat {
    pub const AllFormats: Self = Self(0);
    pub const PlainText: Self = Self(1);
}
impl windows_core::imp::TypeKind for RichEditClipboardFormat {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for RichEditClipboardFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.RichEditClipboardFormat;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RichEditMathMode(pub i32);
impl RichEditMathMode {
    pub const NoMath: Self = Self(0);
    pub const MathOnly: Self = Self(1);
}
impl windows_core::imp::TypeKind for RichEditMathMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for RichEditMathMode {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.RichEditMathMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichEditTextDocument(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RichEditTextDocument,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for RichEditTextDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextDocument>();
}
unsafe impl windows_core::Interface for RichEditTextDocument {
    type Vtable = <ITextDocument as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextDocument as windows_core::Interface>::IID;
}
impl core::ops::Deref for RichEditTextDocument {
    type Target = ITextDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RichEditTextDocument {
    const NAME: &'static str = "Microsoft.UI.Text.RichEditTextDocument";
}
unsafe impl Send for RichEditTextDocument {}
unsafe impl Sync for RichEditTextDocument {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichEditTextRange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RichEditTextRange,
    windows_core::IUnknown,
    windows_core::IInspectable,
    ITextRange
);
impl windows_core::RuntimeType for RichEditTextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextRange>();
}
unsafe impl windows_core::Interface for RichEditTextRange {
    type Vtable = <ITextRange as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextRange as windows_core::Interface>::IID;
}
impl core::ops::Deref for RichEditTextRange {
    type Target = ITextRange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RichEditTextRange {
    const NAME: &'static str = "Microsoft.UI.Text.RichEditTextRange";
}
unsafe impl Send for RichEditTextRange {}
unsafe impl Sync for RichEditTextRange {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    RoutedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl RoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IRoutedEventArgsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IRoutedEventArgsFactory<
        R,
        F: FnOnce(&IRoutedEventArgsFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RoutedEventArgs, IRoutedEventArgsFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IRoutedEventArgs>();
}
unsafe impl windows_core::Interface for RoutedEventArgs {
    type Vtable = <IRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRoutedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for RoutedEventArgs {
    type Target = IRoutedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for RoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.RoutedEventArgs";
}
unsafe impl Send for RoutedEventArgs {}
unsafe impl Sync for RoutedEventArgs {}
windows_core::imp::define_interface!(
    RoutedEventHandler,
    RoutedEventHandler_Vtbl,
    0xdae23d85_69ca_5bdf_805b_6161a3a215cc
);
impl windows_core::RuntimeType for RoutedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl RoutedEventHandler {
    pub fn new<
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>)
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &RoutedEventHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct RoutedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct RoutedEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) + 'static,
> RoutedEventHandlerBox<F>
{
    const VTABLE: RoutedEventHandler_Vtbl = RoutedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<RoutedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScrollBarVisibility(pub i32);
impl ScrollBarVisibility {
    pub const Disabled: Self = Self(0);
    pub const Auto: Self = Self(1);
    pub const Hidden: Self = Self(2);
    pub const Visible: Self = Self(3);
}
impl windows_core::imp::TypeKind for ScrollBarVisibility {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ScrollBarVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Controls.ScrollBarVisibility;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SelectionOptions(pub u32);
impl SelectionOptions {
    pub const StartActive: Self = Self(1);
    pub const AtEndOfLine: Self = Self(2);
    pub const Overtype: Self = Self(4);
    pub const Active: Self = Self(8);
    pub const Replace: Self = Self(16);
}
impl windows_core::imp::TypeKind for SelectionOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for SelectionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.SelectionOptions;u4)");
}
impl SelectionOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SelectionOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SelectionOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SelectionOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for SelectionOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for SelectionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SelectionType(pub i32);
impl SelectionType {
    pub const None: Self = Self(0);
    pub const InsertionPoint: Self = Self(1);
    pub const Normal: Self = Self(2);
    pub const InlineShape: Self = Self(7);
    pub const Shape: Self = Self(8);
}
impl windows_core::imp::TypeKind for SelectionType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for SelectionType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.SelectionType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SizeChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SizeChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SizeChangedEventArgs, RoutedEventArgs);
impl windows_core::RuntimeType for SizeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISizeChangedEventArgs>();
}
unsafe impl windows_core::Interface for SizeChangedEventArgs {
    type Vtable = <ISizeChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISizeChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for SizeChangedEventArgs {
    type Target = ISizeChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SizeChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.SizeChangedEventArgs";
}
unsafe impl Send for SizeChangedEventArgs {}
unsafe impl Sync for SizeChangedEventArgs {}
windows_core::imp::define_interface!(
    SizeChangedEventHandler,
    SizeChangedEventHandler_Vtbl,
    0x8d7b1a58_14c6_51c9_892c_9fcce368e77d
);
impl windows_core::RuntimeType for SizeChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl SizeChangedEventHandler {
    pub fn new<
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SizeChangedEventArgs>,
            ) + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &SizeChangedEventHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct SizeChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct SizeChangedEventHandlerBox<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<SizeChangedEventArgs>)
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<SizeChangedEventArgs>)
        + 'static,
> SizeChangedEventHandlerBox<F>
{
    const VTABLE: SizeChangedEventHandler_Vtbl = SizeChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SizeChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SizeChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SizeChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SizeChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
pub const SpellCheckerFactory: windows_core::GUID =
    windows_core::GUID::from_u128(0x7ab36653_1796_484b_bdfa_e74f1db7c1dc);
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Style, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Style, DependencyObject);
impl windows_core::RuntimeType for Style {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IStyle>();
}
unsafe impl windows_core::Interface for Style {
    type Vtable = <IStyle as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStyle as windows_core::Interface>::IID;
}
impl core::ops::Deref for Style {
    type Target = IStyle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Style {
    const NAME: &'static str = "Microsoft.UI.Xaml.Style";
}
unsafe impl Send for Style {}
unsafe impl Sync for Style {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TabAlignment(pub i32);
impl TabAlignment {
    pub const Left: Self = Self(0);
    pub const Center: Self = Self(1);
    pub const Right: Self = Self(2);
    pub const Decimal: Self = Self(3);
    pub const Bar: Self = Self(4);
}
impl windows_core::imp::TypeKind for TabAlignment {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TabAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TabAlignment;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TabLeader(pub i32);
impl TabLeader {
    pub const Spaces: Self = Self(0);
    pub const Dots: Self = Self(1);
    pub const Dashes: Self = Self(2);
    pub const Lines: Self = Self(3);
    pub const ThickLines: Self = Self(4);
    pub const Equals: Self = Self(5);
}
impl windows_core::imp::TypeKind for TabLeader {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TabLeader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TabLeader;i4)");
}
pub struct TextConstants;
impl TextConstants {
    pub fn AutoColor() -> windows_core::Result<Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoColor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn MinUnitCount() -> windows_core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinUnitCount)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn MaxUnitCount() -> windows_core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxUnitCount)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn UndefinedColor() -> windows_core::Result<Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndefinedColor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn UndefinedFloatValue() -> windows_core::Result<f32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndefinedFloatValue)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn UndefinedInt32Value() -> windows_core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndefinedInt32Value)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    fn ITextConstantsStatics<R, F: FnOnce(&ITextConstantsStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextConstants, ITextConstantsStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for TextConstants {
    const NAME: &'static str = "Microsoft.UI.Text.TextConstants";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextControlPasteEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    TextControlPasteEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for TextControlPasteEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITextControlPasteEventArgs>();
}
unsafe impl windows_core::Interface for TextControlPasteEventArgs {
    type Vtable = <ITextControlPasteEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextControlPasteEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for TextControlPasteEventArgs {
    type Target = ITextControlPasteEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for TextControlPasteEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TextControlPasteEventArgs";
}
unsafe impl Send for TextControlPasteEventArgs {}
unsafe impl Sync for TextControlPasteEventArgs {}
windows_core::imp::define_interface!(
    TextControlPasteEventHandler,
    TextControlPasteEventHandler_Vtbl,
    0xddf3a6c0_5ab1_5d74_8289_892c0614658f
);
impl windows_core::RuntimeType for TextControlPasteEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl TextControlPasteEventHandler {
    pub fn new<
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<TextControlPasteEventArgs>,
            ) + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &TextControlPasteEventHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct TextControlPasteEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
struct TextControlPasteEventHandlerBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<TextControlPasteEventArgs>,
        ) + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<TextControlPasteEventArgs>,
        ) + 'static,
> TextControlPasteEventHandlerBox<F>
{
    const VTABLE: TextControlPasteEventHandler_Vtbl = TextControlPasteEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<TextControlPasteEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<TextControlPasteEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<TextControlPasteEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<TextControlPasteEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextGetOptions(pub u32);
impl TextGetOptions {
    pub const None: Self = Self(0);
    pub const AdjustCrlf: Self = Self(1);
    pub const UseCrlf: Self = Self(2);
    pub const UseObjectText: Self = Self(4);
    pub const AllowFinalEop: Self = Self(8);
    pub const NoHidden: Self = Self(32);
    pub const IncludeNumbering: Self = Self(64);
    pub const FormatRtf: Self = Self(8192);
    pub const UseLf: Self = Self(16777216);
}
impl windows_core::imp::TypeKind for TextGetOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TextGetOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextGetOptions;u4)");
}
impl TextGetOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextGetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextGetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextGetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for TextGetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for TextGetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextRangeUnit(pub i32);
impl TextRangeUnit {
    pub const Character: Self = Self(0);
    pub const Word: Self = Self(1);
    pub const Sentence: Self = Self(2);
    pub const Paragraph: Self = Self(3);
    pub const Line: Self = Self(4);
    pub const Story: Self = Self(5);
    pub const Screen: Self = Self(6);
    pub const Section: Self = Self(7);
    pub const Window: Self = Self(8);
    pub const CharacterFormat: Self = Self(9);
    pub const ParagraphFormat: Self = Self(10);
    pub const Object: Self = Self(11);
    pub const HardParagraph: Self = Self(12);
    pub const Cluster: Self = Self(13);
    pub const Bold: Self = Self(14);
    pub const Italic: Self = Self(15);
    pub const Underline: Self = Self(16);
    pub const Strikethrough: Self = Self(17);
    pub const ProtectedText: Self = Self(18);
    pub const Link: Self = Self(19);
    pub const SmallCaps: Self = Self(20);
    pub const AllCaps: Self = Self(21);
    pub const Hidden: Self = Self(22);
    pub const Outline: Self = Self(23);
    pub const Shadow: Self = Self(24);
    pub const Imprint: Self = Self(25);
    pub const Disabled: Self = Self(26);
    pub const Revised: Self = Self(27);
    pub const Subscript: Self = Self(28);
    pub const Superscript: Self = Self(29);
    pub const FontBound: Self = Self(30);
    pub const LinkProtected: Self = Self(31);
    pub const ContentLink: Self = Self(32);
}
impl windows_core::imp::TypeKind for TextRangeUnit {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TextRangeUnit {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextRangeUnit;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextScript(pub i32);
impl TextScript {
    pub const Undefined: Self = Self(0);
    pub const Ansi: Self = Self(1);
    pub const EastEurope: Self = Self(2);
    pub const Cyrillic: Self = Self(3);
    pub const Greek: Self = Self(4);
    pub const Turkish: Self = Self(5);
    pub const Hebrew: Self = Self(6);
    pub const Arabic: Self = Self(7);
    pub const Baltic: Self = Self(8);
    pub const Vietnamese: Self = Self(9);
    pub const Default: Self = Self(10);
    pub const Symbol: Self = Self(11);
    pub const Thai: Self = Self(12);
    pub const ShiftJis: Self = Self(13);
    pub const GB2312: Self = Self(14);
    pub const Hangul: Self = Self(15);
    pub const Big5: Self = Self(16);
    pub const PC437: Self = Self(17);
    pub const Oem: Self = Self(18);
    pub const Mac: Self = Self(19);
    pub const Armenian: Self = Self(20);
    pub const Syriac: Self = Self(21);
    pub const Thaana: Self = Self(22);
    pub const Devanagari: Self = Self(23);
    pub const Bengali: Self = Self(24);
    pub const Gurmukhi: Self = Self(25);
    pub const Gujarati: Self = Self(26);
    pub const Oriya: Self = Self(27);
    pub const Tamil: Self = Self(28);
    pub const Telugu: Self = Self(29);
    pub const Kannada: Self = Self(30);
    pub const Malayalam: Self = Self(31);
    pub const Sinhala: Self = Self(32);
    pub const Lao: Self = Self(33);
    pub const Tibetan: Self = Self(34);
    pub const Myanmar: Self = Self(35);
    pub const Georgian: Self = Self(36);
    pub const Jamo: Self = Self(37);
    pub const Ethiopic: Self = Self(38);
    pub const Cherokee: Self = Self(39);
    pub const Aboriginal: Self = Self(40);
    pub const Ogham: Self = Self(41);
    pub const Runic: Self = Self(42);
    pub const Khmer: Self = Self(43);
    pub const Mongolian: Self = Self(44);
    pub const Braille: Self = Self(45);
    pub const Yi: Self = Self(46);
    pub const Limbu: Self = Self(47);
    pub const TaiLe: Self = Self(48);
    pub const NewTaiLue: Self = Self(49);
    pub const SylotiNagri: Self = Self(50);
    pub const Kharoshthi: Self = Self(51);
    pub const Kayahli: Self = Self(52);
    pub const UnicodeSymbol: Self = Self(53);
    pub const Emoji: Self = Self(54);
    pub const Glagolitic: Self = Self(55);
    pub const Lisu: Self = Self(56);
    pub const Vai: Self = Self(57);
    pub const NKo: Self = Self(58);
    pub const Osmanya: Self = Self(59);
    pub const PhagsPa: Self = Self(60);
    pub const Gothic: Self = Self(61);
    pub const Deseret: Self = Self(62);
    pub const Tifinagh: Self = Self(63);
}
impl windows_core::imp::TypeKind for TextScript {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TextScript {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextScript;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextSetOptions(pub u32);
impl TextSetOptions {
    pub const None: Self = Self(0);
    pub const UnicodeBidi: Self = Self(1);
    pub const Unlink: Self = Self(8);
    pub const Unhide: Self = Self(16);
    pub const CheckTextLimit: Self = Self(32);
    pub const FormatRtf: Self = Self(8192);
    pub const ApplyRtfDocumentDefaults: Self = Self(16384);
}
impl windows_core::imp::TypeKind for TextSetOptions {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TextSetOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextSetOptions;u4)");
}
impl TextSetOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextSetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextSetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextSetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for TextSetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for TextSetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolTip(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToolTip,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    ToolTip,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    ContentControl,
    Control,
    FrameworkElement,
    UIElement,
    DependencyObject
);
impl ToolTip {
    pub fn new() -> windows_core::Result<Self> {
        Self::IToolTipFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IToolTipFactory<R, F: FnOnce(&IToolTipFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToolTip, IToolTipFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToolTip {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToolTip>();
}
unsafe impl windows_core::Interface for ToolTip {
    type Vtable = <IToolTip as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToolTip as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToolTip {
    type Target = IToolTip;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToolTip {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ToolTip";
}
unsafe impl Send for ToolTip {}
unsafe impl Sync for ToolTip {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolTipService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ToolTipService,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl ToolTipService {
    pub fn SetToolTip<P0, P1>(element: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        Self::IToolTipServiceStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).SetToolTip)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
                value.param().abi(),
            )
            .ok()
        })
    }
    fn IToolTipServiceStatics<R, F: FnOnce(&IToolTipServiceStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToolTipService, IToolTipServiceStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToolTipService {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IToolTipService>();
}
unsafe impl windows_core::Interface for ToolTipService {
    type Vtable = <IToolTipService as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToolTipService as windows_core::Interface>::IID;
}
impl core::ops::Deref for ToolTipService {
    type Target = IToolTipService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ToolTipService {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ToolTipService";
}
unsafe impl Send for ToolTipService {}
unsafe impl Sync for ToolTipService {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedEventHandler<TSender, TResult>(
    windows_core::IUnknown,
    core::marker::PhantomData<TSender>,
    core::marker::PhantomData<TResult>,
)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
unsafe impl<
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
> windows_core::Interface for TypedEventHandler<TSender, TResult>
{
    type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static>
    windows_core::RuntimeType for TypedEventHandler<TSender, TResult>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}")
        .push_slice(b";")
        .push_other(TSender::SIGNATURE)
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b")");
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static>
    TypedEventHandler<TSender, TResult>
{
    pub fn new<F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) + 'static>(
        invoke: F,
    ) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct TypedEventHandler_Vtbl<TSender, TResult>
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: windows_core::imp::AbiType<TSender>,
        args: windows_core::imp::AbiType<TResult>,
    ) -> windows_core::HRESULT,
    TSender: core::marker::PhantomData<TSender>,
    TResult: core::marker::PhantomData<TResult>,
}
struct TypedEventHandlerBox<
    TSender,
    TResult,
    F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) + 'static,
>(core::marker::PhantomData<(TSender, TResult, fn() -> F)>)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
impl<
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
    F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) + 'static,
> TypedEventHandlerBox<TSender, TResult, F>
{
    const VTABLE : TypedEventHandler_Vtbl < TSender , TResult , > = TypedEventHandler_Vtbl::< TSender , TResult , > { base__ : windows_core::IUnknown_Vtbl { QueryInterface : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::QueryInterface , AddRef : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::AddRef , Release : windows_core::imp::DelegateBox::< TypedEventHandler < TSender , TResult > , F >::Release , } , Invoke : Self::Invoke , TSender : core::marker::PhantomData::< TSender > , TResult : core::marker::PhantomData::< TResult > } ;
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: windows_core::imp::AbiType<TSender>,
        args: windows_core::imp::AbiType<TResult>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<TypedEventHandler<TSender, TResult>, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&args),
            );
            windows_core::HRESULT(0)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIColorType(pub i32);
impl UIColorType {
    pub const Background: Self = Self(0);
    pub const Foreground: Self = Self(1);
    pub const AccentDark3: Self = Self(2);
    pub const AccentDark2: Self = Self(3);
    pub const AccentDark1: Self = Self(4);
    pub const Accent: Self = Self(5);
    pub const AccentLight1: Self = Self(6);
    pub const AccentLight2: Self = Self(7);
    pub const AccentLight3: Self = Self(8);
    pub const Complement: Self = Self(9);
}
impl windows_core::imp::TypeKind for UIColorType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for UIColorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.UI.ViewManagement.UIColorType;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UIElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UIElement,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    UIElement,
    IAnimationObject,
    IVisualElement,
    IVisualElement2,
    DependencyObject
);
impl UIElement {
    pub fn RegisterAsScrollPort<P0>(element: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        Self::IUIElementStatics(|this| unsafe {
            (windows_core::Interface::vtable(this).RegisterAsScrollPort)(
                windows_core::Interface::as_raw(this),
                element.param().abi(),
            )
            .ok()
        })
    }
    fn IUIElementStatics<R, F: FnOnce(&IUIElementStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUIElement>();
}
unsafe impl windows_core::Interface for UIElement {
    type Vtable = <IUIElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUIElement as windows_core::Interface>::IID;
}
impl core::ops::Deref for UIElement {
    type Target = IUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UIElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.UIElement";
}
unsafe impl Send for UIElement {}
unsafe impl Sync for UIElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UISettings,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for UISettings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUISettings>();
}
unsafe impl windows_core::Interface for UISettings {
    type Vtable = <IUISettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUISettings as windows_core::Interface>::IID;
}
impl core::ops::Deref for UISettings {
    type Target = IUISettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UISettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettings";
}
unsafe impl Send for UISettings {}
unsafe impl Sync for UISettings {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettingsAnimationsEnabledChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UISettingsAnimationsEnabledChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for UISettingsAnimationsEnabledChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IUISettingsAnimationsEnabledChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for UISettingsAnimationsEnabledChangedEventArgs {
    type Vtable = <IUISettingsAnimationsEnabledChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IUISettingsAnimationsEnabledChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for UISettingsAnimationsEnabledChangedEventArgs {
    type Target = IUISettingsAnimationsEnabledChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UISettingsAnimationsEnabledChangedEventArgs {
    const NAME: &'static str =
        "Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs";
}
unsafe impl Send for UISettingsAnimationsEnabledChangedEventArgs {}
unsafe impl Sync for UISettingsAnimationsEnabledChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettingsAutoHideScrollBarsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UISettingsAutoHideScrollBarsChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for UISettingsAutoHideScrollBarsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IUISettingsAutoHideScrollBarsChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for UISettingsAutoHideScrollBarsChangedEventArgs {
    type Vtable =
        <IUISettingsAutoHideScrollBarsChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IUISettingsAutoHideScrollBarsChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for UISettingsAutoHideScrollBarsChangedEventArgs {
    type Target = IUISettingsAutoHideScrollBarsChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UISettingsAutoHideScrollBarsChangedEventArgs {
    const NAME: &'static str =
        "Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs";
}
unsafe impl Send for UISettingsAutoHideScrollBarsChangedEventArgs {}
unsafe impl Sync for UISettingsAutoHideScrollBarsChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettingsMessageDurationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    UISettingsMessageDurationChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for UISettingsMessageDurationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        IUISettingsMessageDurationChangedEventArgs,
    >();
}
unsafe impl windows_core::Interface for UISettingsMessageDurationChangedEventArgs {
    type Vtable = <IUISettingsMessageDurationChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IUISettingsMessageDurationChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for UISettingsMessageDurationChangedEventArgs {
    type Target = IUISettingsMessageDurationChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for UISettingsMessageDurationChangedEventArgs {
    const NAME: &'static str =
        "Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs";
}
unsafe impl Send for UISettingsMessageDurationChangedEventArgs {}
unsafe impl Sync for UISettingsMessageDurationChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UnderlineType(pub i32);
impl UnderlineType {
    pub const Undefined: Self = Self(0);
    pub const None: Self = Self(1);
    pub const Single: Self = Self(2);
    pub const Words: Self = Self(3);
    pub const Double: Self = Self(4);
    pub const Dotted: Self = Self(5);
    pub const Dash: Self = Self(6);
    pub const DashDot: Self = Self(7);
    pub const DashDotDot: Self = Self(8);
    pub const Wave: Self = Self(9);
    pub const Thick: Self = Self(10);
    pub const Thin: Self = Self(11);
    pub const DoubleWave: Self = Self(12);
    pub const HeavyWave: Self = Self(13);
    pub const LongDash: Self = Self(14);
    pub const ThickDash: Self = Self(15);
    pub const ThickDashDot: Self = Self(16);
    pub const ThickDashDotDot: Self = Self(17);
    pub const ThickDotted: Self = Self(18);
    pub const ThickLongDash: Self = Self(19);
}
impl windows_core::imp::TypeKind for UnderlineType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for UnderlineType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.UnderlineType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VerticalCharacterAlignment(pub i32);
impl VerticalCharacterAlignment {
    pub const Top: Self = Self(0);
    pub const Baseline: Self = Self(1);
    pub const Bottom: Self = Self(2);
}
impl windows_core::imp::TypeKind for VerticalCharacterAlignment {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for VerticalCharacterAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Text.VerticalCharacterAlignment;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: Self = Self(0);
    pub const LeftButton: Self = Self(1);
    pub const RightButton: Self = Self(2);
    pub const Cancel: Self = Self(3);
    pub const MiddleButton: Self = Self(4);
    pub const XButton1: Self = Self(5);
    pub const XButton2: Self = Self(6);
    pub const Back: Self = Self(8);
    pub const Tab: Self = Self(9);
    pub const Clear: Self = Self(12);
    pub const Enter: Self = Self(13);
    pub const Shift: Self = Self(16);
    pub const Control: Self = Self(17);
    pub const Menu: Self = Self(18);
    pub const Pause: Self = Self(19);
    pub const CapitalLock: Self = Self(20);
    pub const Kana: Self = Self(21);
    pub const Hangul: Self = Self(21);
    pub const ImeOn: Self = Self(22);
    pub const Junja: Self = Self(23);
    pub const Final: Self = Self(24);
    pub const Hanja: Self = Self(25);
    pub const Kanji: Self = Self(25);
    pub const ImeOff: Self = Self(26);
    pub const Escape: Self = Self(27);
    pub const Convert: Self = Self(28);
    pub const NonConvert: Self = Self(29);
    pub const Accept: Self = Self(30);
    pub const ModeChange: Self = Self(31);
    pub const Space: Self = Self(32);
    pub const PageUp: Self = Self(33);
    pub const PageDown: Self = Self(34);
    pub const End: Self = Self(35);
    pub const Home: Self = Self(36);
    pub const Left: Self = Self(37);
    pub const Up: Self = Self(38);
    pub const Right: Self = Self(39);
    pub const Down: Self = Self(40);
    pub const Select: Self = Self(41);
    pub const Print: Self = Self(42);
    pub const Execute: Self = Self(43);
    pub const Snapshot: Self = Self(44);
    pub const Insert: Self = Self(45);
    pub const Delete: Self = Self(46);
    pub const Help: Self = Self(47);
    pub const Number0: Self = Self(48);
    pub const Number1: Self = Self(49);
    pub const Number2: Self = Self(50);
    pub const Number3: Self = Self(51);
    pub const Number4: Self = Self(52);
    pub const Number5: Self = Self(53);
    pub const Number6: Self = Self(54);
    pub const Number7: Self = Self(55);
    pub const Number8: Self = Self(56);
    pub const Number9: Self = Self(57);
    pub const A: Self = Self(65);
    pub const B: Self = Self(66);
    pub const C: Self = Self(67);
    pub const D: Self = Self(68);
    pub const E: Self = Self(69);
    pub const F: Self = Self(70);
    pub const G: Self = Self(71);
    pub const H: Self = Self(72);
    pub const I: Self = Self(73);
    pub const J: Self = Self(74);
    pub const K: Self = Self(75);
    pub const L: Self = Self(76);
    pub const M: Self = Self(77);
    pub const N: Self = Self(78);
    pub const O: Self = Self(79);
    pub const P: Self = Self(80);
    pub const Q: Self = Self(81);
    pub const R: Self = Self(82);
    pub const S: Self = Self(83);
    pub const T: Self = Self(84);
    pub const U: Self = Self(85);
    pub const V: Self = Self(86);
    pub const W: Self = Self(87);
    pub const X: Self = Self(88);
    pub const Y: Self = Self(89);
    pub const Z: Self = Self(90);
    pub const LeftWindows: Self = Self(91);
    pub const RightWindows: Self = Self(92);
    pub const Application: Self = Self(93);
    pub const Sleep: Self = Self(95);
    pub const NumberPad0: Self = Self(96);
    pub const NumberPad1: Self = Self(97);
    pub const NumberPad2: Self = Self(98);
    pub const NumberPad3: Self = Self(99);
    pub const NumberPad4: Self = Self(100);
    pub const NumberPad5: Self = Self(101);
    pub const NumberPad6: Self = Self(102);
    pub const NumberPad7: Self = Self(103);
    pub const NumberPad8: Self = Self(104);
    pub const NumberPad9: Self = Self(105);
    pub const Multiply: Self = Self(106);
    pub const Add: Self = Self(107);
    pub const Separator: Self = Self(108);
    pub const Subtract: Self = Self(109);
    pub const Decimal: Self = Self(110);
    pub const Divide: Self = Self(111);
    pub const F1: Self = Self(112);
    pub const F2: Self = Self(113);
    pub const F3: Self = Self(114);
    pub const F4: Self = Self(115);
    pub const F5: Self = Self(116);
    pub const F6: Self = Self(117);
    pub const F7: Self = Self(118);
    pub const F8: Self = Self(119);
    pub const F9: Self = Self(120);
    pub const F10: Self = Self(121);
    pub const F11: Self = Self(122);
    pub const F12: Self = Self(123);
    pub const F13: Self = Self(124);
    pub const F14: Self = Self(125);
    pub const F15: Self = Self(126);
    pub const F16: Self = Self(127);
    pub const F17: Self = Self(128);
    pub const F18: Self = Self(129);
    pub const F19: Self = Self(130);
    pub const F20: Self = Self(131);
    pub const F21: Self = Self(132);
    pub const F22: Self = Self(133);
    pub const F23: Self = Self(134);
    pub const F24: Self = Self(135);
    pub const NavigationView: Self = Self(136);
    pub const NavigationMenu: Self = Self(137);
    pub const NavigationUp: Self = Self(138);
    pub const NavigationDown: Self = Self(139);
    pub const NavigationLeft: Self = Self(140);
    pub const NavigationRight: Self = Self(141);
    pub const NavigationAccept: Self = Self(142);
    pub const NavigationCancel: Self = Self(143);
    pub const NumberKeyLock: Self = Self(144);
    pub const Scroll: Self = Self(145);
    pub const LeftShift: Self = Self(160);
    pub const RightShift: Self = Self(161);
    pub const LeftControl: Self = Self(162);
    pub const RightControl: Self = Self(163);
    pub const LeftMenu: Self = Self(164);
    pub const RightMenu: Self = Self(165);
    pub const GoBack: Self = Self(166);
    pub const GoForward: Self = Self(167);
    pub const Refresh: Self = Self(168);
    pub const Stop: Self = Self(169);
    pub const Search: Self = Self(170);
    pub const Favorites: Self = Self(171);
    pub const GoHome: Self = Self(172);
    pub const GamepadA: Self = Self(195);
    pub const GamepadB: Self = Self(196);
    pub const GamepadX: Self = Self(197);
    pub const GamepadY: Self = Self(198);
    pub const GamepadRightShoulder: Self = Self(199);
    pub const GamepadLeftShoulder: Self = Self(200);
    pub const GamepadLeftTrigger: Self = Self(201);
    pub const GamepadRightTrigger: Self = Self(202);
    pub const GamepadDPadUp: Self = Self(203);
    pub const GamepadDPadDown: Self = Self(204);
    pub const GamepadDPadLeft: Self = Self(205);
    pub const GamepadDPadRight: Self = Self(206);
    pub const GamepadMenu: Self = Self(207);
    pub const GamepadView: Self = Self(208);
    pub const GamepadLeftThumbstickButton: Self = Self(209);
    pub const GamepadRightThumbstickButton: Self = Self(210);
    pub const GamepadLeftThumbstickUp: Self = Self(211);
    pub const GamepadLeftThumbstickDown: Self = Self(212);
    pub const GamepadLeftThumbstickRight: Self = Self(213);
    pub const GamepadLeftThumbstickLeft: Self = Self(214);
    pub const GamepadRightThumbstickUp: Self = Self(215);
    pub const GamepadRightThumbstickDown: Self = Self(216);
    pub const GamepadRightThumbstickRight: Self = Self(217);
    pub const GamepadRightThumbstickLeft: Self = Self(218);
}
impl windows_core::imp::TypeKind for VirtualKey {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for VirtualKey {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VisualTreeHelper(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    VisualTreeHelper,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl VisualTreeHelper {
    pub fn GetChild<P0>(reference: P0, childindex: i32) -> windows_core::Result<DependencyObject>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChild)(
                windows_core::Interface::as_raw(this),
                reference.param().abi(),
                childindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn GetChildrenCount<P0>(reference: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenCount)(
                windows_core::Interface::as_raw(this),
                reference.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn GetParent<P0>(reference: P0) -> windows_core::Result<DependencyObject>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParent)(
                windows_core::Interface::as_raw(this),
                reference.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IVisualTreeHelperStatics<
        R,
        F: FnOnce(&IVisualTreeHelperStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VisualTreeHelper, IVisualTreeHelperStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VisualTreeHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVisualTreeHelper>();
}
unsafe impl windows_core::Interface for VisualTreeHelper {
    type Vtable = <IVisualTreeHelper as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVisualTreeHelper as windows_core::Interface>::IID;
}
impl core::ops::Deref for VisualTreeHelper {
    type Target = IVisualTreeHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for VisualTreeHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.VisualTreeHelper";
}
unsafe impl Send for VisualTreeHelper {}
unsafe impl Sync for VisualTreeHelper {}
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
pub struct XamlReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlReader,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl XamlReader {
    pub fn Load(xaml: &str) -> windows_core::Result<windows_core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Load)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&windows_core::HSTRING::from(xaml)),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XamlReader, IXamlReaderStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlReader {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlReader>();
}
unsafe impl windows_core::Interface for XamlReader {
    type Vtable = <IXamlReader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlReader as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlReader {
    type Target = IXamlReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlReader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlReader";
}
unsafe impl Send for XamlReader {}
unsafe impl Sync for XamlReader {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlRoot(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlRoot,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for XamlRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlRoot>();
}
unsafe impl windows_core::Interface for XamlRoot {
    type Vtable = <IXamlRoot as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlRoot as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlRoot {
    type Target = IXamlRoot;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlRoot {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlRoot";
}
unsafe impl Send for XamlRoot {}
unsafe impl Sync for XamlRoot {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlRootChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    XamlRootChangedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for XamlRootChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IXamlRootChangedEventArgs>();
}
unsafe impl windows_core::Interface for XamlRootChangedEventArgs {
    type Vtable = <IXamlRootChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlRootChangedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for XamlRootChangedEventArgs {
    type Target = IXamlRootChangedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for XamlRootChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlRootChangedEventArgs";
}
unsafe impl Send for XamlRootChangedEventArgs {}
unsafe impl Sync for XamlRootChangedEventArgs {}
