package main

type Callee struct {
    ID      string `json:"id"`
	Message string `json:"message"`
}

type Health struct {
	Status string `json:"status"`
}