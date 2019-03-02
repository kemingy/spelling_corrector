package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"regexp"
	"time"
)

var ALPHABET = []byte("abcdefghijklmnopqrstuvwxyz")

type Dictionary struct {
	Words map[string]uint32
}

func build(file string) *Dictionary {
	data, err := ioutil.ReadFile(file)
	if err != nil {
		fmt.Println("Failed when reading file. ", err)
	}

	regex, _ := regexp.Compile(`\w+`)
	words := make(map[string]uint32)
	for _, word := range regex.FindAllString(string(data), -1) {
		words[word]++
	}
	return &Dictionary{Words: words}
}

func (d *Dictionary) known(words []string) []string {
	candidates := words[:0]
	for _, word := range words {
		if _, ok := d.Words[word]; ok {
			candidates = append(candidates, word)
		}
	}
	return candidates
}

func (d *Dictionary) correct(word string) string {
	if _, ok := d.Words[word]; ok {
		return word
	}
	var correction string
	var freq uint32
	for _, cand := range d.candidate(word) {
		if f, _ := d.Words[cand]; f > freq {
			freq = f
			correction = cand
		}
	}
	if len(correction) == 0 {
		return word
	}
	return correction
}

func (d *Dictionary) candidate(word string) []string {
	if cand := d.known(edit(word)); len(cand) > 0 {
		return cand
	} else if cand := d.known(edit_twice(word)); len(cand) > 0 {
		return cand
	} else {
		return []string{word}
	}
}

type Pair struct {
	left, right string
}

func edit_twice(word string) []string {
	var words []string
	for _, candidate := range edit(word) {
		words = append(words, edit(candidate)...)
	}
	return words
}

func edit(word string) []string {
	var words []string
	var buf bytes.Buffer
	var splits []Pair

	for i := 0; i <= len(word); i++ {
		splits = append(splits, Pair{left: word[:i], right: word[i:]})
	}

	for _, p := range splits {
		// insert
		for _, c := range ALPHABET {
			buf.Reset()
			buf.WriteString(p.left)
			buf.WriteByte(c)
			buf.WriteString(p.right)
			words = append(words, buf.String())
		}

		if len(p.right) == 0 {
			continue
		}

		// delete
		buf.Reset()
		buf.WriteString(p.left)
		buf.WriteString(p.right[1:])
		words = append(words, buf.String())

		// replace
		for _, c := range ALPHABET {
			buf.Reset()
			buf.WriteString(p.left)
			buf.WriteByte(c)
			buf.WriteString(p.right[1:])
			words = append(words, buf.String())
		}

		if len(p.right) <= 1 {
			continue
		}
		// transpose
		buf.Reset()
		buf.WriteString(p.left)
		buf.WriteByte(p.right[1])
		buf.WriteByte(p.right[0])
		buf.WriteString(p.right[2:])
		words = append(words, buf.String())
	}

	return words
}

func main() {
	var dic Dictionary = *build("./sherlock.txt")
	fmt.Println("Numbers of words: ", len(dic.Words))

	for _, word := range []string{"helle", "world", "pythn", "nica", "dictionere"} {
		t0 := time.Now()
		fmt.Printf("%v => %v\t", word, dic.correct(word))
		t1 := time.Now()
		fmt.Printf("%v\n", t1.Sub(t0))
	}
}
