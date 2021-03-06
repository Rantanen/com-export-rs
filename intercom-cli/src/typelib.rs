use intercom::{
    type_system::{AutomationTypeSystem, TypeSystemName},
    typelib::IIntercomTypeLib,
};
use std::path::Path;

#[derive(Fail, Debug)]
pub enum TypeLibError
{
    #[fail(display = "Could not acquire IntercomTypeLib: {}", _0)]
    AcquiringTypeLib(String),
}

pub fn read_typelib(path: &Path) -> Result<intercom::typelib::TypeLib, failure::Error>
{
    let lib = libloading::Library::new(path)?;
    let typelib = unsafe {
        let fn_get_type_lib: libloading::Symbol<
            unsafe extern "C" fn(
                TypeSystemName,
                *mut intercom::raw::RawComPtr,
            ) -> intercom::raw::HRESULT,
        > = lib.get(b"IntercomTypeLib")?;

        let mut ptr: intercom::raw::RawComPtr = std::ptr::null_mut();
        fn_get_type_lib(TypeSystemName::Automation, &mut ptr as *mut _);

        let comptr =
            intercom::raw::InterfacePtr::<AutomationTypeSystem, dyn IIntercomTypeLib>::new(ptr)
                .ok_or_else(|| TypeLibError::AcquiringTypeLib("Null ptr".to_owned()))?;
        intercom::ComRc::wrap(comptr)
    };

    Ok(intercom::typelib::TypeLib::from_comrc(&typelib)?)
}
