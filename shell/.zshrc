#
# .zshrc is sourced in interactive shells.
# It should contain commands to set up aliases,
# functions, options, key bindings, etc.
#

# .zhrc

# User specific environment
export PATH=$PATH:/usr/bin
export PATH=$PATH:/usr/bin/arduino
export PATH=$PATH:/usr/bin/c3
export PATH=$PATH:/usr/bin/go/bin
export PATH=$PATH:/usr/bin/go/pkg/tool/bin
export PATH=$PATH:/usr/bin/helix
export PATH=$PATH:/usr/bin/hyaena-technologies
export PATH=$PATH:/usr/bin/nvim/bin
export PATH=$PATH:/usr/bin/vulkan/x86_64/bin
export PATH=$PATH:/usr/bin/zig
export PATH=$PATH:/usr/include
export PATH=$PATH:/usr/lib
export PATH=$PATH:/usr/local/bin
export PATH=$PATH:/usr/local/include
export PATH=$PATH:/usr/local/lib
export PATH=$PATH:~/.cargo/bin
export PATH=$PATH:~/arduino-ide
export PATH=$PATH:~/blender
export PATH=$PATH:~/grafana/bin
export PATH=$PATH:~/podman-desktop
export PATH=$PATH:~/VSCode
export PATH=$PATH:~/zen

autoload -U compinit
compinit

#allow tab completion in the middle of a word
setopt COMPLETE_IN_WORD

## keep background processes at full speed
#setopt NOBGNICE
## restart running processes on exit
#setopt HUP

## history
#setopt APPEND_HISTORY
## for sharing history between zsh processes
#setopt INC_APPEND_HISTORY
#setopt SHARE_HISTORY

## never ever beep ever
#setopt NO_BEEP

## automatically decide when to page a list of completions
#LISTMAX=0

## disable mail checking
#MAILCHECK=0

# autoload -U colors
#colors
