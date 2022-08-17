package util

import (
	"github.com/jedib0t/go-pretty/v6/table"
	"os"
)

func ShowTable(header []interface{}, data [][]interface{}) {
	prettyTable := table.NewWriter()
	prettyTable.SetOutputMirror(os.Stdout)
	//prettyTable.AppendHeader([]interface{}{"#", "First Name", "Last Name", "Salary"})
	prettyTable.AppendHeader(header)
	if len(data) > 0 {
		for _, v := range data {
			prettyTable.AppendRow(v)
		}
	}
	prettyTable.Render()
}
