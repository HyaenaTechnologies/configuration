package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

func main() {
	var ubuntuRelease *exec.Cmd = exec.Command(
		"sudo",
		"do-release-upgrade",
	)

	var ubuntuReleaseOutput strings.Builder

	ubuntuRelease.Stdout = &ubuntuReleaseOutput

	var ubuntuReleaseError error = ubuntuRelease.Run()

	if ubuntuReleaseError != nil {
		log.Fatalln("Ubuntu Release Upgrade Failed: \n", ubuntuReleaseError)
	} else {
		fmt.Println("Executed Command: \n", ubuntuRelease)
		fmt.Println("Ubuntu Release Upgrade Output: \n", ubuntuReleaseOutput.String())

		os.Exit(0)
	}
}
