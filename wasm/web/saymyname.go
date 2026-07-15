package main

import "syscall/js"

func sayMyName(name string) string {
	return "My name is: " + name
}

func add(x, y int) int {
	return x + y
}

func main() {
	js.Global().Set("sayMyName", js.FuncOf(func(this js.Value, args []js.Value) any {
		name := args[0].String()
		return sayMyName(name)
	}))

	js.Global().Set("add", js.FuncOf(func(this js.Value, args []js.Value) any {
		x := args[0].Int()
		y := args[1].Int()
		return add(x, y)
	}))

	// Keep the Go runtime alive
	select {}
}
