
#ifndef INTERCOM_CPP_DLLGETCLASSOBJECT
#define INTERCOM_CPP_DLLGETCLASSOBJECT

#include "comdef.h"
#include "error_codes.h"

HRESULT DllGetClassObject(
  REFCLSID rclsid,
  REFIID riid,
  void** ppv
);

#endif
