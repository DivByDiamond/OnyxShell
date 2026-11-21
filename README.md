<p align="center">
<pre class="not-prose" style="text-align:center;font-family:monospace;">
    ███████                                     
  ███▒▒▒▒▒███                                   
 ███     ▒▒███ ████████   █████ ████ █████ █████
▒███      ▒███▒▒███▒▒███ ▒▒███ ▒███ ▒▒███ ▒▒███ 
▒███      ▒███ ▒███ ▒███  ▒███ ▒███  ▒▒▒█████▒  
▒▒███     ███  ▒███ ▒███  ▒███ ▒███   ███▒▒▒███ 
 ▒▒▒███████▒   ████ █████ ▒▒███████  █████ █████
   ▒▒▒▒▒▒▒    ▒▒▒▒ ▒▒▒▒▒   ▒▒▒▒▒███ ▒▒▒▒▒ ▒▒▒▒▒ 
                           ███ ▒███             
                          ▒▒██████              
                           ▒▒▒▒▒▒               
  █████████  █████               ████  ████     
 ███▒▒▒▒▒███▒▒███               ▒▒███ ▒▒███     
▒███    ▒▒▒  ▒███████    ██████  ▒███  ▒███     
▒▒█████████  ▒███▒▒███  ███▒▒███ ▒███  ▒███     
 ▒▒▒▒▒▒▒▒███ ▒███ ▒███ ▒███████  ▒███  ▒███     
 ███    ▒███ ▒███ ▒███ ▒███▒▒▒   ▒███  ▒███     
▒▒█████████  ████ █████▒▒██████  █████ █████    
 ▒▒▒▒▒▒▒▒▒  ▒▒▒▒ ▒▒▒▒▒  ▒▒▒▒▒▒  ▒▒▒▒▒ ▒▒▒▒▒     
                                                
                                                
                                                
</pre>
</p>

<p align="center"><em>A user-space shell for OnyxOS with built-in file operations, navigation, and system commands</em></p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-RISC--V%2064--bit-green" alt="RISC-V 64">
  <img src="https://img.shields.io/badge/language-Rust%2090%25-orange" alt="Rust 90%">
  <img src="https://img.shields.io/badge/version-v0.3-blue" alt="v0.3">
  <img src="https://img.shields.io/badge/target-OnyxOS-yellow" alt="OnyxOS">
  <img src="https://img.shields.io/badge/license-GPL--3.0-red" alt="GPL-3.0">
</p>

----

