package main

/*
#cgo LDFLAGS: -L./lib -lrust_lib_ffi
#include "./lib/librust.h"
*/
import "C"

import (
	"fmt"
	"os"
)

func main() {
	cStr := C.concat_strs(C.CString("a"), C.CString("b"))
	defer C.free_cstring(cStr)	// do not forget to free Rust memory the Rust way

	result := C.GoString(cStr)
	if result != "ab" {
		fmt.Println("expected 'ab', got: '%s'", result)
		os.Exit(1)
	}
	fmt.Println("success!")
}