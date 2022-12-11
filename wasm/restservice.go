package main

import "net/http"

func main() {
    println("starting to serve you on http://localhost:8080/catalog/products")
	http.HandleFunc("/catalog/products", productsHandler)
	http.ListenAndServe(":8080", nil)
}

func productsHandler(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Schuhe, Hose, Hemd"))
}
