extern crate proc_macro;
extern crate serde_derive;
use crate::prelude::*;
use self::serde_derive::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy,
    Serialize, Deserialize,
    Hash, PartialOrd, PartialEq,
)]
pub enum TypeSystemName {
    Automation,
    Raw,
}
impl std::cmp::Eq for TypeSystemName {}

/// Common trait for type systems.
pub trait TypeSystem : Clone + Copy {
    const AUTOMATION : TypeSystemName = TypeSystemName::Automation;
    const RAW : TypeSystemName = TypeSystemName::Raw;

    fn key() -> TypeSystemName;
}

/// Automation type system.
#[derive(Clone, Copy)]
pub struct AutomationTypeSystem;
impl TypeSystem for AutomationTypeSystem {
    fn key() -> TypeSystemName { TypeSystemName::Automation }
}

/// Raw type system.
#[derive(Clone, Copy)]
pub struct RawTypeSystem;
impl TypeSystem for RawTypeSystem {
    fn key() -> TypeSystemName { TypeSystemName::Raw }
}

/// Defines the category of a COM item.
#[derive(Serialize, Deserialize)]
pub enum ComItemCategory {
    Class,
    Interface,
    Method,
    Primitive,
}

/// Defines information of an item.
pub trait ItemInfo {

    /// The category of the item.
    fn category() -> ComItemCategory;

    /// The name of the item.
    fn name() -> &'static str;
}


/// Defines a type that has identical representation for both input and output directions.
pub trait BidirectionalTypeInfo {

    /// The name of the type.
    fn type_name() -> &'static str;
    fn indirection_level() -> u32 { 0 }
}

/// Defines details of the type that specify how to pass it as an input parameter.
pub trait InputTypeInfo {

    /// The name of the type.
    fn type_name() -> &'static str;
    fn indirection_level() -> u32 { 0 }
}

/// Defines details of the type that specify how to pass it as an output parameter.
pub trait OutputTypeInfo {

    /// The name of the type.
    fn type_name() -> &'static str;
    fn indirection_level() -> u32 { 0 }
}

/// Defines a type that is compatible with Intercom interfaces.
pub trait ExternType<TS: TypeSystem> : Sized {

    /// Type used when the Self type is encountered as an input parameter.
    type ExternInputType : InputTypeInfo;

    /// Type used when the Self type is encountered as an output type.
    type ExternOutputType : OutputTypeInfo;

    /// A possible temporary type used for converting `Self` into
    /// `ExternInputType` when calling Intercom interfaces from Rust.
    type OwnedExternType : IntercomFrom< Self > = Self;

    /// A possible temporary type used for converting `ExternInputType` into
    /// `Self` type when calling Rust through an Intercom interface.
    type OwnedNativeType : IntercomFrom< Self::ExternInputType > = Self;
}

/// A conversion that may fail by resulting in a `ComError .
pub trait IntercomFrom<TSource> : Sized {
    fn intercom_from( source : TSource ) -> ComResult<Self>;
}

/// Default identity blanket implementation.
impl<T> IntercomFrom<T> for T {
    default fn intercom_from( source: T ) -> ComResult<T> { Ok( source ) }
}

/// Blanket implementation for all cloneable instance references.
impl<TSource: Clone> IntercomFrom<&TSource> for TSource {
    fn intercom_from( source: &TSource ) -> ComResult<Self> {
        Ok( source.clone() )
    }
}

/// A conversion that may fail by resulting in a `ComError .
pub trait IntercomInto<TTarget> {
    fn intercom_into( self : Self ) -> ComResult<TTarget>;
}

/// Blanket implementation for reversing IntercomFrom into IntercomInto.
impl<TSource, TTarget: IntercomFrom<TSource>>
        IntercomInto<TTarget> for TSource
{
    default fn intercom_into( self: Self ) -> ComResult<TTarget> {
        TTarget::intercom_from( self )
    }
}


/// Bidirectional types can be used as input types.
impl<BT> InputTypeInfo for BT where BT: BidirectionalTypeInfo {

