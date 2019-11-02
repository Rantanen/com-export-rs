use crate::{
    com_class, com_impl, com_interface, type_system::TypeSystemName, ComBox, ComError, ComItf,
    ComRc, ComResult, GUID,
};

use std::borrow::Cow;

#[derive(Fail, Debug)]
pub enum TypeLibError
{
    #[fail(display = "COM error occurred: {}", _0)]
    ComError(ComError),
}

impl From<ComError> for TypeLibError
{
    fn from(s: ComError) -> Self
    {
        TypeLibError::ComError(s)
    }
}

mod from_impls;

// pub mod raw;
// use raw::*;

#[derive(Debug)]
pub struct InterfaceRef
{
    pub name: Cow<'static, str>,
    pub iid_automation: GUID,
    pub iid_raw: GUID,
}

// TypeLib

#[com_class(IIntercomTypeLib)]
#[derive(Debug)]
pub struct TypeLib
{
    pub name: Cow<'static, str>,
    pub libid: GUID,
    pub version: Cow<'static, str>,
    pub types: Vec<TypeInfo>,
}

#[com_interface]
pub trait IIntercomTypeLib
{
    fn get_info(&self) -> ComResult<(String, GUID, String)>;
    fn get_type_count(&self) -> ComResult<u32>;
    fn get_type(&self, idx: u32) -> ComResult<ComRc<dyn IIntercomTypeInfo>>;
}

// TypeInfo

#[derive(Debug)]
pub enum TypeInfo
{
    Class(ComBox<CoClass>),
    Interface(ComBox<Interface>),
    Struct(ComBox<Struct>),
}

#[derive(intercom::ExternType, intercom::BidirectionalTypeInfo, Debug)]
#[repr(C)]
pub enum TypeInfoKind
{
    CoClass,
    Interface,
    Struct,
}

#[com_interface]
pub trait IIntercomTypeInfo
{
    fn get_name(&self) -> ComResult<String>;
    fn get_kind(&self) -> ComResult<TypeInfoKind>;
}

// TypeInfo::CoClass

#[com_class(IIntercomTypeInfo, IIntercomCoClass)]
#[derive(Debug)]
pub struct CoClass
{
    pub name: Cow<'static, str>,
    pub clsid: GUID,
    pub interfaces: Vec<InterfaceRef>,
}

#[com_interface]
pub trait IIntercomCoClass
{
    // FIXME: Support interface inheritance
    fn get_name(&self) -> ComResult<String>;

    fn get_clsid(&self) -> ComResult<GUID>;
    fn get_interface_count(&self) -> ComResult<u32>;
    fn get_interface_ref(&self, idx: u32, ts: TypeSystemName) -> ComResult<(String, GUID)>;
}

// TypeInfo::Interface

#[com_class(IIntercomTypeInfo, IIntercomInterface)]
#[derive(Debug)]
pub struct Interface
{
    pub name: Cow<'static, str>,
    pub variants: Vec<ComBox<InterfaceVariant>>,
    pub options: InterfaceOptions,
}

#[derive(Debug, Clone, Default, intercom::ExternType, intercom::BidirectionalTypeInfo)]
#[repr(C)]
pub struct InterfaceOptions
{
    pub class_impl_interface: bool,
    pub __non_exhaustive: (),
}

#[com_class(IIntercomInterfaceVariant)]
#[derive(Debug)]
pub struct InterfaceVariant
{
    pub ts: TypeSystemName,
    pub iid: GUID,
    pub methods: Vec<ComBox<Method>>,
}

#[com_interface]
pub trait IIntercomInterface
{
    // FIXME: Support interface inheritance
    fn get_name(&self) -> ComResult<String>;
    fn get_options(&self) -> ComResult<InterfaceOptions>;

    fn get_variant_count(&self) -> ComResult<u32>;
    fn get_variant(&self, idx: u32) -> ComResult<ComRc<dyn IIntercomInterfaceVariant>>;
}

#[com_interface]
pub trait IIntercomInterfaceVariant
{
    fn get_type_system(&self) -> ComResult<TypeSystemName>;
    fn get_iid(&self) -> ComResult<GUID>;
    fn get_method_count(&self) -> ComResult<u32>;
    fn get_method(&self, idx: u32) -> ComResult<ComRc<dyn IIntercomMethod>>;
}

