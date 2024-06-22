FROM immunant/c2rust:ubuntu-focal-latest

# build and install c2rust
ENV LLVM_CONFIG_PATH=/usr/bin/llvm-config
RUN git clone https://github.com/immunant/c2rust.git -b v0.18.0
RUN mkdir -p /c2rust/build && cd /c2rust/build && cmake /c2rust/c2rust-ast-exporter/src -DLLVM_DIR=/usr/lib/llvm-10/lib/cmake/llvm -DClang_DIR=/usr/lib/llvm-10/lib/cmake/clang -DCMAKE_INSTALL_PREFIX=/c2rust/build/ "-DCMAKE_C_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" -DCMAKE_C_COMPILER=/usr/bin/cc "-DCMAKE_CXX_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" -DCMAKE_CXX_COMPILER=/usr/bin/c++ -DCMAKE_BUILD_TYPE=Release && cmake --build . --target clangAstExporter --config Release && cp /c2rust/build/lib/*.a /c2rust/build/
ENV C2RUST_AST_EXPORTER_LIB_DIR=/c2rust/build
WORKDIR /c2rust
RUN cargo build --release
RUN cp ./target/release/c2rust* /usr/bin

# copy transpiler
WORKDIR /transpiler
COPY . ./