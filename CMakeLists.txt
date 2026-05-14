include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG master
)
FetchContent_MakeAvailable(Corrosion)
corrosion_import_crate(MANIFEST_PATH Cargo.toml FEATURES cxx)

set(CXX_TARGET gdl90-cxx)
set(INC_DIR
    "${CMAKE_CURRENT_BINARY_DIR}/corrosion_generated/cxxbridge/${CXX_TARGET}/include"
)
corrosion_add_cxxbridge("${CXX_TARGET}" CRATE gdl90 FILES ffi/cxx.rs)

add_custom_command(
    OUTPUT "${INC_DIR}/gdl90.h"
    COMMAND
        ${CMAKE_COMMAND} -E copy "${CMAKE_CURRENT_SOURCE_DIR}/src/ffi/gdl90.h"
        "${INC_DIR}/gdl90.h"
    DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/ffi/gdl90.h"
    COMMENT "Copying gdl90.h to the include directory"
)
add_custom_target(copy_gdl90_header ALL DEPENDS "${INC_DIR}/gdl90.h")
add_dependencies(${CXX_TARGET} copy_gdl90_header)

unset(CXX_TARGET)
unset(INC_DIR)
