# 跨语言读取 `pytotch` 模型

## 生成模型

```shell
python3 digit.py
```

## 转化模型

```shell
python3 convert.py
```

## 读取模型

### c++

```shell
cmake -B build && cmake --build build
build/digit model/digit.jit image/sample.png
```

### rust

```shell
cargo run 
```
