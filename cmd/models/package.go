package models

type Package struct {
	Structs   map[string]*Struct
	Functions []Function
}

func NewPackage() *Package {
	return &Package{
		Structs: make(map[string]*Struct),
	}
}

func (c *Package) AppendFunction(m Function) {
	c.Functions = append(c.Functions, m)
}

func (conv *Package) Struct(name string) *Struct {
	s := conv.Structs[name]
	if s == nil {
		s = new(Struct)
		s.Name = name
		conv.Structs[name] = s
	}
	return s
}
