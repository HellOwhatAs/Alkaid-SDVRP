#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "AlkaidSD::AlkaidSD" for configuration "Release"
set_property(TARGET AlkaidSD::AlkaidSD APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(AlkaidSD::AlkaidSD PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/AlkaidSD-1.0/libAlkaidSD.a"
  )

list(APPEND _cmake_import_check_targets AlkaidSD::AlkaidSD )
list(APPEND _cmake_import_check_files_for_AlkaidSD::AlkaidSD "${_IMPORT_PREFIX}/lib/AlkaidSD-1.0/libAlkaidSD.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
