# Experiment with AddressSanitizer

So AddressSanitizer works well, better than valgrind.
Although, valgrind is "easier" to setup, i.e. you don't
have to do anything execpt run the executable.

Question how to add `--target` to Cargo.toml or .cargo/config.toml

https://discord.com/channels/273534239310479360/335502067432947748/1053084774379561011

## Build:

```
$ cargo build --profile=sanitizer --target x86_64-unknown-linux-gnu
warning: unused manifest key: build
   Compiling compiler_builtins v0.1.85
   Compiling core v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   ...
   Compiling proc_macro v0.0.0 (/home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro)
   Compiling exper_sanitizer v0.2.0 (/home/wink/prgs/rust/myrepos/exper_sanitizer)
    Finished sanitizer [unoptimized + debuginfo] target(s) in 12.11s
```

## Run:

Run with array on stack and getting first element
```
wink@3900x 22-12-15T23:13:56.762Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo run --profile=sanitizer --target x86_64-unknown-linux-gnu -- stack 0
warning: unused manifest key: build
    Finished sanitizer [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer stack 0`
main:- xs[0]=0x0
```

Run with array on heap and getting last element
```
wink@3900x 22-12-15T23:14:57.862Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo run --profile=sanitizer --target x86_64-unknown-linux-gnu -- heap 3
warning: unused manifest key: build
    Finished sanitizer [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer heap 3`
main:- xs[3]=0x3
```

Run indexing pass the last element and AddressSanitizer screams as us:
```
wink@3900x 22-12-15T23:15:03.543Z:~/prgs/rust/myrepos/exper_sanitizer (main)
$ cargo run --profile=sanitizer --target x86_64-unknown-linux-gnu -- heap 4
warning: unused manifest key: build
    Finished sanitizer [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer heap 4`
=================================================================
==98017==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x602000000080 at pc 0x55c9ac508ede bp 0x7ffc107b47d0 sp 0x7ffc107b47c8
READ of size 4 at 0x602000000080 thread T0
    #0 0x55c9ac508edd in exper_sanitizer::main::ha438dd185b8326be /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:34:22
    #1 0x55c9ac5093ea in core::ops::function::FnOnce::call_once::h5c5a19036119e3c7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    #2 0x55c9ac506527 in std::sys_common::backtrace::__rust_begin_short_backtrace::h61eb74c7a690b1ec /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:121:18
    #3 0x55c9ac509f23 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::had040cfbcb9a20e5 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    #4 0x55c9ac851c1d in core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h750779b9c5158193 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:606:13
    #5 0x55c9ac63de4e in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #6 0x55c9ac6419da in __rust_try std.63ac60a6-cgu.35
    #7 0x55c9ac63ca75 in std::panicking::try::h2155994ae210243d /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #8 0x55c9ac550219 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #9 0x55c9ac548540 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    #10 0x55c9ac63e026 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #11 0x55c9ac6419da in __rust_try std.63ac60a6-cgu.35
    #12 0x55c9ac63d065 in std::panicking::try::hd2b4bb59e4bb40ad /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:447:19
    #13 0x55c9ac5500a9 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #14 0x55c9ac547f75 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    #15 0x55c9ac509e7f in std::rt::lang_start::hcdd154a7a9f20701 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    #16 0x55c9ac50939d in main (/home/wink/prgs/rust/myrepos/exper_sanitizer/target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer+0x19939d) (BuildId: 9976a65f81b85d8d61a70f6b679b204ff9ec5494)
    #17 0x7f10798c028f  (/usr/lib/libc.so.6+0x2328f) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    #18 0x7f10798c0349 in __libc_start_main (/usr/lib/libc.so.6+0x23349) (BuildId: 1e94beb079e278ac4f2c8bce1f53091548ea1584)
    #19 0x55c9ac466254 in _start /build/glibc/src/glibc/csu/../sysdeps/x86_64/start.S:115

0x602000000080 is located 0 bytes to the right of 16-byte region [0x602000000070,0x602000000080)
allocated by thread T0 here:
    #0 0x55c9ac4dacbe in malloc /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_malloc_linux.cpp:69:3
    #1 0x55c9ac54c8a6 in std::sys::unix::alloc::_$LT$impl$u20$core..alloc..global..GlobalAlloc$u20$for$u20$std..alloc..System$GT$::alloc::hf2cbfb265a23c366 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:14:13
    #2 0x55c9ac5a9d4f in __rdl_alloc /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/alloc.rs:381:13
    #3 0x55c9ac505856 in alloc::alloc::alloc::hd12f4b1d28c39726 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:95:14
    #4 0x55c9ac505b5f in alloc::alloc::Global::alloc_impl::hc22a9f5a02780e58 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:177:73
    #5 0x55c9ac5063f0 in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::allocate::h2fd53543f7a49e8c /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:237:9
    #6 0x55c9ac505591 in alloc::alloc::exchange_malloc::hbfb1b1019fd97450 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:326:11
    #7 0x55c9ac508c21 in alloc::boxed::Box$LT$T$GT$::new::h85e7f3fea1f1a87b /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:220:9
    #8 0x55c9ac508c21 in exper_sanitizer::main::ha438dd185b8326be /home/wink/prgs/rust/myrepos/exper_sanitizer/src/main.rs:33:22
    #9 0x55c9ac5093ea in core::ops::function::FnOnce::call_once::h5c5a19036119e3c7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:507:5
    #10 0x55c9ac509f23 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::had040cfbcb9a20e5 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:166:18
    #11 0x55c9ac63de4e in std::panicking::try::do_call::h41b4d73441e5a27f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #12 0x55c9ac6419da in __rust_try std.63ac60a6-cgu.35
    #13 0x55c9ac550219 in std::panic::catch_unwind::hc7326399cad39c91 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #14 0x55c9ac548540 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h0fa04e3694c250f7 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:48
    #15 0x55c9ac63e026 in std::panicking::try::do_call::h928ac4600abff174 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:483:40
    #16 0x55c9ac6419da in __rust_try std.63ac60a6-cgu.35
    #17 0x55c9ac5500a9 in std::panic::catch_unwind::h8883ea9e4dca3787 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:137:14
    #18 0x55c9ac547f75 in std::rt::lang_start_internal::hceb5e973a747532f /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:148:20
    #19 0x55c9ac509e7f in std::rt::lang_start::hcdd154a7a9f20701 /home/wink/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:165:17
    #20 0x55c9ac50939d in main (/home/wink/prgs/rust/myrepos/exper_sanitizer/target/x86_64-unknown-linux-gnu/sanitizer/exper_sanitizer+0x19939d) (BuildId: 9976a65f81b85d8d61a70f6b679b204ff9ec5494)

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
==98017==ABORTING
```

A most interesting thing is 
## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