    /// The name of the type.
    fn type_name() -> &'static str { <BT as BidirectionalTypeInfo>::type_name() }
    fn indirection_level() -> u32 { <BT as BidirectionalTypeInfo>::indirection_level() }
}

/// Bidirectional types can be used as output types.
impl<BT> OutputTypeInfo for BT where BT: BidirectionalTypeInfo {

    /// The name of the type.
    fn type_name() -> &'static str { <BT as BidirectionalTypeInfo>::type_name() }
    fn indirection_level() -> u32 { <BT as BidirectionalTypeInfo>::indirection_level() }
}

/// A quick macro for implementing ExternType for various basic types that
/// should represent themselves.
///
/// Ideally we would use specialization here to implement ExternType for T,
/// but that prevents other crates from implementing a specialized version for
/// some reason.
macro_rules! self_extern {
    ( $t:ty ) => {

        impl BidirectionalTypeInfo for $t {

            /// The default name is the name of the type.
            fn type_name() -> &'static str { stringify!( $t ) }
        }

        impl<TS: TypeSystem> ExternType<TS> for $t {
            type ExternInputType = $t;
            type ExternOutputType = $t;
            type OwnedExternType = $t;
            type OwnedNativeType = $t;
        }
    }
}

// Define all types that should have built-in Self extern type.
self_extern!( () );
self_extern!( i8 );
self_extern!( i16 );
self_extern!( i32 );
self_extern!( i64 );
self_extern!( isize );
self_extern!( u8 );
self_extern!( u16 );
self_extern!( u32 );
self_extern!( u64 );
self_extern!( usize );
self_extern!( f32 );
self_extern!( f64 );
self_extern!( crate::raw::HRESULT );
self_extern!( crate::GUID );

#[cfg(not(windows))]
self_extern!( libc::c_void );
self_extern!( std::ffi::c_void );
self_extern!(TypeSystemName);

// Any raw pointer is passed as is.

impl<TPtr> BidirectionalTypeInfo for *mut TPtr where TPtr: BidirectionalTypeInfo {

    /// The name of the type.
    fn type_name() -> &'static str { <TPtr as BidirectionalTypeInfo>::type_name() }
    fn indirection_level() -> u32 { <TPtr as BidirectionalTypeInfo>::indirection_level() + 1 }
}

impl<TS: TypeSystem, TPtr> ExternType<TS> for *mut TPtr where TPtr: BidirectionalTypeInfo {
    type ExternInputType = *mut TPtr;
    type ExternOutputType = *mut TPtr;
    type OwnedExternType = *mut TPtr;
    type OwnedNativeType = *mut TPtr;
}

impl<TPtr> BidirectionalTypeInfo for *const TPtr where TPtr: BidirectionalTypeInfo {

    /// The name of the type.
    fn type_name() -> &'static str { <TPtr as BidirectionalTypeInfo>::type_name() }
    fn indirection_level() -> u32 { <TPtr as BidirectionalTypeInfo>::indirection_level() + 1 }
}

impl<TS: TypeSystem, TPtr> ExternType<TS> for *const TPtr where TPtr: BidirectionalTypeInfo {
    type ExternInputType = *const TPtr;
    type ExternOutputType = *const TPtr;
    type OwnedExternType = *const TPtr;
    type OwnedNativeType = *const TPtr;
}

/// `ComItf` extern type implementation.

