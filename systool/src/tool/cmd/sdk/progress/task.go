package progress

import (
	"fmt"
	_ "io"
	"log"
	_ "net/http"
	_ "os"
	"strings"
	"sync"
	"time"
)

// FixedThreadPool 线程池
type FixedThreadPool struct {
	*TotalProgress         // 总体进程
	Tasks          []*Task // 所有任务
	GoroutineNum   int     // 线程数
	IsVisible      bool    // 是否显示进度条
}

// NewFixedThreadPool 创建线程池
func NewFixedThreadPool(tasks []*Task, goroutineNum int, isVisible bool) *FixedThreadPool {
	return &FixedThreadPool{
		Tasks:         tasks,
		TotalProgress: NewTotalProgress(len(tasks), goroutineNum),
		GoroutineNum:  goroutineNum,
		IsVisible:     isVisible,
	}
}

// TotalProgress 总进度
type TotalProgress struct {
	FinishNum   int                  // 完成数
	TaskNum     int                  // 总任务数
	AllProgress []*GoroutineProgress // 所有goroutine的进度
}

// NewTotalProgress 初始化总进度
func NewTotalProgress(taskNum, goroutineNum int) *TotalProgress {
	return &TotalProgress{
		FinishNum:   0,
		TaskNum:     taskNum,
		AllProgress: make([]*GoroutineProgress, goroutineNum),
	}
}

// GoroutineProgress 每个goroutine的进度
type GoroutineProgress struct {
	GoroutineId int  // goroutine的ID
	*Task            // 正在执行的任务
	isIdle      bool // 当前线程是否空闲
}

// Exec 执行任务
func (goProgress *GoroutineProgress) Exec() {
	goProgress.isIdle = false
	goProgress.Run(goProgress.TaskProgress) // 执行任务
	goProgress.isIdle = true
}

// Task 每个goroutine所执行的任务
type Task struct {
	TaskProgress *TaskProgress       // 任务的进度取值范围[1-100]
	Run          func(*TaskProgress) // 执行
}

func (t Task) GetProgress() int {
	return t.TaskProgress.CurrentProgress
}

func NewTask(f func(*TaskProgress)) *Task {
	return &Task{
		TaskProgress: &TaskProgress{CurrentProgress: 0},
		Run:          f,
	}
}

// TaskProgress 任务的进度
type TaskProgress struct {
	CurrentProgress int // 任务进度单位%
}

// Exec 线程池开始执行任务
func (pool *FixedThreadPool) Exec() {
	// 将所有任务放入到channel中
	taskChannel := make(chan *Task, pool.TaskNum)
	for _, task := range pool.Tasks {
		taskChannel <- task
	}
	// 同步主线程与创建的goroutine
	wg := &sync.WaitGroup{}
	wg.Add(pool.TaskNum)
	// 通知线程关闭的channel,+1是为了关闭打印进度的线程
	done := make(chan struct{}, pool.GoroutineNum+1)

	// 开启goroutine线程
	for i := 0; i < pool.GoroutineNum; i++ {
		// 监听每个线程的进度
		progress := &GoroutineProgress{
			GoroutineId: i,
		}
		pool.TotalProgress.AllProgress[i] = progress

		// 开启线程
		go func(done chan struct{}, wg *sync.WaitGroup, progress *GoroutineProgress, totalProgress *TotalProgress) {
		Label:
			for {
				select {
				case progress.Task = <-taskChannel:
					progress.Exec()
					wg.Done()
					totalProgress.FinishNum++
				case <-done:
					// 结束
					break Label
				}
			}
		}(done, wg, progress, pool.TotalProgress)
	}

	if pool.IsVisible {
		// 单独开启一个监听正在执行的任务的进度的goroutine
		go func(done chan struct{}, totalProgress *TotalProgress) {
		Label:
			for {
				select {
				case <-done:
					break Label
				default:
					// 输出任务的进度
					fmt.Printf("当前进度: %d/%d\n", totalProgress.FinishNum, totalProgress.TaskNum)
					for _, progress := range totalProgress.AllProgress {
						if !progress.isIdle && progress.Task != nil {
							fmt.Printf("任务进度: [%s%s] %d%%\n", strings.Repeat("=", progress.GetProgress()), strings.Repeat(" ", 100-progress.GetProgress()), progress.GetProgress())
						}
					}
					// 间隔1s
					time.Sleep(time.Second)
					// 清屏
					fmt.Println("\033c")
				}
			}
		}(done, pool.TotalProgress)
	}

	// 等待所有任务执行结束
	wg.Wait()
	for i := 0; i <= pool.GoroutineNum; i++ {
		done <- struct{}{}
	}
	log.Println("所有任务执行完毕")
}
