package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
)

func main() {
	var aptUpdate *exec.Cmd = exec.Command(
		"sudo",
		"apt",
		"update",
		"&&",
		"sudo",
		"apt",
		"-y",
		"full-upgrade",
	)

	var aptUpdateError error = aptUpdate.Run()

	if aptUpdateError != nil {
		log.Fatalln("APT Update Failed: \n", aptUpdateError)
	}

	fmt.Println("Running APT Update: \n", aptUpdate)

	os.Exit(0)
}
