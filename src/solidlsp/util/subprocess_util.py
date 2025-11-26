import os
import platform
import subprocess
from typing import Any


def subprocess_kwargs() -> dict:
    """
    Returns a dictionary of keyword arguments for subprocess calls, adding platform-specific
    flags that we want to use consistently.
    """
    kwargs = {}
    if platform.system() == "Windows":
        kwargs["creationflags"] = subprocess.CREATE_NO_WINDOW  # type: ignore
    return kwargs


def run_rustup_command(args: list[str], **kwargs: Any) -> subprocess.CompletedProcess:
    """
    Run a rustup command with proper PATH resolution on all platforms.

    On Windows, uses shell=True to ensure PATH is searched correctly.
    On Unix-like systems, uses list form without shell.

    :param args: Command arguments (e.g., ["rustup", "--version"])
    :param kwargs: Additional kwargs to pass to subprocess.run
    :return: CompletedProcess instance
    """
    # Ensure environment variables are inherited
    if "env" not in kwargs:
        kwargs["env"] = os.environ.copy()

    # Ensure capture_output or stdout/stderr are set appropriately if not provided
    if "capture_output" not in kwargs and "stdout" not in kwargs:
        kwargs["capture_output"] = True

    # Add platform-specific subprocess kwargs
    kwargs.update(subprocess_kwargs())

    if platform.system() == "Windows":
        # On Windows, use shell=True with a command string to ensure PATH is searched by cmd.exe
        # Also set text=True if not already set for string output
        if "text" not in kwargs:
            kwargs["text"] = True
        command_str = " ".join(args)
        return subprocess.run(command_str, shell=True, check=False, **kwargs)  # type: ignore
    else:
        # On Unix-like systems, use list form without shell
        if "text" not in kwargs:
            kwargs["text"] = True
        return subprocess.run(args, shell=False, check=False, **kwargs)  # type: ignore


def quote_arg(arg: str) -> str:
    """
    Adds quotes around an argument if it contains spaces.
    """
    if " " not in arg:
        return arg
    return f'"{arg}"'
