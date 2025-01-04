package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

func main() {
	var dnfRelease *exec.Cmd = exec.Command(
		"sudo",
		"dnf",
		"system-upgrade",
		"download",
		"--releasever=41",
	)

	var dnfReleaseOutput strings.Builder

	dnfRelease.Stdout = &dnfReleaseOutput

	var dnfReleaseError error = dnfRelease.Run()

	if dnfReleaseError != nil {
		log.Fatalln("DNF Release Download Failed: \n", dnfReleaseError)
	} else {
		fmt.Println("Executed Command: \n", dnfRelease)
		fmt.Println("Running DNF Release Download: \n", dnfReleaseOutput.String())

		os.Exit(0)
	}
}
