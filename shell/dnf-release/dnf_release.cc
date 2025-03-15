#include <cstdlib>

// DNF Release System Upgrade
int main() {
  system("sudo dnf -y system-upgrade download --releasever=41");

  return 0;
}

