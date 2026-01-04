module help_message
  implicit none

  !> Print Help Message
  function print_help() result(value)
    integer :: value
    
    print "(A)", "\t\x1b[32;1;3;4mGitHub Synchronize\x1b[0m\n\n"//
                &"\x1b[32;1;3mCommands:\tDescription:\x1b[0m\n\n"//
                &"\x1b[32;3musers/GitHubUser/repos\x1b[0m\t\tInput GitHub User\n"//
                &"\x1b[32;3mhelp\x1b[0m\t\tPrint Commands and Flags\n"//
                &"\x1b[32;3mversion\x1b[0m\t\tPrint Version Number\n"//
                &"\x1b[32;1;3mFlags:\t\tDescription:\x1b[0m\n\n"//
                &"\x1b[32;3m--h\x1b[0m\t\tPrint Commands and Flags\n"//
                &"\x1b[32;3m--v\x1b[0m\t\tPrint Version Number\n"
    value = 0
  end function print_help

end module help_message

