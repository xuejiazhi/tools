package util

import (
	"github.com/jedib0t/go-pretty/v6/table"
	"github.com/jedib0t/go-pretty/v6/text"
	"github.com/muesli/gotable"
	"github.com/shirou/gopsutil/v3/mem"
	"github.com/spf13/cast"
	"os"
	"tool/cmd/define"
)

func ShowTable(title string, header []interface{}, data [][]interface{}) {
	prettyTable := table.NewWriter()
	prettyTable.SetTitle(title)
	prettyTable.SetStyle(table.StyleColoredBlackOnYellowWhite)
	prettyTable.SetColumnConfigs([]table.ColumnConfig{
		{
			Name:         "total",
			Colors:       text.Colors{text.BgHiGreen, text.Bold},
			ColorsHeader: text.Colors{text.BgHiGreen, text.FgHiYellow, text.Bold},
			ColorsFooter: text.Colors{text.BgHiGreen, text.FgHiYellow},
		},
		{
			Name:         "used%",
			Colors:       text.Colors{text.BgHiBlack, text.FgHiGreen, text.Bold},
			ColorsHeader: text.Colors{text.BgHiRed, text.FgGreen, text.Bold},
			ColorsFooter: text.Colors{text.BgHiRed, text.FgGreen},
		},
	})
	prettyTable.SetOutputMirror(os.Stdout)
	prettyTable.AppendHeader(header)
	if len(data) > 0 {
		for _, v := range data {
			prettyTable.AppendRow(v)
			//prettyTable.AppendSeparator()
		}
	}
	prettyTable.Render()
}

type MonitorData struct {
	VM *mem.VirtualMemoryStat
}

func ShowAllTable(data *MonitorData) {
	rowConfigAutoMerge := table.RowConfig{AutoMerge: true}
	t := table.NewWriter()
	//t.SetColumnConfigs([]table.ColumnConfig{
	//	{Number: 1, AutoMerge: true},
	//	{
	//		Name: "Name",
	//		//Colors:       text.Colors{text.BgHiGreen},
	//		//ColorsHeader: text.Colors{text.BgHiGreen, text.FgHiYellow},
	//		//ColorsFooter: text.Colors{text.BgHiGreen, text.FgHiYellow},
	//	},
	//	{
	//		Name:         "Value",
	//		Colors:       text.Colors{text.BgHiBlack, text.FgHiGreen},
	//		ColorsHeader: text.Colors{text.BgHiRed, text.FgGreen},
	//		ColorsFooter: text.Colors{text.BgHiRed, text.FgGreen},
	//	},
	//})
	t.AppendHeader(table.Row{"#", "参数", "参数", "百分比%"}, rowConfigAutoMerge)
	t.AppendHeader(table.Row{"", "Name", "Value", ""})
	t.AppendRow(table.Row{"Mem", "Total", data.VM.Total, ""}, rowConfigAutoMerge)
	t.AppendRow(table.Row{"Mem", "Used", data.VM.Used, ""}, rowConfigAutoMerge)
	t.AppendRow(table.Row{"Mem", "Free", data.VM.Free, ""}, rowConfigAutoMerge)
	t.AppendRow(table.Row{"Mem", "Shared", data.VM.Shared, ""}, rowConfigAutoMerge)
	t.AppendRow(table.Row{"Mem", "Buffers", data.VM.Buffers, ""}, rowConfigAutoMerge)
	t.AppendRow(table.Row{"Mem", "Cached", data.VM.Cached, ""}, rowConfigAutoMerge)
	t.AppendRow(table.Row{"Mem", "Available", data.VM.Available, ""}, rowConfigAutoMerge)
	t.SetOutputMirror(os.Stdout)
	t.SetStyle(table.StyleColoredBright)
	//t.Style().Options.SeparateRows = true
	t.Render()
}

func ShowSimpleTable[T string](header []T, width []int64, data [][]interface{}) {
	var hd []string
	for _, v := range header {
		hd = append(hd, cast.ToString(v))
	}

	tab := gotable.NewTable(hd,
		width,
		define.NoDataInTbErrorMsg)
	if len(data) > 0 {
		for _, v := range data {
			tab.AppendRow(v)
		}
	}
	tab.Print()
}
