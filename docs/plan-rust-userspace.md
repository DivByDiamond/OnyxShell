# Rust userspace support — план реализации

> **For agentic workers:** REQUIRED SUB-SKILL: subagent-driven-development
> (или executing-plans). Шаги — чекбоки (`- [ ]`). Очередь: после M2
> OnyxDesktop (docs/plan-onyxdesktop-m2.md).

**Goal:** писать юзерспейс-программы OnyxOS на Rust: общий crate с
syscall-биндингами и рантайм-минимумом, образец приложения в образе,
CI и документация.

**Architecture:** слой syscalls оболочки (osh) выносится в crate
`onyx-rt` в репозитории OnyxShell (cargo workspace: osh + rt + apps);
пользовательские Rust-программы = отдельные crate'ы того же workspace,
сборка cargo → riscv64gc-unknown-none-elf → elf2onx (как osh).

**Tech Stack:** Rust nightly (riscv64gc-unknown-none-elf), elf2onx
(OnyxKernel/tools), OnyxExec v2, GitHub Actions.

**Spec:** этот файл; syscall ABI: OnyxCompiller/include/sys/syscall_numbers.h
(проверяется `OnyxOS/scripts/check-abi.sh` — Rust-константы добавляются
в его сверку или дублируются с тестом на согласованность).

## Global Constraints

- ABI-номера в Rust обязаны совпадать с OnyxCompiller/include/sys/syscall_numbers.h;
  расхождение ловится тестом (host, сравнение по подключаемому списку),
  не молчанием.
- `bash build.sh` osh не ломается: osh переходит на onyx-rt без изменения
  поведения (существующие Rust-тесты OnyxShell зелёные, .onx собирается).
- Никаких кросс-репозиторных path-dependencies: rt живёт в репо OnyxShell;
  Rust-приложения тоже (OnyxApps остаётся C/onyxcc-ландом).
- ≤200 строк на файл .rs не является правилом Rust-репо; правило: один
  модуль — одна ответственность; тесты отдельными файлами.
- Новые строки без «—», без ИИ-паттернов; коммиты `feat:`/`build:`/`docs:`,
  явные пути, push.

## Review Focus

1. Расхождение номеров SYS_* между Rust и C-заголовком → тест красный
   до фикса (генератор или сверка).
2. osh после выноса syscalls: поведение идентично (тесты зелёные, размер
   .onx изменился в пределах разумного, smoke `qemu-interactive-smoke.sh`
   PASS).
3. rt без памяти: crate не тянет std/alloc неявно (no_std с первого дня;
   cargo tree чистый).
4. Образец: rhello читает /etc/passwd (или другой файл из образа) и
   печатает его в stdout, exit 0; провал чтения → exit 1 (не паника).
5. CI: job собирает workspace + ONX1; падение Rust-сборки роняет job.

---

### Task 1: workspace + crate onyx-rt (вынос syscalls из osh)

**Files:** Create: `Cargo.toml` (workspace), `rt/Cargo.toml`, `rt/src/*`
(перенос src/syscalls → rt с сохранением путей импорта osh);
Modify: `src/main.rs` (use onyx_rt), `build.sh`.
- [ ] RED: host-тест согласованности SYS_* с C-номерами (список держим
  рядом, сверка в CI).
- [ ] Вынос; `build.sh` osh зелёный; тесты OnyxShell зелёные.
- [ ] Commit `feat: extract onyx-rt syscall crate`.

### Task 2: рантайм-минимум onyx-rt

**Files:** `rt/src/entry.rs` (_start + main-конвенция), `rt/src/panic.rs`
(panic_handler, код возврата), `rt/src/errno.rs`.
- [ ] RED: unit-тесты panic-пути/errno-строк.
- [ ] osh переходит на rt-панik-хендлер (или осознанно оставляет свой —
  решение зафиксировать в rt/README.md).
- [ ] Commit `feat: onyx-rt entry and panic runtime`.

### Task 3: приложение rhello

**Files:** `apps/rhello/Cargo.toml`, `apps/rhello/src/main.rs`
(read /etc/passwd → печать первых строк → exit 0/1).
- [ ] Сборка → `build/rhello.onx`; локальный QEMU-прогон из osh
  (`/bin/rhello` в ручной сборке образа).
- [ ] Commit `feat: rhello sample rust app`.

### Task 4: CI

**Files:** `.github/workflows/ci.yml` (job rust: toolchain, build
workspace, ONX1-проверка rhello).
- [ ] Зелёный прогон на push.
- [ ] Commit `build: ci job for rust workspace`.

### Task 5: сборка образа + доки

**Files:** OnyxOS `scripts/build-all.sh` (шаг Rust-приложений после
OnyxShell: cargo + elf2onx → staging /bin/rhello), `docs/dev/rust.md`
(как писать Rust-программы для Onyx: зависимости, _start/main, syscall
слой, сборка, отладка), roadmap-строка.
- [ ] `build-all.sh` собирает образ с /bin/rhello; interactive-smoke
  PASS (не сломан).
- [ ] Commits `build: package rust apps into the image`, `docs: rust
  userspace guide`.
