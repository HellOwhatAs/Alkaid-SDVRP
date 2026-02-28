# Install script for directory: /home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/usr/local")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set path to fallback-tool for dependency-resolution.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "AlkaidSD_Development" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/AlkaidSD-1.0" TYPE DIRECTORY FILES "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build/PackageProjectInclude/")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "AlkaidSD_Development" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/AlkaidSD-1.0" TYPE STATIC_LIBRARY FILES "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build/libAlkaidSD.a")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "AlkaidSD_Development" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/AlkaidSD-1.0/AlkaidSDTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/AlkaidSD-1.0/AlkaidSDTargets.cmake"
         "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build/CMakeFiles/Export/e3f9f21b040d97b01cbf0100f6115568/AlkaidSDTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/AlkaidSD-1.0/AlkaidSDTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/AlkaidSD-1.0/AlkaidSDTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/AlkaidSD-1.0" TYPE FILE FILES "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build/CMakeFiles/Export/e3f9f21b040d97b01cbf0100f6115568/AlkaidSDTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/AlkaidSD-1.0" TYPE FILE FILES "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build/CMakeFiles/Export/e3f9f21b040d97b01cbf0100f6115568/AlkaidSDTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "AlkaidSD_Development" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/AlkaidSD-1.0" TYPE FILE FILES
    "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build/AlkaidSDConfigVersion.cmake"
    "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build/AlkaidSDConfig.cmake"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "AlkaidSD_Development" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/AlkaidSD-1.0" TYPE DIRECTORY FILES "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/include/" FILES_MATCHING REGEX "/[^/]*$")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
if(CMAKE_INSTALL_LOCAL_ONLY)
  file(WRITE "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build/install_local_manifest.txt"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
endif()
