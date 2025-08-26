
## killw for Windows

This is a simple command-line utility for Windows that mimics some of the functionality of the Linux `kill` command. It is written in Rust and uses the Windows API directly.

### Usage

The command-line syntax is as follows:

```sh
killw [-SIGNAL] <PID>
```

-   `<PID>`: The process ID of the process you want to interact with. This is a required argument.
-   `[-SIGNAL]`: An optional signal that determines the action to be taken. If no signal is specified, it defaults to `-TERM`.

### Supported Signals

-   `-TERM` (Default): This signal terminates the specified process.
    ```sh
    # These two commands are equivalent
    killw 1234
    killw -TERM 1234
    ```
-   `-0`: This doesn't send a signal but checks for the existence of the process.
    -   If the process with the specified PID exists, the command exits with a status code of `0`.
    -   If the process does not exist or cannot be accessed, it prints an error and exits with a status code of `1`.
    ```sh
    killw -0 1234
    echo $?
    ```

### How It Works

The tool is built using the `windows` crate in Rust to call native Windows API functions.

-   **Process Handle**: To interact with a process (either to terminate it or check its status), the program first needs to get a "handle" to it using the `OpenProcess` function.
    -   For termination (`-TERM`), it requests `PROCESS_TERMINATE` permissions.
    -   For checking existence (`-0`), it requests `PROCESS_QUERY_LIMITED_INFORMATION` permissions, which is more restrictive and secure.
-   **Termination**: If the signal is `-TERM`, the program calls the `TerminateProcess` function with the obtained handle to end the process.
-   **Existence Check**: If the signal is `-0`, the program simply checks if `OpenProcess` was successful. A successful call means the process exists.
-   **Error Handling**: The program provides descriptive error messages if it fails to get a process handle or terminate a process. All handles are properly closed using `CloseHandle`.
