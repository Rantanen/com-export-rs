
#include <array>
#include <intercom.h>
namespace test_lib
{
namespace raw
{
    struct IRefCount;
    struct IPrimitiveOperations;
    struct IStatefulOperations;
    struct IResultOperations;
    struct IClassCreator;
    struct ICreatedClass;
    struct IParent;
    struct IRefCountOperations;
    struct ISharedInterface;
    struct IErrorSource;
    struct IRefCount : IUnknown
    {
        static constexpr intercom::IID ID = {0xaa5b7352,0x5d7a,0x35b9,{0x52,0x06,0x14,0x5b,0x04,0x1f,0x2c,0x1c}};

        virtual uint32_t INTERCOM_CC GetRefCount(  ) = 0;
    };

    struct IPrimitiveOperations : IUnknown
    {
        static constexpr intercom::IID ID = {0x12341234,0x1234,0x1234,{0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x02}};

        virtual int8_t INTERCOM_CC I8( int8_t v ) = 0;
        virtual uint8_t INTERCOM_CC U8( uint8_t v ) = 0;
        virtual uint16_t INTERCOM_CC U16( uint16_t v ) = 0;
        virtual int16_t INTERCOM_CC I16( int16_t v ) = 0;
        virtual int32_t INTERCOM_CC I32( int32_t v ) = 0;
        virtual uint32_t INTERCOM_CC U32( uint32_t v ) = 0;
        virtual int64_t INTERCOM_CC I64( int64_t v ) = 0;
        virtual uint64_t INTERCOM_CC U64( uint64_t v ) = 0;
        virtual double INTERCOM_CC F64( double v ) = 0;
        virtual float INTERCOM_CC F32( float v ) = 0;
    };

    struct IStatefulOperations : IUnknown
    {
        static constexpr intercom::IID ID = {0x2b9bddd2,0x31f5,0x3d4b,{0x79,0xa0,0xac,0x8e,0x8d,0x11,0xa9,0x3e}};

        virtual void INTERCOM_CC PutValue( int32_t v ) = 0;
        virtual int32_t INTERCOM_CC GetValue(  ) = 0;
    };

    struct IResultOperations : IUnknown
    {
        static constexpr intercom::IID ID = {0xffb673d9,0x7896,0x3a4c,{0x4f,0xa8,0xf7,0x24,0x06,0x58,0x8a,0xa1}};

        virtual intercom::HRESULT INTERCOM_CC SOk(  ) = 0;
        virtual intercom::HRESULT INTERCOM_CC NotImpl(  ) = 0;
        virtual intercom::HRESULT INTERCOM_CC Sqrt( double value, double* __out ) = 0;
        virtual intercom::HRESULT INTERCOM_CC Tuple( uint32_t value, uint16_t* __out1, uint16_t* __out2 ) = 0;
    };

    struct IClassCreator : IUnknown
    {
        static constexpr intercom::IID ID = {0x2e7e23e8,0xf66d,0x3779,{0x6c,0x74,0x61,0x89,0x8d,0x7b,0x40,0xcd}};

        virtual intercom::HRESULT INTERCOM_CC CreateRoot( int32_t id, ICreatedClass** __out ) = 0;
        virtual intercom::HRESULT INTERCOM_CC CreateChild( int32_t id, IParent* parent, ICreatedClass** __out ) = 0;
    };

    struct ICreatedClass : IUnknown
    {
        static constexpr intercom::IID ID = {0x104eb174,0xfd00,0x3ecf,{0x7e,0x0d,0xd9,0x65,0x56,0x42,0x79,0xe7}};

        virtual intercom::HRESULT INTERCOM_CC GetId( int32_t* __out ) = 0;
        virtual intercom::HRESULT INTERCOM_CC GetParentId( int32_t* __out ) = 0;
    };

    struct IParent : IUnknown
    {
        static constexpr intercom::IID ID = {0xcea1c199,0xbf71,0x3b0a,{0x5a,0x4c,0xee,0x3f,0x5a,0x0a,0xe5,0xce}};

        virtual int32_t INTERCOM_CC GetId(  ) = 0;
    };

    struct IRefCountOperations : IUnknown
    {
        static constexpr intercom::IID ID = {0x6b198a07,0x2d86,0x340e,{0x71,0x7e,0xf4,0x16,0xa3,0x90,0x5d,0x6e}};

        virtual intercom::HRESULT INTERCOM_CC GetNew( IRefCountOperations** __out ) = 0;
        virtual uint32_t INTERCOM_CC GetRefCount(  ) = 0;
    };

