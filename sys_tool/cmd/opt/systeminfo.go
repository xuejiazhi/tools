package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/mem"
	"sys_tool/cmd/util"
)

type SysInfo struct {
}

func (s *SysInfo) ShowMemory(flag string) {
	fmt.Println("#Memory")
	//util.ShowTable()
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
	util.ShowTable(headers, data)
}

func (s *SysInfo) ShowCpu() {
	fmt.Println("cpu")
}

func (s *SysInfo) ShowDisk() {
	fmt.Println("disk")
}

func (s *SysInfo) ShowAll() {
	fmt.Println("all")
}
