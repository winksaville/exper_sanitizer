# Experiment with sanitizer

So sanitizer/AddressSanitizer works well, better than valgrind.
Although, valgrind is "easier" to setup, i.e. you don't
have to do anything execpt run the executable.

See [here](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html) for information
on stanitizer.

## Build:

With sanitizer:
```
wink@3900x 22-12-16T16:03:39.914Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ RUSTFLAGS=-Zsanitizer=address cargo build -Zbuild-std --target x86_64-unknown-linux-gnu
   Compiling compiler_builtins v0.1.85
   Compiling core v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling libc v0.2.138
   ...
   Compiling gimli v0.26.1
   Compiling std_detect v0.1.5 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.5.3
   Compiling hashbrown v0.12.3
   Compiling object v0.29.0
   Compiling addr2line v0.17.0
   Compiling proc_macro v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro)
   Compiling exper_sanitizer v0.2.0 (/home/wink/prgs/rust/myrepos/exper_sanitizer)
    Finished dev [unoptimized + debuginfo] target(s) in 12.07s
```

Build without sanitizer (use with valgrind):
```
wink@3900x 22-12-16T16:12:01.021Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo clean
wink@3900x 22-12-16T16:12:03.958Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo build
   Compiling exper_sanitizer v0.2.0 (/home/wink/prgs/rust/myrepos/exper_sanitizer)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
wink@3900x 22-12-16T16:12:10.166Z:~/prgs/rust/myrepos/exper_sanitizer (main)
```

## Run:

Run with array on stack and getting first element:
```
wink@3900x 22-12-16T16:05:12.634Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ RUSTFLAGS=-Zsanitizer=address cargo run -Zbuild-std --target x86_64-unknown-linux-gnu -- stack 0
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/debug/exper_sanitizer stack 0`
main:- xs[0]=0x0
```

Run with array on heap and getting last element:
```
wink@3900x 22-12-16T16:05:29.747Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ RUSTFLAGS=-Zsanitizer=address cargo run -Zbuild-std --target x86_64-unknown-linux-gnu -- heap 3
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/debug/exper_sanitizer heap 3`
main:- xs[3]=0x3
```

Run using valgrind with array on stack and getting first element:
```
wink@3900x 22-12-16T16:12:36.693Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ valgrind target/debug/exper_sanitizer stack 0
==13883== Memcheck, a memory error detector
==13883== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==13883== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==13883== Command: target/debug/exper_sanitizer stack 0
==13883== 
main:- xs[0]=0x0
==13883== 
==13883== HEAP SUMMARY:
==13883==     in use at exit: 0 bytes in 0 blocks
==13883==   total heap usage: 16 allocs, 16 frees, 3,383 bytes allocated
==13883== 
==13883== All heap blocks were freed -- no leaks are possible
==13883== 
==13883== For lists of detected and suppressed errors, rerun with: -s
==13883== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

Run using valgrind with array on heap and getting last element
```
wink@3900x 22-12-16T16:15:38.269Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ valgrind target/debug/exper_sanitizer heap 3
==13978== Memcheck, a memory error detector
==13978== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==13978== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==13978== Command: target/debug/exper_sanitizer heap 3
==13978== 
main:- xs[3]=0x3
==13978== 
==13978== HEAP SUMMARY:
==13978==     in use at exit: 0 bytes in 0 blocks
==13978==   total heap usage: 17 allocs, 17 frees, 3,398 bytes allocated
==13978== 
==13978== All heap blocks were freed -- no leaks are possible
==13978== 
==13978== For lists of detected and suppressed errors, rerun with: -s
==13978== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

