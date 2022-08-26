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
	Cache *cache.Cache
)
