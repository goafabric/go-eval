package logic

import "callee-service/controller/dto"

func SayMyName(name string) dto.Callee {
	return dto.Callee{ID: "0", Message: "Your name is: " + name}
}

func SayMyOtherName(name string) dto.Callee {
	return dto.Callee{ID: "0", Message: "Your other name is: " + name}
}

func Save(callee dto.Callee) dto.Callee {
	return dto.Callee{ID: "0", Message: "Storing your message: " + callee.Message}
}