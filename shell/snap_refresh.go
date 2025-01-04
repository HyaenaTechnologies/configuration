package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
)

func main() {
	var snapRefresh *exec.Cmd = exec.Command(
		"sudo",
		"snap",
		"refresh",
	)

	var snapRefreshError error = snapRefresh.Run()

	if snapRefreshError != nil {
		log.Fatalln("Snap Refresh Failed: \n", snapRefreshError)
	}

	fmt.Println("Running Snap Refresh: \n", snapRefresh)

	os.Exit(0)
}
