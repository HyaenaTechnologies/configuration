package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
)

func main() {
	var dnfUpdate *exec.Cmd = exec.Command(
		"sudo",
		"dnf",
		"-y",
		"upgrade",
	)

	var dnfUpdateError error = dnfUpdate.Run()

	if dnfUpdateError != nil {
		log.Fatalln("DNF Update Failed: \n", dnfUpdateError)
	}

	fmt.Println("Running DNF Update: \n", dnfUpdate)

	os.Exit(0)
}
