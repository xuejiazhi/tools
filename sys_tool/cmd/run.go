package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
	"os"
	"strings"
	"sys_tool/cmd/define"
	"sys_tool/cmd/opt"
	"sys_tool/cmd/util"
)

var (
	SysInfoCxt = new(opt.SysInfo)
	MonitorCxt = new(opt.Monitor)
)

// Ps  CMD
var Ps = &cobra.Command{
	Use:   "ps",
	Short: "show Memory Information",
	Long:  "show Detail Memory Information",
	Run: func(cmd *cobra.Command, args []string) {
		//get host
		if o, _ := cmd.Flags().GetBool("host"); o {
			SysInfoCxt.ShowHost()
		}
		//get memory
		if m, _ := cmd.Flags().GetBool("memory"); m {
			SysInfoCxt.ShowMemory(judgePsMemoryArgs(args))
		}
		//get cpu
		if c, _ := cmd.Flags().GetBool("cpu"); c {
			SysInfoCxt.ShowCpu()
		}
		//get disk
		if d, _ := cmd.Flags().GetBool("disk"); d {
			SysInfoCxt.ShowDisk()
		}

		if a, _ := cmd.Flags().GetBool("all"); a {
			SysInfoCxt.ShowAll()
		}

		if av, _ := cmd.Flags().GetBool("avg"); av {
			SysInfoCxt.ShowLoadAvg()
		}
	},
}

func addPsFlags() {
	//定义ps flags
	Ps.Flags().BoolVarP(&define.AvgRp, "avg", "v", false, "show load avg information")
	Ps.Flags().BoolVarP(&define.HostRp, "host", "o", false, "show host information")
	Ps.Flags().BoolVarP(&define.MemoryRp, "memory", "m", false, "show memory information")
	Ps.Flags().BoolVarP(&define.CpuRp, "cpu", "c", false, "show cpu information")
	Ps.Flags().BoolVarP(&define.DiskRp, "disk", "d", false, "show disk information")
	Ps.Flags().BoolVarP(&define.DiskRp, "all", "a", false, "show host memory cpu disk information")
}

func judgePsMemoryArgs(args []string) string {
	if len(args) > 0 {
		if util.InSlice([]string{"g", "m", "k", "b"}, strings.ToLower(args[0])) {
			return strings.ToLower(args[0])
		} else {
			return "k"
		}
	}
	return "k"
}

// Monitor CMD
var Monitor = &cobra.Command{
	Use:   "monitor",
	Short: "monitoring system",
	Long:  "monitoring system",
	Run: func(cmd *cobra.Command, args []string) {
		MonitorCxt.MonitorPercent()
	},
}

func addMonitorFlags() {
	//定义ps flags
	Monitor.Flags().BoolVarP(&define.MemoryRp, "memory", "m", false, "show memory information")
}

func Run() {
	var rootCmd = &cobra.Command{Use: "sys_tool"}

	//add cmd flags
	addPsFlags()
	addMonitorFlags()

	//add command
	rootCmd.AddCommand(Ps)
	rootCmd.AddCommand(Monitor)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err.Error())
		os.Exit(2)
	}
}