    struct ISharedInterface : IUnknown
    {
        static constexpr intercom::IID ID = {0x1df08ff6,0xaafb,0x37ec,{0x53,0xcf,0xcd,0xe2,0x49,0xce,0xeb,0x4b}};

        virtual uint32_t INTERCOM_CC GetValue(  ) = 0;
        virtual void INTERCOM_CC SetValue( uint32_t v ) = 0;
        virtual intercom::HRESULT INTERCOM_CC DivideBy( ISharedInterface* divisor, uint32_t* __out ) = 0;
    };

    struct IErrorSource : IUnknown
    {
        static constexpr intercom::IID ID = {0x5505b7b6,0x5ca4,0x3e38,{0x66,0x7b,0xa9,0x82,0x3f,0x1d,0x5a,0x0f}};

        virtual intercom::HRESULT INTERCOM_CC StoreError( intercom::HRESULT hr, intercom::BSTR desc ) = 0;
    };


    class RefCountOperationsDescriptor
    {
        static constexpr CLSID ID = {0xf06af5f0,0x745a,0x3b29,{0x48,0x39,0xd2,0xd3,0x9a,0x3f,0x08,0xcd}};

        static constexpr std::array<intercom::IID, 1> INTERFACES = {
            test_lib::raw::IRefCountOperations::ID
        };

        RefCountOperationsDescriptor() = delete;
        ~RefCountOperationsDescriptor() = delete;

    };
    

    class PrimitiveOperationsDescriptor
    {
        static constexpr CLSID ID = {0x12341234,0x1234,0x1234,{0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x01}};

        static constexpr std::array<intercom::IID, 1> INTERFACES = {
            test_lib::raw::IPrimitiveOperations::ID
        };

        PrimitiveOperationsDescriptor() = delete;
        ~PrimitiveOperationsDescriptor() = delete;

    };
    

    class StatefulOperationsDescriptor
    {
        static constexpr CLSID ID = {0x694c1893,0x2fa8,0x3d4c,{0x6a,0xcf,0x69,0xc5,0x93,0x66,0x72,0x1e}};

        static constexpr std::array<intercom::IID, 1> INTERFACES = {
            test_lib::raw::IStatefulOperations::ID
        };

        StatefulOperationsDescriptor() = delete;
        ~StatefulOperationsDescriptor() = delete;

    };
    

    class ResultOperationsDescriptor
    {
        static constexpr CLSID ID = {0xe5ce34c4,0xc1ad,0x34bc,{0x69,0xf6,0xd1,0xbf,0xa6,0xbb,0x25,0x96}};

        static constexpr std::array<intercom::IID, 1> INTERFACES = {
            test_lib::raw::IResultOperations::ID
        };

        ResultOperationsDescriptor() = delete;
        ~ResultOperationsDescriptor() = delete;

    };
    

    class ClassCreatorDescriptor
    {
        static constexpr CLSID ID = {0x3323cccd,0x1a38,0x33a4,{0x4a,0xe1,0x4d,0xc9,0x2a,0x7e,0x8d,0xc5}};

        static constexpr std::array<intercom::IID, 1> INTERFACES = {
            test_lib::raw::IClassCreator::ID
        };

        ClassCreatorDescriptor() = delete;
        ~ClassCreatorDescriptor() = delete;

    };
    

    class CreatedClassDescriptor
    {
        static constexpr CLSID ID = {0x51ed4fb8,0x35d8,0x36c6,{0x78,0xfd,0x6b,0xc5,0x82,0xc8,0x4b,0x19}};

        static constexpr std::array<intercom::IID, 3> INTERFACES = {
            test_lib::raw::ICreatedClass::ID,
            test_lib::raw::IParent::ID,
            test_lib::raw::IRefCount::ID
        };

        CreatedClassDescriptor() = delete;
        ~CreatedClassDescriptor() = delete;

    };
    

    class SharedImplementationDescriptor
    {
        static constexpr CLSID ID = {0x88687644,0x9cb2,0x3bd6,{0x4c,0x23,0xdb,0x54,0x7d,0x39,0x90,0x29}};

        static constexpr std::array<intercom::IID, 1> INTERFACES = {
            test_lib::raw::ISharedInterface::ID
        };

        SharedImplementationDescriptor() = delete;
        ~SharedImplementationDescriptor() = delete;

    };
    

    class ErrorSourceDescriptor
    {
        static constexpr CLSID ID = {0x2af563c2,0xdc1c,0x339a,{0x60,0x35,0xf5,0xf8,0x18,0x0f,0xae,0x86}};

