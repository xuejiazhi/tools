package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/disk"
	"time"
	"tool/cmd/sdk/goPrint"
	"tool/cmd/util"
)

type Monitor struct {
	MemBar  *goPrint.Bar
	SwapBar *goPrint.Bar
	CpuBar  *goPrint.Bar
}

const (
	VirtualMemory int = iota
	SwapMemory
)

const (
	CpuModelName int = iota
	CpuPhysicalID
	CpuCoreID
	CpuCores
	CpuMhz
)

var CpuSuffix = map[int]string{
	CpuModelName:  "model_name",
	CpuPhysicalID: "physical_ID",
	CpuCoreID:     "core_ID",
	CpuCores:      "cores",
	CpuMhz:        "mhz",
}

// MonitorPercent  监控内存
func (m *Monitor) MonitorPercent() {
	for {
		fmt.Print("\033c")
		m.SniffMonitorTable()
		time.Sleep(3 * time.Second)
	}
}

func getDiskUsePercent() float64 {
	parts, _ := disk.Partitions(true)
	diskInfo, _ := disk.Usage(parts[0].Mountpoint)
	return diskInfo.UsedPercent
}

//// MonitorCpu  监控内存
//func (m *Monitor) MonitorCpu() {
//
//}
//
//// MonitorDisk   监控硬盘
//func (m *Monitor) MonitorDisk() {
//
//}
//
//// MonitorNet MonitorDisk   监控网络
//func (m *Monitor) MonitorNet() {
//
//}

func (m *Monitor) SniffMonitorTable() {
	loadTxt := GetLoadAver()
	//VM
	vmTxt := GetMemStr(VirtualMemory)
	//SM
	smTxt := GetMemStr(SwapMemory)
	//CPU
	cpuTxt := GetCpuStr()
	fmt.Print(fmt.Sprintf("\r%s%s%s%s", loadTxt, cpuTxt, vmTxt, smTxt))
}

func setC(title, value string, spc int, danger ...bool) (reStr string) {
	if value == "" {
		reStr = util.SetColor(title, util.LightSeaBlue, true)
	} else {
		reStr = fmt.Sprintf("%s:%s",
			util.SetColor(title, util.LightBlue, true, true),
			util.SetColor(value, util.LightGreen),
		)
		if len(danger) > 0 {
			if danger[0] == true {
				reStr = fmt.Sprintf("%s:%s",
					util.SetColor(title, util.LightBlue, true, true),
					util.SetColor(value, util.LightRed),
				)
			}
		}
	}
	reStr = util.FillStrBuilder(reStr, len(reStr)+spc)
	return
}
