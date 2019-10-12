//!
//! COM library parse model.
//!
//! Defines the items constructed from the various COM attributes.
//!
//! Should unify COM attribute expansion and crate parsing for IDL/Manifest/etc.
//! purposes in the future.
//!

use super::*;

use crate::ast_converters::*;
use crate::builtin_model;
use ::std::path::{Path, PathBuf};
use ::std::fs;
use ::std::io::Read;
use ::indexmap::IndexMap;
use ::std::iter::FromIterator;
use toml;

#[derive(Debug, PartialEq)]
pub struct ComCrate {
    lib : Option<ComLibrary>,
    interfaces : IndexMap<String, ComInterface>,
    interfaces_by_variants : IndexMap<String, String>,
    structs : IndexMap<String, ComStruct>,
    impls : Vec<ComImpl>,
    incomplete : bool,
}

#[derive(Default)]
struct ComCrateBuilder {
    pub libs : Vec<ComLibrary>,
    pub interfaces : Vec<ComInterface>,
    pub structs : Vec<ComStruct>,
    pub impls : Vec<ComImpl>,
    pub incomplete : bool,
}

impl ComCrateBuilder {

    pub fn build( self ) -> ParseResult<ComCrate>
    {
        if self.libs.len() > 1 {
            return Err( ParseError::ComLibrary(
                    "Multiple [com_library] attributes".into() ) );
        }

        let interfaces_by_variants = {
            IndexMap::from_iter( self.interfaces.iter()
                    .flat_map( |itf| itf.variants().iter().map( |(_, itf_variant)|
                         ( itf_variant.unique_name().to_string(),
                            itf.name().to_string() ) ).collect::<Vec<_>>() ) )
        };

        Ok( ComCrate {
            lib: self.libs.into_iter().next(),
            interfaces: IndexMap::from_iter(
                self.interfaces.into_iter().map( |i| ( i.name().to_string(), i ) ) ),
            interfaces_by_variants,
            structs: IndexMap::from_iter(
                self.structs.into_iter().map( |i| ( i.name().to_string(), i ) ) ),
            impls: self.impls,
            incomplete: self.incomplete,
        } )
    }

