extern crate intercom;
extern crate intercom_common;
mod os;
mod host;

use std::io::Write;

#[derive(Debug)]
pub enum BuildError {
    ParseError( String ),
    IoError( String, std::io::Error ),
    CommandError( String, String, String ),
}

impl BuildError {
    fn print_as_warning( &self, w : &mut dyn Write ) -> Result<(), std::io::Error> {

        let msg : std::borrow::Cow<str> = match *self {
            BuildError::ParseError( ref m, .. ) => m.into(),
            BuildError::IoError( ref m, ref e ) => format!( "{}: {}", m, e ).into(),
            BuildError::CommandError( ref m, .. ) => m.into(),
        };

        writeln!( w, "cargo:warning={}", msg )?;

        // Handle all the errors that provide extra info.
        // (Only one currently, but we'll still want to structure this as a match)
        #[allow(clippy::single_match)]
        match *self {
            BuildError::CommandError( _, ref stdout, ref stderr ) => {
                if ! stdout.is_empty() {
                    writeln!( w, "cargo:warning=" )?;
                    writeln!( w, "cargo:warning=Program stdout:" )?;
                    writeln!( w, "cargo:warning=---------------" )?;
                    for line in stdout.split( '\n' ) {
                        writeln!( w, "cargo:warning={}", line )?;
                    }
                }

                if ! stderr.is_empty() {
                    writeln!( w, "cargo:warning=" )?;
                    writeln!( w, "cargo:warning=Program stderr:" )?;
                    writeln!( w, "cargo:warning=---------------" )?;
                    for line in stderr.split( '\n' ) {
                        writeln!( w, "cargo:warning={}", line )?;
                    }
                }

                // If the program didn't provide stdout or stderr, we'll want to
                // inform the user of that so they aren't left confused on why
                // our error messages are crap.
                if stdout.is_empty() && stderr.is_empty() {
                    writeln!( w, "cargo:warning=(Command produced no output)" )?;
                }
            },
            _ => {}  // No extra lines.
        }

        Ok(())
    }
}

pub fn build( all_type_systems : bool ) {
    match os::build( all_type_systems ) {
        Ok(..) => {}
        Err( e ) => {
            e.print_as_warning( &mut std::io::stdout() )
                    .expect( "Cannot write to stdout" );
            println!( "cargo:warning=Some Intercom functionality might not be available" );
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_basic_error() {
        let e = BuildError::ParseError( "Error message".to_string() );
        let mut v = vec![];
        e.print_as_warning( &mut v ).expect( "Failed to write string" );

        let s = String::from_utf8_lossy( &v );
        assert_eq!( s, "cargo:warning=Error message\n" );
    }

    #[test]
    fn test_command_error() {
        let e = BuildError::CommandError(
            "Error message".to_string(),
            "stdout".to_string(),
            "stderr".to_string(),
        );
        let mut v = vec![];
        e.print_as_warning( &mut v ).expect( "Failed to write string" );

        let s = String::from_utf8_lossy( &v );
        assert_eq!( s, "\
            cargo:warning=Error message\n\
            cargo:warning=\n\
            cargo:warning=Program stdout:\n\
            cargo:warning=---------------\n\
            cargo:warning=stdout\n\
            cargo:warning=\n\
            cargo:warning=Program stderr:\n\
            cargo:warning=---------------\n\
            cargo:warning=stderr\n" );
    }

    #[test]
    fn test_command_error_no_output() {
        let e = BuildError::CommandError(
            "Error message".to_string(),
            String::new(),
            String::new() );
        let mut v = vec![];
        e.print_as_warning( &mut v ).expect( "Failed to write string" );

        let s = String::from_utf8_lossy( &v );
        assert_eq!( s, "\
            cargo:warning=Error message\n\
            cargo:warning=(Command produced no output)\n" );
    }
}
