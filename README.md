# Build Your Own Git - Rust Implementation

![progress-banner](https://backend.codecrafters.io/progress/git/c9ea89fa-af0b-4af3-907c-2a7262b1b763)

Welcome to my **Rust-based implementation** of Git, built as part of the ["Build Your Own Git" Challenge](https://codecrafters.io/challenges/git) on CodeCrafters.io.

## 🚀 Challenge Overview

This project is an opportunity to explore the inner workings of Git, one of the most widely used version control systems in software development. Throughout this challenge, I’ve implemented a simplified version of Git from scratch using **Rust**, focusing on core functionalities such as:

- Initializing a Git repository (`.git` directory)
- Handling **blob** objects (create and read)
- Handling **tree** objects (create and read)
- Creating **commits**
- Cloning a public GitHub repository

By the end of the challenge, my Git implementation will be capable of performing many fundamental Git operations. This has been an excellent learning experience in both **Rust programming** and **Git internals**.

## 🏁 Current Progress

I have successfully completed **6 out of 7 stages** in the challenge. Below are the key stages:

1. **Initialize the .git directory** - ✅
2. **Read a blob object** - ✅
3. **Create a blob object** - ✅
4. **Read a tree object** - ✅
5. **Write a tree object** - ✅
6. **Create a commit** - ✅
7. **Clone a repository** - In Progress

Each step has deepened my understanding of the way Git operates under the hood, especially how it manages and stores different objects (blobs, trees, commits) and the challenges related to handling them efficiently in Rust.

## 🧑‍💻 How to Run

You can experiment with this project locally. The entry point is located in `src/bin/git.rs`. Here's how you can get started:

1. Ensure you have **cargo (>=1.80)** installed.
2. Build and run the project with the following commands:

    ```sh
    cargo build
    cargo run -- init
    ```

3. You can also use the option `-h` or `--help` to get more detailed documentation and usage information:

    ```sh
    cargo run -- -h
    ```

4. To avoid any accidental changes to your Git repository during local testing, it’s recommended to test the program in a separate directory:

    ```sh
    mkdir -p /tmp/testing && cd /tmp/testing
    cargo run -- init
    ```

## 🧪 Testing

For safety, execute the program in a different folder to prevent modifying the `.git` folder of this project:

```sh
mkdir -p /tmp/testing && cd /tmp/testing
cargo run -- init
```

## 🎯 Future Improvements

- Completing the final step: **Cloning a repository**
- Adding more test cases to ensure the robustness of the implementation
- Improving the error handling and performance of the Rust code

## 🔧 Technologies Used

- **Rust**: The core programming language used to build the project
- **Git**: The system being implemented
- **CodeCrafters.io**: The platform providing the challenge

Feel free to explore, or reach out for any questions!
