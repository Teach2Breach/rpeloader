# rpeloader
use python on windows with full submodule support without installation

this is a PoC

rpeloader is the rust-python-embeddable-loader.

What it does is to leverage the python embeddable package, which is signed and trusted by EDRs, to execute python scripts.

rpeloader pulls down the python embeddable package, makes some minor changes to the files to allow loading 3rd party libraries, manually adds pip from source, and then accepts a python script in the form of a path or url from the user. In this implementation, if any py scripts that have 3rd party imports, rpeloader will identify the needed libraries and add them via pip, before executing the script. You can run essentially any python script from this Rust program. 

You could use this logic in a c2 framework or as a standalone payload with a hardcoded py script to grab, or whatever. Below is an example usage.

``` bash 
cargo build --release
./rpeloader.exe
```
NOTE - the python scripts are included for PoC testing. They are not needed to use the tool.
