package models

type Struct struct {
	Name      string
	Functions []Function
}

func (s *Struct) AppendMethod(m Function) {
	s.Functions = append(s.Functions, m)
}
