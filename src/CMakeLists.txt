
set(SCITER_PATH "c:/Users/cvobo/.vcpkg/packages/sciter_x64-windows")
set(SCITER_INCLUDE "${SCITER_PATH}/include/sciter")
set(SCITER_PACK_TOOL "${SCITER_PATH}/tools/sciter/packfolder.exe")
set(SCITER_LIB "${SCITER_PATH}/bin")

set(LULU_UI_FOLDER "${CMAKE_SOURCE_DIR}/src/ui")
set(LULU_SRC_FOLDER "${CMAKE_SOURCE_DIR}/src")

execute_process(COMMAND ${SCITER_PACK_TOOL} ${LULU_UI_FOLDER} "${LULU_SRC_FOLDER}/resources.cpp" -v "resources")

add_executable (Lulu WIN32 "${SCITER_INCLUDE}/sciter-win-main.cpp" "lulu.cpp")

target_include_directories(Lulu PUBLIC ${SCITER_INCLUDE})

add_custom_command(TARGET Lulu POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    ${SCITER_LIB} $<TARGET_FILE_DIR:Lulu>)


install(TARGETS Lulu DESTINATION .)
install(DIRECTORY ${SCITER_LIB}/ DESTINATION .)