Run indexing past the last element and AddressSanitizer screams at us:
```
wink@3900x 22-12-16T16:05:39.427Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ RUSTFLAGS=-Zsanitizer=address cargo run -Zbuild-std --target x86_64-unknown-linux-gnu -- heap 4
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/debug/exper_sanitizer heap 4`
=================================================================
==12255==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x602000000080 at pc 0x5565aae57ede bp 0x7ffcb15b67d0 sp 0x7ffcb15b67c8
READ of size 4 at 0x602000000080 thread T0
    #0 0x5565aae57edd in exper_sanitizer::main::ha438dd185b8326be /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:34:22
    #1 0x5565aae583ea in core::ops::function::FnOnce::call_once::h5c5a19036119e3c7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    #2 0x5565aae55527 in std::sys_common::backtrace::__rust_begin_short_backtrace::h61eb74c7a690b1ec /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:121:18
    #3 0x5565aae58f23 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::had040cfbcb9a20e5 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    #4 0x5565ab1a0c1d in core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h750779b9c5158193 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:606:13
    #5 0x5565aaf8ce4e in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #6 0x5565aaf909da in __rust_try std.63ac60a6-cgu.35
    #7 0x5565aaf8ba75 in std::panicking::try::h2155994ae210243d /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #8 0x5565aae9f219 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #9 0x5565aae97540 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    #10 0x5565aaf8d026 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #11 0x5565aaf909da in __rust_try std.63ac60a6-cgu.35
    #12 0x5565aaf8c065 in std::panicking::try::hd2b4bb59e4bb40ad /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #13 0x5565aae9f0a9 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #14 0x5565aae96f75 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    #15 0x5565aae58e7f in std::rt::lang_start::hcdd154a7a9f20701 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    #16 0x5565aae5839d in main (/home/wink/prgs/rust/myrepos/exper_sanitizer/target/x86_64-unknown-linux-gnu/debug/exper_sanitizer+0x19939d) (BuildId: 9976a65f81b85d8d61a70f6b679b204ff9ec5494)
    #17 0x7ff04057528f  (/usr/lib/libc.so.6+0x2328f) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    #18 0x7ff040575349 in __libc_start_main (/usr/lib/libc.so.6+0x23349) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    #19 0x5565aadb5254 in _start /build/glibc/src/glibc/csu/../sysdeps/x86_64/start.S:115

0x602000000080 is located 0 bytes to the right of 16-byte region [0x602000000070,0x602000000080)
allocated by thread T0 here:
    #0 0x5565aae29cbe in malloc /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_malloc_linux.cpp:69:3
    #1 0x5565aae9b8a6 in std::sys::unix::alloc::_$LT$impl$u20$core..alloc..global..GlobalAlloc$u20$for$u20$std..alloc..System$GT$::alloc::hf2cbfb265a23c366 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:14:13
    #2 0x5565aaef8d4f in __rdl_alloc /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/alloc.rs:381:13
    #3 0x5565aae54856 in alloc::alloc::alloc::hd12f4b1d28c39726 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:95:14
    #4 0x5565aae54b5f in alloc::alloc::Global::alloc_impl::hc22a9f5a02780e58 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:177:73
    #5 0x5565aae553f0 in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::allocate::h2fd53543f7a49e8c /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:237:9
    #6 0x5565aae54591 in alloc::alloc::exchange_malloc::hbfb1b1019fd97450 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:326:11
    #7 0x5565aae57c21 in alloc::boxed::Box$LT$T$GT$::new::h85e7f3fea1f1a87b /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:220:9
    #8 0x5565aae57c21 in exper_sanitizer::main::ha438dd185b8326be /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:33:22
    #9 0x5565aae583ea in core::ops::function::FnOnce::call_once::h5c5a19036119e3c7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    #10 0x5565aae58f23 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::had040cfbcb9a20e5 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    #11 0x5565aaf8ce4e in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #12 0x5565aaf909da in __rust_try std.63ac60a6-cgu.35
    #13 0x5565aae9f219 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #14 0x5565aae97540 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    #15 0x5565aaf8d026 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #16 0x5565aaf909da in __rust_try std.63ac60a6-cgu.35
    #17 0x5565aae9f0a9 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #18 0x5565aae96f75 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    #19 0x5565aae58e7f in std::rt::lang_start::hcdd154a7a9f20701 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    #20 0x5565aae5839d in main (/home/wink/prgs/rust/myrepos/exper_sanitizer/target/x86_64-unknown-linux-gnu/debug/exper_sanitizer+0x19939d) (BuildId: 9976a65f81b85d8d61a70f6b679b204ff9ec5494)

