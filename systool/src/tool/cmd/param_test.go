package cmd

import (
	"fmt"
	"testing"
)

func Test_interface(t *testing.T) {
	v := [][]interface{}{{"a"}, {"b"}, {"c"}}
	fmt.Println("v=>", v)
}
