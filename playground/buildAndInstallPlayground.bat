@ECHO off

SET BUILD_CONFIG=debug
IF NOT "%~1"=="" (
  SET BUILD_CONFIG=%~1
)

SET ROOT=%cd%
  
IF NOT EXIST %ROOT%\target (
  MKDIR %ROOT%\target
)

SET BUILD_CONFIG_OPTION=
IF NOT "%BUILD_CONFIG%"=="debug" (
  SET BUILD_CONFIG_OPTION=--release
)

CD %ROOT%\target
  
cargo build --target=wasm32-unknown-emscripten %BUILD_CONFIG_OPTION%
xcopy "%ROOT%\target\wasm32-unknown-emscripten\%BUILD_CONFIG%\playground.js"   "%ROOT%\app\" /Y
xcopy "%ROOT%\target\wasm32-unknown-emscripten\%BUILD_CONFIG%\playground.wasm" "%ROOT%\app\" /Y

cd %ROOT%

goto :eof