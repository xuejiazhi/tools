package opt

import (
	"fmt"
	"strings"
	"testing"
	"time"
)

func TestMonitor_GetProcess(t *testing.T) {
	var m = new(Monitor)
	m.GetProcess()
}

func TestGetTop50CpuStr(t *testing.T) {
	data := []string{"sss", "bbb", "cccc"}
	head := strings.Repeat("%s", len(data))
	datas := fmt.Sprintf(head)
	fmt.Println("data1->", datas)
	datas = fmt.Sprintf(datas, "sss")
	fmt.Println("data2->", datas)
	datas = fmt.Sprintf(datas, "bbb")
	fmt.Println("data->", datas)
}

func TestGetSmallUser(t *testing.T) {
	GetSmallUser("DESKTOP-H7D20RB\\xuejiazhi1661529187164DESKTOP-H7D20RB\\xuejiazhi")
}

func TestDateTime(t *testing.T) {
	times := int64(1661559671532)
	fmt.Println(times)
	ts := time.Unix(times, 0).Format("2006-01-02 15:04:05")
	fmt.Println(ts)
}
