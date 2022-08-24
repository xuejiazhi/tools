package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/cpu"
	"github.com/shirou/gopsutil/v3/disk"
	"github.com/shirou/gopsutil/v3/host"
	"github.com/shirou/gopsutil/v3/load"
	"github.com/shirou/gopsutil/v3/mem"
	"github.com/spf13/cast"
	"tool/cmd/define"
	"tool/cmd/sdk/goPrint"
	"tool/cmd/util"
)

type SysInfo struct {
}

func (s *SysInfo) ShowHost() {
	hostInfo, _ := host.Info()
	var data [][]interface{}
	data = append(data, []interface{}{
		"Host:",
		hostInfo.Hostname,
		hostInfo.Uptime,
		hostInfo.BootTime,
		hostInfo.Procs,
		hostInfo.OS,
		hostInfo.Platform,
		hostInfo.PlatformFamily,
		hostInfo.PlatformVersion,
		hostInfo.KernelVersion,
		hostInfo.KernelArch,
		hostInfo.HostID,
	})
	util.ShowTable("#!--  Host Information", HostHeader, data)
}

// ShowMemory 显示内存的使用情况
func (s *SysInfo) ShowMemory(flag string) {
	//width := []int64{3, 10, 10, 10, 10, 10, 10, 10}
	var data [][]interface{}
	getMemoryList(flag, &data)
	//util.ShowSimpleTable(headers, width, data)
	util.ShowTable("#!--  Memory Information", MemoryHeader, data)
}

func getMemoryList(flag string, data *[][]interface{}) {
	//virtual Memory
	vm, _ := mem.VirtualMemory()
	//Swap Memory
	sm, _ := mem.SwapMemory()
	//data
	*data = append(*data,
		//virtual memory
		[]interface{}{
			"Mem:",
			util.FormatSize(vm.Total, flag),
			util.FormatSize(vm.Used, flag),
			util.FormatSize(vm.Free, flag),
			util.FormatSize(vm.Shared, flag),
			util.FormatSize(vm.Buffers, flag),
			util.FormatSize(vm.Cached, flag),
			util.FormatSize(vm.Available, flag),
			getMemoPercent(vm.UsedPercent),
		},
		//swap memory
		[]interface{}{
			"Swap",
			util.FormatSize(sm.Total, flag),
			util.FormatSize(sm.Used, flag),
			util.FormatSize(sm.Free, flag),
			"",
			"",
			"",
			"",
			getMemoPercent(sm.UsedPercent),
		})
}

func getMemoPercent(value float64) (grossbar string) {
	memBar := goPrint.NewBar(20)
	memBar.SetRatioColor(1)
	//memBar.HideRatio()
	memBar.SetGraph(">")
	grossbar = memBar.PrintBar(cast.ToInt(value) / 5)
	return
}

func (s *SysInfo) ShowCpu() {
	if cpuInfos, err := cpu.Info(); len(cpuInfos) > 0 && err == nil {
		var data [][]interface{}
		data = append(data, []interface{}{
			"Cpu:",
			cpuInfos[0].ModelName,
			cpuInfos[0].PhysicalID,
			cpuInfos[0].CPU,
			cpuInfos[0].VendorID,
			cpuInfos[0].Family,
			cpuInfos[0].Model,
			cpuInfos[0].Stepping,
			cpuInfos[0].CoreID,
			cpuInfos[0].Cores,
			cpuInfos[0].Mhz,
			cpuInfos[0].CacheSize,
			cpuInfos[0].Microcode,
		})
		util.ShowTable("#!--  Cpu Information", CpuHeader, data)
	} else {
		fmt.Println(define.GetCpuInfoErrorMsg, err)
		return
	}
}

func (s *SysInfo) ShowDisk() {
	if parts, err := disk.Partitions(true); err == nil {
		var data [][]interface{}
		//IO 状态统计
		ioStat, _ := disk.IOCounters()
		for _, part := range parts {
			data = append(data, []interface{}{
				part.Device,
				part.Mountpoint,
				part.Fstype,
				part.Opts,
				ioStat[part.Device].ReadCount,
				ioStat[part.Device].WriteCount,
				ioStat[part.Device].ReadBytes,
				ioStat[part.Device].WriteBytes,
				ioStat[part.Device].ReadTime,
				ioStat[part.Device].WriteTime,
				ioStat[part.Device].IoTime,
				ioStat[part.Device].SerialNumber,
				ioStat[part.Device].Label,
			})

		}
		util.ShowTable("#!-- Disk Information", DiskHeader, data)
	} else {
		fmt.Println(define.GetDiskInfoErrorMsg, err)
		return
	}
}

func (s *SysInfo) ShowLoadAvg() {
	info, _ := load.Avg()
	fmt.Printf("%v\n", info)
}

func (s *SysInfo) ShowAll() {
	s.ShowHost()
	s.ShowMemory("k")
	s.ShowCpu()
	s.ShowDisk()
}
