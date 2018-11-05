
pub enum Variant
{
    I8( i8 ),
    I16( i16 ),
    I32( i32 ),
    I64( i64 ),
    U8( u8 ),
    U16( u16 ),
    U32( u32 ),
    U64( u64 ),
    F32( f32 ),
    F64( f64 ),
    Bool( bool ),
    String( ::IntercomString ),
    SystemTime( std::time::SystemTime ),
    Raw( raw::Variant ),
}

impl From<raw::Variant> for Variant {
    fn from( src: raw::Variant ) -> Variant {
        unsafe {
            if src.vt.0 & raw::var_type::BYREF == 0 {
                match src.vt.0 & raw::var_type::TYPEMASK {
                    raw::var_type::I1 => Variant::I8( src.data.bVal ),
                    raw::var_type::I2 => Variant::I16( src.data.iVal ),
                    raw::var_type::I4 => Variant::I32( src.data.lVal ),
                    raw::var_type::I8 => Variant::I64( src.data.llVal ),
                    raw::var_type::UI1 => Variant::U8( src.data.cVal ),
                    raw::var_type::UI2 => Variant::U16( src.data.uiVal ),
                    raw::var_type::UI4 => Variant::U32( src.data.ulVal ),
                    raw::var_type::UI8 => Variant::U64( src.data.ullVal ),
                    raw::var_type::R4 => Variant::F32( src.data.fltVal ),
                    raw::var_type::R8 => Variant::F64( src.data.dblVal ),
                    raw::var_type::BOOL => Variant::Bool( src.data.boolVal.into() ),
                    raw::var_type::BSTR => 
                        Variant::String( ::IntercomString::BString(
                                ::BString::from_ptr( src.data.bstrVal ) ) ),
                    raw::var_type::DATE =>
                        Variant::SystemTime( src.data.date.into() ),
                    _ => Variant::Raw( src ),
                }
            } else {
                match src.vt.0 & raw::var_type::TYPEMASK {
                    raw::var_type::I1 => Variant::I8( *src.data.pbVal ),
                    raw::var_type::I2 => Variant::I16( *src.data.piVal ),
                    raw::var_type::I4 => Variant::I32( *src.data.plVal ),
                    raw::var_type::I8 => Variant::I64( *src.data.pllVal ),
                    raw::var_type::UI1 => Variant::U8( *src.data.pcVal ),
                    raw::var_type::UI2 => Variant::U16( *src.data.puiVal ),
                    raw::var_type::UI4 => Variant::U32( *src.data.pulVal ),
                    raw::var_type::UI8 => Variant::U64( *src.data.pullVal ),
                    raw::var_type::R4 => Variant::F32( *src.data.pfltVal ),
                    raw::var_type::R8 => Variant::F64( *src.data.pdblVal ),
                    raw::var_type::BOOL => Variant::Bool( (*src.data.pboolVal).into() ),
                    raw::var_type::BSTR =>
                        Variant::String( ::IntercomString::BString(
                                ::BString::from_ptr( *src.data.pbstrVal ) ) ),
                    raw::var_type::DATE =>
                        Variant::SystemTime( (*src.data.pdate).into() ),
                    _ => Variant::Raw( src ),
                }
            }
        }
    }
}

pub mod raw {

    use strings::BString;
    use std;
    use std::convert::TryFrom;
    use std::time::{SystemTime, Duration};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VariantBool(u16);

    impl From<VariantBool> for bool {
        fn from( src : VariantBool ) -> bool {
            src.0 != 0
        }
    }

