# Trusty Timer

A simple pomodoro timer which supports blocking of distracting websites via the hosts file present on windows and unix (not yet supported) systems. Built using Rust and the [OrbTk UI toolkit](https://github.com/redox-os/orbtk)!
<br><br>
## Windows Setup
<hr>
<br>
Note: currently, this app only supports Windows systems. 

### User Install
A distributable version of the app can be built using the following command in the project root:

 `make build`

 The result is a folder called dist, also in the project root. This folder can be installed to the Start Menu for easy access using the included `install.bat` script. This will ask for admin permissions in order to add the software to the Start Menu. Alternatively, feel free to copy the dist folder manually (renaming the folder to "Trusty Timer" so the shortcut works properly).    
 
 The software requires administrator permissions when running because these permissions are necessary to change the `hosts` file and block websites across your entire computer.

### Dev Install

When working on development on Windows, if you do nothing special, the software will not work, as it requires admin permissions (automatically requested by the software only once compiled for release). You use one of the following tricks to make things work:

1. Use cargo/run the software from a terminal with admin permissions
2. Change the permissions on your hosts file so your account is allowed to edit them
3. Use the included elevate.exe before each command (like sudo on unix, but more annoying).