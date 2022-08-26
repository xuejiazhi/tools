package opt

import (
	"fmt"
	"github.com/shirou/gopsutil/v3/cpu"
	"github.com/spf13/cast"
	"time"
	"tool/cmd/define"
	"tool/cmd/util"
)

func GetCpuStr() (cpuStr string) {
	//CPU
	cpuHeader := setC("[Cpu]", "", 1)
	cpuModName := getCpuCacheMsg(CpuModelName)
	cpuPhysic := getCpuCacheMsg(CpuPhysicalID)
	cpuCores := getCpuCacheMsg(CpuCores)
	cpuMhz := getCpuCacheMsg(CpuMhz)
	cpuPercent, _ := cpu.Percent(0, false)
	cpuBars := setC("used%", util.GetMemoPercent(util.Decimal(cpuPercent[0])), 1)
	cpuStr = fmt.Sprintf("%s%s%s%s%s%s\n%s", cpuHeader, cpuModName, cpuPhysic, cpuCores, cpuMhz, cpuBars, getCpusPer())
	return
}

func getCpusPer() (cpulist string) {
	header := fmt.Sprintf("%s%s%s%s%s%s%s%s%s%s\n",
		util.FillStr("#name", 12),
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

	cpulist = fmt.Sprintf("%s%s", cpulist, util.SetColor(header, util.LightWhite, true, true))
	aver, _ := cpu.Times(true)
	//fmt.Println(aver)
	cpusPer, _ := cpu.Percent(0, true)
	for k, v := range aver {
		cpulist = fmt.Sprintf("%s%s", cpulist, util.SetColor(fmt.Sprintf("%s%s%s%s%s%s%s%s%s%s\n",
			util.FillStr(v.CPU, 12),
			util.FillStr(cast.ToString(util.Decimal2(v.User)), 12),
			util.FillStr(cast.ToString(util.Decimal2(v.System)), 12),
			util.FillStr(cast.ToString(util.Decimal2(v.Idle)), 12),
			util.FillStr(cast.ToString(util.Decimal2(v.Nice)), 12),
			util.FillStr(cast.ToString(util.Decimal2(v.Iowait)), 12),
			util.FillStr(cast.ToString(util.Decimal2(v.Irq)), 12),
			util.FillStr(cast.ToString(util.Decimal2(v.Softirq)), 12),
			util.FillStr(cast.ToString(util.Decimal2(v.Steal)), 12),
			util.FillStr(cast.ToString(util.Decimal2(cpusPer[k]))+"%", 12),
		), util.LightYellow))
	}
	return
}

func getCpuCacheMsg(types int) (retry string) {
	//cache key
	key := fmt.Sprintf("cpu.info.%s", CpuSuffix[types])
	if v, ok := define.Cache.Get(key); ok {
		return cast.ToString(v)
	}

	cpuInfos, _ := cpu.Info()
	switch types {
	case CpuModelName:
		retry = setC("model-name", cast.ToString(cpuInfos[0].ModelName), 1)
	case CpuPhysicalID:
		retry = setC("phys_id", cast.ToString(cpuInfos[0].PhysicalID), 1)
	case CpuCoreID:
		retry = setC("coreid", cast.ToString(cpuInfos[0].CoreID), 1)
	case CpuCores:
		retry = setC("cores", cast.ToString(cpuInfos[0].Cores), 1)
	case CpuMhz:
		retry = setC("mhz", cast.ToString(cpuInfos[0].Cores), 1)
	}
	define.Cache.Set(key, retry, 30*60*time.Second)
	return
}
