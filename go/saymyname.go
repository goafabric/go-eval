package main

func main() {
    var name = "Slim Shady"
    println(sayMyName(name))
}

func sayMyName(name string) string {
    return "My name is: " + name
}

//export sayMyName
func sayMyNameByte() *byte {
    return &(([]byte)(sayMyName("Slim Shady"))[0])
}

//export add
func add(x int, y int) int {
    return x + y
}
