Duplicate a file directly from the GPSE repository, run:
Copy-Item "src\archive\bible\book.rs" "templates\bible\book.rs.template"

To duplicate a whole folder and all its content, use:
Copy-Item "src\archive\bible" "templates\bible" -Recurse