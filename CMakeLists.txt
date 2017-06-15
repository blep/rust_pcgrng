cmake_minimum_required(VERSION 2.8)

PROJECT(PCG_testdata_gen)
include(CheckCSourceCompiles) 
INCLUDE(ExternalProject)

# Output all libraries and executables in the same directory
set(LIBRARY_OUTPUT_PATH ${PROJECT_BINARY_DIR}/bin
    CACHE PATH "Single directory for all libraries.")
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_BINARY_DIR}/bin
    CACHE PATH "Single directory for all executables.")
mark_as_advanced( LIBRARY_OUTPUT_PATH EXECUTABLE_OUTPUT_PATH )

macro(use_c99)
  if (CMAKE_VERSION VERSION_LESS "3.1")
    if (CMAKE_C_COMPILER_ID STREQUAL "GNU")
      set (CMAKE_C_FLAGS "--std=c99 ${CMAKE_C_FLAGS}")
    endif ()
  else ()
    set (CMAKE_C_STANDARD 99)
  endif ()
endmacro(use_c99)

use_c99()

set(THIRDPARTIES_INSTALL_DIR ${CMAKE_BINARY_DIR}/thirdparties)

set(PCG_C_DIR thirdparties/pcg-c/)

# Adds third-parties include/lib dir to look-up paths
include_directories(${PCG_C_DIR}/include)

# sub-projects
option(PCG_C_BUILD_SAMPLES "Build pgc-c samples" OFF)
option(PCG_C_BUILD_TESTS "Build pgc-c tests" OFF)

add_subdirectory( ${PCG_C_DIR} )

set(SOURCES
    testdata/pcg_testdata_gen.cpp
)

add_executable( pcg_testdata_gen ${SOURCES} )
target_link_libraries( pcg_testdata_gen pcg_random_lib )  
