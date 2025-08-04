# FAISS C API 构建指南（macOS）

本指南适用于在 **macOS（Apple Silicon）** 上构建 FAISS 的 **C API 动态库**（`libfaiss_c.dylib`），无需 GPU 和 Python 支持。

> linux或者其他环境请参考本文最后的参考链接。

---

## ✅ 构建产物

构建完成后，你将获得：

- `build/c_api/libfaiss_c.dylib`：FAISS C API 动态库
- `faiss/c_api/faiss_c.h`：C API 头文件

---

## 🛠️ 前提依赖

请先安装以下工具：

```bash
brew install cmake swig libomp
```

---

## 🔧 构建步骤

### 1. 克隆仓库（推荐clone下面命令的fork仓库）

```bash
git clone git@github.com:Enet4/faiss.git
cd faiss
```

### 2. 清理旧构建缓存（可选但推荐）

```bash
rm -rf build
```

### 3. 配置 CMake（关键命令）

```bash
cmake -B build \
  -DFAISS_ENABLE_C_API=ON \
  -DBUILD_SHARED_LIBS=ON \
  -DCMAKE_BUILD_TYPE=Release \
  -DFAISS_ENABLE_GPU=OFF \
  -DFAISS_ENABLE_PYTHON=OFF \
  -DBUILD_TESTING=OFF \
  -DCMAKE_CXX_COMPILER=clang++ \
  -DCMAKE_CXX_FLAGS="-Xpreprocessor -fopenmp -I/opt/homebrew/opt/libomp/include" \
  -DCMAKE_EXE_LINKER_FLAGS="-L/opt/homebrew/opt/libomp/lib -lomp" \
  -DCMAKE_SHARED_LINKER_FLAGS="-L/opt/homebrew/opt/libomp/lib -lomp"
```

### 4. 构建

```bash
cmake --build build -j$(sysctl -n hw.ncpu)
```

---

## 📦 使用方式

### 链接动态库

在你的项目中：

```bash
clang++ your_program.cpp -I./faiss/c_api -L./build/c_api -lfaiss_c -lomp -o your_program
```

### 运行程序前设置库路径

```bash
export DYLD_LIBRARY_PATH=./build/c_api:$DYLD_LIBRARY_PATH
./your_program
```

---

## ✅ 验证构建成功

检查产物是否存在：

```bash
ls -lh build/c_api/libfaiss_c.dylib
```

---

## 移动到对应目录

```bash
sudo cp ./build/c_api/libfaiss_c.dylib /usr/local/lib/libfaiss_c.dylib
```

---

## 🧩 常见问题速查

| 问题 | 解决方案 |
|------|----------|
| `omp.h not found` | 确保已安装 `libomp`，并设置 `CMAKE_CXX_FLAGS` |
| `SWIG not found` | `brew install swig` |
| `Python not found` | 禁用 Python：`-DFAISS_ENABLE_PYTHON=OFF` |
| `gtest dylib missing` | 禁用测试：`-DBUILD_TESTING=OFF` |

---


## ✅ 构建完成

你已成功构建 FAISS C API 动态库，可用于 Rust/C++ 项目集成。

## 参考

- kimi-k2
- <https://github.com/Enet4/faiss-rs?tab=readme-ov-file>