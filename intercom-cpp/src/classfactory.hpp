#ifndef INTERCOM_CPP_CLASSFACTORY_H
#define INTERCOM_CPP_CLASSFACTORY_H

#include <memory>
#include <mutex>

#include "detail/iclassfactory.hpp"

#include "activator.hpp"
#include "cominterop.hpp"

namespace intercom
{
    template< class TClass >
    class ClassFactory
    {
    public:

        ClassFactory() :
            m_initializationGuard()
        {
        }

        ~ClassFactory()
        {
        }

        /**
         * @brief Instantiates a new COM object.
         *
         * @param iid  Identifies the interface.
         * @param instance  Receives a pointer to specified interface of the new object.
         * @return intercom::HRESULT Returns SC_OK on success.
         */
        template< typename TInterface >
        intercom::RawInterface<TInterface> create()
        {
            // Need an activator to create objects.
            intercom::HRESULT preparation = prepare_activator();
            if( preparation != SC_OK )
            {
                throw intercom::RuntimeError( preparation,
                        std::stringstream() << "Preparing activator for the class factory of class \""
                        << TClass::ID << "\" failed." );
            }

            return m_activator->create<TInterface>();
        }

        /**
         * @brief Instantiates a new COM object.
         *
         * @param iid  Identifies the interface.
         * @param instance  Receives a pointer to specified interface of the new object.
         * @return intercom::HRESULT Returns SC_OK on success.
         */
        intercom::HRESULT create(
            const intercom::IID& iid,
            void** instance
        )
        {
            // Need an activator to create objects.
            intercom::HRESULT preparation = prepare_activator();
            if( preparation != SC_OK )
                return preparation;

            return m_activator->create( iid, instance );
        }

    private:

        /**
         * @brief Ensures the activator has been initialized. Initializes the activator if not already initialized.
         *
         * @return intercom::HRESULT
         */
        intercom::HRESULT prepare_activator()
        {
            try
            {
                std::call_once(m_initializationGuard, [&](){
                        this->m_activator = std::make_unique< Activator >( TClass::Library::NAME, TClass::ID ); } );
            }
            catch( std::bad_alloc )
            {
                return EC_OUTOFMEMORY;
            }
            catch( ... )
            {
                return EC_FAIL;
            }
            return SC_OK;
        }

        std::once_flag m_initializationGuard;
        std::unique_ptr< intercom::Activator > m_activator;

    };
}

#endif