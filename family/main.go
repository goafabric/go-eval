package main

import (
	"family/father"
	"family/father/son"

	"fmt"
)

func main() {
	f := new(father.Father)
	fmt.Println(f.Data("Mr. Jeremy Maclin"))

	c := new(son.Son)
	fmt.Println(c.Data("Riley Maclin"))
}