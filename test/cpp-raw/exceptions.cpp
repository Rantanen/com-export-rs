
#ifdef __GNUC__

#include <functional>

#include "os.h"
#include "catch.hpp"

#include "testlib.h"
#include "utility/dummy_interface.h"

TEST_CASE( "Exceptions have correct error message." )
{
    SECTION( "NoSuchInterface with only interface id set has correct error message." )
    {
        try
        {
            throw intercom::NoSuchInterface( IID_IUnknown );
            FAIL( "Exception not thrown." );
        }
        catch( intercom::NoSuchInterface& ex )
        {
            CHECK( std::string( "Interface \"{00000000-0000-0000-C000-000000000046}\" not available." )
                    == ex.what() );
        }
    }

    SECTION( "NoSuchInterface with both interface id and class id set has correct error message." )
    {
        try
        {
            throw intercom::NoSuchInterface(
                    cppraw::utility::DummyInterfaceDescriptor::ID,
                    cppraw::utility::IDummyInterface::ID );
            FAIL( "Exception not thrown." );
        }
        catch( intercom::NoSuchInterface& ex )
        {
            CHECK( std::string( "Interface \"{12345678-0000-0000-1234-567800000000}\" not "\
                    "available for the class \"{12345678-1234-1234-1234-567800000000}\"." ) == ex.what() );
        }
    }

    SECTION( "RuntimeError preserves error message." )
    {
        try
        {
            throw intercom::RuntimeError( intercom::E_FAIL, "This message is preserved." );
            FAIL( "Exception not thrown." );
        }
        catch( intercom::RuntimeError& ex )
        {
            CHECK( std::string( "This message is preserved." ) == ex.what() );
        }
    }
}

TEST_CASE( "Exception specific data is passed correctly." )
{
    SECTION( "NoSuchInterface preserves interface id." )
    {
        try
        {
            throw intercom::NoSuchInterface( IID_IUnknown );
            FAIL( "Exception not thrown." );
        }
        catch( intercom::NoSuchInterface& ex )
        {
            CHECK( IID_IUnknown == ex.interface_id() );
        }
    }

    SECTION( "RuntimeError preserved its error code." )
    {
        try
        {
            throw intercom::RuntimeError( intercom::E_INVALIDARG, "This message is preserved." );
            FAIL( "Exception not thrown." );
        }
        catch( intercom::RuntimeError& ex )
        {
            CHECK( E_INVALIDARG == ex.error_code() );
        }
    }
}

#endif