use super::macros::*;
use super::*;
use crate::prelude::*;

use crate::guid::GUID;
use ::syn::{Ident, Visibility};

intercom_attribute!(
    ComClassAttr<ComClassAttrParam, Ident> {
        clsid : StrOption,
    }
);

/// Details of a struct marked with `#[com_class]` attribute.
#[derive(Debug, PartialEq)]
pub struct ComClass
{
    pub name: Ident,
    pub clsid: Option<GUID>,
    pub visibility: Visibility,
    pub interfaces: Vec<Ident>,
}

impl ComClass
{
    /// Creates ComClass from AST elements.
    pub fn parse(
        crate_name: &str,
        attr_params: TokenStream,
        item: TokenStream,
    ) -> ParseResult<ComClass>
    {
        // Parse the inputs.
        let item: ::syn::ItemStruct = ::syn::parse2(item)
            .map_err(|_| ParseError::ComClass("<Unknown>".into(), "Item syntax error".into()))?;

        let attr: ComClassAttr = ::syn::parse2(attr_params).map_err(|e| {
            ParseError::ComClass(
                item.ident.to_string(),
                format!("Attribute syntax error: {}", e),
            )
        })?;

        // First attribute parameter is the CLSID. Parse it.
        let clsid_attr = attr
            .clsid()
            .map_err(|msg| ParseError::ComClass(item.ident.to_string(), msg))?;
        let clsid = match clsid_attr {
            None => Some(crate::utils::generate_clsid(
                crate_name,
                &item.ident.to_string(),
            )),
            Some(StrOption::Str(clsid)) => Some(GUID::parse(&clsid.value()).map_err(|_| {
                ParseError::ComClass(item.ident.to_string(), "Bad CLSID format".into())
            })?),
            Some(StrOption::None) => None,
        };

        // Remaining parameters are coclasses.
        let interfaces = attr.args().into_iter().cloned().collect();

        Ok(ComClass {
            name: item.ident.clone(),
            visibility: item.vis.clone(),
            clsid,
            interfaces,
        })
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_com_class()
    {
        let cls = ComClass::parse(
            "not used",
            quote!(clsid = "12345678-1234-1234-1234-567890ABCDEF", Foo, Bar),
            quote!(
                struct S;
            ),
        )
        .expect("com_class attribute parsing failed");

        assert_eq!(cls.name, "S");
        assert_eq!(
            cls.clsid,
            Some(GUID::parse("12345678-1234-1234-1234-567890ABCDEF").unwrap())
        );
        assert_eq!(cls.interfaces.len(), 2);
        assert_eq!(cls.interfaces[0], "Foo");
        assert_eq!(cls.interfaces[1], "Bar");
    }

    #[test]
    fn parse_com_class_with_auto_guid()
    {
        // This test derives the GUID from the library name.
        //
        // What the final GUID is isn't important, what _is_ important however
        // is that the final GUID will not change ever as long as the library
        // name stays the same.
        let cls = ComClass::parse(
            "not used",
            quote!(MyStruct, IThings, IStuff),
            quote!(
                struct MyStruct
                {
                    a: u32,
                }
            ),
        )
        .expect("com_class attribute parsing failed");

        assert_eq!(cls.name, "MyStruct");
        assert_eq!(
            cls.clsid,
            Some(GUID::parse("28F57CBA-6AF4-3D3F-7C55-1CF1394D5C7A").unwrap())
        );
        assert_eq!(cls.interfaces.len(), 3);
        assert_eq!(cls.interfaces[0], "MyStruct");
        assert_eq!(cls.interfaces[1], "IThings");
        assert_eq!(cls.interfaces[2], "IStuff");
    }

    #[test]
    fn parse_com_class_with_no_data()
    {
        let cls = ComClass::parse(
            "not used",
            quote!(clsid = None),
            quote!(
                struct EmptyType;
            ),
        )
        .expect("com_class attribute parsing failed");

        assert_eq!(cls.name, "EmptyType");
        assert_eq!(cls.clsid, None);
        assert_eq!(cls.interfaces.len(), 0);
    }

    #[test]
    fn parse_com_class_with_no_guid_with_interface()
    {
        let cls = ComClass::parse(
            "not used",
            quote!(clsid = None, ITestInterface),
            quote!(
                struct EmptyType;
            ),
        )
        .expect("com_class attribute parsing failed");

        assert_eq!(cls.name, "EmptyType");
        assert_eq!(cls.clsid, None);
        assert_eq!(cls.interfaces.len(), 1);
    }
}
