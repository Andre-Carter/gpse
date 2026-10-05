//things that needs a fix
- da kine when using cli calc and pressing enter you get a error which is fine but can we handle that or avoid it with refactoring

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