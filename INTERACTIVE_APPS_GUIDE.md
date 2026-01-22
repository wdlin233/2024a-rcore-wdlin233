# Interactive User Applications for rCore OS

## 概述 (Overview)

这个文档说明了在 rCore OS 上可以创建哪些交互式用户应用程序。

This document explains what kind of interactive user applications can be created on the rCore OS.

## 能力总结 (Capabilities Summary)

### ✅ 支持的功能 (Supported Features)

1. **控制台输入/输出 (Console I/O)**
   - 标准输入 (stdin, fd=0): 逐字符读取用户输入
   - 标准输出 (stdout, fd=1): 格式化文本输出
   - 支持 `println!()`, `print!()` 宏 (Rust) 和 `printf()` (C)

2. **定时器和休眠 (Timer and Sleep)**
   - `sleep(ms)`: 毫秒级休眠
   - `get_time()`: 获取当前时间戳
   - 精确的时间测量用于动画帧控制

3. **文件系统 (File System)**
   - 文件读写操作
   - 文件创建和删除
   - 目录操作

4. **进程和线程 (Processes and Threads)**
   - 进程 fork/exec
   - 线程创建
   - 同步原语 (Mutex, Semaphore, CondVar)

### ❌ 限制 (Limitations)

1. **仅支持 ASCII 字符输出** - 无图形能力
2. **无光标控制** - 不支持 ANSI 转义序列来控制光标位置
3. **逐字符输入** - 无行缓冲或原始模式
4. **无清屏功能** - 只能通过打印换行符来模拟

## 可以创建的应用类型 (Types of Applications You Can Create)

### 1. ASCII 动画 (ASCII Animations) ✅

**示例**: 类似 "Bad Apple" 的 ASCII 艺术动画

**工作原理**:
```c
while (running) {
    // 显示当前帧
    printf("%s\n", frame_buffer);
    
    // 帧延迟控制 (例如 60 FPS)
    usleep(16666); // ~16ms per frame
    
    // 加载下一帧
    load_next_frame();
}
```

**实现要点**:
- 预先准备动画帧数据
- 使用 `sleep()` 或 `usleep()` 控制帧率
- 每帧输出到 stdout

**示例文件**: `testcases/src/ascii_animation.c`

### 2. 交互式菜单/CLI 应用 (Interactive Menu/CLI Apps) ✅

**示例**: 交互式命令行界面，菜单驱动的程序

**工作原理**:
```c
while (running) {
    print_menu();
    
    // 读取用户选择
    char choice;
    read(0, &choice, 1);
    
    // 处理用户输入
    switch (choice) {
        case '1': do_action1(); break;
        case '2': do_action2(); break;
        // ...
    }
}
```

**适用场景**:
- 系统管理工具
- 文件管理器
- 配置工具
- 交互式 shell

**示例文件**: `testcases/src/interactive_menu.c`

### 3. 文本游戏 (Text-based Games) ✅

**示例**: 猜数字游戏、文字冒险游戏、回合制游戏

**工作原理**:
```c
int secret = generate_random_number();
int attempts = 0;

while (attempts < max_attempts) {
    printf("Enter your guess: ");
    int guess = read_number();
    
    if (guess == secret) {
        printf("Correct!\n");
        break;
    } else if (guess < secret) {
        printf("Too low!\n");
    } else {
        printf("Too high!\n");
    }
    attempts++;
}
```

**可行的游戏类型**:
- 猜数字游戏
- 文字冒险游戏
- 石头剪刀布
- 井字棋 (Tic-Tac-Toe)
- 回合制 RPG

**示例文件**: `testcases/src/ascii_game.c`

### 4. 进度条和加载动画 (Progress Bars and Loading Animations) ✅

**示例**: 显示任务进度的应用

**工作原理**:
```c
for (int progress = 0; progress <= 100; progress += 10) {
    printf("\rProgress: [");
    for (int i = 0; i < progress/10; i++) printf("=");
    for (int i = progress/10; i < 10; i++) printf(" ");
    printf("] %d%%", progress);
    
    sleep(100); // 100ms delay
}
printf("\nComplete!\n");
```

### 5. 系统监控工具 (System Monitoring Tools) ✅

**示例**: 实时显示系统状态

**工作原理**:
```rust
loop {
    let info = get_task_info();
    println!("CPU: {}%, Memory: {} KB", info.cpu, info.mem);
    sleep(1000); // 每秒更新
}
```

