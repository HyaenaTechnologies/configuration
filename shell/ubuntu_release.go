package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
)

func main() {
	var ubuntuRelease *exec.Cmd = exec.Command(
		"sudo",
		"do-release-upgrade",
	)

	var ubuntuReleaseError error = ubuntuRelease.Run()

	if ubuntuReleaseError != nil {
		log.Fatalln("Ubuntu Release Upgrade Failed: \n", ubuntuReleaseError)
	}

	fmt.Println("Running Ubuntu Release Upgrade: \n", ubuntuRelease)

	os.Exit(0)
}
