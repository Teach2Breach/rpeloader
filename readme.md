### rpeloader

rpeloader is the rust-python-embeddable-loader. It's a Rust program, based off my earlier tool: [Teach2Breach/rust_pyramid: Rust project that leverages the signed embeddable python package to evade EDR. (github.com)](https://github.com/Teach2Breach/rust_pyramid) 

That it does is to leverage the python embeddable package, which is signed and trusted by EDRs, to execute python scripts for evasion. Sounds stupid I know, but it works against Falcon and probably others. 

rpeloader pulls down the python embeddable package, makes some minor changes to the files to allow loading 3rd party libraries, manually adds pip from source, and then accepts a python script in the form of a path or url from the user. In this implementation, if any py scripts that have 3rd party imports, rpeloader will identify the needed libraries and add them via pip, before executing the script. You can run essentially any python script from this Rust program. 

You could use this logic in a c2 framework or as a standalone payload with a hardcoded py script to grab, or whatever. Below is an example usage.

``` bash 
cargo build --release
./rpeloader.exe
```



