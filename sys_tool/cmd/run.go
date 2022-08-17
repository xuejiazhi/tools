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

// Ps  CMD
var Ps = &cobra.Command{
	Use:   "ps",
	Short: "show Memory Information",
	Long:  "show Detail Memory Information",
	Run: func(cmd *cobra.Command, args []string) {
		s := new(opt.SysInfo)
		//get host
		if o, _ := cmd.Flags().GetBool("host"); o {
			s.ShowHost()
		}
		//get memory
		if m, _ := cmd.Flags().GetBool("memory"); m {
			s.ShowMemory(judgePsMemoryArgs(args))
		}
		//get cpu
		if c, _ := cmd.Flags().GetBool("cpu"); c {
			s.ShowCpu()
		}
		//get disk
		if d, _ := cmd.Flags().GetBool("disk"); d {
			s.ShowDisk()
		}

		if a, _ := cmd.Flags().GetBool("all"); a {
			s.ShowAll()
		}
	},
}

func addPsFlags() {
	//定义ps flags
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

func Run() {
	var rootCmd = &cobra.Command{Use: "sys_tool"}

	addPsFlags()
	//add command
	rootCmd.AddCommand(Ps)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err.Error())
		os.Exit(2)
	}
}
