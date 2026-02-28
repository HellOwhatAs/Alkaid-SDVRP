# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION ${CMAKE_VERSION}) # this file comes with cmake

# If CMAKE_DISABLE_SOURCE_CHANGES is set to true and the source directory is an
# existing directory in our source tree, calling file(MAKE_DIRECTORY) on it
# would cause a fatal error, even though it would be a no-op.
if(NOT EXISTS "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/..")
  file(MAKE_DIRECTORY "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/..")
endif()
file(MAKE_DIRECTORY
  "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-build"
  "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-subbuild/alkaidsd-populate-prefix"
  "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-subbuild/alkaidsd-populate-prefix/tmp"
  "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-subbuild/alkaidsd-populate-prefix/src/alkaidsd-populate-stamp"
  "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-subbuild/alkaidsd-populate-prefix/src"
  "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-subbuild/alkaidsd-populate-prefix/src/alkaidsd-populate-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-subbuild/alkaidsd-populate-prefix/src/alkaidsd-populate-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "/home/runner/work/Alkaid-SDVRP/Alkaid-SDVRP/standalone/build/_deps/alkaidsd-subbuild/alkaidsd-populate-prefix/src/alkaidsd-populate-stamp${cfgdir}") # cfgdir has leading slash
endif()
