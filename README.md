## Wastebin
This is a Rust program that can be used to move files or directories to the Trash folder on Unix-based systems. The program takes one or two arguments:

```
Usage: wb [-rf] <file or directory>
```

The first argument is optional and, when specified as -rf, allows for the removal of directories and their contents.

The program will first check if the specified file or directory exists. If the argument is a directory and -rf is not specified, the program will exit with an error message. If the argument is a directory and -rf is specified, the program will recursively remove the directory's contents and then remove the directory itself.

If the argument is a file, the program will move the file to the Trash folder. The program will first check if the Trash folder exists and create it if it does not. If the file already exists in the Trash folder, it will be overwritten.

Note that the program does not currently support moving files or directories to the Recycle Bin on Windows-based systems.

Usage
To use the program, you must have Rust installed on your system. Install as follows.

```
cargo install --git https://github.com/abhinavmir/wastebin --branch main --bin wastebin
```
