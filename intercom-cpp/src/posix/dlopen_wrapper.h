
#ifndef INTERCOM_CPP_POSIX_DLOPENWRAPPER_H
#define INTERCOM_CPP_POSIX_DLOPENWRAPPER_H

#include <dlfcn.h>
#include <string>

namespace intercom
{
namespace cpp
{
namespace posix
{

//!
class dlopen_wrapper
{
public:

    //!
    enum class rtld
    {
        /*!
         Perform lazy binding. Only resolve symbols as the code that references them is executed.
         If the symbol is never referenced, then it is never resolved. (Lazy binding is only performed for
         function references; references to variables are always immediately bound when the library is loaded.)
         */
        lazy = 0x00001,

        /*!
        If this value is specified, or the environment variable LD_BIND_NOW is set to a nonempty string,
        all undefined symbols in the library are resolved before dlopen() returns.
        If this cannot be done, an error is returned.
        Zero or more of the following values may also be ORed in flag:
        */
        now = 0x00002,

        /*!
        The symbols defined by this library will be made available for symbol resolution of subsequently loaded libraries.
        */
        global = 0x00100,

        /*!
        This is the converse of RTLD_GLOBAL, and the default if neither flag is specified.
        Symbols defined in this library are not made available to resolve references in subsequently loaded libraries.
        */
        local = 0,

        /*!
        Do not unload the library during dlclose().
        Consequently, the library's static variables are not reinitialized if
        the library is reloaded with dlopen() at a later time.
        This flag is not specified in POSIX.1-2001. (since glibc 2.2)
        */
        nodelete = 0x01000,

        /*!
        Don't load the library. This can be used to test if the library is already resident (dlopen()
        returns NULL if it is not, or the library's handle if it is resident).
        This flag can also be used to promote the flags on a library that is already loaded.
        For example, a library that was previously loaded with RTLD_LOCAL can be reopened
        with RTLD_NOLOAD | RTLD_GLOBAL.
        This flag is not specified in POSIX.1-2001. (since glibc 2.2)
        */
        noload = 0x00004,

        /*!
        Place the lookup scope of the symbols in this library ahead of the global scope.
        This means that a self-contained library will use its own symbols in preference to
        global symbols with the same name contained in libraries that have already been loaded.
        This flag is not specified in POSIX.1-2001. (since glibc 2.3.4)
        */
        deepbind = 0x00008,
    };

    //! Loads a dynamic shared objec into process' address space.
    dlopen_wrapper(
        const char* file,
        rtld flags
    ) :
        m_file( file ),
        m_flags( flags ),
        m_handle( nullptr )
    {
        reset_error_status();
        m_handle = dlopen( m_file.c_str(), ( int ) m_flags );
        if( m_handle == nullptr )
            throw_if_failed();
    }

    //! Decrements the reference count of the dynamic shared object.
    ~dlopen_wrapper()
    {
        // Proper lifetime management has not been implemented for dlopen_wrapper.
        //if( m_handle != nullptr )
        //    dlclose( m_handle );
    }

    //! Loads an address of a function in the library.
    template< typename TFunction >
    TFunction load_function(
        const char* name
    )
    {
        // Process errors as describe in the man page of dlsym.
        // https://linux.die.net/man/3/dlsym
        reset_error_status();
        void* function = dlsym( m_handle, name );
        throw_if_failed();

        return ( TFunction ) function;
    }

private:

    //! Throws an exception if last call to dl* failed.
    void throw_if_failed()
    {
        std::string error;
        if( try_get_error( &error ) )
            throw std::exception();
    }

    //! Attempts to get an
    bool try_get_error(
        std::string* error
    ) const
    {
        char* errorLocal = dlerror();
        if( errorLocal != nullptr )
        {
            (*error) = errorLocal;
            return true;
        }
        else
        {
            error->clear();
            return false;
        }
    }

    //! Resets any previous error f
    void reset_error_status() const { dlerror(); }

private:

    std::string m_file;  //!< The library load with the wrapper.
    rtld m_flags;  //!< Flags used to load the library.
    void* m_handle;  //!< Handle to a library opened wiht dlopen.
};

}
}
}

#endif