// Method

#[com_class(IIntercomMethod)]
#[derive(Debug)]
pub struct Method
{
    pub name: Cow<'static, str>,
    pub return_type: Arg,
    pub parameters: Vec<Arg>,
}

#[derive(Debug)]
pub struct Arg
{
    pub name: Cow<'static, str>,
    pub ty: Cow<'static, str>,
    pub indirection_level: u32,
    pub direction: Direction,
}

#[derive(
    Debug, Clone, Copy, intercom::ExternType, intercom::BidirectionalTypeInfo, PartialEq, Eq,
)]
#[repr(C)]
pub enum Direction
{
    In,
    Out,
    Retval,
    Return,
}

#[com_interface]
pub trait IIntercomMethod
{
    fn get_name(&self) -> ComResult<String>;
    fn get_return_type(&self) -> ComResult<(String, u32)>;
    fn get_parameter_count(&self) -> ComResult<u32>;
    fn get_parameter(&self, idx: u32) -> ComResult<(String, String, u32, Direction)>;
}

#[com_class(IIntercomTypeInfo, IIntercomStruct)]
#[derive(Debug)]
pub struct Struct
{
    pub name: Cow<'static, str>,
    pub variants: Vec<ComBox<StructVariant>>,
}

#[com_class(IIntercomStructVariant)]
#[derive(Debug)]
pub struct StructVariant
{
    pub ts: TypeSystemName,
    pub fields: Vec<Arg>,
}

#[com_interface]
pub trait IIntercomStruct
{
    fn get_name(&self) -> ComResult<String>;
    fn get_variant_count(&self) -> ComResult<u32>;
    fn get_variant(&self, idx: u32) -> ComResult<ComRc<dyn IIntercomStructVariant>>;
}

#[com_interface]
pub trait IIntercomStructVariant
{
    fn get_type_system(&self) -> ComResult<TypeSystemName>;
    fn get_field_count(&self) -> ComResult<u32>;
    fn get_field(&self, idx: u32) -> ComResult<(String, String, u32, Direction)>;
}

// Impls

#[com_impl]
impl IIntercomTypeLib for TypeLib
{
    fn get_info(&self) -> ComResult<(String, GUID, String)>
    {
        Ok((
            self.name.to_string(),
            self.libid.clone(),
            self.version.to_string(),
        ))
    }

    fn get_type_count(&self) -> ComResult<u32>
    {
        Ok(self.types.len() as u32)
    }

    fn get_type(&self, idx: u32) -> ComResult<ComRc<dyn IIntercomTypeInfo>>
    {
        Ok(match &self.types[idx as usize] {
            TypeInfo::Class(cls) => ComRc::from(cls),
            TypeInfo::Interface(itf) => ComRc::from(itf),
            TypeInfo::Struct(stru) => ComRc::from(stru),
        })
    }
}

#[com_impl]
impl IIntercomTypeInfo for CoClass
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }

    fn get_kind(&self) -> ComResult<TypeInfoKind>
    {
        Ok(TypeInfoKind::CoClass)
    }
}

#[com_impl]
impl IIntercomCoClass for CoClass
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }

    fn get_clsid(&self) -> ComResult<GUID>
    {
        Ok(self.clsid.clone())
    }
    fn get_interface_count(&self) -> ComResult<u32>
    {
        Ok(self.interfaces.len() as u32)
    }

    fn get_interface_ref(&self, idx: u32, ts: TypeSystemName) -> ComResult<(String, GUID)>
    {
        let itf = &self.interfaces[idx as usize];
        Ok((
            itf.name.to_string(),
            match ts {
                TypeSystemName::Automation => itf.iid_automation.clone(),
                TypeSystemName::Raw => itf.iid_raw.clone(),
            },
        ))
    }
}

#[com_impl]
impl IIntercomTypeInfo for Interface
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }

    fn get_kind(&self) -> ComResult<TypeInfoKind>
    {
        Ok(TypeInfoKind::Interface)
    }
}

