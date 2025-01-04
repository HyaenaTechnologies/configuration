package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
)

func main() {
	var dnfRelease *exec.Cmd = exec.Command(
		"sudo",
		"dnf",
		"system-upgrade",
		"download",
		"--releasever=41",
	)

	var dnfReleaseError error = dnfRelease.Run()

	if dnfReleaseError != nil {
		log.Fatalln("DNF Release Download Failed: \n", dnfReleaseError)
	}

	fmt.Println("Running DNF Release Download: \n", dnfRelease)

	os.Exit(0)
}
