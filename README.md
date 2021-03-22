# pass
A simple command line password manager written in Rust.<br>
It offers command-based control. You can add, remove, update, copy your passwords and more.
Most importantly, you can import/export your encrypted passwords to move them securely from system to system.

You can see a complete list of supported commands by simply calling the application with no arguments:
```batchfile
pass
```
## Building

### Debug build
```batchfile
cargo build
```
### Install
You can use python 3 to build release and create install folder:
```batchfile
python install.py
```
After this command check `pass/install/`. There will be a directory containing a ready-to-use application.
Move this directory wherever you want and add it to `PATH` for simpler use.
