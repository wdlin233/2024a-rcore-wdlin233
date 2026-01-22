# rCore-Camp-Code-2024A

### Code
- [Soure Code of labs for 2024A](https://github.com/LearningOS/rCore-Camp-Code-2024A)

### Interactive User Applications 🎮

**New!** This repository includes example interactive user applications that demonstrate what you can create with rCore OS:

- 📺 **ASCII Animations** - "Bad Apple" style character animations with frame control
- 🎯 **Interactive Menus** - Menu-driven CLI applications
- 🎲 **Text Games** - Number guessing game and more
- 📊 **Progress Bars** - Real-time progress visualization

**Quick Links:**
- [Interactive Apps README](./INTERACTIVE_APPS_README.md) - Quick overview
- [Detailed Guide](./INTERACTIVE_APPS_GUIDE.md) - Complete technical documentation
- Example Apps: [`testcases/src/`](./testcases/src/) - ascii_animation.c, interactive_menu.c, ascii_game.c

**Answer to "Can I create Bad Apple on rCore?"** - **YES!** ✅ See the guide for details.

### Documents

- Concise Manual: [rCore-Camp-Guide-2024A](https://LearningOS.github.io/rCore-Camp-Guide-2024A/)

- Detail Book [rCore-Tutorial-Book-v3](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)


### OS API docs
- [ch1](https://learningos.github.io/rCore-Camp-Code-2024A/ch1/os/index.html) [ch2](https://learningos.github.io/rCore-Camp-Code-2024A/ch2/os/index.html) [ch3](https://learningos.github.io/rCore-Camp-Code-2024A/ch3/os/index.html) [ch4](https://learningos.github.io/rCore-Camp-Code-2024A/ch4/os/index.html)
- [ch5](https://learningos.github.io/rCore-Camp-Code-2024A/ch5/os/index.html) [ch6](https://learningos.github.io/rCore-Camp-Code-2024A/ch6/os/index.html) [ch7](https://learningos.github.io/rCore-Camp-Code-2024A/ch7/os/index.html) [ch8](https://learningos.github.io/rCore-Camp-Code-2024A/ch8/os/index.html)


### Related Resources
- [Learning Resource](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/relatedinfo.md)


### Build & Run

Replace `<YourName>` with your github ID, and replace `<Number>` with the chapter ID.

Notice: `<Number>` is chosen from `[1,2,3,4,5,6,7,8]`

```bash
# 
$ git clone git@github.com:LearningOS/2024a-rcore-<YourName>
$ cd 2024a-rcore-<YourName>
$ git clone git@github.com:LearningOS/rCore-Tutorial-Test-2024A user
$ git checkout ch<Number>
$ cd os
$ make run
```

### Grading

Replace `<YourName>` with your github ID, and replace `<Number>` with the chapter ID.

Notice: `<Number>` is chosen from `[3,4,5,6,8]`

```bash
# Replace <YourName> with your github ID 
$ git clone git@github.com:LearningOS/2024a-rcore-<YourName>
$ cd 2024a-rcore-<YourName>
$ rm -rf ci-user
$ git clone git@github.com:LearningOS/rCore-Tutorial-Checker-2024A ci-user
$ git clone git@github.com:LearningOS/rCore-Tutorial-Test-2024A ci-user/user
$ git checkout ch<Number>
$ cd ci-user
$ make test CHAPTER=<Number>
```
