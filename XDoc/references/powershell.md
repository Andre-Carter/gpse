Duplicate a file directly from the GPSE repository, run:
Copy-Item "src\archive\bible\book.rs" "templates\bible\book.rs.template"

To duplicate a whole folder and all its content, use:
Copy-Item "src\archive\bible" "templates\bible" -Recurse

Another Example:
Copy-Item -Path "C:\path\to\left\folder\*" -Destination "D:\path\to\right\folder" -Recurve

Working Example
Copy-Item `
>>     -Path "vault\templates\bible\bible.rs" `
>>     -Destination "src\archive\bible\nlt\mod.rs"

Copying: your powershell.md example is right. Copy-Item "vault\templates\bible\bible.rs" "src\archive\bible\nlt\mod.rs" works for a single file. If the template has more than one file, use -Recurse on the folder.

// Directory Tree

tree /F src, or tree /F XDoc

File types: 
scripts     - .ps1
modules     - .psm1
data files  - .psd1