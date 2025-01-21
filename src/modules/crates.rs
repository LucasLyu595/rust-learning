/* 'compliation unit' for Rust is the *crate*, which is either an executable or a library
* To separately compile the files from the last section, first build foo.rs as a Rust static library crate:
* ```bash
* src$ rustc foo.rs --crate-type=lib
* src$ ls -l libfoo.rlib
* -rw-rw-r-- 1 steve steve 7888 Jan  5 13:35 libfoo.rlib
* ```
* We can now link this into our main program:
* ```bash
* src$ rustc mod4.rs --extern foo=libfoo.rlib
* ```
* But the main program must now look like this, where the extern name is the same as the one used when linking. There is an implicit top-level module foo associated with the library crate:
* ```Rust
* extern crate foo;
* ```
* It's time to understand why Rust binaries are so large:
* ```bash
* src$ ls -lh mod4
* ```bash
* -rwxrwxr-x 1 steve steve 3,4M Jan  5 13:39 mod4
* ```
* That's rather fat! There is a lot of debug information in that executable
* This is not a Bad Thing, if you want to use a debugger and actually want meaningful backtraces when your program panics.
* So let's strip that debug information and see:
* ```bash
* src$ strip mod4
* src$ ls -lh mod4
* -rwxrwxr-x 1 steve steve 300K Jan  5 13:49 mod4
* ```
* Still feels a little large for something so simple
* but this program links statically against the Rust standard library
* This is a Good Thing, since you can hand this executable to anyone with the right operating system
* they will not need a 'Rust runtime'
* And rustup will even let you cross-compile for other operating systems and platforms as well.
* We can link dynamically against the Rust runtime and get truly tiny exes:
* ```bash
* src$ rustc -C prefer-dynamic mod4.rs --extern foo=libfoo.rlib
* src$ ls -lh mod4
* -rwxrwxr-x 1 steve steve 14K Jan  5 13:53 mod4
* src$ ldd mod4
*     linux-vdso.so.1 =>  (0x00007fffa8746000)
*     libstd-b4054fae3db32020.so => not found
*     libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f3cd47aa000)
*     /lib64/ld-linux-x86-64.so.2 (0x00007f3cd4d72000)
* ```
* That 'not found' is because rustup doesn't install the dynamic libraries globally. We can hack our way to happiness, at least on Unix (yes, I know the best solution is a symlink.)
* ```bash
* src$ export LD_LIBRARY_PATH=~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib
* src$ ./mod4
* Foo { s: "hello" }
* ```
* Rust does not have a philosophical problem with dynamic linking, in the same way as Go does. It's just that when there's a stable release every 6 weeks it becomes inconvenient to have to recompile everything. If you have a stable version that Works For You, then cool. As stable versions of Rust get increasingly delivered by the OS package manager, dynamic linking will become more popular.
*/
pub fn run() {
    println!("Hello from crates.rs");
}
