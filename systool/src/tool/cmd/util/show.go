package util

import (
	"github.com/jedib0t/go-pretty/v6/table"
	"github.com/jedib0t/go-pretty/v6/text"
	"github.com/spf13/cast"
	"os"
	"tool/cmd/sdk/goPrint"
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
			prettyTable.AppendSeparator()
		}
	}
	prettyTable.Render()
}

func GetMemoPercent(value float64) (grossbar string) {
	memBar := goPrint.NewBar(20)
	memBar.SetRatioColor(1)
	//memBar.HideRatio()
	memBar.SetGraph("·")
	barValue := cast.ToInt(value) / 5
	if barValue == 0 {
		barValue = cast.ToInt(value)
	}
	grossbar = memBar.PrintBar(cast.ToInt(value) / 5)
	return
}

//func ShowSimpleTable[T string](header []T, width []int64, data [][]interface{}) {
//	var hd []string
//	for _, v := range header {
//		hd = append(hd, cast.ToString(v))
//	}
//
//	tab := gotable.NewTable(hd,
//		width,
//		define.NoDataInTbErrorMsg)
//	if len(data) > 0 {
//		for _, v := range data {
//			tab.AppendRow(v)
//		}
//	}
//	tab.Print()
//}
