package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	var countBytes bool
	flag.BoolVar(&countBytes, "c", false, "print the byte counts")
	flag.Parse()
	args := flag.Args()
	if len(args) == 0 {
		fmt.Println("no provided file")
		os.Exit(1)
	}

	file, err := os.Open(args[0])
	defer file.Close()
	if err != nil {
		fmt.Println("file not found")
	}

	if countBytes {
		fileInfo, err := file.Stat()
		if err != nil {
			fmt.Println("can't get file info")
		}
		size := fileInfo.Size()
		fmt.Println(size, file.Name())
	}

}
