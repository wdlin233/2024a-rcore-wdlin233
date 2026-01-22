# 交互式用户应用示例 (Interactive User Applications Examples)

[English](#english) | [中文](#chinese)

## <a name="chinese"></a>中文说明

### 概述

这个仓库包含了为 rCore OS 创建的交互式用户应用示例，回答了 **"对我现在的这个OS，我可以做一些什么可以交互性的user app呢？比如播放bad apple这种的我可以做到吗"** 这个问题。

**简短回答：是的，你可以！** 但有一定限制。

### 可以创建的应用类型

✅ **你可以创建：**
1. **ASCII 动画** - 类似 "Bad Apple" 的字符艺术动画
2. **交互式菜单** - 菜单驱动的 CLI 应用
3. **文字游戏** - 猜数字、文字冒险等
4. **进度条和加载动画** - 实时进度显示
5. **系统监控工具** - 显示系统状态
6. **数据可视化** - 使用 ASCII 字符的图表

### 包含的示例应用

1. **`testcases/src/ascii_animation.c`** - ASCII 动画演示
   - 展示如何创建帧动画
   - 使用 `sleep()` 控制帧率
   - 适合 "Bad Apple" 风格的动画

2. **`testcases/src/interactive_menu.c`** - 交互式菜单系统
   - 展示用户输入处理
   - 菜单驱动的界面
   - 系统信息显示

3. **`testcases/src/ascii_game.c`** - 猜数字游戏
   - 交互式游戏玩法
   - 用户输入验证
   - 游戏状态管理

### 技术能力

**支持的功能：**
- ✅ 标准输入/输出 (stdin/stdout)
- ✅ 毫秒级定时器和休眠
- ✅ 文件系统操作
- ✅ 进程和线程管理

**限制：**
- ❌ 仅支持 ASCII 字符（无图形）
- ❌ 无光标控制（无 ANSI 转义序列）
- ❌ 逐字符输入（无行缓冲）

### 详细文档

查看 **[INTERACTIVE_APPS_GUIDE.md](./INTERACTIVE_APPS_GUIDE.md)** 获取：
- 完整的 API 参考
- 实现细节和示例代码
- "Bad Apple" 实现指南
- 最佳实践建议

### 如何使用

1. 查看示例应用的源代码
2. 根据需要修改或创建新应用
3. 参考文档了解可用的系统调用
4. 使用提供的模板创建你自己的交互式应用

---

## <a name="english"></a>English

### Overview

This repository contains interactive user application examples created for rCore OS, answering the question **"What kind of interactive user apps can I create for my OS? For example, can I do something like playing 'Bad Apple'?"**

**Short Answer: Yes, you can!** But with some limitations.

### Types of Applications You Can Create

✅ **You CAN create:**
1. **ASCII Animations** - Character art animations like "Bad Apple"
2. **Interactive Menus** - Menu-driven CLI applications
3. **Text-based Games** - Number guessing, text adventures, etc.
4. **Progress Bars and Loading Animations** - Real-time progress display
5. **System Monitoring Tools** - Display system status
6. **Data Visualization** - Charts using ASCII characters

### Included Example Applications

1. **`testcases/src/ascii_animation.c`** - ASCII Animation Demo
   - Shows how to create frame-based animations
   - Uses `sleep()` for frame rate control
   - Suitable for "Bad Apple" style animations

2. **`testcases/src/interactive_menu.c`** - Interactive Menu System
   - Demonstrates user input handling
   - Menu-driven interface
   - System information display

3. **`testcases/src/ascii_game.c`** - Number Guessing Game
   - Interactive gameplay
   - Input validation
   - Game state management

### Technical Capabilities

**Supported Features:**
- ✅ Standard input/output (stdin/stdout)
- ✅ Millisecond-precision timers and sleep
- ✅ File system operations
- ✅ Process and thread management

**Limitations:**
- ❌ ASCII-only display (no graphics)
- ❌ No cursor control (no ANSI escape sequences)
- ❌ Character-by-character input (no line buffering)

### Detailed Documentation

See **[INTERACTIVE_APPS_GUIDE.md](./INTERACTIVE_APPS_GUIDE.md)** for:
- Complete API reference
- Implementation details and example code
- "Bad Apple" implementation guide
- Best practices and recommendations

### How to Use

1. Review the example application source code
2. Modify or create new applications as needed
3. Refer to the documentation for available syscalls
4. Use the provided templates to create your own interactive applications

### Key Takeaway

**Can you create "Bad Apple" style animations on rCore OS?**

**YES!** The OS supports:
- Frame-based ASCII art display
- Precise timing control for animations
- User input for interactivity
- Simple text-based games and demos

The examples provided demonstrate the core techniques needed to create interactive applications, including the famous "Bad Apple" animation style.
