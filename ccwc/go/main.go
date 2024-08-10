package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

type dataSource struct {
	fileName string
	reader   io.Reader
}

func (ds *dataSource) close() {
	if ds.fileName != "" {
		file, ok := ds.reader.(*os.File)
		if ok {
			file.Close()
		}
	}
}

type file struct {
	stats  stats
	source dataSource
}

func (f *file) calcStats() {
	f.stats.calcStats(f.source.reader)
}

func (f *file) printStats(output io.Writer) {
	var result []string
	if f.stats.statsOptions.lines {
		result = append(result, strconv.Itoa(f.stats.lines))
	}
	if f.stats.statsOptions.words {
		result = append(result, strconv.Itoa(f.stats.words))
	}
	if f.stats.statsOptions.bytes {
		result = append(result, strconv.Itoa(f.stats.bytes))
	}
	if f.stats.statsOptions.symbols {
		result = append(result, strconv.Itoa(f.stats.symbols))
	}

	result = append(result, f.source.fileName)

	fmt.Fprintf(output, "%s", strings.Join(result, " "))
}

type stats struct {
	bytes        int
	lines        int
	words        int
	symbols      int
	statsOptions statsOptions
}

func (s *stats) calcStats(reader io.Reader) {
	s.statsOptions.reconcile()

	scanner := bufio.NewScanner(reader)
	scanner.Split(bufio.ScanRunes)

	inWord := false
	var wordEnded int

	for scanner.Scan() {
		symbol := scanner.Text()

		if s.statsOptions.bytes {
			s.bytes += numBytesIn(symbol)
		}

		if s.statsOptions.lines {
			s.lines += isEndOfLine(symbol)
		}

		if s.statsOptions.words {
			wordEnded, inWord = isEndOfWord(symbol, inWord)
			s.words += wordEnded
		}

		if s.statsOptions.symbols {
			s.symbols += 1
		}

	}
}

type statsOptions struct {
	bytes   bool
	lines   bool
	words   bool
	symbols bool
}

func (sc *statsOptions) reconcile() {
	if !sc.bytes && !sc.lines && !sc.words && !sc.symbols {
		sc.bytes, sc.lines, sc.words = true, true, true
	}
}

func numBytesIn(symbol string) int {
	return len([]byte(symbol))
}

func isEndOfLine(symbol string) int {
	if symbol == "\n" {
		return 1
	}
	return 0
}

func isEndOfWord(symbol string, isWordStarted bool) (int, bool) {
	var isWordEnd int
	var inWord bool

	if !isASCIIWhitespace(symbol) {
		isWordEnd, inWord = 0, true
	}
	if isASCIIWhitespace(symbol) && isWordStarted {
		isWordEnd, inWord = 1, false
	}
	return isWordEnd, inWord
}

func isASCIIWhitespace(symbol string) bool {
	var result bool
	switch symbol {
	case "\t", "\n", "\x0C", "\r", " ":
		result = true
	default:
		result = false
	}
	return result
}

func getSource(fileName string) dataSource {
	if fileName == "" {
		return dataSource{fileName: fileName, reader: os.Stdin}
	}
	file, err := os.Open(fileName)
	if err != nil {
		panic("file not found")
		os.Exit(0)
	}
	return dataSource{fileName: fileName, reader: file}
}

func printFileStats(fileName string, statsOptions statsOptions, output io.Writer) {
	source := getSource(fileName)
	defer source.close()
	file := file{stats: stats{statsOptions: statsOptions}, source: source}
	file.calcStats()
	file.printStats(output)
}

func main() {
	statsOptions := statsOptions{}
	flag.BoolVar(&statsOptions.bytes, "c", false, "print the byte counts")
	flag.BoolVar(&statsOptions.lines, "l", false, "print the line counts")
	flag.BoolVar(&statsOptions.words, "w", false, "print the words counts")
	flag.BoolVar(&statsOptions.symbols, "m", false, "print the symbols counts")
	flag.Parse()

	args := flag.Args()
	var fileName string
	if len(args) == 0 {
		fileName = ""
	} else {
		fileName = args[0]
	}

	printFileStats(fileName, statsOptions, os.Stdout)
}
