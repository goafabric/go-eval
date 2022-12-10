package main

func main() {
    var name = "Slim Shady"
    println(sayMyName(name))
}


func sayMyName(name string) string {
    return "My name is: " + name
}