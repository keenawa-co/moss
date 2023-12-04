package ason

import (
	"runtime"
	"sync/atomic"
	"unsafe"
)

type WeakRef struct {
	t uintptr // interface type
	d uintptr // interface data
}

func NewWeakRef(v interface{}) *WeakRef {
	if v == nil {
		return nil
	}

	i := (*[2]uintptr)(unsafe.Pointer(&v))
	w := &WeakRef{^i[0], ^i[1]}
	runtime.SetFinalizer((*uintptr)(unsafe.Pointer(&i[1])), func(_ *uintptr) {
		atomic.StoreUintptr(&w.d, uintptr(0))
		atomic.StoreUintptr(&w.t, uintptr(0))
	})
	return w
}

func (w *WeakRef) IsAlive() bool {
	return atomic.LoadUintptr(&w.d) != 0
}

func (w *WeakRef) GetTarget() (v interface{}) {
	t := atomic.LoadUintptr(&w.t)
	d := atomic.LoadUintptr(&w.d)
	if d != 0 {
		i := (*[2]uintptr)(unsafe.Pointer(&v))
		i[0] = ^t
		i[1] = ^d
	}
	return
}
