package main

import (
	"fmt"
	"net/http"
	"time"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(5 * time.Second)

		fmt.Fprintf(w, "Hello!")
	})

	println("Started on port :9000")
	http.ListenAndServe(":9000", nil)
}
