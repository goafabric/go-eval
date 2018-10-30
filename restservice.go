package main

import "net/http"

func main() {
	http.HandleFunc("/catalog/products", productsHandler)
	http.ListenAndServe(":8080", nil)
}

func productsHandler(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Schuhe, Hose, Hemd"))
}