    pub fn include_builtin( &mut self, crate_name : &str ) {

        let built_in_types = builtin_model::builtin_intercom_types( crate_name );
        for bti in built_in_types {
            self.structs.push( bti.class );
            self.interfaces.push( bti.interface );
            self.impls.push( bti.implementation );
        }

        let built_in_types = builtin_model::builtin_intercom_types( crate_name );
        for lib in &mut self.libs {
            for clsid in built_in_types.iter().filter_map( |bti|
                    if bti.class.clsid().is_some() {
                        Some( bti.class.name().clone() )
                    } else {
                        None
                    } ) {

                lib.add_coclass( parse_quote!( intercom::#clsid ) )
            }
        }
    }
}

impl ComCrate
{
    pub fn parse(
        crate_name : &str,
        sources : &[&str]
    ) -> ParseResult<ComCrate>
    {
        let mut builder : ComCrateBuilder = Default::default();

        for src in sources {
            let krate : ::syn::File = ::syn::parse_str( src )
                .map_err( |_| ParseError::ComCrate(
                        "Failed to parse source".into() ) )?;

            Self::process_crate_items(
                crate_name,
                None,
                &krate.items,
                &mut builder )?;
        }

        builder.include_builtin( crate_name );
        builder.build()
    }

    pub fn parse_package(
        crate_path : &Path,
    ) -> ParseResult<ComCrate>
    {
        if crate_path.is_file() {
            Self::parse_cargo_toml( crate_path )
        } else {
            Self::parse_cargo_toml( &crate_path.join( "Cargo.toml" ) )
        }
    }

    pub fn parse_cargo_toml(
        toml_path : &Path,
    ) -> ParseResult<ComCrate>
    {
        let mut f = fs::File::open( toml_path )
                .map_err( |_| ParseError::CargoToml(
                        "Could not open Cargo toml".into() ) )?;
        let mut buf = String::new();
        f.read_to_string( &mut buf )
                .map_err( |_| ParseError::CargoToml(
                        "Could not read Cargo toml".into() ) )?;

        let toml = buf.parse::<toml::Value>()
                .map_err( |_| ParseError::CargoToml(
                        "Could not parse Cargo toml".into() ) )?;
        let root = match toml {
            toml::Value::Table( root ) => root,
            _ => return Err( ParseError::CargoToml(
                        "Invalid TOML root element".into() ) ),
        };

        let lib_name = match root.get( "package" ) {
                    Some( &toml::Value::Table( ref package ) )
                        => match package.get( "name" ) {
                            Some( &toml::Value::String( ref name ) )
                                => name,
                            _ => return Err( ParseError::CargoToml(
                                    "No 'name' parameter under [package]".into() ) ),
                        },
                    _ => return Err( ParseError::CargoToml(
                            "Could not find [package] in Cargo.toml".into() ) ),
                };

        let rel_lib_path = PathBuf::from( &match root.get( "lib" ) {
                    Some( &toml::Value::Table( ref package ) )
                        => match package.get( "path" ) {
                            Some( &toml::Value::String( ref path ) )
                                => path.clone(),
                            _ => "src/lib.rs".to_owned(),
                        },
                    _ => "src/lib.rs".to_owned(),
                } );
        let lib_path = match toml_path.parent() {
                    Some( p ) => p.join( rel_lib_path ),
                    _ => rel_lib_path
                };

        Self::parse_file( lib_name, &lib_path )
    }

    pub fn parse_file(
        crate_name : &str,
        path : &Path
    ) -> ParseResult<ComCrate>
    {
        let mut builder : ComCrateBuilder = Default::default();

        Self::parse_file_internal( crate_name, path, &mut builder )?;

        builder.include_builtin( crate_name );
        builder.build()
    }

    fn parse_file_internal(
        crate_name : &str,
        path : &Path,
        b : &mut ComCrateBuilder
    ) -> ParseResult<()>
    {
        let mut f = fs::File::open( path )
                .map_err( |_| ParseError::ComCrate(
                        format!( "Could not open file {}", path.to_string_lossy() ) ) )?;

        let mut buf = String::new();
        f.read_to_string( &mut buf )
                .map_err( |_| ParseError::ComCrate(
                        format!( "Could not read file {}", path.to_string_lossy() ) ) )?;

        let krate = ::syn::parse_file( &buf )
                .map_err( |_| ParseError::ComCrate(
                        format!( "Failed to parse source {}", path.to_string_lossy() ) ) )?;

        Self::process_crate_items( crate_name, Some( path ), &krate.items, b )
    }

    fn process_crate_items(
        crate_name : &str,
        path : Option< &Path >,
        items : &[ ::syn::Item ],
        b : &mut ComCrateBuilder,
    ) -> ParseResult<()>
    {
        Self::collect_items( crate_name, items, b )?;

        for item in items {
            let mod_item =
                    if let ::syn::Item::Mod( ref m ) = *item {
                        m
                    } else {
                        continue;
                    };

            match mod_item.content {
                None => {

                    // The mod doesn't have immediate items so this is an
                    // external mod. We need to resolve the file.
                    let path = if let Some( p ) = path { p } else {

                        // No path given. Mark the crate as incomplete as we
                        // couldn't resolve all pieces but return with Ok
                        // result.
                        //
                        // This is a case where we were given file contents
                        // without the caller knowing (or telling) where the
                        // file was located. We can't resolve relative mod-paths
                        // in this case.
                        b.incomplete = true;
                        return Ok(());
                    };

                    // We have couple of options. Find the first one that
                    // matches an existing file.
                    let mut mod_paths = vec![
                        path.parent().unwrap().join( format!( "{}.rs", mod_item.ident ) ),
                        path.parent().unwrap().join( format!( "{}/mod.rs", mod_item.ident ) ),
                    ].into_iter()
                        .filter( |p| p.exists() );

                    let mod_path = mod_paths.next()
                        .ok_or_else( || ParseError::ComCrate(
                                format!( "Could not find mod {}", mod_item.ident ) ) )?;

                    let more = mod_paths.next();
                    if more.is_some() {
                        return Err( ParseError::ComCrate(
                                format!( "Ambiguous mod, both {0}.rs and \
                                          {0}/mod.rs present", mod_item.ident ) ) );
                    }

                    Self::parse_file_internal( crate_name, &mod_path, b )?;
                },
                Some( ( _, ref mod_items ) )
                    => Self::process_crate_items( crate_name, path, mod_items, b )?
            }
        }

        Ok(())
    }

    fn collect_items(
        crate_name : &str,
        items : &[ ::syn::Item ],
        b : &mut ComCrateBuilder,
    ) -> ParseResult<()>
    {
        for item in items {

            // Test for 'com_library' macro.
            if let ::syn::Item::Macro( m ) = item {
                if let Some( ident ) = m.mac.path.get_ident() {
                    if ident == "com_library" {
                        b.libs.push(ComLibrary::parse(crate_name, m.mac.tokens.clone())?);
                    }
                }
            }

            // The rest of the items we're interested in are attributes.
            for attr in &item.get_attributes().unwrap() {
                match attr.path.get_ident().unwrap().to_string().as_ref() {
                    "com_interface" =>
                        b.interfaces.push( ComInterface::from_ast(
                                crate_name, attr.tokens.clone(), item )? ),
                    "com_class" =>
                        if let ::syn::Item::Struct( ref s ) = *item {
                            b.structs.push( ComStruct::from_ast(
                                    crate_name, attr.tokens.clone(), s )? )
                        } else {
                            return Err( ParseError::ComStruct(
                                    item.get_ident().unwrap().to_string(),
                                    "Only structs may be COM classes".to_string() ) );
                        },
                    "com_impl" =>
                        b.impls.push( ComImpl::from_ast( item )? ),
                    _ => { }
                }
            }
        }

        Ok(())
    }


    pub fn lib( &self ) -> &Option<ComLibrary> { &self.lib }
    pub fn interfaces( &self ) -> &IndexMap<String, ComInterface> { &self.interfaces }
    pub fn structs( &self ) -> &IndexMap<String, ComStruct> { &self.structs }
    pub fn impls( &self ) -> &Vec<ComImpl> { &self.impls }
    pub fn is_incomplete( &self ) -> bool { self.incomplete }

    pub fn interface_by_name( &self, name : &str ) -> Option<&ComInterface> {
        self.interfaces_by_variants
                .get( name )
                .and_then( |itf_name| self.interfaces.get( itf_name ) )
                .or_else( || self.interfaces.get( name ) )
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use crate::guid::GUID;
    use crate::tyhandlers::ModelTypeSystem::*;

    #[test]
    fn parse_crate() {
        let krate = ComCrate::parse( "my_crate", &[
            r#"
                com_library!(
                        libid = "12345678-1234-1234-1234-567890000000",
                        Foo, Bar );

                #[com_interface(
                        com_iid = "12345678-1234-1234-1234-567890000001",
                        raw_iid = "12345678-1234-1234-1234-567890000002",
                )]
                trait IFoo {}

                trait IBar {}
            "#,
            r#"
                #[com_class(
                        clsid = "12345678-1234-1234-1234-567890000003",
                        IFoo )]
                struct S;

                #[com_impl]
                impl IFoo for S {}
            "#
        ] ).expect( "Parsing the crate failed" );