    impl From<bool> for VariantBool {
        fn from( src : bool ) -> VariantBool {
            VariantBool( if src { 0xffff } else { 0 } )
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VariantDate(f64);

    impl VariantDate {
        pub fn com_epoch() -> SystemTime {
            SystemTime::UNIX_EPOCH - Duration::from_secs( 2209161600 )
        }
    }

    impl From<VariantDate> for SystemTime {
        fn from( src : VariantDate ) -> SystemTime {
            let days = src.0.trunc() as i64;
            let time = src.0.fract().abs();

            let com_epoch = SystemTime::UNIX_EPOCH - Duration::from_secs( 2209161600 );
            const DAY_SECONDS : u64 = 24 * 60 * 60;
            const DAY_SECONDS_F : f64 = DAY_SECONDS as f64;

            let date = if days > 0 {
                com_epoch + Duration::from_secs( days as u64 * DAY_SECONDS )
            } else {
                com_epoch - Duration::from_secs( ( -days ) as u64 * DAY_SECONDS )
            };

            let datetime = date + Duration::from_nanos(
                    ( time * DAY_SECONDS_F * 1_000_000_000f64 ).round() as u64 );

            datetime
        }
    }

    impl From<SystemTime> for VariantDate {
        fn from( src : SystemTime ) -> VariantDate {

            let com_epoch = SystemTime::UNIX_EPOCH - Duration::from_secs( 2209161600 );
            const DAY_SECONDS : u64 = 24 * 60 * 60;
            const DAY_SECONDS_F : f64 = DAY_SECONDS as f64;
            
            match src.duration_since( com_epoch ) {
                Ok( duration ) => {
                    let days = duration.as_secs() / DAY_SECONDS;
                    let time = duration.as_secs() % ( DAY_SECONDS );
                    let time_nanos = duration.subsec_nanos() as f64 / 1_000_000_000f64;
                    let time = ( time as f64 + time_nanos ) / DAY_SECONDS_F;

                    VariantDate( days as f64 + time )
                },
                Err( err ) => {
                    let duration = err.duration();
                    let days = duration.as_secs() / DAY_SECONDS;
                    let time = duration.as_secs() % ( DAY_SECONDS );
                    let time_nanos = duration.subsec_nanos() as f64 / 1_000_000_000f64;
                    let time = 1f64 - ( time as f64 + time_nanos ) / DAY_SECONDS_F;

                    VariantDate( days as f64 + time )
                }
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[allow(non_snake_case)]
    pub union VariantData {
        pub llVal : i64,
        pub lVal : i32,
        pub bVal : i8,
        pub iVal : i16,
        pub fltVal : f32,
        pub dblVal : f64,
        pub boolVal : VariantBool,
        pub scode : ::HRESULT,
        //cyVal : CY,
        pub date : VariantDate,
        pub bstrVal : *mut u16,
        pub punkVal : ::raw::InterfacePtr<::IUnknown>,
        //*pdispVal : ComItf<IDispatch>,
        //parray : SafeArray,
        pub pbVal : *mut i8,
        pub piVal : *mut i16,
        pub plVal : *mut i32,
        pub pllVal : *mut i64,
        pub pfltVal : *mut f32,
        pub pdblVal : *mut f64,
        pub pboolVal : *mut VariantBool,
        //*pscode : SCODE,
        //*pcyVal : CY,
        pub pdate : *mut VariantDate,
        pub pbstrVal : *mut *mut u16,
        pub ppunkVal : *mut ::raw::InterfacePtr<::IUnknown>,
        //ppdispVal : *mut ComItf<IDispatch>,
        //pparray : *mut SafeArray,
        pub pvarVal : *mut Variant,
        pub byref : *mut std::os::raw::c_void,
        pub cVal : u8,
        pub uiVal : u16,
        pub ulVal : u32,
        pub ullVal : u64,
        pub intVal : i32,
        pub uintVal : u32,
        //*pdecVal : DECIMAL,
        pub pcVal : *mut u8,
        pub puiVal : *mut u16,
        pub pulVal : *mut u32,
        pub pullVal : *mut u64,
        pub pintVal : *mut i32,
        pub puintVal : *mut u32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Variant {
        pub vt : VariantType,
        reserved1 : u16,
        reserved2 : u16,
        reserved3 : u16,
        pub data : VariantData,
    }

    impl Variant {
        fn new( vt : VariantType, data : VariantData ) -> Variant {
            Variant {
                vt: vt,
                reserved1: 0,
                reserved2: 0,
                reserved3: 0,
                data: data
            }
        }
    }

    impl Default for Variant {
        fn default() -> Variant {
            Variant::new(
                VariantType::new( EMPTY ),
                VariantData { lVal : 0 }
            )
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct VariantType(pub u16);

    impl VariantType {
        fn new( vt : u16 ) -> VariantType {
            VariantType( vt as u16 )
        }
    }

    #[allow(unused)]
    pub mod var_type {
        pub const EMPTY : u16 = 0;
        pub const NULL : u16 = 1;
        pub const I2 : u16 = 2;
        pub const I4 : u16 = 3;
        pub const R4 : u16 = 4;
        pub const R8 : u16 = 5;
        pub const CY : u16 = 6;
        pub const DATE : u16 = 7;
        pub const BSTR : u16 = 8;
        pub const DISPATCH : u16 = 9;
        pub const ERROR : u16 = 10;
        pub const BOOL : u16 = 11;
        pub const VARIANT : u16 = 12;
        pub const UNKNOWN : u16 = 13;
        pub const DECIMAL : u16 = 14;
        pub const I1 : u16 = 16;
        pub const UI1 : u16 = 17;
        pub const UI2 : u16 = 18;
        pub const UI4 : u16 = 19;
        pub const I8 : u16 = 20;
        pub const UI8 : u16 = 21;
        pub const INT : u16 = 22;
        pub const UINT : u16 = 23;
        pub const VOID : u16 = 24;
        pub const HRESULT : u16 = 25;
        pub const PTR : u16 = 26;
        pub const SAFEARRAY : u16 = 27;
        pub const CARRAY : u16 = 28;
        pub const USERDEFINED : u16 = 29;
        pub const LPSTR : u16 = 30;
        pub const LPWSTR : u16 = 31;
        pub const RECORD : u16 = 36;
        pub const INTPTR : u16 = 37;
        pub const UINTPTR : u16 = 38;
        pub const FILETIME : u16 = 64;
        pub const BLOB : u16 = 65;
        pub const STREAM : u16 = 66;
        pub const STORAGE : u16 = 67;
        pub const STREAMEDOBJECT : u16 = 68;
        pub const STOREDOBJECT : u16 = 69;
        pub const BLOBOBJECT : u16 = 70;
        pub const CF : u16 = 71;
        pub const CLSID : u16 = 72;
        pub const VERSIONEDSTREAM : u16 = 73;
        pub const BSTRBLOB : u16 = 0xFFF;

        pub const VECTOR : u16 = 0x1000;
        pub const ARRAY : u16 = 0x2000;
        pub const BYREF : u16 = 0x4000;
        pub const RESERVED : u16 = 0x8000;
        pub const ILLEGAL : u16 = 0xffff;
        pub const ILLEGALMASKED : u16 = 0xfff;
        pub const TYPEMASK : u16 = 0xfff;
    }

    pub struct VariantError( VariantType );

    impl From<VariantError> for ::ComError
    {
        fn from( _ : VariantError ) -> Self { ::E_INVALIDARG.into() }
    }

    use self::var_type::*;

    macro_rules! impl_from {
        ( $var_field:ident : $var_type:ident => $rust_type:ty ) => {

            impl From< $rust_type > for Variant {
                fn from( src : $rust_type ) -> Variant {
                    Variant::new(
                        VariantType::new( $var_type ),
                        VariantData { $var_field : src } )
                }
            }
        }
    }

    impl_from!( iVal : I2 => i16 );
    impl_from!( lVal : I4 => i32 );
    impl_from!( fltVal : R4 => f32 );
    impl_from!( dblVal : R8 => f64 );
    impl_from!( boolVal : BOOL => VariantBool );
    impl_from!( pvarVal : VARIANT => *mut Variant );
    impl_from!( bVal : I1 => i8 );
    impl_from!( cVal : UI1 => u8 );
    impl_from!( uiVal : UI2 => u16 );
    impl_from!( ulVal : UI4 => u32 );
    impl_from!( llVal : I8 => i64 );
    impl_from!( ullVal : UI8 => u64 );
    impl_from!( scode : HRESULT => ::HRESULT );

    impl From<BString> for Variant {
        fn from( src : BString ) -> Variant {
            Variant::new(
                VariantType::new( BSTR ),
                VariantData { bstrVal : src.into_ptr() } )
        }
    }

    impl TryFrom< Variant > for i8 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< i8, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::I1 => Ok( src.data.bVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::I1 => Ok( *src.data.pbVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for u8 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< u8, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::UI1 => Ok( src.data.cVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::UI1 => Ok( *src.data.pcVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for i16 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< i16, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::I2 => Ok( src.data.iVal as Self ),
                        var_type::I1 => Ok( src.data.bVal as Self ),
                        var_type::UI1 => Ok( src.data.cVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::I2 => Ok( *src.data.piVal as Self ),
                        var_type::I1 => Ok( *src.data.pbVal as Self ),
                        var_type::UI1 => Ok( *src.data.pcVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for u16 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< u16, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::UI2 => Ok( src.data.uiVal as Self ),
                        var_type::UI1 => Ok( src.data.cVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::UI2 => Ok( *src.data.puiVal as Self ),
                        var_type::UI1 => Ok( *src.data.pcVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for i32 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< i32, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::I4 => Ok( src.data.lVal ),
                        var_type::UI4 => Ok( src.data.ulVal as Self ),
                        var_type::I2 => Ok( src.data.iVal as Self ),
                        var_type::UI2 => Ok( src.data.uiVal as Self ),
                        var_type::I1 => Ok( src.data.bVal as Self ),
                        var_type::UI1 => Ok( src.data.cVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::I4 => Ok( *src.data.plVal ),
                        var_type::UI4 => Ok( *src.data.pulVal as Self ),
                        var_type::I2 => Ok( *src.data.piVal as Self ),
                        var_type::UI2 => Ok( *src.data.puiVal as Self ),
                        var_type::I1 => Ok( *src.data.pbVal as Self ),
                        var_type::UI1 => Ok( *src.data.pcVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for u32 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< u32, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::UI4 => Ok( src.data.ulVal as Self ),
                        var_type::UI2 => Ok( src.data.uiVal as Self ),
                        var_type::UI1 => Ok( src.data.cVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::UI4 => Ok( *src.data.pulVal as Self ),
                        var_type::UI2 => Ok( *src.data.puiVal as Self ),
                        var_type::UI1 => Ok( *src.data.pcVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for i64 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< i64, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::I8 => Ok( src.data.llVal ),
                        var_type::I4 => Ok( src.data.lVal as Self ),
                        var_type::UI4 => Ok( src.data.ulVal as Self ),
                        var_type::I2 => Ok( src.data.iVal as Self ),
                        var_type::UI2 => Ok( src.data.uiVal as Self ),
                        var_type::I1 => Ok( src.data.bVal as Self ),
                        var_type::UI1 => Ok( src.data.cVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::I8 => Ok( *src.data.pllVal ),
                        var_type::I4 => Ok( *src.data.plVal as Self ),
                        var_type::UI4 => Ok( *src.data.pulVal as Self ),
                        var_type::I2 => Ok( *src.data.piVal as Self ),
                        var_type::UI2 => Ok( *src.data.puiVal as Self ),
                        var_type::I1 => Ok( *src.data.pbVal as Self ),
                        var_type::UI1 => Ok( *src.data.pcVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for u64 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< u64, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::UI8 => Ok( src.data.ullVal ),
                        var_type::UI4 => Ok( src.data.ulVal as Self ),
                        var_type::UI2 => Ok( src.data.uiVal as Self ),
                        var_type::UI1 => Ok( src.data.cVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::UI8 => Ok( *src.data.pullVal ),
                        var_type::UI4 => Ok( *src.data.pulVal as Self ),
                        var_type::UI2 => Ok( *src.data.puiVal as Self ),
                        var_type::UI1 => Ok( *src.data.pcVal as Self ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for f32 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< f32, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::R4 => Ok( src.data.fltVal ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::R4 => Ok( *src.data.pfltVal ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for f64 {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< f64, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::R8 => Ok( src.data.dblVal ),
                        var_type::R4 => Ok( src.data.fltVal as f64 ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::R8 => Ok( *src.data.pdblVal ),
                        var_type::R4 => Ok( *src.data.pfltVal as f64 ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for bool {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< bool, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::BOOL => Ok( src.data.boolVal.0 != 0 ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::BOOL => Ok( ( *src.data.pboolVal ).0 != 0 ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for SystemTime {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< SystemTime, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::DATE => Ok( src.data.date.into() ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::DATE => Ok( ( *src.data.pdate ).into() ),
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    impl TryFrom< Variant > for BString {
        type Error = VariantError;
        fn try_from( src : Variant ) -> Result< BString, Self::Error > {
            unsafe {
                if src.vt.0 & var_type::BYREF == 0 {
                    match src.vt.0 & var_type::TYPEMASK {
                        var_type::BSTR => Ok( BString::from_ptr( src.data.bstrVal ) ),
                        _ => Err( VariantError( src.vt ) )
                    }
                } else {
                    match src.vt.0 & var_type::TYPEMASK {
                        _ => Err( VariantError( src.vt ) )
                    }
                }
            }
        }
    }

    // impl_from!( Z : EMPTY => u16 );
    // impl_from!( Z : NULL => u16 );
    // impl_from!( Z : CY => u16 );
    // impl_from!( Z : DATE => u16 );
    // impl_from!( bstrVal : BSTR => BString );
    // impl_from!( Z : DISPATCH => u16 );
    // impl_from!( Z : ERROR => u16 );
    // impl_from!( Z : UNKNOWN => u16 );
    // impl_from!( Z : DECIMAL => u16 );
    // impl_from!( intVal : INT => u16 );
    // impl_from!( uintVal : UINT => u16 );
    // impl_from!( Z : VOID => u16 );
    // impl_from!( Z : PTR => u16 );
    // impl_from!( Z : SAFEARRAY => u16 );
    // impl_from!( Z : CARRAY => u16 );
    // impl_from!( Z : USERDEFINED => u16 );
    // impl_from!( Z : LPSTR => u16 );
    // impl_from!( Z : LPWSTR => u16 );
    // impl_from!( Z : RECORD => u16 );
    // impl_from!( Z : INTPTR => u16 );
    // impl_from!( Z : UINTPTR => u16 );
    // impl_from!( Z : FILETIME => u16 );
    // impl_from!( Z : BLOB => u16 );
    // impl_from!( Z : STREAM => u16 );
    // impl_from!( Z : STORAGE => u16 );
    // impl_from!( Z : STREAMEDOBJECT => u16 );
    // impl_from!( Z : STOREDOBJECT => u16 );
    // impl_from!( Z : BLOBOBJECT => u16 );
    // impl_from!( Z : CF => u16 );
    // impl_from!( Z : CLSID => u16 );
    // impl_from!( Z : VERSIONEDSTREAM => u16 );
    // impl_from!( Z : BSTRBLOB => u16 );
}