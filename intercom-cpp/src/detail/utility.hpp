
#ifndef INTERCOM_CPP_DETAIL_UTILITY_H
#define INTERCOM_CPP_DETAIL_UTILITY_H


#include <cstring>
#include <iomanip>
#include <sstream>
#include <type_traits>
#include <functional>


namespace intercom
{
namespace detail
{

    /**
     * @brief Tests whether the machine is little endian or not.
     *
     * @return true The machine is little endian.
     * @return false The machine is big endian.
     */
    inline bool is_little_endian()
    {
        static_assert( sizeof( unsigned char ) < sizeof( int ), "" );
        int test_value = 1;
        unsigned char* asBytes = reinterpret_cast< unsigned char* >( &test_value );
        return asBytes[ 0 ] == 1;
    }

    /**
     * @brief Writes the specified binary data into the stream as hex.
     *
     * @param stream Target stream.
     * @param converter The binary data to write.
     * @return std::ostream& Returns the stream.
     */
    template< typename TData, typename = typename std::enable_if< std::is_unsigned< TData >::value, void >::type >
    inline std::ostream& write_as_hex(
        std::ostream& stream,
        TData data
    )
    {
        // Determine the order in which we need to print the data.
        stream << std::hex << std::uppercase;
        unsigned char* asBytes = reinterpret_cast< unsigned char* >( &data );
        if( is_little_endian() )
        {
            // On little-endian machine the bytes are on reverse order.
            unsigned char* rbegin = asBytes + sizeof( TData ) - 1;
            unsigned char* rend = asBytes - 1;
            for( auto byte = rbegin; byte != rend; --byte )
            {
                // Explicit cast required, otherwise byte is treated as unsigned char.
                stream << static_cast< unsigned int >( ( *byte & 0xF0 ) >> 4 );
                stream << static_cast< unsigned int >( *byte & 0x0F );
            }
        }
        else
        {
            unsigned char* begin = asBytes;
            unsigned char* end = asBytes + sizeof( TData );
            for( auto byte = begin; byte != end; ++byte )
            {
                // Explicit cast required, otherwise byte is treated as unsigned char.
                stream << static_cast< unsigned int >( ( *byte & 0xF0 ) >> 4 );
                stream << static_cast< unsigned int >( *byte & 0x0F );
            }
        }

        return stream;
    }

    //! Hashes a value to an existing hash.
    template< typename TValue >
    inline void hash_combine(
        size_t& seed,
        const TValue& value
    ) noexcept
    {
        // Implemenation copied from boost.
        std::hash< TValue > hasher;
        seed ^= hasher( value ) + 0x9e3779b9 + (seed<<6) + (seed>>2);
    }
}
}

#endif
