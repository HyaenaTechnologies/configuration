package update

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

// APT Update
func APTUpdate() {
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

	var aptUpdateOutput strings.Builder

	aptUpdate.Stdout = &aptUpdateOutput

	var aptUpdateError error = aptUpdate.Run()

	if aptUpdateError != nil {
		log.Fatalln("APT Update Failed: \n", aptUpdateError)
	} else {
		fmt.Println("Executed Command: \n", aptUpdate)
		fmt.Println("APT Update Output: \n", aptUpdateOutput.String())

		os.Exit(0)
	}
}
