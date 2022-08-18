package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/disk"
	"github.com/shirou/gopsutil/v3/mem"
	"github.com/spf13/cast"
	"sys_tool/cmd/sdk/goPrint"
	"sys_tool/cmd/util"
	"time"
)

type Monitor struct {
	MemBar  *goPrint.Bar
	SwapBar *goPrint.Bar
	CpuBar  *goPrint.Bar
}

// MonitorPercent  监控内存
func (m *Monitor) MonitorPercent() {
	for {
		memInfo, _ := mem.VirtualMemory()
		swapInfo, _ := mem.SwapMemory()
		fmt.Print("\033c")
		fmt.Print("【Memory】\n")
		m.GetMemUsePercent(memInfo)
		m.GetSwapUsePercent(swapInfo)
		new(SysInfo).ShowMemory("g")
		time.Sleep(500 * time.Millisecond)
	}
}

func (m *Monitor) GetMemUsePercent(memInfo *mem.VirtualMemoryStat) {
	m.MemBar = goPrint.NewBar(100)
	m.MemBar.SetNotice("内存使用率：")
	m.MemBar.SetRatioColor(1)
	m.MemBar.SetGraph(">")
	m.MemBar.SetRatioSuffix("GB")
	m.MemBar.SetRatioTotalData(util.FormatSizeFloat(memInfo.Total, "g"))
	m.MemBar.SetRatioUseData(util.FormatSizeFloat(memInfo.Used, "g"))
	m.MemBar.PrintBar(cast.ToInt(memInfo.UsedPercent))
}

func (m *Monitor) GetSwapUsePercent(swapInfo *mem.SwapMemoryStat) {
	m.SwapBar = goPrint.NewBar(100)
	m.SwapBar.SetNotice("Swap使用率：")
	m.SwapBar.SetRatioColor(1)
	m.SwapBar.SetGraph(">")
	m.SwapBar.SetRatioSuffix("GB")
	m.SwapBar.SetRatioTotalData(util.FormatSizeFloat(swapInfo.Total, "g"))
	m.SwapBar.SetRatioUseData(util.FormatSizeFloat(swapInfo.Used, "g"))
	m.SwapBar.PrintBar(cast.ToInt(swapInfo.UsedPercent))
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
