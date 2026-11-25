#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreWebView2(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CoreWebView2,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for CoreWebView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICoreWebView2>();
}
unsafe impl windows_core::Interface for CoreWebView2 {
    type Vtable = <ICoreWebView2 as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreWebView2 as windows_core::Interface>::IID;
}
impl core::ops::Deref for CoreWebView2 {
    type Target = ICoreWebView2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CoreWebView2 {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2";
}
unsafe impl Send for CoreWebView2 {}
unsafe impl Sync for CoreWebView2 {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CoreWebView2HostResourceAccessKind(pub i32);
impl CoreWebView2HostResourceAccessKind {
    pub const Deny: Self = Self(0);
    pub const Allow: Self = Self(1);
    pub const DenyCors: Self = Self(2);
}
impl windows_core::imp::TypeKind for CoreWebView2HostResourceAccessKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for CoreWebView2HostResourceAccessKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.Web.WebView2.Core.CoreWebView2HostResourceAccessKind;i4)",
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreWebView2Settings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CoreWebView2Settings,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for CoreWebView2Settings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICoreWebView2Settings>();
}
unsafe impl windows_core::Interface for CoreWebView2Settings {
    type Vtable = <ICoreWebView2Settings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreWebView2Settings as windows_core::Interface>::IID;
}
impl core::ops::Deref for CoreWebView2Settings {
    type Target = ICoreWebView2Settings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CoreWebView2Settings {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Settings";
}
unsafe impl Send for CoreWebView2Settings {}
unsafe impl Sync for CoreWebView2Settings {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreWebView2WebMessageReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CoreWebView2WebMessageReceivedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for CoreWebView2WebMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        ICoreWebView2WebMessageReceivedEventArgs,
    >();
}
unsafe impl windows_core::Interface for CoreWebView2WebMessageReceivedEventArgs {
    type Vtable = <ICoreWebView2WebMessageReceivedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <ICoreWebView2WebMessageReceivedEventArgs as windows_core::Interface>::IID;
}
impl core::ops::Deref for CoreWebView2WebMessageReceivedEventArgs {
    type Target = ICoreWebView2WebMessageReceivedEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for CoreWebView2WebMessageReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs";
}
unsafe impl Send for CoreWebView2WebMessageReceivedEventArgs {}
unsafe impl Sync for CoreWebView2WebMessageReceivedEventArgs {}
windows_core::imp::define_interface!(
    ICoreWebView2,
    ICoreWebView2_Vtbl,
    0x3a3f559a_e5e9_5338_bb67_4eb0504a4f14
);
impl windows_core::RuntimeType for ICoreWebView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICoreWebView2 {
    pub fn Settings(&self) -> windows_core::Result<CoreWebView2Settings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Settings)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn WebMessageReceived<F>(
        &self,
        handler: F,
    ) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(
                windows_core::Ref<CoreWebView2>,
                windows_core::Ref<CoreWebView2WebMessageReceivedEventArgs>,
            ) + 'static,
    {
        let handler: TypedEventHandler<CoreWebView2, CoreWebView2WebMessageReceivedEventArgs> = {
            let com = windows_core::imp::DelegateBox::< TypedEventHandler < CoreWebView2 , CoreWebView2WebMessageReceivedEventArgs > , F >::new (& TypedEventHandlerBox::< CoreWebView2 , CoreWebView2WebMessageReceivedEventArgs , F >::VTABLE , handler) ;
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).WebMessageReceived)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveWebMessageReceived,
            ))
        }
    }
    pub fn Navigate(&self, uri: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Navigate)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(uri)),
            )
            .ok()
        }
    }
    pub fn PostWebMessageAsJson(&self, webmessageasjson: &str) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).PostWebMessageAsJson)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(webmessageasjson)),
            )
            .ok()
        }
    }
    pub fn OpenDevToolsWindow(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).OpenDevToolsWindow)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Settings: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    Source: usize,
    BrowserProcessId: usize,
    CanGoBack: usize,
    CanGoForward: usize,
    DocumentTitle: usize,
    ContainsFullScreenElement: usize,
    NavigationStarting: usize,
    RemoveNavigationStarting: usize,
    ContentLoading: usize,
    RemoveContentLoading: usize,
    SourceChanged: usize,
    RemoveSourceChanged: usize,
    HistoryChanged: usize,
    RemoveHistoryChanged: usize,
    NavigationCompleted: usize,
    RemoveNavigationCompleted: usize,
    FrameNavigationStarting: usize,
    RemoveFrameNavigationStarting: usize,
    FrameNavigationCompleted: usize,
    RemoveFrameNavigationCompleted: usize,
    ScriptDialogOpening: usize,
    RemoveScriptDialogOpening: usize,
    PermissionRequested: usize,
    RemovePermissionRequested: usize,
    ProcessFailed: usize,
    RemoveProcessFailed: usize,
    pub WebMessageReceived: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveWebMessageReceived:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    NewWindowRequested: usize,
    RemoveNewWindowRequested: usize,
    DocumentTitleChanged: usize,
    RemoveDocumentTitleChanged: usize,
    ContainsFullScreenElementChanged: usize,
    RemoveContainsFullScreenElementChanged: usize,
    WebResourceRequested: usize,
    RemoveWebResourceRequested: usize,
    WindowCloseRequested: usize,
    RemoveWindowCloseRequested: usize,
    pub Navigate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    NavigateToString: usize,
    AddScriptToExecuteOnDocumentCreatedAsync: usize,
    RemoveScriptToExecuteOnDocumentCreated: usize,
    ExecuteScriptAsync: usize,
    CapturePreviewAsync: usize,
    Reload: usize,
    pub PostWebMessageAsJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    PostWebMessageAsString: usize,
    CallDevToolsProtocolMethodAsync: usize,
    GoBack: usize,
    GoForward: usize,
    GetDevToolsProtocolEventReceiver: usize,
    Stop: usize,
    AddHostObjectToScript: usize,
    RemoveHostObjectFromScript: usize,
    pub OpenDevToolsWindow:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2Settings,
    ICoreWebView2Settings_Vtbl,
    0x003b325e_74cd_52dd_8024_ebb8be38e48e
);
impl windows_core::RuntimeType for ICoreWebView2Settings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICoreWebView2Settings {
    pub fn IsStatusBarEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStatusBarEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsStatusBarEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsStatusBarEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn AreDevToolsEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreDevToolsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAreDevToolsEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreDevToolsEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn AreDefaultContextMenusEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreDefaultContextMenusEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAreDefaultContextMenusEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAreDefaultContextMenusEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn IsZoomControlEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsZoomControlEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetIsZoomControlEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetIsZoomControlEnabled)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2Settings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsScriptEnabled: usize,
    SetIsScriptEnabled: usize,
    IsWebMessageEnabled: usize,
    SetIsWebMessageEnabled: usize,
    AreDefaultScriptDialogsEnabled: usize,
    SetAreDefaultScriptDialogsEnabled: usize,
    pub IsStatusBarEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsStatusBarEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AreDevToolsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAreDevToolsEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AreDefaultContextMenusEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAreDefaultContextMenusEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    AreHostObjectsAllowed: usize,
    SetAreHostObjectsAllowed: usize,
    pub IsZoomControlEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsZoomControlEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2WebMessageReceivedEventArgs,
    ICoreWebView2WebMessageReceivedEventArgs_Vtbl,
    0xeb066159_b725_5d5b_adc8_f5d7b9290304
);
impl windows_core::RuntimeType for ICoreWebView2WebMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICoreWebView2WebMessageReceivedEventArgs {
    pub fn WebMessageAsJson(&self) -> windows_core::Result<String> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WebMessageAsJson)(
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
pub struct ICoreWebView2WebMessageReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Source: usize,
    pub WebMessageAsJson: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICoreWebView2_3,
    ICoreWebView2_3_Vtbl,
    0xa8c76ae7_6170_5dfe_8f00_79cd76a9b4d9
);
impl windows_core::RuntimeType for ICoreWebView2_3 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ICoreWebView2_3 {
    pub fn SetVirtualHostNameToFolderMapping(
        &self,
        hostname: &str,
        folderpath: &str,
        accesskind: CoreWebView2HostResourceAccessKind,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetVirtualHostNameToFolderMapping)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(&windows_core::HSTRING::from(hostname)),
                core::mem::transmute_copy(&windows_core::HSTRING::from(folderpath)),
                accesskind,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ICoreWebView2_3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    IsSuspended: usize,
    TrySuspendAsync: usize,
    Resume: usize,
    pub SetVirtualHostNameToFolderMapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        CoreWebView2HostResourceAccessKind,
    ) -> windows_core::HRESULT,
}
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
