module parser
  implicit none

  !> Parse Command Line Arguments
  function parse_arguments() result(value)
    integer :: value

    value = 0
  end function parser_arguments

end module parser

