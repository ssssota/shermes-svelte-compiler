#!/bin/bash

set -eux

# `-O`: Optimize the generated code
# `-c`: Compiles to a native object file
# `-exported-unit`: Name of the unit and skip generating main function
shermes \
  -O \
  -c \
  -o=./dist/svelte.o \
  -exported-unit=svelte \
  ./dist/index.cjs

# # Emits C source code
# shermes \
#   -O \
#   -emit-c \
#   -o=./dist/index.c \
#   -exported-unit=svelte \
#   ./dist/index.cjs

# `-c`: Compiles to a native object file
# `-O3`: Optimize the generated code with level 3
# `-std=c++17`: Use C++17 standard
# `-I*`: Include directories
clang++ \
  -c \
  -O3 \
  -std=c++17 \
  -stdlib=libstdc++ \
  -I/hermes/hermes/API \
  -I/hermes/hermes/API/jsi \
  -I/hermes/hermes/include \
  -I/hermes/hermes/public \
  -I/hermes/build/lib/config \
  ./src/mod.c \
  -o ./dist/mod.o

# `-O`: Optimize the generated code
# `-C link-arg=*`: Pass an argument to the linker
# `-L*`: Add a directory to the library search path
# `-l*`: Link the specified library
rustc \
  ./src/main.rs \
  -O \
  -C link-arg=./dist/svelte.o \
  -C link-arg=./dist/mod.o \
  -L/hermes/build/lib \
  -L/hermes/build/jsi \
  -L/hermes/build/tools/shermes \
  -L/hermes/build/external/boost/boost_1_86_0/libs/context/ \
  -C link-arg=-lshermes_console_a \
  -C link-arg=-lhermesvm_a \
  -C link-arg=-ljsi \
  -C link-arg=-lboost_context \
  -C link-arg=-licuuc \
  -C link-arg=-licui18n \
  -C link-arg=-latomic \
  -C link-arg=-lstdc++ \
  -C link-arg=-lm \
  -C link-arg=-lgcc \
  -o ./dist/main
  # -l framework=Foundation
