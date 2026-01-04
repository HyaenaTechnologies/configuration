module version_message
  implicit none

  !> Print Version Message
  function print_version() result(value)
    integer :: value
    character(len=5) :: version_number

    version_number = "1.0.0"
    
    print "(A)", "\x1b[32;1;3;4mNull Log Tool\x1b[0m"
    print "(A)", ""
    print "(A)", "\x1b[32;1;3mVersion Number:\x1b[0m\t", version_number

    value = 0
  end function print_version

end module version_message