### 6. 数据可视化 (Data Visualization in ASCII) ✅

**示例**: 使用 ASCII 字符绘制图表

**工作原理**:
```c
// 简单的条形图
int data[] = {5, 8, 3, 10, 7};
for (int i = 0; i < 5; i++) {
    printf("Item %d: ", i);
    for (int j = 0; j < data[i]; j++) {
        printf("*");
    }
    printf("\n");
}
```

## 技术细节 (Technical Details)

### 输入处理 (Input Handling)

```c
// C 语言示例
char buffer[128];
int len = read(0, buffer, sizeof(buffer));

// 逐字符读取
char c;
while (read(0, &c, 1) > 0) {
    if (c == '\n') break;
    // 处理字符
}
```

```rust
// Rust 语言示例
use user_lib::read;

let mut buf = [0u8; 128];
let len = read(0, &mut buf);

// 逐字符读取
let mut c = [0u8; 1];
while read(0, &mut c) > 0 {
    if c[0] == b'\n' { break; }
    // 处理字符
}
```

### 输出处理 (Output Handling)

```c
// C 语言
printf("Hello, %s!\n", name);
write(1, buffer, len);
```

```rust
// Rust 语言
println!("Hello, {}!", name);
use user_lib::write;
write(1, buffer.as_bytes());
```

### 时间控制 (Timing Control)

```c
// C 语言
#include <unistd.h>
usleep(16666); // 微秒 (16.666 ms for 60 FPS)
```

```rust
// Rust 语言
use user_lib::sleep_blocking;
sleep_blocking(16); // 毫秒 (16 ms for ~60 FPS)
```

## "Bad Apple" 动画实现指南 (Bad Apple Implementation Guide)

要在 rCore 上播放 "Bad Apple" 风格的动画，你需要：

### 步骤 1: 准备动画数据

1. 将视频转换为 ASCII 艺术帧
2. 降低分辨率 (例如 80x24 字符)
3. 将帧数据嵌入到程序中或存储在文件中

### 步骤 2: 实现播放器

```c
// 伪代码
const char* frames[] = {
    /* 预先生成的 ASCII 帧 */
};

int frame_count = sizeof(frames) / sizeof(frames[0]);
int fps = 30; // 30 帧每秒
int frame_delay = 1000 / fps; // 毫秒

for (int i = 0; i < frame_count; i++) {
    // 输出当前帧
    printf("%s", frames[i]);
    
    // 等待下一帧时间
    usleep(frame_delay * 1000);
}
```

### 步骤 3: 优化

- **压缩帧数据**: 只存储帧间差异
- **调整帧率**: 根据系统性能调整 FPS
- **简化分辨率**: 使用更小的字符矩阵

### 限制和注意事项

1. **内存限制**: 用户堆大小为 16KB，需要小心管理帧数据
2. **无清屏**: 每帧会累积显示，可能需要大量换行符分隔
3. **性能**: 大量输出可能影响性能

## 实际示例 (Practical Examples)

项目中包含了三个示例应用：

1. **`ascii_animation.c`** - 简单的旋转线条动画
2. **`interactive_menu.c`** - 交互式菜单系统
3. **`ascii_game.c`** - 猜数字游戏

### 构建和运行 (Build and Run)

```bash
cd os
make run
```

在 OS shell 中运行应用：
```
# 如果应用已编译到文件系统中
./ascii_animation
./interactive_menu
./ascii_game
```

## 结论 (Conclusion)

**可以在 rCore OS 上创建 "Bad Apple" 风格的动画吗？**

**答案：是的！** 但有以下限制：

✅ **可以做到**:
- ASCII 字符动画
- 帧率控制
- 交互式输入
- 简单的文本游戏

❌ **做不到**:
- 像素级图形
- 光标自由定位（无 ANSI 支持）
- 高分辨率动画
- 音频播放

**最佳实践**:
- 保持动画简单
- 使用较低的分辨率 (40x20 字符)
- 控制帧率 (10-30 FPS)
- 合理管理内存

---

## 参考资源 (References)

- [rCore Tutorial Book](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)
- OS syscall 文档: 查看 `os/src/syscall/` 目录
- 用户库 API: 查看 `user/src/lib.rs`

---

创建时间: 2026-01-22
版本: 1.0
