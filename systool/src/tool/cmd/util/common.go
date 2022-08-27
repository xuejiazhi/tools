package util

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"reflect"
	"runtime"
	"strconv"
	"strings"
)

// FormatSize 字节的单位转换 保留两位小数
func FormatSize(size uint64, flag string) (reSize string) {
	reSize = fmt.Sprintf("%.2f%sB",
		FormatSizeFloat(size, flag),
		strings.ToUpper(flag),
	)
	return
}

// FormatSizeFloat  字节的单位转换 保留两位小数
func FormatSizeFloat(size uint64, flag string) (reSize float64) {
	stepMap := map[string]float64{
		"b": float64(1),
		"k": float64(1024),
		"m": float64(1024 * 1024),
		"g": float64(1024 * 1024 * 1024),
	}

	if _, ok := stepMap[flag]; ok {
		reSize = Decimal(float64(size) / stepMap[flag])
	} else {
		reSize = Decimal(float64(size) / float64(1))
	}
	return
}

func Decimal(value float64) float64 {
	value, _ = strconv.ParseFloat(fmt.Sprintf("%.2f", value), 64)
	return value
}

func Decimal2(value float64) float64 {
	value, _ = strconv.ParseFloat(fmt.Sprintf("%.2f", value), 64)
	return value
}

func InSlice(haystack interface{}, needle interface{}) bool {
	sVal := reflect.ValueOf(haystack)
	kind := sVal.Kind()
	if kind == reflect.Slice || kind == reflect.Array {
		for i := 0; i < sVal.Len(); i++ {
			if sVal.Index(i).Interface() == needle {
				return true
			}
		}
		return false
	}
	return false
}

func Json2Map(val string) (map[string]interface{}, error) {
	var data map[string]interface{}
	if err := json.Unmarshal([]byte(val), &data); err == nil {
		return data, nil
	} else {
		return data, err
	}
}

func Json2Array(val string) ([]interface{}, error) {
	var data []interface{}
	if err := json.Unmarshal([]byte(val), &data); err == nil {
		return data, nil
	} else {
		return data, err
	}
}

func ClearMonitor() {
	if runtime.GOOS == "windows" {
		cmd := exec.Command("cmd", "/c", "cls") //Windows example, its tested
		cmd.Stdout = os.Stdout
		cmd.Run()

	} else {
		cmd := exec.Command("clear") //Linux example, its tested
		cmd.Stdout = os.Stdout
		cmd.Run()
	}
}

func FillStr(str string, length int) (retStr string) {
	retStr = str
	for i := 0; i < length-len(str); i++ {
		if i%2 == 0 {
			retStr = " " + retStr
		} else {
			retStr = retStr + " "
		}

	}
	return
}

func FillStrBuilder(str string, length int) (retStr string) {
	var strBuiler strings.Builder
	strBuiler.WriteString(str)
	for i := 0; i < length-len(str); i++ {
		strBuiler.WriteString(" ")
	}
	retStr = strBuiler.String()
	return
}

func If[T any](condition bool, true, false T) interface{} {
	if condition {
		return true
	} else {
		return false
	}
}
