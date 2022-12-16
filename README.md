# Experiment with AddressSanitizer

So AddressSanitizer works well, better than valgrind.
Although, valgrind is "easier" to setup, i.e. you don't
have to do anything execpt run the executable.

Question how to add `--target` to Cargo.toml or .cargo/config.toml

https://discord.com/channels/273534239310479360/335502067432947748/1053084774379561011

But this technique won't allow me to run a "regular" build:

```
wink@3900x 22-12-16T15:52:05.971Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo build
error: -Zbuild-std requires --target
```

## Build:

```
wink@3900x 22-12-16T15:48:00.570Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo build --profile=sanitizer --target x86_64-unknown-linux-gnu
warning: unused manifest key: build
   Compiling compiler_builtins v0.1.85
   Compiling core v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling libc v0.2.138
   Compiling cc v1.0.76
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std)
   Compiling unwind v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
   Compiling rustc-std-workspace-alloc v1.99.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-alloc)
   Compiling panic_abort v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/panic_abort)
   Compiling panic_unwind v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/panic_unwind)
   Compiling gimli v0.26.1
   Compiling hashbrown v0.12.3
   Compiling miniz_oxide v0.5.3
   Compiling object v0.29.0
   Compiling std_detect v0.1.5 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect)
   Compiling addr2line v0.17.0
   Compiling proc_macro v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro)
   Compiling exper_sanitizer v0.2.0 (/home/wink/prgs/rust/myrepos/exper_sanitizer)
    Finished sanitizer [unoptimized + debuginfo] target(s) in 12.29s
```

## Run:

Run with array on stack and getting first element
```
wink@3900x 22-12-16T15:48:15.227Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo run --profile=sanitizer --target x86_64-unknown-linux-gnu -- stack 0
warning: unused manifest key: build
    Finished sanitizer [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer stack 0`