#[com_impl]
impl IIntercomInterface for Interface
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }

    fn get_options(&self) -> ComResult<InterfaceOptions>
    {
        Ok(self.options.clone())
    }

    fn get_variant_count(&self) -> ComResult<u32>
    {
        Ok(self.variants.len() as u32)
    }

    fn get_variant(&self, idx: u32) -> ComResult<ComRc<dyn IIntercomInterfaceVariant>>
    {
        Ok(ComRc::from(&self.variants[idx as usize]))
    }
}

#[com_impl]
impl IIntercomInterfaceVariant for InterfaceVariant
{
    fn get_type_system(&self) -> ComResult<TypeSystemName>
    {
        Ok(self.ts)
    }

    fn get_iid(&self) -> ComResult<GUID>
    {
        Ok(self.iid.clone())
    }

    fn get_method_count(&self) -> ComResult<u32>
    {
        Ok(self.methods.len() as u32)
    }

    fn get_method(&self, idx: u32) -> ComResult<ComRc<dyn IIntercomMethod>>
    {
        Ok(ComRc::from(&self.methods[idx as usize]))
    }
}

#[com_impl]
impl IIntercomMethod for Method
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }

    fn get_return_type(&self) -> ComResult<(String, u32)>
    {
        Ok((
            self.return_type.ty.to_string(),
            self.return_type.indirection_level,
        ))
    }

    fn get_parameter_count(&self) -> ComResult<u32>
    {
        Ok(self.parameters.len() as u32)
    }
    fn get_parameter(&self, idx: u32) -> ComResult<(String, String, u32, Direction)>
    {
        let arg = &self.parameters[idx as usize];
        Ok((
            arg.name.to_string(),
            arg.ty.to_string(),
            arg.indirection_level,
            arg.direction,
        ))
    }
}

#[com_impl]
impl IIntercomTypeInfo for Struct
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }

    fn get_kind(&self) -> ComResult<TypeInfoKind>
    {
        Ok(TypeInfoKind::Struct)
    }
}

#[com_impl]
impl IIntercomStruct for Struct
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }
    fn get_variant_count(&self) -> ComResult<u32>
    {
        Ok(self.variants.len() as u32)
    }
    fn get_variant(&self, idx: u32) -> ComResult<ComRc<dyn IIntercomStructVariant>>
    {
        Ok(ComRc::from(&self.variants[idx as usize]))
    }
}

#[com_impl]
impl IIntercomStructVariant for StructVariant
{
    fn get_type_system(&self) -> ComResult<TypeSystemName>
    {
        Ok(self.ts)
    }

    fn get_field_count(&self) -> ComResult<u32>
    {
        Ok(self.fields.len() as u32)
    }

    fn get_field(&self, idx: u32) -> ComResult<(String, String, u32, Direction)>
    {
        let arg = &self.fields[idx as usize];
        Ok((
            arg.name.to_string(),
            arg.ty.to_string(),
            arg.indirection_level,
            arg.direction,
        ))
    }
}

impl CoClass
{
    pub fn __new(name: Cow<'static, str>, clsid: GUID, interfaces: Vec<InterfaceRef>) -> Self
    {
        Self {
            name,
            clsid,
            interfaces,
        }
    }
}

impl TypeLib
{
    pub fn __new(
        name: Cow<'static, str>,
        libid: GUID,
        version: Cow<'static, str>,
        mut types: Vec<TypeInfo>,
    ) -> TypeLib
    {
        types.sort_by_key(|item| match item {
            TypeInfo::Class(cls) => ("class", cls.as_ref().name.to_string()),
            TypeInfo::Interface(itf) => ("itf", itf.as_ref().name.to_string()),
            TypeInfo::Struct(stru) => ("struct", stru.as_ref().name.to_string()),
        });
        types.dedup_by_key(|item| match item {
            TypeInfo::Class(cls) => ("class", cls.as_ref().name.to_string()),
            TypeInfo::Interface(itf) => ("itf", itf.as_ref().name.to_string()),
            TypeInfo::Struct(stru) => ("stru", stru.as_ref().name.to_string()),
        });
        TypeLib {
            name,
            libid,
            version,
            types,
        }
    }
}
