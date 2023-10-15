package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/mem"
	"github.com/spf13/cast"
	"tool/cmd/sdk/goPrint"
	"tool/cmd/util"
)

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

func GetMemStr(types int) (reStr string) {
	//VM
	switch types {
	case VirtualMemory:
		vm, _ := mem.VirtualMemory()
		vmHead := setC("[Mem]", "", 1)
		vmTota := setC("total", util.FormatSize(vm.Total, "g"), 1)
		vmUsed := setC("us", util.FormatSize(vm.Used, "g"), 1)
		vmFree := setC("free", util.FormatSize(vm.Free, "g"), 1)
		vmShad := setC("shared", util.FormatSize(vm.Shared, "g"), 1)
		vmBuff := setC("buff", util.FormatSize(vm.Buffers, "g"), 1)
		vmCach := setC("cached", util.FormatSize(vm.Buffers, "g"), 1)
		vmAvai := setC("avail", util.FormatSize(vm.Available, "g"), 1)
		vmBars := setC("used%", util.GetMemoPercent(vm.UsedPercent), 1)
		if vm.UsedPercent > 50 {
			vmBars = setC("used%", util.GetMemoPercent(vm.UsedPercent), 1, true)
		}
		reStr = fmt.Sprintf("%s%s%s%s%s%s%s%s%s\n", vmHead, vmTota, vmUsed, vmFree, vmShad, vmBuff, vmCach, vmAvai, vmBars)
	case SwapMemory:
		sm, _ := mem.SwapMemory()
		smHead := setC("[Swap]", "", 1)
		smTota := setC("total", util.FormatSize(sm.Total, "g"), 1)
		smUsed := setC("used", util.FormatSize(sm.Used, "g"), 1)
		smFree := setC("free", util.FormatSize(sm.Free, "g"), 1)
		smBars := setC("used%", util.GetMemoPercent(sm.UsedPercent), 1)
		if sm.UsedPercent > 50 {
			smBars = setC("used%", util.GetMemoPercent(sm.UsedPercent), 1, true)
		}
		reStr = fmt.Sprintf("%s%s%s%s%s\n", smHead, smTota, smUsed, smFree, smBars)
	}

	return
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
