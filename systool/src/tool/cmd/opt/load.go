package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/load"
)

func GetLoadAver() (loadStr string) {
	loadInfo, _ := load.Avg()
	loadHead := setC("[Load Average]", "", 1)
	loadValue := setC("", fmt.Sprintf("%.2f,%.2f,%.2f", loadInfo.Load1, loadInfo.Load5, loadInfo.Load15), 1)
	loadStr = fmt.Sprintf("%s%s\n", loadHead, loadValue)
	return
}
