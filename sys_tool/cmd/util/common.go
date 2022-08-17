package util

import (
	"encoding/json"
	"fmt"
	"reflect"
)

// 字节的单位转换 保留两位小数
func FormatSize(size uint64, flag string) (reSize string) {
	switch flag {
	case "k": //return KB
		return fmt.Sprintf("%.2fKB", float64(size)/float64(1024))
	case "m": //return MB
		return fmt.Sprintf("%.2fMB", float64(size)/float64(1024*1024))
	case "g":
		return fmt.Sprintf("%.2fGB", float64(size)/float64(1024*1024*1024))
	case "b":
		return fmt.Sprintf("%.2fB", float64(size)/float64(1))
	default:
		return fmt.Sprintf("%.2fB", float64(size)/float64(1))
	}
	return
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