main:- xs[0]=0x0
```

Run with array on heap and getting last element
```
wink@3900x 22-12-16T15:49:33.269Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo run --profile=sanitizer --target x86_64-unknown-linux-gnu -- heap 3
warning: unused manifest key: build
    Finished sanitizer [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer heap 3`
main:- xs[3]=0x3
```

Run indexing past the last element and AddressSanitizer screams at us:
```
wink@3900x 22-12-16T15:50:04.077Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo run --profile=sanitizer --target x86_64-unknown-linux-gnu -- heap 4
warning: unused manifest key: build
    Finished sanitizer [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer heap 4`
=================================================================
==8257==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x602000000080 at pc 0x55e9e1b88ede bp 0x7ffeeb54dfb0 sp 0x7ffeeb54dfa8
READ of size 4 at 0x602000000080 thread T0
    #0 0x55e9e1b88edd in exper_sanitizer::main::ha438dd185b8326be /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:34:22
    #1 0x55e9e1b893ea in core::ops::function::FnOnce::call_once::h5c5a19036119e3c7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    #2 0x55e9e1b86527 in std::sys_common::backtrace::__rust_begin_short_backtrace::h61eb74c7a690b1ec /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:121:18
    #3 0x55e9e1b89f23 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::had040cfbcb9a20e5 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    #4 0x55e9e1ed1c1d in core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h750779b9c5158193 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:606:13
    #5 0x55e9e1cbde4e in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #6 0x55e9e1cc19da in __rust_try std.63ac60a6-cgu.35
    #7 0x55e9e1cbca75 in std::panicking::try::h2155994ae210243d /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #8 0x55e9e1bd0219 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #9 0x55e9e1bc8540 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    #10 0x55e9e1cbe026 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #11 0x55e9e1cc19da in __rust_try std.63ac60a6-cgu.35
    #12 0x55e9e1cbd065 in std::panicking::try::hd2b4bb59e4bb40ad /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #13 0x55e9e1bd00a9 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #14 0x55e9e1bc7f75 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    #15 0x55e9e1b89e7f in std::rt::lang_start::hcdd154a7a9f20701 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    #16 0x55e9e1b8939d in main (/home/wink/prgs/rust/myrepos/exper_sanitizer/target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer+0x19939d) (BuildId: 9976a65f81b85d8d61a70f6b679b204ff9ec5494)
    #17 0x7f318849628f  (/usr/lib/libc.so.6+0x2328f) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    #18 0x7f3188496349 in __libc_start_main (/usr/lib/libc.so.6+0x23349) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    #19 0x55e9e1ae6254 in _start /build/glibc/src/glibc/csu/../sysdeps/x86_64/start.S:115

0x602000000080 is located 0 bytes to the right of 16-byte region [0x602000000070,0x602000000080)
allocated by thread T0 here:
    #0 0x55e9e1b5acbe in malloc /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_malloc_linux.cpp:69:3
    #1 0x55e9e1bcc8a6 in std::sys::unix::alloc::_$LT$impl$u20$core..alloc..global..GlobalAlloc$u20$for$u20$std..alloc..System$GT$::alloc::hf2cbfb265a23c366 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:14:13
    #2 0x55e9e1c29d4f in __rdl_alloc /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/alloc.rs:381:13
    #3 0x55e9e1b85856 in alloc::alloc::alloc::hd12f4b1d28c39726 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:95:14
    #4 0x55e9e1b85b5f in alloc::alloc::Global::alloc_impl::hc22a9f5a02780e58 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:177:73
    #5 0x55e9e1b863f0 in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::allocate::h2fd53543f7a49e8c /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:237:9
    #6 0x55e9e1b85591 in alloc::alloc::exchange_malloc::hbfb1b1019fd97450 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:326:11
    #7 0x55e9e1b88c21 in alloc::boxed::Box$LT$T$GT$::new::h85e7f3fea1f1a87b /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:220:9
    #8 0x55e9e1b88c21 in exper_sanitizer::main::ha438dd185b8326be /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:33:22
    #9 0x55e9e1b893ea in core::ops::function::FnOnce::call_once::h5c5a19036119e3c7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    #10 0x55e9e1b89f23 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::had040cfbcb9a20e5 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    #11 0x55e9e1cbde4e in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #12 0x55e9e1cc19da in __rust_try std.63ac60a6-cgu.35
    #13 0x55e9e1bd0219 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #14 0x55e9e1bc8540 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    #15 0x55e9e1cbe026 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #16 0x55e9e1cc19da in __rust_try std.63ac60a6-cgu.35
    #17 0x55e9e1bd00a9 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #18 0x55e9e1bc7f75 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    #19 0x55e9e1b89e7f in std::rt::lang_start::hcdd154a7a9f20701 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    #20 0x55e9e1b8939d in main (/home/wink/prgs/rust/myrepos/exper_sanitizer/target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer+0x19939d) (BuildId: 9976a65f81b85d8d61a70f6b679b204ff9ec5494)

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
==8257==ABORTING
```

And also on the stack before the first element:
```
wink@3900x 22-12-16T15:50:53.699Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo run --profile=sanitizer --target x86_64-unknown-linux-gnu -- stack -1
warning: unused manifest key: build
    Finished sanitizer [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer stack -1`
=================================================================
==8341==ERROR: AddressSanitizer: stack-buffer-overflow on address 0x7fde0ed0021c at pc 0x55795ac539b9 bp 0x7fff4a4cedf0 sp 0x7fff4a4cede8
READ of size 4 at 0x7fde0ed0021c thread T0
    #0 0x55795ac539b8 in exper_sanitizer::main::ha438dd185b8326be /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:30:22
    #1 0x55795ac543ea in core::ops::function::FnOnce::call_once::h5c5a19036119e3c7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    #2 0x55795ac51527 in std::sys_common::backtrace::__rust_begin_short_backtrace::h61eb74c7a690b1ec /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:121:18
    #3 0x55795ac54f23 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::had040cfbcb9a20e5 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    #4 0x55795af9cc1d in core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h750779b9c5158193 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:606:13
    #5 0x55795ad88e4e in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #6 0x55795ad8c9da in __rust_try std.63ac60a6-cgu.35
    #7 0x55795ad87a75 in std::panicking::try::h2155994ae210243d /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #8 0x55795ac9b219 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #9 0x55795ac93540 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    #10 0x55795ad89026 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #11 0x55795ad8c9da in __rust_try std.63ac60a6-cgu.35
    #12 0x55795ad88065 in std::panicking::try::hd2b4bb59e4bb40ad /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #13 0x55795ac9b0a9 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #14 0x55795ac92f75 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    #15 0x55795ac54e7f in std::rt::lang_start::hcdd154a7a9f20701 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    #16 0x55795ac5439d in main (/home/wink/prgs/rust/myrepos/exper_sanitizer/target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer+0x19939d) (BuildId: 9976a65f81b85d8d61a70f6b679b204ff9ec5494)
    #17 0x7fde105c828f  (/usr/lib/libc.so.6+0x2328f) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    #18 0x7fde105c8349 in __libc_start_main (/usr/lib/libc.so.6+0x23349) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    #19 0x55795abb1254 in _start /build/glibc/src/glibc/csu/../sysdeps/x86_64/start.S:115

Address 0x7fde0ed0021c is located in stack of thread T0 at offset 540 in frame
    #0 0x55795ac52ddf in alloc::boxed::Box$LT$T$GT$::new::h85e7f3fea1f1a87b /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:10
    #1 0x55795ac52ddf in exper_sanitizer::main::ha438dd185b8326be /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:33:22

  This frame has 23 object(s):
    [32, 48) ''
    [64, 80) 'self.dbg.spill.i6'
    [96, 112) 'self.dbg.spill.i5'
    [128, 144) '' (line 10)
    [160, 192) '_80' (line 45)
    [224, 272) '_73' (line 45)
    [304, 328) '_71' (line 41)
    [368, 384) '_65' (line 37)
    [400, 448) '_58' (line 37)
    [480, 496) '_51' (line 33)
    [512, 520) 'xs1' (line 33)
    [544, 560) 'xs' (line 29) <== Memory access at offset 540 underflows this variable
    [576, 580) 'val' (line 27)
    [592, 616) '_36' (line 21)
    [656, 672) '_32' (line 20)
    [688, 736) '_25' (line 20)
    [768, 769) 'e' (line 19)
    [784, 800) '_15' (line 17)
    [816, 824) 'index' (line 17)
    [848, 872) '_13' (line 14)
    [912, 960) '_6' (line 13)
    [992, 1024) '_2' (line 11)
    [1056, 1080) 'args' (line 11)
HINT: this may be a false positive if your program uses some custom stack unwind mechanism, swapcontext or vfork
      (longjmp and C++ exceptions *are* supported)
SUMMARY: AddressSanitizer: stack-buffer-overflow /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:30:22 in exper_sanitizer::main::ha438dd185b8326be
Shadow bytes around the buggy address:
  0x0ffc41d97ff0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0ffc41d98000: f1 f1 f1 f1 f8 f8 f2 f2 00 00 f2 f2 00 00 f2 f2
  0x0ffc41d98010: f8 f8 f2 f2 f8 f8 f8 f8 f2 f2 f2 f2 f8 f8 f8 f8
  0x0ffc41d98020: f8 f8 f2 f2 f2 f2 f8 f8 f8 f2 f2 f2 f2 f2 f8 f8
  0x0ffc41d98030: f2 f2 f8 f8 f8 f8 f8 f8 f2 f2 f2 f2 f8 f8 f2 f2
=>0x0ffc41d98040: f8 f2 f2[f2]00 00 f2 f2 04 f2 f8 f8 f8 f2 f2 f2
  0x0ffc41d98050: f2 f2 f8 f8 f2 f2 f8 f8 f8 f8 f8 f8 f2 f2 f2 f2
  0x0ffc41d98060: f8 f2 f8 f8 f2 f2 00 f2 f2 f2 f8 f8 f8 f2 f2 f2
  0x0ffc41d98070: f2 f2 f8 f8 f8 f8 f8 f8 f2 f2 f2 f2 f8 f8 f8 f8
  0x0ffc41d98080: f2 f2 f2 f2 00 00 00 f3 f3 f3 f3 f3 00 00 00 00
  0x0ffc41d98090: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
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
==8341==ABORTING
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
