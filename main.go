package main

import (
    "bufio"
    "flag"
    "fmt"
    "log"
    "os"
)

func main() {

    var fileName = flag.String("i", "icons.txt", "input file name")
    flag.Parse()

    file, err := os.Open(*fileName)
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        fmt.Println(scanner.Text())
    }

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
}
