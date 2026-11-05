10/15/2026 12:15 AM

error: could not execute process `target\debug\gpse.exe` (never executed)

Caused by:
  An Application Control policy has blocked this file. (os error 4551)

10/7/26 5:10 AM

Caused by:
  could not execute process `C:\Users\hlvdr\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\rustdoc.exe --edition=2024 --crate-type lib --color auto --crate-name gpse --test src\lib.rs --test-run-directory C:\Users\hlvdr\rubbah\gpse --extern gpse=C:\Users\hlvdr\rubbah\gpse\target\debug\deps\libgpse-a2e5a0c159bc439f.rlib -L dependency=C:\Users\hlvdr\rubbah\gpse\target\debug\deps -C embed-bitcode=no --check-cfg cfg(docsrs,test) --check-cfg "cfg(feature, values())" --error-format human` (never executed)

Caused by:
  An Application Control policy has blocked this file. (os error 4551)

10/8/2026 

error: failed to run custom build command for `getrandom v0.4.3`

Caused by:
  could not execute process `C:\Users\hlvdr\rubbah\gpse\target\debug\build\getrandom-9819a584ece59e9e\build-script-build` (never executed)

Caused by:
  An Application Control policy has blocked this file. (os error 4551)

10/29/26 

Note: moved this one from issues

thread 'main' (23628) panicked at src\cli\calculator.rs:26:51:
called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Empty }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\gpse.exe` (exit code: 101)

error: could not execute process `target\debug\gpse.exe` (never executed)

Caused by:
  An Application Control policy has blocked this file. (os error 4551)

error: test failed, to rerun pass `--lib`

Caused by:
  could not execute process `C:\Users\hlvdr\rubbah\gpse\target\debug\deps\gpse-df7123b25fbc86f1.exe` (never executed)

11/3/2026 10:14 

PS C:\Users\hlvdr\rubbah\gpse> cargo check
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
PS C:\Users\hlvdr\rubbah\gpse> cargo fmt --check
memory allocation of 47193798960 bytes failed
stack backtrace:
   0:     0x7ff9965f19b9 - std::backtrace_rs::backtrace::win64::trace
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\..\..\backtrace\src\backtrace\win64.rs:85
   1:     0x7ff9965f19b9 - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff9965f19b9 - std::sys::backtrace::_print_fmt
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\sys\backtrace.rs:74
   3:     0x7ff9965f19b9 - std::sys::backtrace::impl$0::print::impl$0::fmt
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\sys\backtrace.rs:44
   4:     0x7ff99661a9f1 - core::fmt::write
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\core\src\fmt\mod.rs:0
   5:     0x7ff996601874 - std::io::default_write_fmt
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\io\mod.rs:626
   6:     0x7ff996601874 - std::io::Write::write_fmt<std::sys::stdio::windows::Stderr>
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\io\mod.rs:1730
   7:     0x7ff9965e393c - std::sys::backtrace::BacktraceLock::print
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\sys\backtrace.rs:47
   8:     0x7ff9965e393c - std::alloc::default_alloc_error_hook
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\alloc.rs:397
   9:     0x7ff9965c815c - std::alloc::rust_oom::closure$0
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\alloc.rs:428
  10:     0x7ff9965c578f - std::sys::backtrace::__rust_end_short_backtrace<std::alloc::rust_oom::closure_env$0,never$>
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\sys\backtrace.rs:182
  11:     0x7ff9965e39db - std::alloc::rust_oom
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\alloc.rs:424
  12:     0x7ff9965c8f98 - std::alloc::_::__rust_alloc_error_handler
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\alloc.rs:423
  13:     0x7ff99663d9e9 - alloc::alloc::handle_alloc_error::rt_error
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\alloc\src\alloc.rs:551
  14:     0x7ff99663d9e9 - alloc::alloc::handle_alloc_error
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\alloc\src\alloc.rs:557
  15:     0x7ff99663da06 - alloc::raw_vec::handle_error
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\alloc\src\raw_vec\mod.rs:894
  16:     0x7ff704699c5a - <unknown>
  17:     0x7ff7045eb9bb - <unknown>
  18:     0x7ff704682f98 - <unknown>
  19:     0x7ff7044c6ce7 - <unknown>
  20:     0x7ff7044c11ae - <unknown>
  21:     0x7ff7044d77cf - <unknown>
  22:     0x7ff7044d4043 - <unknown>
  23:     0x7ff7044d2a76 - <unknown>
  24:     0x7ff7044d30ec - <unknown>
  25:     0x7ff9965e1e70 - std::rt::lang_start_internal::closure$0
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\rt.rs:175
  26:     0x7ff9965e1e70 - std::panicking::catch_unwind::do_call
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\panicking.rs:576
  27:     0x7ff9965e1e70 - std::panicking::catch_unwind
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\panicking.rs:544
  28:     0x7ff9965e1e70 - std::panic::catch_unwind
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\panic.rs:359
  29:     0x7ff9965e1e70 - std::rt::lang_start_internal
                               at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library\std\src\rt.rs:171
  30:     0x7ff7044e424c - <unknown>
  31:     0x7ff7047d8d9f - <unknown>
  32:     0x7ffa2567dda7 - BaseThreadInitThunk
  33:     0x7ffa26dbee4c - RtlUserThreadStart
PS C:\Users\hlvdr\rubbah\gpse>