## Trying to create an operating system

### Features

VGA Buffer    
Allocator   
GDT    
Keyboard    
Async  
Rtc (Clock and date)  

##  What do you need to run it?
    x86-64 CPU (real or virtualized)
    Qemu
    Rust-nightly
    Cargo bootimage
    rustup component toolchain nightly-x86_64-unknown-linux-gnu
    rustup component llvm-tools-preview

## How run it?
    cargo run
