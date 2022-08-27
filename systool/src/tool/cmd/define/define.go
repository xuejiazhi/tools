package define

import "github.com/patrickmn/go-cache"

var (
	AvgRp    bool
	HostRp   bool
	MemoryRp bool
	CpuRp    bool
	DiskRp   bool
)

var (
	FlagMonitor   int64
	DefaultLength = 12
	Cache         *cache.Cache
)
