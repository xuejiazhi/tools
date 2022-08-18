package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/cpu"
	"github.com/shirou/gopsutil/v3/disk"
	"github.com/shirou/gopsutil/v3/host"
	"github.com/shirou/gopsutil/v3/load"
	"github.com/shirou/gopsutil/v3/mem"
	"tool/cmd/define"
	"tool/cmd/util"
)

type SysInfo struct {
}

func (s *SysInfo) ShowHost() {
	hostInfo, _ := host.Info()
	headers := []interface{}{"#",
		"hostname",
		"uptime",
		"bootTime",
		"procs",
		"os",
		"platform",
		"platformFamily",
		"platformVersion",
		"kernelVersion",
		"kernelArch",
		"hostId",
	}
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
	util.ShowTable("#!--  Host Information", headers, data)
}

func (s *SysInfo) ShowMemory(flag string) {
	headers := []interface{}{"#", "total", "used", "free", "shared", "buff", "cache", "available"}
	var data [][]interface{}
	//virtual Memory
	vm, _ := mem.VirtualMemory()
	var vmData []interface{}
	vmData = append(vmData, "Mem:")
	vmData = append(vmData, util.FormatSize(vm.Total, flag))
	vmData = append(vmData, util.FormatSize(vm.Used, flag))
	vmData = append(vmData, util.FormatSize(vm.Free, flag))
	vmData = append(vmData, util.FormatSize(vm.Shared, flag))
	vmData = append(vmData, util.FormatSize(vm.Buffers, flag))
	vmData = append(vmData, util.FormatSize(vm.Cached, flag))
	vmData = append(vmData, util.FormatSize(vm.Available, flag))
	data = append(data, vmData)
	//Swap Memory
	sm, _ := mem.SwapMemory()
	var smData []interface{}
	smData = append(smData, "Swap")
	smData = append(smData, util.FormatSize(sm.Total, flag))
	smData = append(smData, util.FormatSize(sm.Used, flag))
	smData = append(smData, util.FormatSize(sm.Free, flag))
	//virtualMemory := mem
	data = append(data, smData)
	util.ShowTable("#!--  Memory Information", headers, data)
}

func (s *SysInfo) ShowCpu() {
	if cpuInfos, err := cpu.Info(); len(cpuInfos) > 0 && err == nil {
		headers := []interface{}{"#",
			"ModelName",
			"PhysicalID",
			"CPU",
			"VendorID",
			"Family",
			"Model",
			"Stepping",
			"CoreID",
			"Cores",
			"Mhz",
			"CacheSize",
			"Microcode",
		}
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
		util.ShowTable("#!--  Cpu Information", headers, data)
	} else {
		fmt.Println(define.GetCpuInfoErrorMsg, err)
		return
	}
}

func (s *SysInfo) ShowDisk() {
	if parts, err := disk.Partitions(true); err == nil {
		headers := []interface{}{"#",
			"MountPoint",
			"FsType",
			"Opts",
			"ReadCount",
			"WriteCount",
			"ReadBytes",
			"WriteBytes",
			"ReadTime",
			"WriteTime",
			"IoTime",
			"SerialNumber",
			"Label",
		}
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
		util.ShowTable("#!-- Disk Information", headers, data)
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
