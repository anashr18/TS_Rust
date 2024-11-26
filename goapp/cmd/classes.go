package main

import "fmt"

type Cookie struct {
	color string
}

func newCookie(color string) *Cookie {
	return &Cookie{color: color}
}
func (c *Cookie) GetColor() string {
	return c.color
}
func (c *Cookie) SetColor(newColor string) {
	c.color = newColor
}

func main() {
	c1 := newCookie("black")
	c2 := newCookie("white")

	fmt.Println(c1.GetColor())
	fmt.Println(c2.GetColor())
	c2.SetColor("Green")
	fmt.Println(c2.GetColor())

}
