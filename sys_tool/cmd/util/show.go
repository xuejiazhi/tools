package util

import (
	"github.com/jedib0t/go-pretty/v6/table"
	"os"
)

func ShowTable(title string, header []interface{}, data [][]interface{}) {
	prettyTable := table.NewWriter()
	prettyTable.SetTitle(title)
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
