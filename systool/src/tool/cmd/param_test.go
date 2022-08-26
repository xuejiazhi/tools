package cmd

import (
	"fmt"
	"testing"
	"tool/cmd/util"
)

func Test_interface(t *testing.T) {
	v := [][]interface{}{{"a"}, {"b"}, {"c"}}
	fmt.Println("v=>", v)
}

func TestRun(t *testing.T) {
	fmt.Println("\x1b[30mtest 29") //red
	fmt.Println("\x1b[30mtest 30") //red
	fmt.Println("\x1b[31mtest 31") //red
	fmt.Println("\x1b[32mtest 32") //green
	fmt.Println("\x1b[33mtest 33") //green
	fmt.Println("\x1b[34mtest 34") //blue
	fmt.Println("\x1b[35mtest 35")
	fmt.Println("\x1b[36mtest 36")
	fmt.Println("\x1b[37mtest 37")
	fmt.Println("\x1b[38mtest 38")
	fmt.Println("\x1b[39mtest 39")
	fmt.Println("\x1b[94mtest 94")
	fmt.Println("\x1b[90mtest 90")
	fmt.Println("\x1b[91mtest 91")
	fmt.Println("\x1b[92mtest 92")
	fmt.Println("\x1b[93mtest 93")
	fmt.Println("\x1b[94mtest 94")
	fmt.Println("\x1b[95mtest 95")
	fmt.Println("\x1b[96mtest 96")
	fmt.Println("\x1b[97mtest 97")
	fmt.Println("\x1b[98mtest 98")
	fmt.Println("\x1b[99mtest 99")
	fmt.Println("\x1b[100mtest 100")
	fmt.Printf("Black Background: %#v\n", "TETS")
}

func TestColor(t *testing.T) {
	str := util.SetColor("hello world", util.LightGreen, true, true)
	fmt.Println(str)
	st1 := util.SetColor("hello world", util.LightGreen)
	fmt.Println(st1)
}
