package callee

func SayMyName(name string) Callee {
	return Callee{ID: "0", Message: "Your name is: " + name}
}

func SayMyOtherName(name string) Callee {
	return Callee{ID: "0", Message: "Your other name is: " + name}
}

func Save(callee Callee) Callee {
	return Callee{ID: "0", Message: "Storing your message: " + callee.Message}
}