SUMMARY: AddressSanitizer: heap-buffer-overflow /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:34:22 in exper_sanitizer::main::ha438dd185b8326be
Shadow bytes around the buggy address:
  0x0c047fff7fc0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c047fff7fd0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c047fff7fe0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c047fff7ff0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c047fff8000: fa fa 05 fa fa fa 04 fa fa fa 01 fa fa fa 00 00
=>0x0c047fff8010:[fa]fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8020: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8030: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8040: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8050: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8060: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07 
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==12255==ABORTING
```


We get similar errors passing `stack -1`, `stack 4` and `heap -1`.
With valgrind we also get errors, here is `stack -1`:
```
wink@3900x 22-12-16T16:15:47.706Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ valgrind target/debug/exper_sanitizer stack -1
==14086== Memcheck, a memory error detector
==14086== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==14086== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==14086== Command: target/debug/exper_sanitizer stack -1
==14086== 
==14086== Conditional jump or move depends on uninitialised value(s)
==14086==    at 0x14AB96: fmt_int<core::fmt::num::LowerHex, u32> (num.rs:83)
==14086==    by 0x14AB96: core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt (num.rs:155)
==14086==    by 0x147271: run (mod.rs:1256)
==14086==    by 0x147271: core::fmt::write (mod.rs:1224)
==14086==    by 0x12A203: write_fmt<std::io::stdio::StdoutLock> (mod.rs:1682)
==14086==    by 0x12A203: <&std::io::stdio::Stdout as std::io::Write>::write_fmt (stdio.rs:716)
==14086==    by 0x12A8E2: write_fmt (stdio.rs:690)
==14086==    by 0x12A8E2: print_to<std::io::stdio::Stdout> (stdio.rs:1008)
==14086==    by 0x12A8E2: std::io::stdio::_print (stdio.rs:1075)
==14086==    by 0x1141C3: exper_sanitizer::main (main.rs:45)
==14086==    by 0x11257A: core::ops::function::FnOnce::call_once (function.rs:507)
==14086==    by 0x11480D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==14086==    by 0x114880: std::rt::lang_start::{{closure}} (rt.rs:166)
==14086==    by 0x12886B: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==14086==    by 0x12886B: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==14086==    by 0x12886B: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==14086==    by 0x12886B: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==14086==    by 0x12886B: {closure#2} (rt.rs:148)
==14086==    by 0x12886B: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==14086==    by 0x12886B: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==14086==    by 0x12886B: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==14086==    by 0x12886B: std::rt::lang_start_internal (rt.rs:148)
==14086==    by 0x114859: std::rt::lang_start (rt.rs:165)
==14086==    by 0x1142FD: main (in /home/wink/prgs/rust/myrepos/exper_sanitizer/target/debug/exper_sanitizer)
==14086== 
==14086== Conditional jump or move depends on uninitialised value(s)
==14086==    at 0x4848E5F: memrchr (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==14086==    by 0x12A37C: memrchr_specific (memchr.rs:23)
==14086==    by 0x12A37C: memrchr (memchr.rs:39)
==14086==    by 0x12A37C: memrchr (memchr.rs:50)
==14086==    by 0x12A37C: write_all<std::io::stdio::StdoutRaw> (linewritershim.rs:248)
==14086==    by 0x12A37C: write_all<std::io::stdio::StdoutRaw> (linewriter.rs:206)
==14086==    by 0x12A37C: <std::io::stdio::StdoutLock as std::io::Write>::write_all (stdio.rs:736)
==14086==    by 0x12B415: <std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str (mod.rs:1671)
==14086==    by 0x14ABC2: fmt_int<core::fmt::num::LowerHex, u32> (num.rs:110)
==14086==    by 0x14ABC2: core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt (num.rs:155)
==14086==    by 0x147271: run (mod.rs:1256)
==14086==    by 0x147271: core::fmt::write (mod.rs:1224)
==14086==    by 0x12A203: write_fmt<std::io::stdio::StdoutLock> (mod.rs:1682)
==14086==    by 0x12A203: <&std::io::stdio::Stdout as std::io::Write>::write_fmt (stdio.rs:716)
==14086==    by 0x12A8E2: write_fmt (stdio.rs:690)
==14086==    by 0x12A8E2: print_to<std::io::stdio::Stdout> (stdio.rs:1008)
==14086==    by 0x12A8E2: std::io::stdio::_print (stdio.rs:1075)
==14086==    by 0x1141C3: exper_sanitizer::main (main.rs:45)
==14086==    by 0x11257A: core::ops::function::FnOnce::call_once (function.rs:507)
==14086==    by 0x11480D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==14086==    by 0x114880: std::rt::lang_start::{{closure}} (rt.rs:166)
==14086==    by 0x12886B: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==14086==    by 0x12886B: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==14086==    by 0x12886B: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==14086==    by 0x12886B: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==14086==    by 0x12886B: {closure#2} (rt.rs:148)
==14086==    by 0x12886B: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==14086==    by 0x12886B: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==14086==    by 0x12886B: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==14086==    by 0x12886B: std::rt::lang_start_internal (rt.rs:148)
==14086== 
==14086== Syscall param write(buf) points to uninitialised byte(s)
==14086==    at 0x49980B4: write (write.c:26)
==14086==    by 0x1297B7: write (fd.rs:152)
==14086==    by 0x1297B7: write (stdio.rs:39)
==14086==    by 0x1297B7: write (stdio.rs:121)
==14086==    by 0x1297B7: std::io::buffered::bufwriter::BufWriter<W>::flush_buf (bufwriter.rs:166)
==14086==    by 0x12A50A: write_all<std::io::stdio::StdoutRaw> (linewritershim.rs:269)
==14086==    by 0x12A50A: write_all<std::io::stdio::StdoutRaw> (linewriter.rs:206)
==14086==    by 0x12A50A: <std::io::stdio::StdoutLock as std::io::Write>::write_all (stdio.rs:736)
==14086==    by 0x12B415: <std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str (mod.rs:1671)
==14086==    by 0x147321: core::fmt::write (mod.rs:1232)
==14086==    by 0x12A203: write_fmt<std::io::stdio::StdoutLock> (mod.rs:1682)
==14086==    by 0x12A203: <&std::io::stdio::Stdout as std::io::Write>::write_fmt (stdio.rs:716)
==14086==    by 0x12A8E2: write_fmt (stdio.rs:690)
==14086==    by 0x12A8E2: print_to<std::io::stdio::Stdout> (stdio.rs:1008)
==14086==    by 0x12A8E2: std::io::stdio::_print (stdio.rs:1075)
==14086==    by 0x1141C3: exper_sanitizer::main (main.rs:45)
==14086==    by 0x11257A: core::ops::function::FnOnce::call_once (function.rs:507)
==14086==    by 0x11480D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==14086==    by 0x114880: std::rt::lang_start::{{closure}} (rt.rs:166)
==14086==    by 0x12886B: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==14086==    by 0x12886B: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==14086==    by 0x12886B: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==14086==    by 0x12886B: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==14086==    by 0x12886B: {closure#2} (rt.rs:148)
==14086==    by 0x12886B: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==14086==    by 0x12886B: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==14086==    by 0x12886B: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==14086==    by 0x12886B: std::rt::lang_start_internal (rt.rs:148)
==14086==  Address 0x4a8bd90 is 16 bytes inside a block of size 1,024 alloc'd
==14086==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==14086==    by 0x10FB75: alloc (alloc.rs:95)
==14086==    by 0x10FB75: alloc_impl (alloc.rs:177)
==14086==    by 0x10FB75: allocate (alloc.rs:237)
==14086==    by 0x10FB75: allocate_in<u8, alloc::alloc::Global> (raw_vec.rs:185)
==14086==    by 0x10FB75: with_capacity_in<u8, alloc::alloc::Global> (raw_vec.rs:131)
==14086==    by 0x10FB75: with_capacity_in<u8, alloc::alloc::Global> (mod.rs:673)
==14086==    by 0x10FB75: with_capacity<u8> (mod.rs:483)
==14086==    by 0x10FB75: with_capacity<std::io::stdio::StdoutRaw> (bufwriter.rs:115)
==14086==    by 0x10FB75: with_capacity<std::io::stdio::StdoutRaw> (linewriter.rs:109)
==14086==    by 0x10FB75: new<std::io::stdio::StdoutRaw> (linewriter.rs:89)
==14086==    by 0x10FB75: {closure#0} (stdio.rs:607)
==14086==    by 0x10FB75: {closure#0}<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::io::stdio::stdout::{closure_env#0}> (once_lock.rs:172)
==14086==    by 0x10FB75: {closure#0}<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::sync::once_lock::{impl#0}::get_or_init::{closure_env#0}<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::io::stdio::stdout::{closure_env#0}>, !> (once_lock.rs:299)
==14086==    by 0x10FB75: {closure#0}<std::sync::once_lock::{impl#0}::initialize::{closure_env#0}<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::sync::once_lock::{impl#0}::get_or_init::{closure_env#0}<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::io::stdio::stdout::{closure_env#0}>, !>> (once.rs:202)
==14086==    by 0x10FB75: std::sys_common::once::futex::Once::call (futex.rs:113)
==14086==    by 0x10ED8D: call_once_force<std::sync::once_lock::{impl#0}::initialize::{closure_env#0}<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::sync::once_lock::{impl#0}::get_or_init::{closure_env#0}<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::io::stdio::stdout::{closure_env#0}>, !>> (once.rs:202)
==14086==    by 0x10ED8D: std::sync::once_lock::OnceLock<T>::initialize (once_lock.rs:298)
==14086==    by 0x12A8F5: get_or_try_init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::sync::once_lock::{impl#0}::get_or_init::{closure_env#0}<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::io::stdio::stdout::{closure_env#0}>, !> (once_lock.rs:219)
==14086==    by 0x12A8F5: get_or_init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::linewriter::LineWriter<std::io::stdio::StdoutRaw>>>, std::io::stdio::stdout::{closure_env#0}> (once_lock.rs:172)
==14086==    by 0x12A8F5: stdout (stdio.rs:606)
==14086==    by 0x12A8F5: print_to<std::io::stdio::Stdout> (stdio.rs:1008)
==14086==    by 0x12A8F5: std::io::stdio::_print (stdio.rs:1075)
==14086==    by 0x1141C3: exper_sanitizer::main (main.rs:45)
==14086==    by 0x11257A: core::ops::function::FnOnce::call_once (function.rs:507)
==14086==    by 0x11480D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:121)
==14086==    by 0x114880: std::rt::lang_start::{{closure}} (rt.rs:166)
==14086==    by 0x12886B: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:606)
==14086==    by 0x12886B: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:483)
==14086==    by 0x12886B: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:447)
==14086==    by 0x12886B: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
==14086==    by 0x12886B: {closure#2} (rt.rs:148)
==14086==    by 0x12886B: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:483)
==14086==    by 0x12886B: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:447)
==14086==    by 0x12886B: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
==14086==    by 0x12886B: std::rt::lang_start_internal (rt.rs:148)
==14086==    by 0x114859: std::rt::lang_start (rt.rs:165)
==14086==    by 0x1142FD: main (in /home/wink/prgs/rust/myrepos/exper_sanitizer/target/debug/exper_sanitizer)
==14086== 
main:- xs[-1]=0xffffffff
==14086== 
==14086== HEAP SUMMARY:
==14086==     in use at exit: 0 bytes in 0 blocks
==14086==   total heap usage: 16 allocs, 16 frees, 3,384 bytes allocated
==14086== 
==14086== All heap blocks were freed -- no leaks are possible
==14086== 
==14086== Use --track-origins=yes to see where uninitialised values come from
==14086== For lists of detected and suppressed errors, rerun with: -s
==14086== ERROR SUMMARY: 16 errors from 3 contexts (suppressed: 0 from 0)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
