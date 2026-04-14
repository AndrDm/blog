the mystery of the while(1); and for(;;);.

Usually the constructions like while(1); or while(true) or sometimes rarely for(;;); used for infinte loops to "stop" but not terminate execution at some point. But they will not always executed as expected and can be optimized away.

In the experiment I'll take clang 11.0.1 and simple code:

```c
static void loop() {
    while(1);    
}

int main() {
    loop();
    return 0;
}
```

You can take any clang between 3.9.0 and 11.0.1.

Can be downloaded from [Releases](https://releases.llvm.org). LLVM [11.0.1](https://releases.llvm.org/download.html#11.0.1). Clang for [Windows (64-bit)](https://github.com/llvm/llvm-project/releases/download/llvmorg-11.0.0/LLVM-11.0.0-win64.exe). [How to install](http://blog.johannesmp.com/2015/09/01/installing-clang-on-windows-pt2/).

Now compile this code above twice - with optimization (-O1) and without.

```bash
clang-cl.exe clang_test.c -o clang_test.exe
clang-cl.exe -O1 clang_test.c -o clang_test.exe
```

And you will be surprised - the version without optimization will running infinitely, but version with O1 (or O2) will be terminated.