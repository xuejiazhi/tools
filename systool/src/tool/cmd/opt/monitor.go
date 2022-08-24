package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/disk"
	"github.com/shirou/gopsutil/v3/mem"
	"github.com/spf13/cast"
	"time"
	"tool/cmd/sdk/goPrint"
)

type Monitor struct {
	MemBar  *goPrint.Bar
	SwapBar *goPrint.Bar
	CpuBar  *goPrint.Bar
}

// MonitorPercent  监控内存
func (m *Monitor) MonitorPercent() {
	for {
		//memInfo, _ := mem.VirtualMemory()
		//swapInfo, _ := mem.SwapMemory()
		fmt.Print("\033c")
		//m.GetMemUsePercent(memInfo)
		//m.GetSwapUsePercent(swapInfo)
		//new(SysInfo).ShowMemory("g")
		new(SysInfo).ShowTest()
		time.Sleep(3 * time.Second)
	}
}

func (m *Monitor) GetMemUsePercent(memInfo *mem.VirtualMemoryStat) {
	m.MemBar = goPrint.NewBar(20)
	//m.MemBar.SetNotice("内存使用率：")
	m.MemBar.SetRatioColor(1)
	m.MemBar.SetGraph(">")
	//m.SwapBar.HideRatio()
	//m.MemBar.SetRatioSuffix("GB")
	//m.MemBar.SetRatioTotalData(util.FormatSizeFloat(memInfo.Total, "g"))
	//m.MemBar.SetRatioUseData(util.FormatSizeFloat(memInfo.Used, "g"))
	m.MemBar.PrintBar(cast.ToInt(memInfo.UsedPercent) / 5)
}

func (m *Monitor) GetSwapUsePercent(swapInfo *mem.SwapMemoryStat) {
	m.SwapBar = goPrint.NewBar(20)
	//m.SwapBar.SetNotice("Swap使用率：")
	m.SwapBar.SetRatioColor(1)
	m.SwapBar.SetGraph(">")
	//m.SwapBar.HideRatio()
	//m.SwapBar.SetRatioSuffix("GB")
	//m.SwapBar.SetRatioTotalData(util.FormatSizeFloat(swapInfo.Total, "g"))
	//m.SwapBar.SetRatioUseData(util.FormatSizeFloat(swapInfo.Used, "g"))
	m.SwapBar.PrintBar(cast.ToInt(swapInfo.UsedPercent) / 5)
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
