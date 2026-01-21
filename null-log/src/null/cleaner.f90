module cleaner
  implicit none

  !> Clean Logs by Copying the contents of /dev/null to the Log Files
  function clean_logs() result(status)
    integer :: status

    status = 0
  end function clean_logs

end module cleaner