OnyxShell (`/bin/osh`) is the default command-line shell for
[OnyxOS](https://github.com/DivByDiamond/OnyxOS). It is a freestanding
RISC-V 64-bit binary written in 90% Rust (`no_std`, `no_main`) that compiles
to the OnyxExec v2 format and runs in ring 1 (root space) when launched by
`/bin/login`.

The shell provides built-in implementations of the most common Unix commands -
`ls`, `cat`, `rm`, `cd`, `cp`, `mv`, `mkdir` - plus `touch`, `stat`, `pwd`,
`echo`, `whoami`, `uname`, `date`, `clear`, `help`, `exit`, `exec`, `run`, and
`ver`. No external binaries are required for basic file management.

Part of the [OnyxOS](https://github.com/DivByDiamond/OnyxOS) ecosystem.

----

## Key Features

- **90% Rust**: `no_std`, `no_main`, compiled with `riscv64gc-unknown-none-elf`
- **OnyxExec v2 format**: compressed with `elf2onx --ring=1 --compress`
- **20 built-in commands**: `ls`, `cat`, `cp`, `mv`, `rm`, `mkdir`, `touch`,
  `stat`, `cd`, `pwd`, `echo`, `whoami`, `uname`, `date`, `clear`, `help`,
  `exit`, `exec`, `run`, `ver`
- **Path resolution**: relative paths (`foo`, `./bar`, `../baz`) resolved
  against the kernel's cwd via `getcwd`; `.` and `..` normalized lexically
- **Long-format `ls -l`**: shows file type, permissions, size, and name
- **Detailed `stat`**: displays inode, mode, size, blocks, timestamps, and
  more from the kernel's Linux-compatible `struct stat`
- **External program execution**: `exec` replaces the shell; `run` spawns a
  child process and waits (root-only, via `SYS_spawn` + `SYS_wait`)
- **Error reporting**: all file-mutation commands translate kernel errno
  codes to human-readable messages
- **Line editing**: the kernel's UART driver provides backspace and echo;
  the shell just reads complete lines via `SYS_read`

----

## Built-in Commands

| Command | Description | Root-only? |
|---------|-------------|:----------:|
| `ls [path] [-l]` | List directory contents (use `-l` for long format) | |
| `cat <file>...` | Print file contents to stdout | |
| `cp <src> <dst>` | Copy a file | ✓ |
| `mv <src> <dst>` | Move or rename a file (uses `rename`, falls back to copy+remove) | ✓ |
| `rm <file>...` | Remove (unlink) a file | ✓ |
| `mkdir <dir>...` | Create a directory | ✓ |
| `touch <file>...` | Create an empty file (no error if it exists) | ✓ |
| `stat <file>` | Show file metadata (inode, size, mode, timestamps) | |
| `cd [path]` | Change working directory (default: `/`) | |
| `pwd` | Print working directory | |
| `echo [text]` | Print text followed by a newline | |
| `whoami` | Print current user (uid) and privilege ring | |
| `uname` | Print system information (sysname, nodename, release, version, machine) | |
| `date` | Print current epoch time (seconds + nanoseconds) | |
| `clear` | Clear the terminal screen (ANSI escape) | |
| `help` | List all available commands | |
| `exec <path> [args]` | Replace the shell process with a binary | |
| `run <path> [args]` | Spawn a binary as a child and wait for it | ✓ |
| `exit` | Exit the shell (calls `SYS_exit(0)`) | |
| `ver` | Print shell version and copyright | |

**Root-only** commands require ring 1 (root space). The default first-boot
account is root with a hashed empty password; login still asks for a username.
The shipped shell is RING1-tagged and cannot be executed from ring 2. A complete
non-root login path (including passwd uid/gid installation) is unfinished.
Ring-2 `unlink` and `mkdir` are denied; `create`/`rename` permit uid 0.

----

## Project Structure

```
osh/
├── Cargo.toml           - package definition (no_std, no_main)
├── .cargo/config.toml   - RISC-V target + linker flags
├── linker.ld            - linker script (page-aligned .bss, entry at 0x10000)
├── build.rs             - passes linker script to rustc
├── src/
│   ├── main.rs          - _start entry point + main REPL loop
│   ├── syscalls.rs      - ecall wrappers for the OnyxKernel syscall ABI
│   ├── io.rs            - write_str / write_u64 / write_hex / read_line
│   ├── path.rs          - relative-to-absolute path resolution + normalization
│   └── commands.rs      - all 20 command implementations
├── build.sh             - builds the shell + converts ELF -> osh.onx
├── test_qemu.sh         - full-stack QEMU test (builds kernel, boot, disk)
└── README.md            - this file
```

----

## Building

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust nightly | ≥ 1.85 | Shell compilation |
| `riscv64gc-unknown-none-elf` target | - | Cross-compilation |
| `elf2onx` | - | ELF -> OnyxExec v2 conversion (from OnyxKernel/tools) |
| `qemu-system-riscv64` | - | Testing (optional) |

### Install Rust target

```console
$ rustup target add riscv64gc-unknown-none-elf
```

### Build the shell

```console
$ ./build.sh
```

This produces `build/osh.onx` - a compressed OnyxExec v2 binary tagged as
ring 1 (root space). Place this file at `/bin/osh` in your OnyxFS disk image.

### Manual build

```console
$ cargo build --release
$ elf2onx --ring=1 --compress target/riscv64gc-unknown-none-elf/release/onyx-osh build/osh.onx
```

> **Why `--ring=1`?**
>
> OnyxKernel's `exec` syscall sets the new process's ring based on the binary's
> RING1 flag. Without `--ring=1`, `/bin/login` (ring 1) execs `/bin/osh` and
> the shell is dropped to ring 2 (user space). Ring-2 `unlink` and `mkdir`
> are denied by the ACL; `create` and `rename` have a uid-0 exception.
> A ring-2 caller cannot use the RING1 tag to regain privileges.

----

## Formatting and lint policy

Every change must format the entire crate and run Clippy with warnings denied.
Do not suppress dead code or other warnings with blanket `allow` attributes:
remove unused helpers, or connect them to real execution paths. Keep the generated
`src/syscalls/numbers.rs` ABI snapshot unchanged; `build.rs` derives the used,
uppercase syscall constants from it for compilation.

```sh
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --target x86_64-unknown-linux-gnu --test stdio --test quoting -- -D warnings
cargo clippy --target riscv64gc-unknown-none-elf --all-targets -- -D warnings
```

The RISC-V `--all-targets` command currently fails because its target has no
`std` or test harness, which the host tests require. Always attempt the full
check and record this limitation; also verify the actual freestanding binary:

```sh
cargo clippy --target riscv64gc-unknown-none-elf --bin onyx-osh -- -D warnings
cargo build --release --target riscv64gc-unknown-none-elf
rustc --edition=2024 -D warnings --test tests/stdio.rs -o /tmp/onyx-shell-stdio-tests
/tmp/onyx-shell-stdio-tests
rustc --edition=2024 -D warnings --test tests/quoting/main.rs -o /tmp/onyx-shell-quoting-tests
/tmp/onyx-shell-quoting-tests
```

Use the Rust host triple reported by `rustc -vV` on other workstations. The
standalone host tests avoid building the RISC-V entry point on the host.

----

## Testing in QEMU

The `test_qemu.sh` script builds the entire OnyxOS stack (OnyxBoot +
OnyxKernel + OnyxShell) and launches QEMU:

```console
$ ./test_qemu.sh          # interactive mode - type commands at osh$ prompt
$ ./test_qemu.sh -s       # scripted mode - runs a test suite and exits
```

### Prerequisites for QEMU testing

- OnyxBoot, OnyxKernel, and OnyxCompiller repos cloned as siblings of `osh/`
- `riscv64-elf-gcc` (or `riscv64-unknown-elf-gcc`) for OnyxBoot
- `qemu-system-riscv64`, `parted`, `mkfs.fat`, `mcopy`

### Expected boot output

```
OnyxBoot v0.4 [riscv-virtio,qemu]
...
[kernel] OnyxKernel v0.4 - RISC-V 64-bit
...
[init] OnyxOS init v0.4 (service manager)
[init] launching /bin/login
[login] no users found - auto-login as root
[login] launching /bin/osh (root, ring 1)
OnyxShell v0.3.0 (built-in commands)
osh$ _
```

### Example session

```
osh$ help
osh$ ls /
osh$ ls -l /bin
osh$ cat /etc/passwd
osh$ mkdir /tmp
osh$ touch /tmp/test.txt
osh$ cp /etc/passwd /tmp/copy.txt
osh$ cat /tmp/copy.txt
osh$ mv /tmp/copy.txt /tmp/moved.txt
osh$ ls /tmp
osh$ rm /tmp/test.txt
osh$ rm /tmp/moved.txt
osh$ stat /bin/osh
osh$ whoami
osh$ uname
osh$ cd /tmp
osh$ pwd
osh$ exit
```

----

## How It Works

### Boot Flow

1. **OnyxBoot** (M-mode) loads `kernel.elf` from the VirtIO disk and jumps to
   `kmain`.
2. **OnyxKernel** initializes hardware, mounts OnyxFS, and spawns `/bin/init`
   (PID 1, ring 1).
3. **`/bin/init`** (OnyxInit service manager) scans `/service/`, then spawns
   `/bin/login`.
4. **`/bin/login`** auto-logs in as root on first boot (no users in
   `/etc/passwd`), then execs `/bin/osh`.
5. **`/bin/osh`** (this shell) enters its read-eval-print loop.

### Syscall ABI

The shell communicates with the kernel via RISC-V `ecall` instructions. Each
syscall wrapper in `src/syscalls.rs` is a thin inline-assembly block that
loads the syscall number into `a7`, arguments into `a0`-`a2`, executes
`ecall`, and reads the return value from `a0`.

The shell uses these syscalls:

| Syscall | Number | Purpose |
|---------|--------|---------|
| `write` | 1 | Write to stdout (fd 1) |
| `read` | 2 | Read from stdin (fd 0) - kernel handles line editing |
| `exit` | 3 | Terminate the shell |
| `open` | 8 | Open a file (O_RDONLY, O_CREAT, O_WRONLY, O_TRUNC) |
| `close` | 9 | Close a file descriptor |
| `stat` | 11 | Get file metadata (128-byte `struct stat`) |
| `readdir` | 16 | Read next directory entry (stateful, path-based) |
| `getring` | 17 | Get current privilege ring (0/1/2) |
| `write_fd` | 24 | Write to a file descriptor (fd ≥ 3) |
| `create` | 25 | Create a new regular file (root-only) |
| `mkdir` | 26 | Create a directory (root-only) |
| `unlink` | 37 | Remove a file (root-only) |
| `rename` | 38 | Rename/move a file (root-only) |
| `chdir` | 39 | Change working directory |
| `getcwd` | 40 | Get current working directory |
| `getuid` | 45 | Get current user ID |
| `uname` | 48 | Get system information |
| `spawn` | 14 | Spawn a child process (root-only) |
| `wait` | 15 | Wait for a child to exit (root-only) |
| `exec` | 12 | Replace process with a new binary |
| `clock_gettime` | 64 | Get current time |

### Path Resolution

OnyxKernel's VFS requires all paths passed to `open`, `stat`, `unlink`,
`mkdir`, `rename`, etc. to be **absolute** (starting with `/`). The shell
accepts relative paths from the user and resolves them in `src/path.rs`:

1. If the path starts with `/`, it is already absolute - normalize `.` and
   `..` components lexically.
2. Otherwise, fetch the cwd via `getcwd`, join `cwd + "/" + path`, then
   normalize.
3. `.` components are skipped; `..` pops the last component.

### Memory Layout

The shell is a freestanding binary with no heap allocation. All buffers are
stack-allocated with fixed sizes:

- Input line: 256 bytes
- Path buffer: 256 bytes
- Stat buffer: 256 bytes
- Read/write buffer: 512 bytes
- Token array: 16 tokens × (offset, length)

The linker script places `.text` at `0x10000` (USER_BASE), `.rodata` after
it, and page-aligns `.bss` so it does not share a page with `.rodata` (which
would cause a page fault on the first `.bss` write).

----

## Integration with OnyxKernel

To integrate the shell into your OnyxKernel build:

1. **Build the shell:**
   ```console
   $ cd osh && ./build.sh
   ```

2. **Copy `build/osh.onx` to your OnyxKernel build directory.**

3. **Update `scripts/run_qemu.sh`** to use the new `osh.onx`:
   ```bash
   elf2onx --ring=1 --compress $OSH_DIR/target/.../onyx-osh $BUILD/osh.onx
   ```

4. **Run QEMU:**
   ```console
   $ ./scripts/run_qemu.sh
   ```

Alternatively, use the included `test_qemu.sh` which automates the entire
build-and-test cycle.

### Kernel patches applied

This shell was tested with the following patches applied to OnyxKernel
(from `onyx-init-patches.zip`):

1. `01-init-service-manager.patch`: rewrites init as a service manager
2. `02-login-auto-root.patch`: auto-login as root on first boot
3. `03-auth-plaintext.patch`: plaintext password storage
4. `04-linker-bss-page-align.patch`: page-aligns `.bss` in init's linker.ld
5. `05-run-qemu-manifest.patch`: conditional manifest entries in run_qemu.sh

### Critical kernel fixes

Four fixes were needed to make the shell work correctly: three in
OnyxKernel and one in OnyxBoot. Each fix is minimal and does not change
the kernel's architecture:

1. **`vfs::create` fd table mismatch** (`kernel/src/fs/vfs/create.rs`):
   `create` called `alloc_fd` (which uses `G_KERNEL_FDS` when
   `is_kernel_boot()` is true) but then initialized the fd in
   `current().fds` directly. This caused `EBADF` on subsequent `write_fd`
   calls because `fd_check` read from a different table than the one
   `create` wrote to. Fixed by using `fd_set` / `fd_get` (which respect
   `is_kernel_boot()`) instead of accessing `current().fds` directly.

2. **`sys_uname` user-pointer dereference** (`kernel/src/syscall/fs_sys3/info.rs`):
   `sys_uname` wrote to the user buffer via `buf as *mut u8` without
   translating the VA to a PA, causing a kernel page fault. Fixed by
   calling `vmm::translate` first.

3. **`current_pid` state check** (`kernel/src/proc/process/current.rs`):
   `current_pid` returned 0 when the current process was not in `Running`
   state, causing `is_kernel_boot()` to return true during syscalls
   (if a timer tick had changed the state). This made `alloc_fd` use
   `G_KERNEL_FDS` instead of the process's own fd table. Fixed by
   returning `(*p).pid` regardless of state.

4. **OnyxBoot `stdbool.h`** (`OnyxBoot/include/types.h`): added
   `#include <stdbool.h>` so `bool` is defined for `ext4.c` and `fat.c`.
   Required for GCC 14+ which enforces C99 type correctness.

----

## Ecosystem

| Repository | Role | Language |
|---|---|---|
| [OnyxOS](https://github.com/DivByDiamond/OnyxOS) | Ecosystem: build, disk image, docs | Bash / Python |
| [OnyxKernel](https://github.com/DivByDiamond/OnyxKernel) | RISC-V 64 kernel (Sv39, 3 rings) | Rust |
| [OnyxBoot](https://github.com/DivByDiamond/OnyxBoot) | Bootloader (M-mode, FDT) | C++20 |
| [OnyxShell](https://github.com/DivByDiamond/OnyxShell) | User shell `osh` | Rust |
| [OnyxCompiller](https://github.com/DivByDiamond/OnyxCompiller) | C99 compiler `onyxcc` + linker | C99 |
| [OnyxApps](https://github.com/DivByDiamond/OnyxApps) | Userland applications | C99 |
| [OnyxDesktop](https://github.com/DivByDiamond/OnyxDesktop) | Desktop environment `onyxdesk` | C99 |

----

## License

GPL-3.0-or-later, the same license as OnyxKernel.

----

## Русская версия

OnyxShell (`/bin/osh`) это командная оболочка по умолчанию для
[OnyxOS](https://github.com/DivByDiamond/OnyxOS). Freestanding-бинарник
RISC-V 64-bit, написанный на 90% на Rust (`no_std`, `no_main`); компилируется
в формат OnyxExec v2 и работает в ring 1 (root space), когда его запускает
`/bin/login`.

В шелл встроены самые распространённые Unix-команды: `ls`, `cat`, `rm`,
`cd`, `cp`, `mv`, `mkdir`, плюс `touch`, `stat`, `pwd`, `echo`, `whoami`,
`uname`, `date`, `clear`, `help`, `exit`, `exec`, `run` и `ver`. Для
базовой работы с файлами внешние бинарники не нужны.

Часть экосистемы [OnyxOS](https://github.com/DivByDiamond/OnyxOS).

----

## Ключевые особенности

- **90% Rust**: `no_std`, `no_main`, компиляция под `riscv64gc-unknown-none-elf`
- **Формат OnyxExec v2**: сжатие через `elf2onx --ring=1 --compress`
- **20 встроенных команд**: `ls`, `cat`, `cp`, `mv`, `rm`, `mkdir`, `touch`,
  `stat`, `cd`, `pwd`, `echo`, `whoami`, `uname`, `date`, `clear`, `help`,
  `exit`, `exec`, `run`, `ver`
- **Разрешение путей**: относительные пути (`foo`, `./bar`, `../baz`)
  разрешаются относительно cwd ядра через `getcwd`; `.` и `..`
  нормализуются лексически
- **Формат `ls -l`**: показывает тип файла, права, размер и имя
- **Подробный `stat`**: inode, режим, размер, блоки, временные метки и
  другое из Linux-совместимой `struct stat` ядра
- **Запуск внешних программ**: `exec` замещает шелл; `run` порождает
  дочерний процесс и ждёт его (только root, через `SYS_spawn` +
  `SYS_wait`)
- **Сообщения об ошибках**: все команды, меняющие файлы, переводят коды
  errno ядра в читаемые сообщения
- **Построчный ввод**: драйвер UART ядра обеспечивает backspace и echo;
  шелл просто читает готовые строки через `SYS_read`

----

## Встроенные команды

| Команда | Описание | Только root? |
|---------|-------------|:----------:|
| `ls [path] [-l]` | Содержимое каталога (`-l` для длинного формата) | |
| `cat <file>...` | Печать содержимого файлов в stdout | |
| `cp <src> <dst>` | Копировать файл | ✓ |
| `mv <src> <dst>` | Переместить или переименовать файл (`rename`, при неудаче копирование + удаление) | ✓ |
| `rm <file>...` | Удалить файл (unlink) | ✓ |
| `mkdir <dir>...` | Создать каталог | ✓ |
| `touch <file>...` | Создать пустой файл (без ошибки, если существует) | ✓ |
| `stat <file>` | Метаданные файла (inode, размер, режим, временные метки) | |
| `cd [path]` | Сменить рабочий каталог (по умолчанию `/`) | |
| `pwd` | Напечатать рабочий каталог | |
| `echo [text]` | Печать текста с переводом строки | |
| `whoami` | Текущий пользователь (uid) и кольцо привилегий | |
| `uname` | Информация о системе (sysname, nodename, release, version, machine) | |
| `date` | Текущее время epoch (секунды + наносекунды) | |
| `clear` | Очистить экран терминала (ANSI-escape) | |
| `help` | Список всех доступных команд | |
| `exec <path> [args]` | Заместить процесс шелла бинарником | |
| `run <path> [args]` | Запустить бинарник как дочерний процесс и дождаться его | ✓ |
| `exit` | Выйти из шелла (вызывает `SYS_exit(0)`) | |
| `ver` | Версия и копирайт шелла | |

Команды **только root** требуют ring 1 (root space). Учётная запись первого
запуска по умолчанию это root с хешированным пустым паролем; login всё равно
спрашивает имя пользователя. Поставляемый шелл помечен RING1 и не может быть
запущен из ring 2. Полный вход для не-root (включая установку uid/gid из
passwd) не завершён. `unlink` и `mkdir` из ring 2 запрещены; `create` и
`rename` допускают uid 0.

----

## Структура проекта

```
osh/
├── Cargo.toml           - определение пакета (no_std, no_main)
├── .cargo/config.toml   - таргет RISC-V + флаги линковки
├── linker.ld            - скрипт линковки (странично выровненный .bss, entry 0x10000)
├── build.rs             - передаёт скрипт линковки в rustc
├── src/
│   ├── main.rs          - точка входа _start + главный цикл REPL
│   ├── syscalls.rs      - обёртки ecall для syscall ABI OnyxKernel
│   ├── io.rs            - write_str / write_u64 / write_hex / read_line
│   ├── path.rs          - разрешение относительных путей в абсолютные + нормализация
│   └── commands.rs      - все 20 реализаций команд
├── build.sh             - сборка шелла + конвертация ELF -> osh.onx
├── test_qemu.sh         - полный QEMU-тест (собирает ядро, загрузчик, диск)
└── README.md            - этот файл
```

----

## Сборка

### Необходимые инструменты

| Инструмент | Версия | Назначение |
|------|---------|---------|
| Rust nightly | ≥ 1.85 | Компиляция шелла |
| таргет `riscv64gc-unknown-none-elf` | - | Кросс-компиляция |
| `elf2onx` | - | Конвертация ELF -> OnyxExec v2 (из OnyxKernel/tools) |
| `qemu-system-riscv64` | - | Тестирование (опционально) |

### Установка таргета Rust

```console
$ rustup target add riscv64gc-unknown-none-elf
```

### Сборка шелла

```console
$ ./build.sh
```

Результат: `build/osh.onx`, сжатый бинарник OnyxExec v2 с меткой ring 1
(root space). Положите этот файл в `/bin/osh` на образе диска OnyxFS.

### Ручная сборка

```console
$ cargo build --release
$ elf2onx --ring=1 --compress target/riscv64gc-unknown-none-elf/release/onyx-osh build/osh.onx
```

> **Зачем `--ring=1`?**
>
> Системный вызов `exec` в OnyxKernel назначает новому процессу ring по
> флагу RING1 бинарника. Без `--ring=1` `/bin/login` (ring 1) запускает
> `/bin/osh`, и шелл падает в ring 2 (user space). `unlink` и `mkdir` из
> ring 2 запрещены ACL; у `create` и `rename` есть исключение для uid 0.
> Вызывающий из ring 2 не может вернуть себе привилегии через метку RING1.

----

## Политика форматирования и линтов

Каждое изменение должно форматировать весь крейт и запускать Clippy с
запретом предупреждений. Не глушите мёртвый код и другие предупреждения
массовыми атрибутами `allow`: удаляйте неиспользуемые хелперы или
подключайте их к реальным путям исполнения. Не меняйте сгенерированный
снимок ABI `src/syscalls/numbers.rs`; `build.rs` выводит из него
используемые константы syscall в верхнем регистре для компиляции.

```sh
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --target x86_64-unknown-linux-gnu --test stdio --test quoting -- -D warnings
cargo clippy --target riscv64gc-unknown-none-elf --all-targets -- -D warnings
```

Команда RISC-V `--all-targets` сейчас падает: у её таргета нет `std` и
тестовой обвязки, которые нужны host-тестам. Всегда запускайте полную
проверку и фиксируйте это ограничение; также проверяйте сам
freestanding-бинарник:

```sh
cargo clippy --target riscv64gc-unknown-none-elf --bin onyx-osh -- -D warnings
cargo build --release --target riscv64gc-unknown-none-elf
rustc --edition=2024 -D warnings --test tests/stdio.rs -o /tmp/onyx-shell-stdio-tests
/tmp/onyx-shell-stdio-tests
rustc --edition=2024 -D warnings --test tests/quoting/main.rs -o /tmp/onyx-shell-quoting-tests
/tmp/onyx-shell-quoting-tests
```

На других рабочих станциях используйте host-триплет Rust из вывода
`rustc -vV`. Отдельные host-тесты не собирают RISC-V точку входа на хосте.

----

## Тестирование в QEMU

Скрипт `test_qemu.sh` собирает весь стек OnyxOS (OnyxBoot + OnyxKernel +
OnyxShell) и запускает QEMU:

```console
$ ./test_qemu.sh          # интерактивный режим: команды вводятся в osh$
$ ./test_qemu.sh -s       # сценарный режим: гоняет тестовый набор и выходит
```

### Требования для QEMU-тестирования

- Репозитории OnyxBoot, OnyxKernel и OnyxCompiller, склонированные рядом с `osh/`
- `riscv64-elf-gcc` (или `riscv64-unknown-elf-gcc`) для OnyxBoot
- `qemu-system-riscv64`, `parted`, `mkfs.fat`, `mcopy`

### Ожидаемый вывод при загрузке

```
OnyxBoot v0.4 [riscv-virtio,qemu]
...
[kernel] OnyxKernel v0.4 - RISC-V 64-bit
...
[init] OnyxOS init v0.4 (service manager)
[init] launching /bin/login
[login] no users found - auto-login as root
[login] launching /bin/osh (root, ring 1)
OnyxShell v0.3.0 (built-in commands)
osh$ _
```

### Пример сессии

```
osh$ help
osh$ ls /
osh$ ls -l /bin
osh$ cat /etc/passwd
osh$ mkdir /tmp
osh$ touch /tmp/test.txt
osh$ cp /etc/passwd /tmp/copy.txt
osh$ cat /tmp/copy.txt
osh$ mv /tmp/copy.txt /tmp/moved.txt
osh$ ls /tmp
osh$ rm /tmp/test.txt
osh$ rm /tmp/moved.txt
osh$ stat /bin/osh
osh$ whoami
osh$ uname
osh$ cd /tmp
osh$ pwd
osh$ exit
```

----

## Как это работает

### Поток загрузки

1. **OnyxBoot** (M-mode) загружает `kernel.elf` с диска VirtIO и прыгает в
   `kmain`.
2. **OnyxKernel** инициализирует железо, монтирует OnyxFS и порождает
   `/bin/init` (PID 1, ring 1).
3. **`/bin/init`** (менеджер сервисов OnyxInit) сканирует `/service/`,
   затем порождает `/bin/login`.
4. **`/bin/login`** автоматически входит как root при первом запуске (нет
   пользователей в `/etc/passwd`), затем делает exec `/bin/osh`.
5. **`/bin/osh`** (этот шелл) входит в свой цикл read-eval-print.

### Syscall ABI

Шелл общается с ядром через инструкции `ecall` RISC-V. Каждая обёртка
syscall в `src/syscalls.rs` это тонкий блок inline-ассемблера: номер
syscall кладётся в `a7`, аргументы в `a0`-`a2`, выполняется `ecall`,
возврат читается из `a0`.

Шелл использует такие syscall:

| Системный вызов | Номер | Назначение |
|---------|--------|---------|
| `write` | 1 | Запись в stdout (fd 1) |
| `read` | 2 | Чтение из stdin (fd 0), построчный ввод делает ядро |
| `exit` | 3 | Завершить шелл |
| `open` | 8 | Открыть файл (O_RDONLY, O_CREAT, O_WRONLY, O_TRUNC) |
| `close` | 9 | Закрыть файловый дескриптор |
| `stat` | 11 | Метаданные файла (`struct stat` на 128 байт) |
| `readdir` | 16 | Следующая запись каталога (stateful, по пути) |
| `getring` | 17 | Текущее кольцо привилегий (0/1/2) |
| `write_fd` | 24 | Запись в файловый дескриптор (fd ≥ 3) |
| `create` | 25 | Создать обычный файл (только root) |
| `mkdir` | 26 | Создать каталог (только root) |
| `unlink` | 37 | Удалить файл (только root) |
| `rename` | 38 | Переименовать/переместить файл (только root) |
| `chdir` | 39 | Сменить рабочий каталог |
| `getcwd` | 40 | Получить текущий рабочий каталог |
| `getuid` | 45 | Получить ID пользователя |
| `uname` | 48 | Получить информацию о системе |
| `spawn` | 14 | Породить дочерний процесс (только root) |
| `wait` | 15 | Дождаться завершения потомка (только root) |
| `exec` | 12 | Заместить процесс новым бинарником |
| `clock_gettime` | 64 | Получить текущее время |

### Разрешение путей

VFS OnyxKernel требует, чтобы все пути в `open`, `stat`, `unlink`,
`mkdir`, `rename` и остальных вызовах были **абсолютными** (начинались с
`/`). Шелл принимает от пользователя относительные пути и разрешает их в
`src/path.rs`:

1. Если путь начинается с `/`, он уже абсолютный: компоненты `.` и `..`
   нормализуются лексически.
2. Иначе берётся cwd через `getcwd`, склеивается `cwd + "/" + path`, затем
   идёт нормализация.
3. Компоненты `.` пропускаются; `..` отбрасывает последнюю компоненту.

### Раскладка памяти

Шелл это freestanding-бинарник без аллокации кучи. Все буферы лежат на
стеке и имеют фиксированный размер:

- Строка ввода: 256 байт
- Буфер пути: 256 байт
- Буфер stat: 256 байт
- Буфер чтения/записи: 512 байт
- Массив токенов: 16 токенов × (offset, length)

Скрипт линковки кладёт `.text` по адресу `0x10000` (USER_BASE), `.rodata`
после него и выравнивает `.bss` по странице, чтобы он не делил страницу с
`.rodata` (иначе page fault при первой записи в `.bss`).

----

## Интеграция с OnyxKernel

Чтобы встроить шелл в свою сборку OnyxKernel:

1. **Соберите шелл:**
   ```console
   $ cd osh && ./build.sh
   ```

2. **Скопируйте `build/osh.onx` в каталог сборки OnyxKernel.**

3. **Обновите `scripts/run_qemu.sh`**, чтобы он использовал новый `osh.onx`:
   ```bash
   elf2onx --ring=1 --compress $OSH_DIR/target/.../onyx-osh $BUILD/osh.onx
   ```

4. **Запустите QEMU:**
   ```console
   $ ./scripts/run_qemu.sh
   ```

Либо используйте входящий `test_qemu.sh`: он автоматизирует весь цикл
сборки и тестирования.

### Патчи ядра

Шелл тестировался со следующими патчами OnyxKernel (из
`onyx-init-patches.zip`):

1. `01-init-service-manager.patch`: переписывает init как менеджер сервисов
2. `02-login-auto-root.patch`: авто-вход как root при первом запуске
3. `03-auth-plaintext.patch`: хранение паролей открытым текстом
4. `04-linker-bss-page-align.patch`: выравнивает `.bss` по странице в linker.ld init
5. `05-run-qemu-manifest.patch`: условные записи манифеста в run_qemu.sh

### Критические исправления ядра

Чтобы шелл работал корректно, потребовались четыре исправления: три в
OnyxKernel и одно в OnyxBoot. Исправления минимальны и не меняют
архитектуру ядра:

1. **Несоответствие таблиц fd в `vfs::create`**
   (`kernel/src/fs/vfs/create.rs`): `create` вызывал `alloc_fd` (который
   при `is_kernel_boot()` использует `G_KERNEL_FDS`), но инициализировал
   fd напрямую в `current().fds`. Из-за этого последующие `write_fd`
   получали `EBADF`: `fd_check` читал из другой таблицы, чем та, куда
   писал `create`. Исправлено использованием `fd_set` / `fd_get` (которые
   уважают `is_kernel_boot()`) вместо прямого доступа к `current().fds`.

2. **Разыменование user-указателя в `sys_uname`**
   (`kernel/src/syscall/fs_sys3/info.rs`): `sys_uname` писал в буфер
   пользователя через `buf as *mut u8` без трансляции VA в PA, что давало
   page fault ядра. Исправлено вызовом `vmm::translate` перед записью.

3. **Проверка состояния в `current_pid`**
   (`kernel/src/proc/process/current.rs`): `current_pid` возвращал 0,
   когда текущий процесс не был в состоянии `Running`, из-за чего
   `is_kernel_boot()` во время syscall возвращал true (если тик таймера
   сменил состояние). Тогда `alloc_fd` использовал `G_KERNEL_FDS` вместо
   собственной таблицы fd процесса. Исправлено возвратом `(*p).pid` вне
   зависимости от состояния.

4. **OnyxBoot `stdbool.h`** (`OnyxBoot/include/types.h`): добавлен
   `#include <stdbool.h>`, чтобы `bool` был определён для `ext4.c` и
   `fat.c`. Требуется для GCC 14+, который строго проверяет типы C99.

----

## Экосистема

| Repository | Role | Language |
|---|---|---|
| [OnyxOS](https://github.com/DivByDiamond/OnyxOS) | Ecosystem: build, disk image, docs | Bash / Python |
| [OnyxKernel](https://github.com/DivByDiamond/OnyxKernel) | RISC-V 64 kernel (Sv39, 3 rings) | Rust |
| [OnyxBoot](https://github.com/DivByDiamond/OnyxBoot) | Bootloader (M-mode, FDT) | C++20 |
| [OnyxShell](https://github.com/DivByDiamond/OnyxShell) | User shell `osh` | Rust |
| [OnyxCompiller](https://github.com/DivByDiamond/OnyxCompiller) | C99 compiler `onyxcc` + linker | C99 |
| [OnyxApps](https://github.com/DivByDiamond/OnyxApps) | Userland applications | C99 |
| [OnyxDesktop](https://github.com/DivByDiamond/OnyxDesktop) | Desktop environment `onyxdesk` | C99 |

----

## Лицензия

GPL-3.0-or-later, та же лицензия, что у OnyxKernel.