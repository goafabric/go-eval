package main

func main() {
	hello := "Hello, World2!"
	println(hello)
	test()
}

func test() {
	//explicit declaration
	var i int

	i = add(47, 11)

	println(i)

	//implicit declaration with :=, type inference via the result
	j := add(20, 1)
	k := i - j

	if k > 30 {
		println(k)
	} else {
		println("to low")
	}

	message := "yo"
	message = "this works"

	println(message)
}

func add(i int, y int) int {
	return i + y
}
