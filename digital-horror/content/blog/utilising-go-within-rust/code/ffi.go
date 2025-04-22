package main

/*
#include <stdlib.h>
*/
import "C"
// You'd likely want to use unsafe in real-world implementations, I'm omitting
// this to avoid compiler error.
// import "unsafe"

//export wappalyzer
func wappalyzer(a, b *C.char) *C.char {
    // Convert incoming C strings to Go strings, if you need them
    // goA := C.GoString(a)
    // goB := C.GoString(b)

    // For our demo, we'll just return a constant.
    // C.CString allocates with malloc; caller must free.
    return C.CString("cloudflare")
}

func main() {
    // A dummy main so cgo will build a shared library, not a CLI.
}

