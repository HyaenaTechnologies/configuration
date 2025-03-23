# APT Upgrade
.PHONY: apt-upgrade
apt-update:
	sudo apt update && sudo apt -y full-upgrade
	
# DNF Release System Upgrade
.PHONY: dnf-release
dnf-release:
	sudo dnf -y system-upgrade download --releasever=41

# DNF Upgrade
.PHONY: dnf-uprade
dnf-upgrade:
	sudo dnf -y upgrade

# Snap Refresh
.PHONY: snap-refresh
snap-refresh:
	sudo snap refresh

# Ubuntu Release System Upgrade
.PHONY: ubuntu-release
ubuntu-release:
	sudo do-release-upgrade