        static constexpr std::array<intercom::IID, 1> INTERFACES = {
            test_lib::raw::IErrorSource::ID
        };

        ErrorSourceDescriptor() = delete;
        ~ErrorSourceDescriptor() = delete;

    };
    
}
}
#ifdef INTERCOM_FLATTEN_DECLARATIONS
    static constexpr intercom::IID IID_IRefCount = {0xaa5b7352,0x5d7a,0x35b9,{0x52,0x06,0x14,0x5b,0x04,0x1f,0x2c,0x1c}};
    using IRefCount = test_lib::raw::IRefCount;
    static constexpr intercom::IID IID_IPrimitiveOperations = {0x12341234,0x1234,0x1234,{0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x02}};
    using IPrimitiveOperations = test_lib::raw::IPrimitiveOperations;
    static constexpr intercom::IID IID_IStatefulOperations = {0x2b9bddd2,0x31f5,0x3d4b,{0x79,0xa0,0xac,0x8e,0x8d,0x11,0xa9,0x3e}};
    using IStatefulOperations = test_lib::raw::IStatefulOperations;
    static constexpr intercom::IID IID_IResultOperations = {0xffb673d9,0x7896,0x3a4c,{0x4f,0xa8,0xf7,0x24,0x06,0x58,0x8a,0xa1}};
    using IResultOperations = test_lib::raw::IResultOperations;
    static constexpr intercom::IID IID_IClassCreator = {0x2e7e23e8,0xf66d,0x3779,{0x6c,0x74,0x61,0x89,0x8d,0x7b,0x40,0xcd}};
    using IClassCreator = test_lib::raw::IClassCreator;
    static constexpr intercom::IID IID_ICreatedClass = {0x104eb174,0xfd00,0x3ecf,{0x7e,0x0d,0xd9,0x65,0x56,0x42,0x79,0xe7}};
    using ICreatedClass = test_lib::raw::ICreatedClass;
    static constexpr intercom::IID IID_IParent = {0xcea1c199,0xbf71,0x3b0a,{0x5a,0x4c,0xee,0x3f,0x5a,0x0a,0xe5,0xce}};
    using IParent = test_lib::raw::IParent;
    static constexpr intercom::IID IID_IRefCountOperations = {0x6b198a07,0x2d86,0x340e,{0x71,0x7e,0xf4,0x16,0xa3,0x90,0x5d,0x6e}};
    using IRefCountOperations = test_lib::raw::IRefCountOperations;
    static constexpr intercom::IID IID_ISharedInterface = {0x1df08ff6,0xaafb,0x37ec,{0x53,0xcf,0xcd,0xe2,0x49,0xce,0xeb,0x4b}};
    using ISharedInterface = test_lib::raw::ISharedInterface;
    static constexpr intercom::IID IID_IErrorSource = {0x5505b7b6,0x5ca4,0x3e38,{0x66,0x7b,0xa9,0x82,0x3f,0x1d,0x5a,0x0f}};
    using IErrorSource = test_lib::raw::IErrorSource;
    static constexpr CLSID CLSID_RefCountOperations = {0xf06af5f0,0x745a,0x3b29,{0x48,0x39,0xd2,0xd3,0x9a,0x3f,0x08,0xcd}};
    static constexpr CLSID CLSID_PrimitiveOperations = {0x12341234,0x1234,0x1234,{0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x01}};
    static constexpr CLSID CLSID_StatefulOperations = {0x694c1893,0x2fa8,0x3d4c,{0x6a,0xcf,0x69,0xc5,0x93,0x66,0x72,0x1e}};
    static constexpr CLSID CLSID_ResultOperations = {0xe5ce34c4,0xc1ad,0x34bc,{0x69,0xf6,0xd1,0xbf,0xa6,0xbb,0x25,0x96}};
    static constexpr CLSID CLSID_ClassCreator = {0x3323cccd,0x1a38,0x33a4,{0x4a,0xe1,0x4d,0xc9,0x2a,0x7e,0x8d,0xc5}};
    static constexpr CLSID CLSID_CreatedClass = {0x51ed4fb8,0x35d8,0x36c6,{0x78,0xfd,0x6b,0xc5,0x82,0xc8,0x4b,0x19}};
    static constexpr CLSID CLSID_SharedImplementation = {0x88687644,0x9cb2,0x3bd6,{0x4c,0x23,0xdb,0x54,0x7d,0x39,0x90,0x29}};
    static constexpr CLSID CLSID_ErrorSource = {0x2af563c2,0xdc1c,0x339a,{0x60,0x35,0xf5,0xf8,0x18,0x0f,0xae,0x86}};
#endif

