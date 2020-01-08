package main

import (
	"flag"
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"strings"
	"time"
)

type flags struct {
	fileOutput     *bool
	fileName       *string
	quantity       *int
	scrambleLength *int
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func generateScramble(source []string, size int) (scramble []string) {
	scramble = make([]string, size)

	numberSource := rand.NewSource(time.Now().UnixNano())
	gen := rand.New(numberSource)

	for i := 0; i < size; i++ {
		scrambleMove := generateMove(source, gen)
		var mostRecentMove string
		if i > 0 {
			mostRecentMove = scramble[i-1]
		} else {
			mostRecentMove = scramble[0]
		}
		for isRedundantMove(mostRecentMove, scrambleMove, i) == true {
			scrambleMove = generateMove(source, gen)
		}
		scramble[i] = scrambleMove
	}
	return scramble
}

func generateMove(moveSource []string, gen *rand.Rand) (scrambleMove string) {
	randIndex := gen.Intn(cap(moveSource))
	scrambleMove = moveSource[randIndex]
	return scrambleMove
}

func isRedundantMove(mostRecentMove string, generatedMove string, pos int) bool {
	// Check if new move is the prime of the last move
	if strings.Contains(generatedMove, "'") == true {
		primeRedundantCheck := strings.Trim(generatedMove, "'")
		if mostRecentMove == primeRedundantCheck {
			return true
		}
	}
	// Check if the last generated move is the prime of the most recently generated move
	antiPrimRedundantCheck := strings.Trim(mostRecentMove, "'")
	if generatedMove == antiPrimRedundantCheck {
		return true
	}
	// Check if the most recent move is the *2 of the last move
	doubleAfterSingleCheck := mostRecentMove + "2"
	if doubleAfterSingleCheck == generatedMove {
		return true
	}
	// Check the reverse of the above condition
	if strings.Contains(mostRecentMove, "2") {
		singleAfterDoubleCheck := generatedMove + "2"
		if singleAfterDoubleCheck == mostRecentMove {
			return true
		}
	}

	// Check if the most recent move is the same as the last move
	if generatedMove == mostRecentMove {
		return true
	}

	return false
}

func printableScramble(scramble []string) (printableScramble string) {
	printableScramble = strings.Join(scramble, " ")
	printableScramble += "\n"
	return printableScramble
}

func initFlags() (flagContainer flags) {
	flagContainer = flags{
		fileOutput:     flag.Bool("file_output", false, "Boolean, determines if program outputs to a file"),
		fileName:       flag.String("file_name", "scramble_", "If specified, is filename that scrambles are outputted to"),
		quantity:       flag.Int("quantity", 20, "Number of scrambles to generate"),
		scrambleLength: flag.Int("length", 20, "Number of moves per scramble")}
	return flagContainer
}

func main() {
	// Init move source and command line flags
	moves := []string{"R", "L", "U", "D", "F", "B", "R2", "L2", "U2", "D2", "F2", "B2", "R'", "L'", "U'", "D'", "F'", "B'"}
	flagContainer := initFlags()
	flag.Parse()
	flag.Usage = func() {
		fmt.Fprintf(flag.CommandLine.Output(), "Usage of %s:\n", os.Args[0])
		flag.PrintDefaults()
	}
	// If program is set to output to a file
	if *flagContainer.fileOutput == true {
		fmt.Println("Working")
		currentDir, err := os.Getwd()
		check(err)
		// If a file name has been chosen
		if *flagContainer.fileName != "scramble_" {
			fileName := currentDir + "/" + *flagContainer.fileName
			file, err := os.Create(fileName)
			check(err)
			defer file.Close()
			scrambleContainer := make([]string, *flagContainer.quantity)
			for i := 0; i < cap(scrambleContainer); i++ {
				newScramble := generateScramble(moves, *flagContainer.scrambleLength)
				scrambleContainer[i] = printableScramble(newScramble)
			}
			file.WriteString("BEGIN SCRAMBLES\n")
			for i := 0; i < cap(scrambleContainer); i++ {
				file.WriteString(scrambleContainer[i])
			}
			file.WriteString("END SCRAMBLES")
			fmt.Println("Success")
		}
		// If a file name was not provided
	 	if *flagContainer.fileName == "scramble_" {
			numberSource := rand.NewSource(time.Now().UnixNano())
			gen := rand.New(numberSource)
			GUID := gen.Intn(999999)
			fileName := currentDir + "/" + *flagContainer.fileName + strconv.Itoa(int(GUID))
			file, err := os.Create(fileName)
			check(err)
			defer file.Close()
			scrambleContainer := make([]string, *flagContainer.quantity)
			for i := 0; i < cap(scrambleContainer); i++ {
				newScramble := generateScramble(moves, *flagContainer.scrambleLength)
				scrambleContainer[i] = printableScramble(newScramble)
			}
			file.WriteString("BEGIN SCRAMBLES\n")
			for i := 0; i < cap(scrambleContainer); i++ {
				file.WriteString(scrambleContainer[i])
			}
			file.WriteString("END SCRAMBLES")
			fmt.Println("Success")
		}
	}
	// If file output is not specified, just print to stdout
	if *flagContainer.fileOutput == false {
		scrambleContainer := make([]string, *flagContainer.quantity)
		for i := 0; i < cap(scrambleContainer); i++ {
			newScramble := generateScramble(moves, *flagContainer.scrambleLength)
			scrambleContainer[i] = printableScramble(newScramble)
		}
		fmt.Println("BEGIN SCRAMBLES")
		for i := 0; i < cap(scrambleContainer); i++ {
			fmt.Println(scrambleContainer[i])
		}
		fmt.Println("END SCRAMBLES")
	}
}
