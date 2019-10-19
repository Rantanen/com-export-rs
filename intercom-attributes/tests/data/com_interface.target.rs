#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate intercom;
use intercom::*;

pub trait Foo {
    fn static_method(a: u16, b: i16);

    fn simple_method(&self);

    fn arg_method(&self, a: u16);

    fn simple_result_method(&self)
    -> u16;
    fn com_result_method(&self)
    -> ComResult<u16>;
    fn rust_result_method(&self)
    -> Result<u16, i32>;

    fn complete_method(&mut self, a: u16, b: i16)
    -> ComResult<bool>;

    fn string_method(&self, msg: String)
    -> String;
    fn comitf_method(&self, itf: ComItf<Foo>)
    -> ComResult<ComItf<IUnknown>>;

    // Should be VARIANT_BOOL in Automation interface.
    fn bool_method(&self, input: bool)
    -> ComResult<bool>;

    fn variant_method(&self, input: Variant)
    -> ComResult<Variant>;
}
#[doc = "`Foo` interface ID."]
#[allow(non_upper_case_globals)]
pub const IID_Foo_Automation: intercom::IID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],};
#[doc = r" Gets type description of the #interface_variant COM class."]
pub(crate) fn get_com_interface_for_Foo_Automation()
 -> intercom::serialization::ComInterfaceVariant {
    intercom::serialization::ComInterfaceVariant::new("Foo_Automation".to_string(),
                                                      <intercom::type_system::AutomationTypeSystem
                                                          as
                                                          intercom::type_system::TypeSystem>::key())
}
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
pub struct __Foo_AutomationVtbl {
    pub __base: intercom::IUnknownVtbl,
    pub simple_method_Automation: unsafe extern "C" fn(self_vtable:
                                                           intercom::RawComPtr)
                                      ->
                                          <() as
                                          intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub arg_method_Automation: unsafe extern "C" fn(self_vtable:
                                                        intercom::RawComPtr,
                                                    a:
                                                        <u16 as
                                                        intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType)
                                   ->
                                       <() as
                                       intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub simple_result_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                  intercom::RawComPtr)
                                             ->
                                                 <u16 as
                                                 intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub com_result_method_Automation: unsafe extern "C" fn(self_vtable:
                                                               intercom::RawComPtr,
                                                           __out:
                                                               *mut <u16 as
                                                                    intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
                                          ->
                                              <intercom::raw::HRESULT as
                                              intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub rust_result_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                intercom::RawComPtr,
                                                            __out:
                                                                *mut <u16 as
                                                                     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
                                           ->
                                               <intercom::raw::HRESULT as
                                               intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub complete_method_Automation: unsafe extern "C" fn(self_vtable:
                                                             intercom::RawComPtr,
                                                         a:
                                                             <u16 as
                                                             intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                         b:
                                                             <i16 as
                                                             intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                         __out:
                                                             *mut <bool as
                                                                  intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
                                        ->
                                            <intercom::raw::HRESULT as
                                            intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub string_method_Automation: unsafe extern "C" fn(self_vtable:
                                                           intercom::RawComPtr,
                                                       msg:
                                                           <String as
                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType)
                                      ->
                                          <String as
                                          intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub comitf_method_Automation: unsafe extern "C" fn(self_vtable:
                                                           intercom::RawComPtr,
                                                       itf:
                                                           <ComItf<Foo> as
                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                       __out:
                                                           *mut <ComItf<IUnknown>
                                                                as
                                                                intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
                                      ->
                                          <intercom::raw::HRESULT as
                                          intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub bool_method_Automation: unsafe extern "C" fn(self_vtable:
                                                         intercom::RawComPtr,
                                                     input:
                                                         <bool as
                                                         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                     __out:
                                                         *mut <bool as
                                                              intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
                                    ->
                                        <intercom::raw::HRESULT as
                                        intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub variant_method_Automation: unsafe extern "C" fn(self_vtable:
                                                            intercom::RawComPtr,
                                                        input:
                                                            <Variant as
                                                            intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                        __out:
                                                            *mut <Variant as
                                                                 intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
                                       ->
                                           <intercom::raw::HRESULT as
                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
}
#[doc = "`Foo` interface ID."]
#[allow(non_upper_case_globals)]
pub const IID_Foo_Raw: intercom::IID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8],};
#[doc = r" Gets type description of the #interface_variant COM class."]
pub(crate) fn get_com_interface_for_Foo_Raw()
 -> intercom::serialization::ComInterfaceVariant {
    intercom::serialization::ComInterfaceVariant::new("Foo_Raw".to_string(),
                                                      <intercom::type_system::RawTypeSystem
                                                          as
                                                          intercom::type_system::TypeSystem>::key())
}
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
pub struct __Foo_RawVtbl {
    pub __base: intercom::IUnknownVtbl,
    pub simple_method_Raw: unsafe extern "C" fn(self_vtable:
                                                    intercom::RawComPtr)
                               ->
                                   <() as
                                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub arg_method_Raw: unsafe extern "C" fn(self_vtable: intercom::RawComPtr,
                                             a:
                                                 <u16 as
                                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType)
                            ->
                                <() as
                                intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
    pub simple_result_method_Raw: unsafe extern "C" fn(self_vtable:
                                                           intercom::RawComPtr)
                                      ->
                                          <u16 as
                                          intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
    pub com_result_method_Raw: unsafe extern "C" fn(self_vtable:
                                                        intercom::RawComPtr,
                                                    __out:
                                                        *mut <u16 as
                                                             intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
                                   ->
                                       <intercom::raw::HRESULT as
                                       intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
    pub rust_result_method_Raw: unsafe extern "C" fn(self_vtable:
                                                         intercom::RawComPtr,
                                                     __out:
                                                         *mut <u16 as
                                                              intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
                                    ->
                                        <intercom::raw::HRESULT as
                                        intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
    pub complete_method_Raw: unsafe extern "C" fn(self_vtable:
                                                      intercom::RawComPtr,
                                                  a:
                                                      <u16 as
                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                  b:
                                                      <i16 as
                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                  __out:
                                                      *mut <bool as
                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
                                 ->
                                     <intercom::raw::HRESULT as
                                     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
    pub string_method_Raw: unsafe extern "C" fn(self_vtable:
                                                    intercom::RawComPtr,
                                                msg:
                                                    <String as
                                                    intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType)
                               ->
                                   <String as
                                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
    pub comitf_method_Raw: unsafe extern "C" fn(self_vtable:
                                                    intercom::RawComPtr,
                                                itf:
                                                    <ComItf<Foo> as
                                                    intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                __out:
                                                    *mut <ComItf<IUnknown> as
                                                         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
                               ->
                                   <intercom::raw::HRESULT as
                                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
    pub bool_method_Raw: unsafe extern "C" fn(self_vtable:
                                                  intercom::RawComPtr,
                                              input:
                                                  <bool as
                                                  intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                              __out:
                                                  *mut <bool as
                                                       intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
                             ->
                                 <intercom::raw::HRESULT as
                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
    pub variant_method_Raw: unsafe extern "C" fn(self_vtable:
                                                     intercom::RawComPtr,
                                                 input:
                                                     <Variant as
                                                     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                 __out:
                                                     *mut <Variant as
                                                          intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
                                ->
                                    <intercom::raw::HRESULT as
                                    intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
}
impl Foo for intercom::ComItf<Foo> {
    fn arg_method(&self, a: u16) -> () {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).arg_method_Automation)(comptr.ptr,
                                                              (&<u16 as
                                                                    intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::intercom_from(a)?).intercom_into()?);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).arg_method_Raw)(comptr.ptr,
                                                       (&<u16 as
                                                             intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::intercom_from(a)?).intercom_into()?);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <() as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn bool_method(&self, input: bool) -> ComResult<bool> {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<bool>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <bool as
                                 intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).bool_method_Automation)(comptr.ptr,
                                                               (&<bool as
                                                                     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::intercom_from(input)?).intercom_into()?,
                                                               &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<bool> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<bool>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <bool as
                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).bool_method_Raw)(comptr.ptr,
                                                        (&<bool as
                                                              intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::intercom_from(input)?).intercom_into()?,
                                                        &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<bool> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <ComResult<bool> as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn com_result_method(&self) -> ComResult<u16> {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<u16>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <u16 as
                                 intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).com_result_method_Automation)(comptr.ptr,
                                                                     &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<u16> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<u16>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <u16 as
                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).com_result_method_Raw)(comptr.ptr,
                                                              &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<u16> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <ComResult<u16> as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn comitf_method(&self, itf: ComItf<Foo>) -> ComResult<ComItf<IUnknown>> {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result:
                    Result<ComResult<ComItf<IUnknown>>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <ComItf<IUnknown> as
                                 intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).comitf_method_Automation)(comptr.ptr,
                                                                 (&<ComItf<Foo>
                                                                       as
                                                                       intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::intercom_from(itf)?).intercom_into()?,
                                                                 &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<ComItf<IUnknown>> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result:
                    Result<ComResult<ComItf<IUnknown>>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <ComItf<IUnknown> as
                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).comitf_method_Raw)(comptr.ptr,
                                                          (&<ComItf<Foo> as
                                                                intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::intercom_from(itf)?).intercom_into()?,
                                                          &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<ComItf<IUnknown>> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <ComResult<ComItf<IUnknown>> as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool> {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<bool>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <bool as
                                 intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).complete_method_Automation)(comptr.ptr,
                                                                   (&<u16 as
                                                                         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::intercom_from(a)?).intercom_into()?,
                                                                   (&<i16 as
                                                                         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::intercom_from(b)?).intercom_into()?,
                                                                   &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<bool> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<bool>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <bool as
                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).complete_method_Raw)(comptr.ptr,
                                                            (&<u16 as
                                                                  intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::intercom_from(a)?).intercom_into()?,
                                                            (&<i16 as
                                                                  intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::intercom_from(b)?).intercom_into()?,
                                                            &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<bool> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <ComResult<bool> as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn rust_result_method(&self) -> Result<u16, i32> {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<Result<u16, i32>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <u16 as
                                 intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).rust_result_method_Automation)(comptr.ptr,
                                                                      &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <Result<u16, i32> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<Result<u16, i32>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <u16 as
                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).rust_result_method_Raw)(comptr.ptr,
                                                               &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <Result<u16, i32> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <Result<u16, i32> as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn simple_method(&self) -> () {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).simple_method_Automation)(comptr.ptr);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).simple_method_Raw)(comptr.ptr);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <() as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn simple_result_method(&self) -> u16 {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<u16, intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).simple_result_method_Automation)(comptr.ptr);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({ __result.intercom_into()? })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <u16 as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<u16, intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).simple_result_method_Raw)(comptr.ptr);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({ __result.intercom_into()? })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <u16 as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <u16 as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn string_method(&self, msg: String) -> String {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<String, intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).string_method_Automation)(comptr.ptr,
                                                                 (&<String as
                                                                       intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::intercom_from(msg)?).intercom_into()?);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({ __result.intercom_into()? })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <String as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<String, intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).string_method_Raw)(comptr.ptr,
                                                          (&<String as
                                                                intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::intercom_from(msg)?).intercom_into()?);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({ __result.intercom_into()? })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <String as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <String as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
    fn variant_method(&self, input: Variant) -> ComResult<Variant> {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<Variant>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <Variant as
                                 intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).variant_method_Automation)(comptr.ptr,
                                                                  (&<Variant
                                                                        as
                                                                        intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::intercom_from(input)?).intercom_into()?,
                                                                  &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<Variant> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<Variant>, intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 <Variant as
                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType =
                             intercom::type_system::ExternDefault::extern_default();
                         let __result =
                             ((**vtbl).variant_method_Raw)(comptr.ptr,
                                                           (&<Variant as
                                                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::intercom_from(input)?).intercom_into()?,
                                                           &mut __out);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({
                                if __result == intercom::raw::S_OK ||
                                       __result == intercom::raw::S_FALSE {
                                    Ok(__out.intercom_into()?)
                                } else {
                                    return Err(intercom::load_error(self.as_ref(),
                                                                    &INTERCOM_iid,
                                                                    __result));
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<Variant> as
                           intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <ComResult<Variant> as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
}
impl intercom::ComInterface for Foo {
    #[doc = "Returns the IID of the requested interface."]
    fn iid(ts: intercom::type_system::TypeSystemName)
     -> Option<&'static intercom::IID> {
        match ts {
            intercom::type_system::TypeSystemName::Automation =>
            Some(&IID_Foo_Automation),
            intercom::type_system::TypeSystemName::Raw => Some(&IID_Foo_Raw),
        }
    }
    fn deref(com_itf: &intercom::ComItf<Foo>) -> &(Foo + 'static) { com_itf }
}
impl intercom::type_system::BidirectionalTypeInfo for Foo {
    #[doc = r" The name of the type."]
    fn type_name() -> &'static str { "Foo" }
}
pub(crate) fn get_intercom_interface_info_for_Foo()
 -> intercom::typelib::TypeInfo {
    let mut variants =
        <[_]>::into_vec(box
                            [intercom::ComStruct::new(intercom::typelib::InterfaceVariant{ts:
                                                                                              intercom::type_system::TypeSystemName::Automation,
                                                                                          iid:
                                                                                              intercom::GUID{data1:
                                                                                                                 0u32,
                                                                                                             data2:
                                                                                                                 0u16,
                                                                                                             data3:
                                                                                                                 0u16,
                                                                                                             data4:
                                                                                                                 [0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8],},
                                                                                          methods:
                                                                                              <[_]>::into_vec(box
                                                                                                                  [intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "simple_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     "void".into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     0,
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              []),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "arg_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     "void".into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     0,
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "a".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "simple_result_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<u16
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<u16
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              []),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "com_result_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "rust_result_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<i32
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<i32
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "complete_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "a".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "b".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<i16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<i16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "string_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<String
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<String
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "msg".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<String
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<String
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "comitf_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "itf".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<ComItf<Foo>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<ComItf<Foo>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<ComItf<IUnknown>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<ComItf<IUnknown>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "bool_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "input".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "variant_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "input".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<Variant
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<Variant
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<Variant
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<Variant
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),})]),}),
                             intercom::ComStruct::new(intercom::typelib::InterfaceVariant{ts:
                                                                                              intercom::type_system::TypeSystemName::Raw,
                                                                                          iid:
                                                                                              intercom::GUID{data1:
                                                                                                                 0u32,
                                                                                                             data2:
                                                                                                                 0u16,
                                                                                                             data3:
                                                                                                                 0u16,
                                                                                                             data4:
                                                                                                                 [0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  0u8,
                                                                                                                  1u8],},
                                                                                          methods:
                                                                                              <[_]>::into_vec(box
                                                                                                                  [intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "simple_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     "void".into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     0,
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              []),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "arg_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     "void".into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     0,
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "a".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "simple_result_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<u16
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<u16
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              []),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "com_result_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "rust_result_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<i32
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<i32
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "complete_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "a".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<u16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "b".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<i16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<i16
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "string_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<String
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<String
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "msg".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<String
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<String
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "comitf_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "itf".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<ComItf<Foo>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<ComItf<Foo>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<ComItf<IUnknown>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<ComItf<IUnknown>
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "bool_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "input".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<bool
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),}),
                                                                                                                   intercom::ComStruct::new(intercom::typelib::Method{name:
                                                                                                                                                                          "variant_method".into(),
                                                                                                                                                                      return_type:
                                                                                                                                                                          intercom::typelib::Arg{name:
                                                                                                                                                                                                     "".into(),
                                                                                                                                                                                                 ty:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                 indirection_level:
                                                                                                                                                                                                     <<intercom::raw::HRESULT
                                                                                                                                                                                                      as
                                                                                                                                                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                         as
                                                                                                                                                                                                         intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                 direction:
                                                                                                                                                                                                     intercom::typelib::Direction::Return,},
                                                                                                                                                                      parameters:
                                                                                                                                                                          <[_]>::into_vec(box
                                                                                                                                                                                              [intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "input".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<Variant
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<Variant
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::InputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::In,},
                                                                                                                                                                                               intercom::typelib::Arg{name:
                                                                                                                                                                                                                          "__out".into(),
                                                                                                                                                                                                                      ty:
                                                                                                                                                                                                                          <<Variant
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::type_name().into(),
                                                                                                                                                                                                                      indirection_level:
                                                                                                                                                                                                                          <<Variant
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              intercom::type_system::OutputTypeInfo>::indirection_level(),
                                                                                                                                                                                                                      direction:
                                                                                                                                                                                                                          intercom::typelib::Direction::Retval,}]),})]),})]);
    intercom::typelib::TypeInfo::Interface(intercom::ComStruct::new(intercom::typelib::Interface{name:
                                                                                                     "Foo".into(),
                                                                                                 variants:
                                                                                                     variants,
                                                                                                 options:
                                                                                                     intercom::typelib::InterfaceOptions{class_impl_interface:
                                                                                                                                             false,
                                                                                                                                                      ..Default::default()},}))
}
