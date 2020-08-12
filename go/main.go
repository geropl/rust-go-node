package main

/*
#cgo CFLAGS: -I./lib
#cgo LDFLAGS: -L./lib -lrust_lib_ffi
#include "./lib/librust.h"
*/
import "C"

import (
	"fmt"
	"os"
)

func main() {
	result := C.create_from_license_key(C.CString(""), C.CString(""))
	defer C.free_result(result)
	if !C.result_is_ok(&result) {
		errStrPtr := result.err
		fmt.Printf("error: %s\n", C.GoString(errStrPtr))
		os.Exit(1)
	}
	evaluator := result.evaluator
	r := C.validate(evaluator)

	fmt.Println("success! result: %v", r)
}
