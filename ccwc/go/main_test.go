package main

import (
	"bytes"
	"testing"
)

func TestCountBytes(t *testing.T) {
	output := bytes.Buffer{}
	printFileStats("../test.txt", statsOptions{bytes: true}, &output)

	got := output.String()
	want := "342190 ../test.txt"

	if got != want {
		t.Errorf("got %s, wanted %s", got, want)
	}
}

func TestCountLines(t *testing.T) {
	output := bytes.Buffer{}
	printFileStats("../test.txt", statsOptions{lines: true}, &output)

	got := output.String()
	want := "7145 ../test.txt"

	if got != want {
		t.Errorf("got %s, wanted %s", got, want)
	}
}

func TestCountWords(t *testing.T) {
	output := bytes.Buffer{}
	printFileStats("../test.txt", statsOptions{words: true}, &output)

	got := output.String()
	want := "58164 ../test.txt"

	if got != want {
		t.Errorf("got %s, wanted %s", got, want)
	}
}

func TestCountSymbols(t *testing.T) {
	output := bytes.Buffer{}
	printFileStats("../test.txt", statsOptions{symbols: true}, &output)

	got := output.String()
	want := "339292 ../test.txt"

	if got != want {
		t.Errorf("got %s, wanted %s", got, want)
	}
}

func TestNoFlags(t *testing.T) {
	output := bytes.Buffer{}
	printFileStats("../test.txt", statsOptions{}, &output)

	got := output.String()
	want := "7145 58164 342190 ../test.txt"

	if got != want {
		t.Errorf("got %s, wanted %s", got, want)
	}
}
