Output docs/demo/transfer.gif
Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 300
Set Padding 24
Set Theme "Catppuccin Mocha"
Set TypingSpeed 40ms
Hide
Type `export PS1='$ ' PATH="$PWD/target/release:$PATH"` Enter
Type `mkdir -p /tmp/ds-demo/my_folder && cd /tmp/ds-demo && rm -rf received_files && echo hello > my_folder/hello.txt` Enter
Type "clear" Enter
Show
Type `deltasafe server --address 127.0.0.1:12345 --password "MySecret123" &`
Sleep 300ms
Enter
Sleep 1s
Type `deltasafe sync --source my_folder --target 127.0.0.1:12345 --password "MySecret123"`
Sleep 300ms
Enter
Sleep 2s
Type `cmp my_folder/hello.txt received_files/hello.txt && echo identical`
Sleep 300ms
Enter
Sleep 2s
Hide
Type `kill %1` Enter
Sleep 500ms
