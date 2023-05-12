# Simulator for the Simple 8-bit Rail Architecture

This project is a simulator for the simple 8-bit Rail architecture. The Rail architecture is not intended to be a serious architecture and was created just for fun and learning purposes.

The simulator is implemented in Rust, and as I'm still learning the language, it might lack best practices and might not be idiomatic Rust code.

To get a better understanding of the architecture and its ASM, please feel free to check the definition documents here: [Rail Architecture](https://github.com/huskyto/rail-arch-definition). I have tried to make it as simple as possible, but I'm open to suggestions on how to improve it.

You can also check out the Kotlin implementation of the simulator here: 
[Kotlin Simulator](https://github.com/huskyto/rail-arch-kt)

## How to get the executable

### Option 1: Build it yourself (recommended)

1. First you need to install Rust. It's very simple, and you can find out how here:

https://www.rust-lang.org/tools/install

2. In the terminal, go to the folder where you want the code to live.

3. Clone this repository:

```
git clone https://github.com/huskyto/rail-arch-rust.git
```

4. Go inside the new folder:

```
cd rail-arch-rust
```

5. Run the build:

```
cargo build --release
```

6. The build binary will be located in:

```
target/release/rail-arch-rust.exe
```

You can run it directly from there, or copy it to wherever you want. You can also rename it as you wish.

### Option 2: User the precompiled executables.

[Coming soon]

## How to use it.

Once you have the executable, there are a few things you may want to do with it. The best way to know what options it provides is to run it with the `--help` flag:

```
rail-arch-rust.exe --help
```

That will print all the options you can call it with.

### Assembling.

To assemble a file into a binary, you can run the following command:

```
rail-arch-rust.exe -a -i input.asm -o output.bin
```

The file extensions don't really matter.

This will assemble `input.asm` and put the resulting binary in `output.bin` so you can run it.

As a test, you can use the following small program for your `input.asm` file. It generates the fibonacci sequence and writes it to the `IO` register:

```
ADD+IM2 R0 1 R1
LABEL loop
MOV R2 0 D0
ADD R1 R2 R2
MOV D0 0 R1
MOV R2 0 IO
JMP 0 0 loop
```

### Running a binary.

Once you have an assembled binary, you can run it with the following command:

```
rail-arch-rust.exe -r -i input.bin
```

If you want a more pretty visualization of what's going on, however, you can add the `-u` flag to have get the terminal GUI while it runs, so you can see the code execution, the RAM and the registers.

```
rail-arch-rust.exe -r -i input.bin -u
```

Running it like this, defaults to only running 64 steps, and there is a 100ms delay between each step. If you wish to modify that, you can set the `-s` argument for the number of evaluated steps, and the `-w` argument, for the delay between steps.

This is one example of what that would look like:

```
rail-arch-rust.exe -r -i input.bin -s 64 -w 100 -u
```

### Visualizing binary

Once you have generated your binary file, you may want to visualize it as separated hex values. You can do this by running:

```
rail-arch-rust.exe -p -i input.bin
```

## Contributing

Contributions are welcome! If you have any suggestions, ideas, or improvements, feel free to create an issue or submit a pull request.

## License

This project is licensed under the Apache License 2.0.

### Summary

- You are free to use, modify, and distribute this software.
- If you modify the software or create a derivative work, you must include the original copyright notice and a notice of the changes you made.
- The software is provided "as is," without warranties of any kind.