impl<I: crate::ComInterface + ?Sized> BidirectionalTypeInfo for crate::ComItf<I>
    where I: BidirectionalTypeInfo
{

    /// The name of the type.
    fn type_name() -> &'static str { <I as BidirectionalTypeInfo>::type_name() }
    fn indirection_level() -> u32 { <I as BidirectionalTypeInfo>::indirection_level() + 1 }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> ExternType<TS>
        for crate::ComItf<I>
    where I: BidirectionalTypeInfo
{

    type ExternInputType = crate::raw::InterfacePtr<TS, I>;
    type ExternOutputType = crate::raw::InterfacePtr<TS, I>;
    type OwnedExternType = crate::raw::InterfacePtr<TS, I>;
    type OwnedNativeType = crate::raw::InterfacePtr<TS, I>;
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> ExternType<TS>
        for crate::raw::InterfacePtr<TS, I>
    where I: BidirectionalTypeInfo
{

    type ExternInputType = crate::raw::InterfacePtr<TS, I>;
    type ExternOutputType = crate::raw::InterfacePtr<TS, I>;
    type OwnedExternType = crate::raw::InterfacePtr<TS, I>;
    type OwnedNativeType = crate::raw::InterfacePtr<TS, I>;
}

impl<I: crate::ComInterface + ?Sized> BidirectionalTypeInfo for crate::ComRc<I>
    where I: BidirectionalTypeInfo
{

    /// The name of the type.
    fn type_name() -> &'static str { <I as BidirectionalTypeInfo>::type_name() }
    fn indirection_level() -> u32 { <I as BidirectionalTypeInfo>::indirection_level() + 1 }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> ExternType<TS>
        for crate::ComRc<I>
    where I: BidirectionalTypeInfo
{

    type ExternInputType = crate::raw::InterfacePtr<TS, I>;
    type ExternOutputType = crate::raw::InterfacePtr<TS, I>;
    type OwnedExternType = crate::raw::InterfacePtr<TS, I>;
    type OwnedNativeType = crate::raw::InterfacePtr<TS, I>;
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> BidirectionalTypeInfo for crate::raw::InterfacePtr<TS, I>
    where I: BidirectionalTypeInfo
{

    /// The name of the type.
    fn type_name() -> &'static str { <I as BidirectionalTypeInfo>::type_name() }
    fn indirection_level() -> u32 { <I as BidirectionalTypeInfo>::indirection_level() + 1 }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized>
IntercomFrom<crate::ComItf<I>> for crate::raw::InterfacePtr<TS, I>
{
    fn intercom_from( source: crate::ComItf<I> ) -> ComResult<Self> {
        Ok( crate::ComItf::ptr( &source ) )
    }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized>
    IntercomFrom<&crate::ComItf<I>> for crate::raw::InterfacePtr<TS, I>
{
    fn intercom_from( source: &crate::ComItf<I> ) -> ComResult<Self> {
        Ok( crate::ComItf::ptr( source ) )
    }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized>
IntercomFrom<crate::ComRc<I>> for crate::raw::InterfacePtr<TS, I>
{
    fn intercom_from( source: crate::ComRc<I> ) -> ComResult<Self> {
        Ok( crate::ComItf::ptr( &crate::ComRc::detach( source ) ) )
    }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized>
    IntercomFrom<crate::raw::InterfacePtr<TS, I>> for crate::ComRc<I>
{
    fn intercom_from( source: crate::raw::InterfacePtr<TS, I> ) -> ComResult<Self> {
        Ok( crate::ComRc::attach(
            crate::ComItf::maybe_wrap( source )
                .ok_or_else( || crate::ComError::E_INVALIDARG )? ) )
    }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized>
    IntercomFrom<crate::raw::InterfacePtr<TS, I>> for crate::ComItf<I>
{
    fn intercom_from( source: crate::raw::InterfacePtr<TS, I> ) -> ComResult<Self> {
        crate::ComItf::maybe_wrap( source )
                .ok_or_else( || crate::ComError::E_INVALIDARG )
    }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized>
    IntercomFrom<&crate::raw::InterfacePtr<TS, I>> for crate::ComItf<I>
{
    fn intercom_from( source: &crate::raw::InterfacePtr<TS, I> ) -> ComResult<Self> {
        crate::ComItf::maybe_wrap( source.clone() )
                .ok_or_else( || crate::ComError::E_INVALIDARG )
    }
}

/// Defines the uninitialized values for out parameters when calling into
/// Intercom interfaces.
pub trait ExternDefault {
    unsafe fn extern_default() -> Self;
}

impl<T> ExternDefault for T {
    default unsafe fn extern_default() -> Self { std::mem::zeroed() }
}

impl<TPtr> ExternDefault for *const TPtr {
    default unsafe fn extern_default() -> Self { std::ptr::null() }
}
