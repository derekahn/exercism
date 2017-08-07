package circular

import (
	"container/ring"
	"errors"
)

const testVersion = 4

type Buffer struct {
	start, end *ring.Ring
}

func NewBuffer(size int) *Buffer {
	r := ring.New(size + 1)
	return &Buffer{r, r}
}

func (b *Buffer) ReadByte() (byte, error) {
	if b.start == b.end {
		return 0, errors.New("buffer empty")
	}
	c := b.start.Value.(byte)
	b.start = b.start.Next()
	return c, nil
}

func (b *Buffer) WriteByte(c byte) error {
	if b.end.Next() == b.start {
		return errors.New("buffer full")
	}
	b.end.Value = c
	b.end = b.end.Next()
	return nil
}

func (b *Buffer) Overwrite(c byte) {
	b.end.Value = c
	b.end = b.end.Next()
	if b.end == b.start {
		b.start = b.start.Next()
	}
}

func (b *Buffer) Reset() {
	b.start = b.end
}
