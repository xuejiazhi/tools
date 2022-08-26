package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/process"
	"github.com/spf13/cast"
	"tool/cmd/util"
)

type ProcessCommData struct {
	PID        string `json:"pid"`
	Command    string `json:"command"`
	User       string `json:"user"`
	CreateTime int64  `json:"create_time"`
}

type ProcessCpuData struct {
	ProcessCommData
	US     float64 `json:"us"`
	SY     float64 `json:"sy"`
	IDLE   float64 `json:"idle"`
	NI     float64 `json:"ni"`
	IOWAIT float64 `json:"iowait"`
	HI     float64 `json:"hi"`
	SI     float64 `json:"si"`
	ST     float64 `json:"st"`
	USED   float64 `json:"used%"`
}

type ProcessMemData struct {
	ProcessCommData
	RSS  uint64  `json:"rss"`
	VMS  uint64  `json:"vms"`
	USED float64 `json:"used%"`
}

func (m *Monitor) GetProcess() {
	//define cpu mem data
	var procCpuList []ProcessCpuData
	var procMemList []ProcessMemData
	processList, _ := process.Processes()
	for _, v := range processList {
		if run, _ := v.IsRunning(); run {
			//set proc common data
			var procData ProcessCommData
			var procCpuData ProcessCpuData
			var procMemData ProcessMemData

			m.setProcCommonData(v, &procData)
			cpuPercent, _ := v.CPUPercent()
			if cpuPercent > 0 {
				procCpuData.USED = util.Decimal2(cpuPercent)
				//get cpu data
				procCpuData.ProcessCommData = procData
				procCpuTimes, _ := v.Times()
				procCpuData.US = util.Decimal2(procCpuTimes.User)
				procCpuData.SY = util.Decimal2(procCpuTimes.System)
				procCpuData.IDLE = util.Decimal2(procCpuTimes.Idle)
				procCpuData.NI = util.Decimal2(procCpuTimes.Nice)
				procCpuData.IOWAIT = util.Decimal2(procCpuTimes.Iowait)
				procCpuData.HI = util.Decimal2(procCpuTimes.Irq)
				procCpuData.SI = util.Decimal2(procCpuTimes.Softirq)
				procCpuList = append(procCpuList, procCpuData)
			}

			//get mem data
			memPercent, _ := v.MemoryPercent()
			if memPercent > 0 {
				procMemData.USED = util.Decimal2(cast.ToFloat64(memPercent))
				procMemData.ProcessCommData = procData
				procMemInfo, _ := v.MemoryInfo()
				procMemData.RSS = procMemInfo.RSS
				procMemData.VMS = procMemInfo.VMS
				procMemList = append(procMemList, procMemData)
			}
		}
	}
	//数据排序
	sortCpuList := buddleCpuSort(procCpuList)
	sortMemList := buddleMemSort(procMemList)

}

func getTop50CpuStr() {
	header := fmt.Sprintf("%s%s%s%s%s%s%s%s%s%s%s%s%s\n",
		util.FillStr("pid", 12),
		util.FillStr("command", 12),
		util.FillStr("user", 12),
		util.FillStr("time", 12),
		util.FillStr("us", 12),
		util.FillStr("sy", 12),
		util.FillStr("idle", 12),
		util.FillStr("ni", 12),
		util.FillStr("iowait", 12),
		util.FillStr("hi", 12),
		util.FillStr("si", 12),
		util.FillStr("st", 12),
		util.FillStr("used% ", 12),
	)
	cpulist := fmt.Sprintf("%s%s", cpulist, util.SetColor(header, util.LightWhite, true, true))
}

func (m *Monitor) setProcCommonData(proc *process.Process, procData *ProcessCommData) {
	procData.PID = proc.String()
	procData.User, _ = proc.Username()
	procData.Command, _ = proc.Name()
	procData.CreateTime, _ = proc.CreateTime()
}

// process map 冒泡排序
func buddleCpuSort(cpulist []ProcessCpuData) (cpuReList []ProcessCpuData) {
	cpuReList = cpulist
	for i := 0; i < len(cpuReList); i++ {
		for j := 1; j < len(cpuReList)-i; j++ {
			if cpuReList[j].USED > cpuReList[j-1].USED {
				cpuReList[j], cpuReList[j-1] = cpuReList[j-1], cpuReList[j]
			}
		}
	}
	//取切片的前50
	cpuReList = cpuReList[0:50]
	return
}

func buddleMemSort(memlist []ProcessMemData) (memReList []ProcessMemData) {
	memReList = memlist
	for i := 0; i < len(memReList); i++ {
		for j := 1; j < len(memReList)-i; j++ {
			if memReList[j].USED > memReList[j-1].USED {
				memReList[j], memReList[j-1] = memReList[j-1], memReList[j]
			}
		}
	}
	//取切片的前50
	memReList = memReList[0:50]
	return
}
