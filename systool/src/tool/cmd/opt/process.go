package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/process"
	"github.com/spf13/cast"
	"strings"
	"tool/cmd/define"
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
	CPUPER float64 `json:"cpu%"`
}

type ProcessMemData struct {
	ProcessCommData
	RSS       uint64  `json:"rss"`
	VMS       uint64  `json:"vms"`
	MEMORYPER float64 `json:"mem%"`
}

//type ProcessDefine struct {
//	Header      []LineDefine     `json:"header"`
//	ProcCpuList []ProcessCpuData `json:"proc_cpu_list"`
//	ProcMemList []ProcessMemData `json:"proc_mem_list"`
//}

type LineDefine struct {
	Name   string `json:"name"`
	Length int    `json:"length"`
}

func (m *Monitor) GetProcess() (string, string) {
	//define cpu mem data
	var procCpuList []ProcessCpuData
	var procMemList []ProcessMemData
	//set cpu mem data list
	m.getProcessDataList(&procCpuList, &procMemList)
	//数据排序
	bubbleSort(&procCpuList, &procMemList)
	return GetTopCpuStr(procCpuList), GetTopMemStr(procMemList)
}

func (m *Monitor) setProcCommonData(proc *process.Process, procData *ProcessCommData) {
	procData.PID = cast.ToString(proc.Pid)
	procData.User, _ = proc.Username()
	procData.Command, _ = proc.Name()
	procData.CreateTime, _ = proc.CreateTime()
}

func GetTopMemStr(memlist []ProcessMemData) (memstr string) {
	//set header
	memstr = util.SetColor(
		setLine(ProcessHeader["mem"]), util.LightSeaBlue, true, true)

	for _, cpuData := range memlist {
		memstr = fmt.Sprintf("%s%s", memstr,
			util.SetColor(setLine(
				[]LineDefine{
					{Name: cpuData.PID},
					{Name: GetSmallUser(cpuData.User)},
					//{Name: time.Unix(cpuData.CreateTime, 0).Format("2006-01-02 15:04:05")},
					{Name: util.ConvUnixTime(cpuData.CreateTime), Length: 20},
					{Name: cast.ToString(cpuData.RSS)},
					{Name: cast.ToString(cpuData.VMS)},
					{Name: fmt.Sprintf("%s%%", cast.ToString(cpuData.MEMORYPER))},
					{Name: cpuData.Command},
				}), util.Yellow),
		)
	}
	return
}

func GetTopCpuStr(cpulist []ProcessCpuData) (cpustr string) {
	//set header
	cpustr = util.SetColor(
		setLine(ProcessHeader["cpu"]), util.LightSeaBlue, true, true)

	for _, cpuData := range cpulist {
		cpustr = fmt.Sprintf("%s%s", cpustr,
			util.SetColor(setLine(
				[]LineDefine{
					{Name: cpuData.PID},
					{Name: GetSmallUser(cpuData.User)},
					//{Name: time.Unix(cpuData.CreateTime, 0).Format("2006-01-02 15:04:05")},
					{Name: util.ConvUnixTime(cpuData.CreateTime), Length: 20},
					{Name: cast.ToString(cpuData.US)},
					{Name: cast.ToString(cpuData.SY)},
					{Name: cast.ToString(cpuData.IDLE)},
					{Name: cast.ToString(cpuData.NI)},
					{Name: cast.ToString(cpuData.IOWAIT)},
					{Name: cast.ToString(cpuData.HI)},
					{Name: cast.ToString(cpuData.SI)},
					{Name: cast.ToString(cpuData.ST)},
					{Name: fmt.Sprintf("%s%%", cast.ToString(cpuData.CPUPER))},
					{Name: cpuData.Command},
				}), util.Yellow),
		)
	}
	return
}

func (m *Monitor) getProcessDataList(procCpuList *[]ProcessCpuData, procMemList *[]ProcessMemData) {
	processList, _ := process.Processes()
	for _, p := range processList {
		if run, _ := p.IsRunning(); run {
			//set proc common data
			var procData ProcessCommData

			//get common data
			m.setProcCommonData(p, &procData)

			//define cpu data
			var procCpuData ProcessCpuData
			//get cpu data
			procCpuData.ProcessCommData = procData
			m.getCpuSingleData(p, &procCpuData)
			*procCpuList = append(*procCpuList, procCpuData)

			//define mem data
			var procMemData ProcessMemData
			//get mem data
			procMemData.ProcessCommData = procData
			m.getMemSingleData(p, &procMemData)
			*procMemList = append(*procMemList, procMemData)
		}
	}
}

// getCpuSingleData get process use cpu times
func (m *Monitor) getCpuSingleData(proc *process.Process, cpuData *ProcessCpuData) {
	cpuPercent, _ := proc.CPUPercent()
	if cpuPercent > 0 {
		cpuData.CPUPER = util.Decimal2(cpuPercent)
		//get cpu data
		cpuTimes, _ := proc.Times()
		cpuData.US = util.Decimal2(cpuTimes.User)
		cpuData.SY = util.Decimal2(cpuTimes.System)
		cpuData.IDLE = util.Decimal2(cpuTimes.Idle)
		cpuData.NI = util.Decimal2(cpuTimes.Nice)
		cpuData.IOWAIT = util.Decimal2(cpuTimes.Iowait)
		cpuData.HI = util.Decimal2(cpuTimes.Irq)
		cpuData.SI = util.Decimal2(cpuTimes.Softirq)
	}
}

func (m *Monitor) getMemSingleData(proc *process.Process, memData *ProcessMemData) {
	memPercent, _ := proc.MemoryPercent()
	if memPercent > 0 {
		memData.MEMORYPER = util.Decimal2(cast.ToFloat64(memPercent))
		procMemInfo, _ := proc.MemoryInfo()
		memData.RSS = procMemInfo.RSS
		memData.VMS = procMemInfo.VMS
	}
}

// process map 冒泡排序
func bubbleSort(cpulist *[]ProcessCpuData, memlist *[]ProcessMemData) {
	for i := 0; i < len(*cpulist); i++ {
		for j := 1; j < len(*cpulist)-i; j++ {
			if (*cpulist)[j].CPUPER > (*cpulist)[j-1].CPUPER {
				(*cpulist)[j], (*cpulist)[j-1] = (*cpulist)[j-1], (*cpulist)[j]
			}
		}
	}

	for i := 0; i < len(*memlist); i++ {
		for j := 1; j < len(*memlist)-i; j++ {
			if (*memlist)[j].MEMORYPER > (*memlist)[j-1].MEMORYPER {
				(*memlist)[j], (*memlist)[j-1] = (*memlist)[j-1], (*memlist)[j]
			}
		}
	}
	//取切片的前50
	*cpulist = (*cpulist)[0:15]
	*memlist = (*memlist)[0:15]
}

func setLine(lines []LineDefine) (line string) {
	for _, hd := range lines {
		line = line + util.FillStr(hd.Name, util.If(hd.Length > 0, hd.Length, define.DefaultLength).(int))
	}

	if strings.TrimSpace(line) != "" {
		line = line + "\n"
	}
	return
}

func GetSmallUser(user string) string {
	users := strings.Split(user, "\\")
	return users[len(users)-1]
}
