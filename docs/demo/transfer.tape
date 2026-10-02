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
Type `clear; printf '$ %s\n' 'deltasafe server --address 127.0.0.1:12345 --password "MySecret123" & sleep 1'; deltasafe server --address 127.0.0.1:12345 --password "MySecret123" & sleep 1; printf '$ %s\n' 'deltasafe sync --source my_folder --target 127.0.0.1:12345 --password "MySecret123"'; deltasafe sync --source my_folder --target 127.0.0.1:12345 --password "MySecret123"; sleep 1; printf '$ %s\n' 'cmp my_folder/hello.txt received_files/hello.txt && echo identical'; cmp my_folder/hello.txt received_files/hello.txt && echo identical` Enter
Sleep 4s
Show
Sleep 3s
Hide
Type `kill %1; wait; rm -rf received_files; clear` Enter
Sleep 1s
Show
Type `deltasafe server --address 127.0.0.1:12345 --password "MySecret123" & sleep 1`
Sleep 300ms
Enter
Sleep 2s
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
