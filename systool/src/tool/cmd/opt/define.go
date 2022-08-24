package opt

var (
	HostHeader = []interface{}{"#",
		"hostname",
		"uptime",
		"bootTime",
		"procs",
		"os",
		"platform",
		"platformFamily",
		"platformVersion",
		"kernelVersion",
		"kernelArch",
		"hostId",
	}

	MemoryHeader = []interface{}{"#",
		"total",
		"used",
		"free",
		"shared",
		"buff",
		"cache",
		"available",
		"used%",
	}

	CpuHeader = []interface{}{"#",
		"ModelName",
		"PhysicalID",
		"CPU",
		"VendorID",
		"Family",
		"Model",
		"Stepping",
		"CoreID",
		"Cores",
		"Mhz",
		"CacheSize",
		"Microcode",
	}

	DiskHeader = []interface{}{"#",
		"MountPoint",
		"FsType",
		"Opts",
		"ReadCount",
		"WriteCount",
		"ReadBytes",
		"WriteBytes",
		"ReadTime",
		"WriteTime",
		"IoTime",
		"SerialNumber",
		"Label",
	}
)