        assert!( krate.lib.is_some() );
        assert_eq!( krate.lib.as_ref().unwrap().libid(),
            &GUID::parse( "12345678-1234-1234-1234-567890000000" ).unwrap() );

        // The interfaces should contain the built-in interface.
        assert_eq!( krate.interfaces().len(), 3 );

        assert_eq!( krate.interfaces()[ "IFoo" ].variants()[ &Automation ].iid(),
            &GUID::parse( "12345678-1234-1234-1234-567890000001" ).unwrap() );
        assert_eq!( krate.interfaces()[ "IFoo" ].variants()[ &Raw ].iid(),
            &GUID::parse( "12345678-1234-1234-1234-567890000002" ).unwrap() );

        assert_eq!( krate.interfaces()[ "Allocator" ].variants()[ &Automation ].iid(),
            &GUID::parse( "18EE22B3-B0C6-44A5-A94A-7A417676FB66" ).unwrap() );
        assert_eq!( krate.interfaces()[ "Allocator" ].variants()[ &Raw ].iid(),
            &GUID::parse( "7A6F6564-04B5-4455-A223-EA0512B8CC63" ).unwrap() );

        assert_eq!( krate.interfaces()[ "ErrorStore" ].variants()[ &Automation ].iid(),
            &GUID::parse( "D7F996C5-0B51-4053-82F8-19A7261793A9" ).unwrap() );
        assert_eq!( krate.interfaces()[ "ErrorStore" ].variants()[ &Raw ].iid(),
            &GUID::parse( "7586C49A-ABBD-4A06-B588-E3D02B431F01" ).unwrap() );

        assert_eq!( krate.structs().len(), 3 );
        assert_eq!( krate.structs()[ "S" ].clsid().as_ref().unwrap(),
            &GUID::parse( "12345678-1234-1234-1234-567890000003" ).unwrap() );
        assert_eq!( krate.structs()[ "Allocator" ].clsid().as_ref().unwrap(),
            &GUID::parse( "1582F0E9-9CAB-3E18-7F37-0CF2CD9DA33A" ).unwrap() );
        assert_eq!( krate.structs()[ "ErrorStore" ].clsid().as_ref().unwrap(),
            &GUID::parse( "1CC4A0E8-C882-37B4-53FA-BC9E6030DF56" ).unwrap() );

        assert_eq!( krate.impls().len(), 3 );
        assert_eq!( krate.impls()[0].struct_name(), "S" );
        assert_eq!( krate.impls()[0].interface_name(), "IFoo" );
        assert_eq!( krate.impls()[1].struct_name(), "Allocator" );
        assert_eq!( krate.impls()[1].interface_name(), "Allocator" );
        assert_eq!( krate.impls()[2].struct_name(), "ErrorStore" );
        assert_eq!( krate.impls()[2].interface_name(), "ErrorStore" );
    }

    #[test]
    fn parse_incomplete_crate() {
        let krate = ComCrate::parse( "my_crate", &[
            r#"
                mod foo;
            "#,
        ] ).expect( "Parsing the crate failed" );

        assert!( krate.is_incomplete() );
    }
}
