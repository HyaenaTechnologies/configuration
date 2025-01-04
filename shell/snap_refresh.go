package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

func main() {
	var snapRefresh *exec.Cmd = exec.Command(
		"sudo",
		"snap",
		"refresh",
	)

	var snapRefreshOutput strings.Builder

	snapRefresh.Stdout = &snapRefreshOutput

	var snapRefreshError error = snapRefresh.Run()

	if snapRefreshError != nil {
		log.Fatalln("Snap Refresh Failed: \n", snapRefreshError)
	} else {
		fmt.Println("Executed Command: \n", snapRefresh)
		fmt.Println("Snap Refresh Output: \n", snapRefreshOutput.String())

		os.Exit(0)
	}
}
