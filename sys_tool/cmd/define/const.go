package define

var (
	HostRp   bool
	MemoryRp bool
	CpuRp    bool
	DiskRp   bool
)

const (
	ParameterErrorMsg = "Command not implemented for that parameter." //参数错误

	NoArgErrorMsg = "There is no argument in the Slice"

	GetCpuInfoErrorMsg  = "Get CPU Information Failed"
	GetDiskInfoErrorMsg = "Get CPU Information Failed"
)
