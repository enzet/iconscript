package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.ListenAndServe(":9090", http.FileServer(http.Dir("../../assets")))
}
