; ModuleID = '-'
source_filename = "-"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx15.0.0"

%struct._opaque_pthread_mutex_t = type { i64, [56 x i8] }
%struct.EpChannel = type { ptr, i64, i64, i64, i64, %struct._opaque_pthread_mutex_t, %struct._opaque_pthread_cond_t, %struct._opaque_pthread_cond_t }
%struct._opaque_pthread_cond_t = type { i64, [40 x i8] }
%struct.timespec = type { i64, i64 }
%struct.EpList = type { ptr, i64, i64 }
%struct.sockaddr_in = type { i8, i8, i16, %struct.in_addr, [8 x i8] }
%struct.in_addr = type { i32 }
%struct.hostent = type { ptr, ptr, i32, i32, ptr }
%union.ep_float_bits = type { i64 }
%struct.EpMap = type { ptr, i64, i64 }
%struct.EpMapEntry = type { ptr, i64, i32 }
%struct.EpGCObject = type { i32, i32, ptr, i64, i64, i32, ptr }
%struct.EpDeque = type { ptr, i64, i64, i64, i64 }
%struct.dirent = type { i64, i64, i16, i16, i8, [1024 x i8] }
%struct.stat = type { i32, i16, i16, i64, i32, i32, i32, %struct.timespec, %struct.timespec, %struct.timespec, %struct.timespec, i64, i64, i32, i32, i32, i32, [2 x i64] }
%struct.EP_SHA256_CTX = type { [64 x i8], i32, i64, [8 x i32] }
%struct.EP_MD5_CTX = type { [2 x i32], [4 x i32], [64 x i8] }
%struct.EpThreadGCState = type { [4096 x ptr], i32 }
%struct.timeval = type { i64, i32 }
%struct.tm = type { i32, i32, i32, i32, i32, i32, i32, i32, i32, i64, ptr }
%struct.EpBarrier = type { %struct._opaque_pthread_mutex_t, %struct._opaque_pthread_cond_t, i32, i32, i32 }
%struct.EpSemaphore = type { %struct._opaque_pthread_mutex_t, %struct._opaque_pthread_cond_t, i64 }
%struct.regex_t = type { i32, i64, ptr, ptr }
%struct.regmatch_t = type { i64, i64 }
%struct.EpStruct_Node = type { i64, i64 }
%struct.EpGCEntry = type { ptr, ptr }

@ep_gc_enabled = internal global i32 1, align 4
@.str = private unnamed_addr constant [14 x i8] c"afplay '%s' &\00", align 1
@.str.1 = private unnamed_addr constant [2 x i8] c".\00", align 1
@.str.2 = private unnamed_addr constant [3 x i8] c"..\00", align 1
@.str.3 = private unnamed_addr constant [3 x i8] c"rb\00", align 1
@.str.4 = private unnamed_addr constant [3 x i8] c"wb\00", align 1
@.str.5 = private unnamed_addr constant [1 x i8] zeroinitializer, align 1
@.str.6 = private unnamed_addr constant [8 x i8] c"http://\00", align 1
@.str.7 = private unnamed_addr constant [39 x i8] c"Error: only http:// protocol supported\00", align 1
@.str.8 = private unnamed_addr constant [2 x i8] c"/\00", align 1
@.str.9 = private unnamed_addr constant [30 x i8] c"Error: socket creation failed\00", align 1
@.str.10 = private unnamed_addr constant [30 x i8] c"Error: host resolution failed\00", align 1
@.str.11 = private unnamed_addr constant [25 x i8] c"Error: connection failed\00", align 1
@.str.12 = private unnamed_addr constant [73 x i8] c"%s %s HTTP/1.1\0D\0AHost: %s\0D\0AContent-Length: %zu\0D\0AConnection: close\0D\0A%s%s\0D\0A\00", align 1
@.str.13 = private unnamed_addr constant [3 x i8] c"\0D\0A\00", align 1
@.str.14 = private unnamed_addr constant [19 x i8] c"Error: send failed\00", align 1
@.str.15 = private unnamed_addr constant [24 x i8] c"Error: send body failed\00", align 1
@.str.16 = private unnamed_addr constant [32 x i8] c"Error: memory allocation failed\00", align 1
@.str.17 = private unnamed_addr constant [5 x i8] c"\0D\0A\0D\0A\00", align 1
@sha256_k = internal constant [64 x i32] [i32 1116352408, i32 1899447441, i32 -1245643825, i32 -373957723, i32 961987163, i32 1508970993, i32 -1841331548, i32 -1424204075, i32 -670586216, i32 310598401, i32 607225278, i32 1426881987, i32 1925078388, i32 -2132889090, i32 -1680079193, i32 -1046744716, i32 -459576895, i32 -272742522, i32 264347078, i32 604807628, i32 770255983, i32 1249150122, i32 1555081692, i32 1996064986, i32 -1740746414, i32 -1473132947, i32 -1341970488, i32 -1084653625, i32 -958395405, i32 -710438585, i32 113926993, i32 338241895, i32 666307205, i32 773529912, i32 1294757372, i32 1396182291, i32 1695183700, i32 1986661051, i32 -2117940946, i32 -1838011259, i32 -1564481375, i32 -1474664885, i32 -1035236496, i32 -949202525, i32 -778901479, i32 -694614492, i32 -200395387, i32 275423344, i32 430227734, i32 506948616, i32 659060556, i32 883997877, i32 958139571, i32 1322822218, i32 1537002063, i32 1747873779, i32 1955562222, i32 2024104815, i32 -2067236844, i32 -1933114872, i32 -1866530822, i32 -1538233109, i32 -1090935817, i32 -965641998], align 4
@.str.18 = private unnamed_addr constant [5 x i8] c"%02x\00", align 1
@ep_argc = global i32 0, align 4
@ep_argv = global ptr null, align 8
@.str.19 = private unnamed_addr constant [3 x i8] c"ab\00", align 1
@.str.20 = private unnamed_addr constant [6 x i8] c"macos\00", align 1
@.str.21 = private unnamed_addr constant [6 x i8] c"arm64\00", align 1
@.str.22 = private unnamed_addr constant [5 x i8] c"HOME\00", align 1
@.str.23 = private unnamed_addr constant [2 x i8] c"r\00", align 1
@b64_table = internal constant [65 x i8] c"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\00", align 1
@.str.24 = private unnamed_addr constant [69 x i8] c"%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x\00", align 1
@.str.25 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@mach_task_self_ = external global i32, align 4
@.str.26 = private unnamed_addr constant [5 x i8] c"%lld\00", align 1
@.str.27 = private unnamed_addr constant [5 x i8] c"true\00", align 1
@ep_gc_minor_count = internal global i32 0, align 4
@ep_gc_major_count = internal global i32 0, align 4
@ep_gc_nursery_count = internal global i64 0, align 8
@__stdinp = external global ptr, align 8
@.str.28 = private unnamed_addr constant [4 x i8] c"%lf\00", align 1
@.str.29 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@.str.30 = private unnamed_addr constant [29 x i8] c"--- Generational GC Test ---\00", align 1
@.str.31 = private unnamed_addr constant [22 x i8] c"Initial minor count: \00", align 1
@.str.32 = private unnamed_addr constant [28 x i8] c"Minor count after trigger: \00", align 1
@__stderrp = external global ptr, align 8
@.str.33 = private unnamed_addr constant [59 x i8] c"Error: Null pointer when accessing field 'next' on 'Node'\0A\00", align 1
@.str.34 = private unnamed_addr constant [58 x i8] c"Error: Null pointer when accessing field 'val' on 'Node'\0A\00", align 1
@.str.35 = private unnamed_addr constant [26 x i8] c"Survived young node val: \00", align 1
@.str.36 = private unnamed_addr constant [28 x i8] c"Generational GC test passed\00", align 1
@.str.37 = private unnamed_addr constant [13 x i8] c"/dev/urandom\00", align 1
@ep_try_active = internal global i32 0, align 4
@ep_try_buf = internal global [48 x i32] zeroinitializer, align 4
@.str.38 = private unnamed_addr constant [59 x i8] c"segmentation fault (null pointer or invalid memory access)\00", align 1
@.str.39 = private unnamed_addr constant [36 x i8] c"arithmetic error (division by zero)\00", align 1
@.str.40 = private unnamed_addr constant [8 x i8] c"aborted\00", align 1
@.str.41 = private unnamed_addr constant [15 x i8] c"unknown signal\00", align 1
@.str.42 = private unnamed_addr constant [32 x i8] c"\0ARuntime Error: %s (signal %d)\0A\00", align 1
@ep_gc_mutex = internal global %struct._opaque_pthread_mutex_t { i64 850045863, [56 x i8] zeroinitializer }, align 8
@ep_gc_remembered_size = internal global i64 0, align 8
@ep_gc_remembered_set = internal global ptr null, align 8
@ep_gc_remembered_cap = internal global i64 0, align 8
@ep_gc_table_cap = internal global i64 0, align 8
@ep_gc_table = internal global ptr null, align 8
@ep_gc_head = internal global ptr null, align 8
@ep_gc_count = internal global i64 0, align 8
@ep_gc_table_size = internal global i64 0, align 8
@ep_thread_local_bottom = internal thread_local global ptr null, align 8
@ep_thread_local_top = internal thread_local global ptr null, align 8
@ep_num_threads = internal global i32 0, align 4
@ep_thread_active = internal global [256 x i32] zeroinitializer, align 4
@ep_thread_tops = internal global [256 x ptr] zeroinitializer, align 8
@ep_thread_bottoms = internal global [256 x ptr] zeroinitializer, align 8
@ep_thread_gc_states = internal global [256 x ptr] zeroinitializer, align 8
@ep_thread_slot = internal thread_local global i32 -1, align 4
@ep_gc_root_sp = internal thread_local global i32 0, align 4
@ep_gc_root_stack = internal thread_local global [4096 x ptr] zeroinitializer, align 8
@ep_gc_nursery_threshold = internal global i64 512, align 8
@ep_gc_threshold = internal global i64 4096, align 8
@llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @ep_install_signal_handlers, ptr null }]

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_install_signal_handlers() #0 {
  %1 = call ptr @signal(i32 noundef 8, ptr noundef @ep_signal_handler)
  %2 = call ptr @signal(i32 noundef 11, ptr noundef @ep_signal_handler)
  %3 = call ptr @signal(i32 noundef 6, ptr noundef @ep_signal_handler)
  ret void
}

declare ptr @signal(i32 noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_signal_handler(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca ptr, align 8
  store i32 %0, ptr %2, align 4
  %4 = load volatile i32, ptr @ep_try_active, align 4
  %5 = icmp ne i32 %4, 0
  br i1 %5, label %6, label %8

6:                                                ; preds = %1
  store volatile i32 0, ptr @ep_try_active, align 4
  %7 = load i32, ptr %2, align 4
  call void @longjmp(ptr noundef @ep_try_buf, i32 noundef %7) #12
  unreachable

8:                                                ; preds = %1
  %9 = load i32, ptr %2, align 4
  %10 = icmp eq i32 %9, 11
  br i1 %10, label %11, label %12

11:                                               ; preds = %8
  br label %23

12:                                               ; preds = %8
  %13 = load i32, ptr %2, align 4
  %14 = icmp eq i32 %13, 8
  br i1 %14, label %15, label %16

15:                                               ; preds = %12
  br label %21

16:                                               ; preds = %12
  %17 = load i32, ptr %2, align 4
  %18 = icmp eq i32 %17, 6
  %19 = zext i1 %18 to i64
  %20 = select i1 %18, ptr @.str.40, ptr @.str.41
  br label %21

21:                                               ; preds = %16, %15
  %22 = phi ptr [ @.str.39, %15 ], [ %20, %16 ]
  br label %23

23:                                               ; preds = %21, %11
  %24 = phi ptr [ @.str.38, %11 ], [ %22, %21 ]
  store ptr %24, ptr %3, align 8
  %25 = load ptr, ptr @__stderrp, align 8
  %26 = load ptr, ptr %3, align 8
  %27 = load i32, ptr %2, align 4
  %28 = call i32 (ptr, ptr, ...) @fprintf(ptr noundef %25, ptr noundef @.str.42, ptr noundef %26, i32 noundef %27) #13
  %29 = load i32, ptr %2, align 4
  %30 = add nsw i32 128, %29
  call void @_exit(i32 noundef %30) #12
  unreachable
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @create_channel() #0 {
  %1 = alloca i64, align 8
  %2 = alloca ptr, align 8
  %3 = call ptr @malloc(i64 noundef 200) #14
  store ptr %3, ptr %2, align 8
  %4 = load ptr, ptr %2, align 8
  %5 = icmp ne ptr %4, null
  br i1 %5, label %7, label %6

6:                                                ; preds = %0
  store i64 0, ptr %1, align 8
  br label %34

7:                                                ; preds = %0
  %8 = load ptr, ptr %2, align 8
  %9 = getelementptr inbounds %struct.EpChannel, ptr %8, i32 0, i32 1
  store i64 1024, ptr %9, align 8
  %10 = load ptr, ptr %2, align 8
  %11 = getelementptr inbounds %struct.EpChannel, ptr %10, i32 0, i32 1
  %12 = load i64, ptr %11, align 8
  %13 = mul i64 %12, 8
  %14 = call ptr @malloc(i64 noundef %13) #14
  %15 = load ptr, ptr %2, align 8
  %16 = getelementptr inbounds %struct.EpChannel, ptr %15, i32 0, i32 0
  store ptr %14, ptr %16, align 8
  %17 = load ptr, ptr %2, align 8
  %18 = getelementptr inbounds %struct.EpChannel, ptr %17, i32 0, i32 2
  store i64 0, ptr %18, align 8
  %19 = load ptr, ptr %2, align 8
  %20 = getelementptr inbounds %struct.EpChannel, ptr %19, i32 0, i32 3
  store i64 0, ptr %20, align 8
  %21 = load ptr, ptr %2, align 8
  %22 = getelementptr inbounds %struct.EpChannel, ptr %21, i32 0, i32 4
  store i64 0, ptr %22, align 8
  %23 = load ptr, ptr %2, align 8
  %24 = getelementptr inbounds %struct.EpChannel, ptr %23, i32 0, i32 5
  %25 = call i32 @pthread_mutex_init(ptr noundef %24, ptr noundef null)
  %26 = load ptr, ptr %2, align 8
  %27 = getelementptr inbounds %struct.EpChannel, ptr %26, i32 0, i32 6
  %28 = call i32 @"\01_pthread_cond_init"(ptr noundef %27, ptr noundef null)
  %29 = load ptr, ptr %2, align 8
  %30 = getelementptr inbounds %struct.EpChannel, ptr %29, i32 0, i32 7
  %31 = call i32 @"\01_pthread_cond_init"(ptr noundef %30, ptr noundef null)
  %32 = load ptr, ptr %2, align 8
  %33 = ptrtoint ptr %32 to i64
  store i64 %33, ptr %1, align 8
  br label %34

34:                                               ; preds = %7, %6
  %35 = load i64, ptr %1, align 8
  ret i64 %35
}

; Function Attrs: allocsize(0)
declare ptr @malloc(i64 noundef) #2

declare i32 @pthread_mutex_init(ptr noundef, ptr noundef) #1

declare i32 @"\01_pthread_cond_init"(ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @send_channel(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i32, align 4
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %8 = load i64, ptr %4, align 8
  %9 = inttoptr i64 %8 to ptr
  store ptr %9, ptr %6, align 8
  %10 = load ptr, ptr %6, align 8
  %11 = icmp ne ptr %10, null
  br i1 %11, label %13, label %12

12:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %63

13:                                               ; preds = %2
  %14 = load i32, ptr @ep_gc_enabled, align 4
  store i32 %14, ptr %7, align 4
  store i32 0, ptr @ep_gc_enabled, align 4
  %15 = load ptr, ptr %6, align 8
  %16 = getelementptr inbounds %struct.EpChannel, ptr %15, i32 0, i32 5
  %17 = call i32 @pthread_mutex_lock(ptr noundef %16)
  br label %18

18:                                               ; preds = %26, %13
  %19 = load ptr, ptr %6, align 8
  %20 = getelementptr inbounds %struct.EpChannel, ptr %19, i32 0, i32 4
  %21 = load i64, ptr %20, align 8
  %22 = load ptr, ptr %6, align 8
  %23 = getelementptr inbounds %struct.EpChannel, ptr %22, i32 0, i32 1
  %24 = load i64, ptr %23, align 8
  %25 = icmp sge i64 %21, %24
  br i1 %25, label %26, label %32

26:                                               ; preds = %18
  %27 = load ptr, ptr %6, align 8
  %28 = getelementptr inbounds %struct.EpChannel, ptr %27, i32 0, i32 7
  %29 = load ptr, ptr %6, align 8
  %30 = getelementptr inbounds %struct.EpChannel, ptr %29, i32 0, i32 5
  %31 = call i32 @"\01_pthread_cond_wait"(ptr noundef %28, ptr noundef %30)
  br label %18, !llvm.loop !6

32:                                               ; preds = %18
  %33 = load i64, ptr %5, align 8
  %34 = load ptr, ptr %6, align 8
  %35 = getelementptr inbounds %struct.EpChannel, ptr %34, i32 0, i32 0
  %36 = load ptr, ptr %35, align 8
  %37 = load ptr, ptr %6, align 8
  %38 = getelementptr inbounds %struct.EpChannel, ptr %37, i32 0, i32 3
  %39 = load i64, ptr %38, align 8
  %40 = getelementptr inbounds i64, ptr %36, i64 %39
  store i64 %33, ptr %40, align 8
  %41 = load ptr, ptr %6, align 8
  %42 = getelementptr inbounds %struct.EpChannel, ptr %41, i32 0, i32 3
  %43 = load i64, ptr %42, align 8
  %44 = add nsw i64 %43, 1
  %45 = load ptr, ptr %6, align 8
  %46 = getelementptr inbounds %struct.EpChannel, ptr %45, i32 0, i32 1
  %47 = load i64, ptr %46, align 8
  %48 = srem i64 %44, %47
  %49 = load ptr, ptr %6, align 8
  %50 = getelementptr inbounds %struct.EpChannel, ptr %49, i32 0, i32 3
  store i64 %48, ptr %50, align 8
  %51 = load ptr, ptr %6, align 8
  %52 = getelementptr inbounds %struct.EpChannel, ptr %51, i32 0, i32 4
  %53 = load i64, ptr %52, align 8
  %54 = add nsw i64 %53, 1
  store i64 %54, ptr %52, align 8
  %55 = load ptr, ptr %6, align 8
  %56 = getelementptr inbounds %struct.EpChannel, ptr %55, i32 0, i32 6
  %57 = call i32 @pthread_cond_signal(ptr noundef %56)
  %58 = load ptr, ptr %6, align 8
  %59 = getelementptr inbounds %struct.EpChannel, ptr %58, i32 0, i32 5
  %60 = call i32 @pthread_mutex_unlock(ptr noundef %59)
  %61 = load i32, ptr %7, align 4
  store i32 %61, ptr @ep_gc_enabled, align 4
  %62 = load i64, ptr %5, align 8
  store i64 %62, ptr %3, align 8
  br label %63

63:                                               ; preds = %32, %12
  %64 = load i64, ptr %3, align 8
  ret i64 %64
}

declare i32 @pthread_mutex_lock(ptr noundef) #1

declare i32 @"\01_pthread_cond_wait"(ptr noundef, ptr noundef) #1

declare i32 @pthread_cond_signal(ptr noundef) #1

declare i32 @pthread_mutex_unlock(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @receive_channel(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i32, align 4
  %6 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %7 = load i64, ptr %3, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %4, align 8
  %9 = load ptr, ptr %4, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %12, label %11

11:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %59

12:                                               ; preds = %1
  %13 = load i32, ptr @ep_gc_enabled, align 4
  store i32 %13, ptr %5, align 4
  store i32 0, ptr @ep_gc_enabled, align 4
  %14 = load ptr, ptr %4, align 8
  %15 = getelementptr inbounds %struct.EpChannel, ptr %14, i32 0, i32 5
  %16 = call i32 @pthread_mutex_lock(ptr noundef %15)
  br label %17

17:                                               ; preds = %22, %12
  %18 = load ptr, ptr %4, align 8
  %19 = getelementptr inbounds %struct.EpChannel, ptr %18, i32 0, i32 4
  %20 = load i64, ptr %19, align 8
  %21 = icmp sle i64 %20, 0
  br i1 %21, label %22, label %28

22:                                               ; preds = %17
  %23 = load ptr, ptr %4, align 8
  %24 = getelementptr inbounds %struct.EpChannel, ptr %23, i32 0, i32 6
  %25 = load ptr, ptr %4, align 8
  %26 = getelementptr inbounds %struct.EpChannel, ptr %25, i32 0, i32 5
  %27 = call i32 @"\01_pthread_cond_wait"(ptr noundef %24, ptr noundef %26)
  br label %17, !llvm.loop !8

28:                                               ; preds = %17
  %29 = load ptr, ptr %4, align 8
  %30 = getelementptr inbounds %struct.EpChannel, ptr %29, i32 0, i32 0
  %31 = load ptr, ptr %30, align 8
  %32 = load ptr, ptr %4, align 8
  %33 = getelementptr inbounds %struct.EpChannel, ptr %32, i32 0, i32 2
  %34 = load i64, ptr %33, align 8
  %35 = getelementptr inbounds i64, ptr %31, i64 %34
  %36 = load i64, ptr %35, align 8
  store i64 %36, ptr %6, align 8
  %37 = load ptr, ptr %4, align 8
  %38 = getelementptr inbounds %struct.EpChannel, ptr %37, i32 0, i32 2
  %39 = load i64, ptr %38, align 8
  %40 = add nsw i64 %39, 1
  %41 = load ptr, ptr %4, align 8
  %42 = getelementptr inbounds %struct.EpChannel, ptr %41, i32 0, i32 1
  %43 = load i64, ptr %42, align 8
  %44 = srem i64 %40, %43
  %45 = load ptr, ptr %4, align 8
  %46 = getelementptr inbounds %struct.EpChannel, ptr %45, i32 0, i32 2
  store i64 %44, ptr %46, align 8
  %47 = load ptr, ptr %4, align 8
  %48 = getelementptr inbounds %struct.EpChannel, ptr %47, i32 0, i32 4
  %49 = load i64, ptr %48, align 8
  %50 = sub nsw i64 %49, 1
  store i64 %50, ptr %48, align 8
  %51 = load ptr, ptr %4, align 8
  %52 = getelementptr inbounds %struct.EpChannel, ptr %51, i32 0, i32 7
  %53 = call i32 @pthread_cond_signal(ptr noundef %52)
  %54 = load ptr, ptr %4, align 8
  %55 = getelementptr inbounds %struct.EpChannel, ptr %54, i32 0, i32 5
  %56 = call i32 @pthread_mutex_unlock(ptr noundef %55)
  %57 = load i32, ptr %5, align 4
  store i32 %57, ptr @ep_gc_enabled, align 4
  %58 = load i64, ptr %6, align 8
  store i64 %58, ptr %2, align 8
  br label %59

59:                                               ; preds = %28, %11
  %60 = load i64, ptr %2, align 8
  ret i64 %60
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @channel_try_recv(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %8 = load i64, ptr %4, align 8
  %9 = inttoptr i64 %8 to ptr
  store ptr %9, ptr %6, align 8
  %10 = load ptr, ptr %6, align 8
  %11 = icmp ne ptr %10, null
  br i1 %11, label %13, label %12

12:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %61

13:                                               ; preds = %2
  %14 = load ptr, ptr %6, align 8
  %15 = getelementptr inbounds %struct.EpChannel, ptr %14, i32 0, i32 5
  %16 = call i32 @pthread_mutex_lock(ptr noundef %15)
  %17 = load ptr, ptr %6, align 8
  %18 = getelementptr inbounds %struct.EpChannel, ptr %17, i32 0, i32 4
  %19 = load i64, ptr %18, align 8
  %20 = icmp sle i64 %19, 0
  br i1 %20, label %21, label %25

21:                                               ; preds = %13
  %22 = load ptr, ptr %6, align 8
  %23 = getelementptr inbounds %struct.EpChannel, ptr %22, i32 0, i32 5
  %24 = call i32 @pthread_mutex_unlock(ptr noundef %23)
  store i64 0, ptr %3, align 8
  br label %61

25:                                               ; preds = %13
  %26 = load ptr, ptr %6, align 8
  %27 = getelementptr inbounds %struct.EpChannel, ptr %26, i32 0, i32 0
  %28 = load ptr, ptr %27, align 8
  %29 = load ptr, ptr %6, align 8
  %30 = getelementptr inbounds %struct.EpChannel, ptr %29, i32 0, i32 2
  %31 = load i64, ptr %30, align 8
  %32 = getelementptr inbounds i64, ptr %28, i64 %31
  %33 = load i64, ptr %32, align 8
  store i64 %33, ptr %7, align 8
  %34 = load ptr, ptr %6, align 8
  %35 = getelementptr inbounds %struct.EpChannel, ptr %34, i32 0, i32 2
  %36 = load i64, ptr %35, align 8
  %37 = add nsw i64 %36, 1
  %38 = load ptr, ptr %6, align 8
  %39 = getelementptr inbounds %struct.EpChannel, ptr %38, i32 0, i32 1
  %40 = load i64, ptr %39, align 8
  %41 = srem i64 %37, %40
  %42 = load ptr, ptr %6, align 8
  %43 = getelementptr inbounds %struct.EpChannel, ptr %42, i32 0, i32 2
  store i64 %41, ptr %43, align 8
  %44 = load ptr, ptr %6, align 8
  %45 = getelementptr inbounds %struct.EpChannel, ptr %44, i32 0, i32 4
  %46 = load i64, ptr %45, align 8
  %47 = sub nsw i64 %46, 1
  store i64 %47, ptr %45, align 8
  %48 = load ptr, ptr %6, align 8
  %49 = getelementptr inbounds %struct.EpChannel, ptr %48, i32 0, i32 7
  %50 = call i32 @pthread_cond_signal(ptr noundef %49)
  %51 = load ptr, ptr %6, align 8
  %52 = getelementptr inbounds %struct.EpChannel, ptr %51, i32 0, i32 5
  %53 = call i32 @pthread_mutex_unlock(ptr noundef %52)
  %54 = load i64, ptr %5, align 8
  %55 = icmp ne i64 %54, 0
  br i1 %55, label %56, label %60

56:                                               ; preds = %25
  %57 = load i64, ptr %7, align 8
  %58 = load i64, ptr %5, align 8
  %59 = inttoptr i64 %58 to ptr
  store i64 %57, ptr %59, align 8
  br label %60

60:                                               ; preds = %56, %25
  store i64 1, ptr %3, align 8
  br label %61

61:                                               ; preds = %60, %21, %12
  %62 = load i64, ptr %3, align 8
  ret i64 %62
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @channel_has_data(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i32, align 4
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %11, label %10

10:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %26

11:                                               ; preds = %1
  %12 = load ptr, ptr %4, align 8
  %13 = getelementptr inbounds %struct.EpChannel, ptr %12, i32 0, i32 5
  %14 = call i32 @pthread_mutex_lock(ptr noundef %13)
  %15 = load ptr, ptr %4, align 8
  %16 = getelementptr inbounds %struct.EpChannel, ptr %15, i32 0, i32 4
  %17 = load i64, ptr %16, align 8
  %18 = icmp sgt i64 %17, 0
  %19 = zext i1 %18 to i64
  %20 = select i1 %18, i32 1, i32 0
  store i32 %20, ptr %5, align 4
  %21 = load ptr, ptr %4, align 8
  %22 = getelementptr inbounds %struct.EpChannel, ptr %21, i32 0, i32 5
  %23 = call i32 @pthread_mutex_unlock(ptr noundef %22)
  %24 = load i32, ptr %5, align 4
  %25 = sext i32 %24 to i64
  store i64 %25, ptr %2, align 8
  br label %26

26:                                               ; preds = %11, %10
  %27 = load i64, ptr %2, align 8
  ret i64 %27
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @channel_select(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca %struct.timespec, align 8
  %8 = alloca %struct.timespec, align 8
  %9 = alloca i64, align 8
  %10 = alloca ptr, align 8
  %11 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %12 = load i64, ptr %4, align 8
  %13 = inttoptr i64 %12 to ptr
  store ptr %13, ptr %6, align 8
  %14 = load ptr, ptr %6, align 8
  %15 = icmp ne ptr %14, null
  br i1 %15, label %16, label %21

16:                                               ; preds = %2
  %17 = load ptr, ptr %6, align 8
  %18 = getelementptr inbounds %struct.EpList, ptr %17, i32 0, i32 1
  %19 = load i64, ptr %18, align 8
  %20 = icmp eq i64 %19, 0
  br i1 %20, label %21, label %22

21:                                               ; preds = %16, %2
  store i64 -1, ptr %3, align 8
  br label %87

22:                                               ; preds = %16
  %23 = call i32 @clock_gettime(i32 noundef 6, ptr noundef %7)
  br label %24

24:                                               ; preds = %22, %85
  store i64 0, ptr %9, align 8
  br label %25

25:                                               ; preds = %59, %24
  %26 = load i64, ptr %9, align 8
  %27 = load ptr, ptr %6, align 8
  %28 = getelementptr inbounds %struct.EpList, ptr %27, i32 0, i32 1
  %29 = load i64, ptr %28, align 8
  %30 = icmp slt i64 %26, %29
  br i1 %30, label %31, label %62

31:                                               ; preds = %25
  %32 = load ptr, ptr %6, align 8
  %33 = getelementptr inbounds %struct.EpList, ptr %32, i32 0, i32 0
  %34 = load ptr, ptr %33, align 8
  %35 = load i64, ptr %9, align 8
  %36 = getelementptr inbounds i64, ptr %34, i64 %35
  %37 = load i64, ptr %36, align 8
  %38 = inttoptr i64 %37 to ptr
  store ptr %38, ptr %10, align 8
  %39 = load ptr, ptr %10, align 8
  %40 = icmp ne ptr %39, null
  br i1 %40, label %41, label %58

41:                                               ; preds = %31
  %42 = load ptr, ptr %10, align 8
  %43 = getelementptr inbounds %struct.EpChannel, ptr %42, i32 0, i32 5
  %44 = call i32 @pthread_mutex_lock(ptr noundef %43)
  %45 = load ptr, ptr %10, align 8
  %46 = getelementptr inbounds %struct.EpChannel, ptr %45, i32 0, i32 4
  %47 = load i64, ptr %46, align 8
  %48 = icmp sgt i64 %47, 0
  br i1 %48, label %49, label %54

49:                                               ; preds = %41
  %50 = load ptr, ptr %10, align 8
  %51 = getelementptr inbounds %struct.EpChannel, ptr %50, i32 0, i32 5
  %52 = call i32 @pthread_mutex_unlock(ptr noundef %51)
  %53 = load i64, ptr %9, align 8
  store i64 %53, ptr %3, align 8
  br label %87

54:                                               ; preds = %41
  %55 = load ptr, ptr %10, align 8
  %56 = getelementptr inbounds %struct.EpChannel, ptr %55, i32 0, i32 5
  %57 = call i32 @pthread_mutex_unlock(ptr noundef %56)
  br label %58

58:                                               ; preds = %54, %31
  br label %59

59:                                               ; preds = %58
  %60 = load i64, ptr %9, align 8
  %61 = add nsw i64 %60, 1
  store i64 %61, ptr %9, align 8
  br label %25, !llvm.loop !9

62:                                               ; preds = %25
  %63 = load i64, ptr %5, align 8
  %64 = icmp sge i64 %63, 0
  br i1 %64, label %65, label %85

65:                                               ; preds = %62
  %66 = call i32 @clock_gettime(i32 noundef 6, ptr noundef %8)
  %67 = getelementptr inbounds %struct.timespec, ptr %8, i32 0, i32 0
  %68 = load i64, ptr %67, align 8
  %69 = getelementptr inbounds %struct.timespec, ptr %7, i32 0, i32 0
  %70 = load i64, ptr %69, align 8
  %71 = sub nsw i64 %68, %70
  %72 = mul nsw i64 %71, 1000
  %73 = getelementptr inbounds %struct.timespec, ptr %8, i32 0, i32 1
  %74 = load i64, ptr %73, align 8
  %75 = getelementptr inbounds %struct.timespec, ptr %7, i32 0, i32 1
  %76 = load i64, ptr %75, align 8
  %77 = sub nsw i64 %74, %76
  %78 = sdiv i64 %77, 1000000
  %79 = add nsw i64 %72, %78
  store i64 %79, ptr %11, align 8
  %80 = load i64, ptr %11, align 8
  %81 = load i64, ptr %5, align 8
  %82 = icmp sge i64 %80, %81
  br i1 %82, label %83, label %84

83:                                               ; preds = %65
  store i64 -1, ptr %3, align 8
  br label %87

84:                                               ; preds = %65
  br label %85

85:                                               ; preds = %84, %62
  %86 = call i32 @"\01_usleep"(i32 noundef 1000)
  br label %24

87:                                               ; preds = %83, %49, %21
  %88 = load i64, ptr %3, align 8
  ret i64 %88
}

declare i32 @clock_gettime(i32 noundef, ptr noundef) #1

declare i32 @"\01_usleep"(i32 noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_net_connect(ptr noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i32, align 4
  %7 = alloca ptr, align 8
  %8 = alloca %struct.sockaddr_in, align 4
  store ptr %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = call i32 @socket(i32 noundef 2, i32 noundef 1, i32 noundef 0)
  store i32 %9, ptr %6, align 4
  %10 = load i32, ptr %6, align 4
  %11 = icmp slt i32 %10, 0
  br i1 %11, label %12, label %13

12:                                               ; preds = %2
  store i64 -1, ptr %3, align 8
  br label %69

13:                                               ; preds = %2
  %14 = load ptr, ptr %4, align 8
  %15 = call ptr @gethostbyname(ptr noundef %14)
  store ptr %15, ptr %7, align 8
  %16 = load ptr, ptr %7, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %21, label %18

18:                                               ; preds = %13
  %19 = load i32, ptr %6, align 4
  %20 = call i32 @"\01_close"(i32 noundef %19)
  store i64 -1, ptr %3, align 8
  br label %69

21:                                               ; preds = %13
  call void @llvm.memset.p0.i64(ptr align 4 %8, i8 0, i64 16, i1 false)
  %22 = getelementptr inbounds %struct.sockaddr_in, ptr %8, i32 0, i32 1
  store i8 2, ptr %22, align 1
  %23 = getelementptr inbounds %struct.sockaddr_in, ptr %8, i32 0, i32 3
  %24 = getelementptr inbounds %struct.in_addr, ptr %23, i32 0, i32 0
  %25 = load ptr, ptr %7, align 8
  %26 = getelementptr inbounds %struct.hostent, ptr %25, i32 0, i32 4
  %27 = load ptr, ptr %26, align 8
  %28 = getelementptr inbounds ptr, ptr %27, i64 0
  %29 = load ptr, ptr %28, align 8
  %30 = load ptr, ptr %7, align 8
  %31 = getelementptr inbounds %struct.hostent, ptr %30, i32 0, i32 3
  %32 = load i32, ptr %31, align 4
  %33 = sext i32 %32 to i64
  %34 = call ptr @__memcpy_chk(ptr noundef %24, ptr noundef %29, i64 noundef %33, i64 noundef 12) #13
  %35 = load i64, ptr %5, align 8
  %36 = call i1 @llvm.is.constant.i64(i64 %35)
  br i1 %36, label %37, label %51

37:                                               ; preds = %21
  %38 = load i64, ptr %5, align 8
  %39 = trunc i64 %38 to i16
  %40 = zext i16 %39 to i32
  %41 = and i32 %40, 65280
  %42 = lshr i32 %41, 8
  %43 = load i64, ptr %5, align 8
  %44 = trunc i64 %43 to i16
  %45 = zext i16 %44 to i32
  %46 = and i32 %45, 255
  %47 = shl i32 %46, 8
  %48 = or i32 %42, %47
  %49 = trunc i32 %48 to i16
  %50 = zext i16 %49 to i32
  br label %56

51:                                               ; preds = %21
  %52 = load i64, ptr %5, align 8
  %53 = trunc i64 %52 to i16
  %54 = call zeroext i16 @_OSSwapInt16(i16 noundef zeroext %53)
  %55 = zext i16 %54 to i32
  br label %56

56:                                               ; preds = %51, %37
  %57 = phi i32 [ %50, %37 ], [ %55, %51 ]
  %58 = trunc i32 %57 to i16
  %59 = getelementptr inbounds %struct.sockaddr_in, ptr %8, i32 0, i32 2
  store i16 %58, ptr %59, align 2
  %60 = load i32, ptr %6, align 4
  %61 = call i32 @"\01_connect"(i32 noundef %60, ptr noundef %8, i32 noundef 16)
  %62 = icmp slt i32 %61, 0
  br i1 %62, label %63, label %66

63:                                               ; preds = %56
  %64 = load i32, ptr %6, align 4
  %65 = call i32 @"\01_close"(i32 noundef %64)
  store i64 -1, ptr %3, align 8
  br label %69

66:                                               ; preds = %56
  %67 = load i32, ptr %6, align 4
  %68 = sext i32 %67 to i64
  store i64 %68, ptr %3, align 8
  br label %69

69:                                               ; preds = %66, %63, %18, %12
  %70 = load i64, ptr %3, align 8
  ret i64 %70
}

declare i32 @socket(i32 noundef, i32 noundef, i32 noundef) #1

declare ptr @gethostbyname(ptr noundef) #1

declare i32 @"\01_close"(i32 noundef) #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #3

; Function Attrs: nounwind
declare ptr @__memcpy_chk(ptr noundef, ptr noundef, i64 noundef, i64 noundef) #4

; Function Attrs: convergent nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.is.constant.i64(i64) #5

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal zeroext i16 @_OSSwapInt16(i16 noundef zeroext %0) #0 {
  %2 = alloca i16, align 2
  store i16 %0, ptr %2, align 2
  %3 = load i16, ptr %2, align 2
  %4 = zext i16 %3 to i32
  %5 = shl i32 %4, 8
  %6 = load i16, ptr %2, align 2
  %7 = zext i16 %6 to i32
  %8 = ashr i32 %7, 8
  %9 = or i32 %5, %8
  %10 = trunc i32 %9 to i16
  ret i16 %10
}

declare i32 @"\01_connect"(i32 noundef, ptr noundef, i32 noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_net_listen(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  %6 = alloca %struct.sockaddr_in, align 4
  store i64 %0, ptr %3, align 8
  %7 = call i32 @socket(i32 noundef 2, i32 noundef 1, i32 noundef 0)
  store i32 %7, ptr %4, align 4
  %8 = load i32, ptr %4, align 4
  %9 = icmp slt i32 %8, 0
  br i1 %9, label %10, label %11

10:                                               ; preds = %1
  store i64 -1, ptr %2, align 8
  br label %58

11:                                               ; preds = %1
  store i32 1, ptr %5, align 4
  %12 = load i32, ptr %4, align 4
  %13 = call i32 @setsockopt(i32 noundef %12, i32 noundef 65535, i32 noundef 4, ptr noundef %5, i32 noundef 4)
  call void @llvm.memset.p0.i64(ptr align 4 %6, i8 0, i64 16, i1 false)
  %14 = getelementptr inbounds %struct.sockaddr_in, ptr %6, i32 0, i32 1
  store i8 2, ptr %14, align 1
  %15 = getelementptr inbounds %struct.sockaddr_in, ptr %6, i32 0, i32 3
  %16 = getelementptr inbounds %struct.in_addr, ptr %15, i32 0, i32 0
  store i32 0, ptr %16, align 4
  %17 = load i64, ptr %3, align 8
  %18 = call i1 @llvm.is.constant.i64(i64 %17)
  br i1 %18, label %19, label %33

19:                                               ; preds = %11
  %20 = load i64, ptr %3, align 8
  %21 = trunc i64 %20 to i16
  %22 = zext i16 %21 to i32
  %23 = and i32 %22, 65280
  %24 = lshr i32 %23, 8
  %25 = load i64, ptr %3, align 8
  %26 = trunc i64 %25 to i16
  %27 = zext i16 %26 to i32
  %28 = and i32 %27, 255
  %29 = shl i32 %28, 8
  %30 = or i32 %24, %29
  %31 = trunc i32 %30 to i16
  %32 = zext i16 %31 to i32
  br label %38

33:                                               ; preds = %11
  %34 = load i64, ptr %3, align 8
  %35 = trunc i64 %34 to i16
  %36 = call zeroext i16 @_OSSwapInt16(i16 noundef zeroext %35)
  %37 = zext i16 %36 to i32
  br label %38

38:                                               ; preds = %33, %19
  %39 = phi i32 [ %32, %19 ], [ %37, %33 ]
  %40 = trunc i32 %39 to i16
  %41 = getelementptr inbounds %struct.sockaddr_in, ptr %6, i32 0, i32 2
  store i16 %40, ptr %41, align 2
  %42 = load i32, ptr %4, align 4
  %43 = call i32 @"\01_bind"(i32 noundef %42, ptr noundef %6, i32 noundef 16)
  %44 = icmp slt i32 %43, 0
  br i1 %44, label %45, label %48

45:                                               ; preds = %38
  %46 = load i32, ptr %4, align 4
  %47 = call i32 @"\01_close"(i32 noundef %46)
  store i64 -1, ptr %2, align 8
  br label %58

48:                                               ; preds = %38
  %49 = load i32, ptr %4, align 4
  %50 = call i32 @"\01_listen"(i32 noundef %49, i32 noundef 10)
  %51 = icmp slt i32 %50, 0
  br i1 %51, label %52, label %55

52:                                               ; preds = %48
  %53 = load i32, ptr %4, align 4
  %54 = call i32 @"\01_close"(i32 noundef %53)
  store i64 -1, ptr %2, align 8
  br label %58

55:                                               ; preds = %48
  %56 = load i32, ptr %4, align 4
  %57 = sext i32 %56 to i64
  store i64 %57, ptr %2, align 8
  br label %58

58:                                               ; preds = %55, %52, %45, %10
  %59 = load i64, ptr %2, align 8
  ret i64 %59
}

declare i32 @setsockopt(i32 noundef, i32 noundef, i32 noundef, ptr noundef, i32 noundef) #1

declare i32 @"\01_bind"(i32 noundef, ptr noundef, i32 noundef) #1

declare i32 @"\01_listen"(i32 noundef, i32 noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_net_accept(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca %struct.sockaddr_in, align 4
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  store i64 %0, ptr %2, align 8
  store i32 16, ptr %4, align 4
  %6 = load i64, ptr %2, align 8
  %7 = trunc i64 %6 to i32
  %8 = call i32 @"\01_accept"(i32 noundef %7, ptr noundef %3, ptr noundef %4)
  store i32 %8, ptr %5, align 4
  %9 = load i32, ptr %5, align 4
  %10 = sext i32 %9 to i64
  ret i64 %10
}

declare i32 @"\01_accept"(i32 noundef, ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_net_send(i64 noundef %0, ptr noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store ptr %1, ptr %5, align 8
  %6 = load ptr, ptr %5, align 8
  %7 = icmp ne ptr %6, null
  br i1 %7, label %9, label %8

8:                                                ; preds = %2
  store i64 0, ptr %3, align 8
  br label %16

9:                                                ; preds = %2
  %10 = load i64, ptr %4, align 8
  %11 = trunc i64 %10 to i32
  %12 = load ptr, ptr %5, align 8
  %13 = load ptr, ptr %5, align 8
  %14 = call i64 @strlen(ptr noundef %13) #13
  %15 = call i64 @"\01_send"(i32 noundef %11, ptr noundef %12, i64 noundef %14, i32 noundef 0)
  store i64 %15, ptr %3, align 8
  br label %16

16:                                               ; preds = %9, %8
  %17 = load i64, ptr %3, align 8
  ret i64 %17
}

declare i64 @"\01_send"(i32 noundef, ptr noundef, i64 noundef, i32 noundef) #1

; Function Attrs: nounwind
declare i64 @strlen(ptr noundef) #4

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define ptr @ep_net_recv(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %5, align 8
  %10 = add nsw i64 %9, 1
  %11 = call ptr @malloc(i64 noundef %10) #14
  store ptr %11, ptr %6, align 8
  %12 = load ptr, ptr %6, align 8
  %13 = icmp ne ptr %12, null
  br i1 %13, label %23, label %14

14:                                               ; preds = %2
  %15 = call ptr @malloc(i64 noundef 1) #14
  store ptr %15, ptr %7, align 8
  %16 = load ptr, ptr %7, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %18, label %21

18:                                               ; preds = %14
  %19 = load ptr, ptr %7, align 8
  %20 = getelementptr inbounds i8, ptr %19, i64 0
  store i8 0, ptr %20, align 1
  br label %21

21:                                               ; preds = %18, %14
  %22 = load ptr, ptr %7, align 8
  store ptr %22, ptr %3, align 8
  br label %37

23:                                               ; preds = %2
  %24 = load i64, ptr %4, align 8
  %25 = trunc i64 %24 to i32
  %26 = load ptr, ptr %6, align 8
  %27 = load i64, ptr %5, align 8
  %28 = call i64 @"\01_recv"(i32 noundef %25, ptr noundef %26, i64 noundef %27, i32 noundef 0)
  store i64 %28, ptr %8, align 8
  %29 = load i64, ptr %8, align 8
  %30 = icmp slt i64 %29, 0
  br i1 %30, label %31, label %32

31:                                               ; preds = %23
  store i64 0, ptr %8, align 8
  br label %32

32:                                               ; preds = %31, %23
  %33 = load ptr, ptr %6, align 8
  %34 = load i64, ptr %8, align 8
  %35 = getelementptr inbounds i8, ptr %33, i64 %34
  store i8 0, ptr %35, align 1
  %36 = load ptr, ptr %6, align 8
  store ptr %36, ptr %3, align 8
  br label %37

37:                                               ; preds = %32, %21
  %38 = load ptr, ptr %3, align 8
  ret ptr %38
}

declare i64 @"\01_recv"(i32 noundef, ptr noundef, i64 noundef, i32 noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_net_close(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = trunc i64 %3 to i32
  %5 = call i32 @"\01_close"(i32 noundef %4)
  %6 = sext i32 %5 to i64
  ret i64 %6
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_sleep_ms(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = mul nsw i64 %3, 1000
  %5 = trunc i64 %4 to i32
  %6 = call i32 @"\01_usleep"(i32 noundef %5)
  ret i64 0
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_system(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @"\01_system"(ptr noundef %4)
  %6 = sext i32 %5 to i64
  ret i64 %6
}

declare i32 @"\01_system"(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_play_sound(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca [512 x i8], align 1
  store i64 %0, ptr %2, align 8
  %4 = getelementptr inbounds [512 x i8], ptr %3, i64 0, i64 0
  %5 = load i64, ptr %2, align 8
  %6 = inttoptr i64 %5 to ptr
  %7 = call i32 (ptr, i64, i32, i64, ptr, ...) @__snprintf_chk(ptr noundef %4, i64 noundef 512, i32 noundef 0, i64 noundef 512, ptr noundef @.str, ptr noundef %6)
  %8 = getelementptr inbounds [512 x i8], ptr %3, i64 0, i64 0
  %9 = call i32 @"\01_system"(ptr noundef %8)
  %10 = sext i32 %9 to i64
  ret i64 %10
}

declare i32 @__snprintf_chk(ptr noundef, i64 noundef, i32 noundef, i64 noundef, ptr noundef, ...) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlopen(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %3, align 8
  %7 = load ptr, ptr %3, align 8
  %8 = call ptr @dlopen(ptr noundef %7, i32 noundef 1)
  store ptr %8, ptr %4, align 8
  %9 = load ptr, ptr %4, align 8
  %10 = ptrtoint ptr %9 to i64
  ret i64 %10
}

declare ptr @dlopen(ptr noundef, i32 noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlsym(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  %8 = load i64, ptr %4, align 8
  %9 = inttoptr i64 %8 to ptr
  %10 = call ptr @dlsym(ptr noundef %7, ptr noundef %9)
  store ptr %10, ptr %5, align 8
  %11 = load ptr, ptr %5, align 8
  %12 = ptrtoint ptr %11 to i64
  ret i64 %12
}

declare ptr @dlsym(ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlclose(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @dlclose(ptr noundef %4)
  %6 = sext i32 %5 to i64
  ret i64 %6
}

declare i32 @dlclose(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall0(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i64 %4()
  ret i64 %5
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall1(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  %7 = load i64, ptr %4, align 8
  %8 = call i64 %6(i64 noundef %7)
  ret i64 %8
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall2(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = load i64, ptr %5, align 8
  %10 = load i64, ptr %6, align 8
  %11 = call i64 %8(i64 noundef %9, i64 noundef %10)
  ret i64 %11
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall3(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3) #0 {
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  store i64 %0, ptr %5, align 8
  store i64 %1, ptr %6, align 8
  store i64 %2, ptr %7, align 8
  store i64 %3, ptr %8, align 8
  %9 = load i64, ptr %5, align 8
  %10 = inttoptr i64 %9 to ptr
  %11 = load i64, ptr %6, align 8
  %12 = load i64, ptr %7, align 8
  %13 = load i64, ptr %8, align 8
  %14 = call i64 %10(i64 noundef %11, i64 noundef %12, i64 noundef %13)
  ret i64 %14
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall4(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4) #0 {
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  store i64 %0, ptr %6, align 8
  store i64 %1, ptr %7, align 8
  store i64 %2, ptr %8, align 8
  store i64 %3, ptr %9, align 8
  store i64 %4, ptr %10, align 8
  %11 = load i64, ptr %6, align 8
  %12 = inttoptr i64 %11 to ptr
  %13 = load i64, ptr %7, align 8
  %14 = load i64, ptr %8, align 8
  %15 = load i64, ptr %9, align 8
  %16 = load i64, ptr %10, align 8
  %17 = call i64 %12(i64 noundef %13, i64 noundef %14, i64 noundef %15, i64 noundef %16)
  ret i64 %17
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall5(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5) #0 {
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  store i64 %0, ptr %7, align 8
  store i64 %1, ptr %8, align 8
  store i64 %2, ptr %9, align 8
  store i64 %3, ptr %10, align 8
  store i64 %4, ptr %11, align 8
  store i64 %5, ptr %12, align 8
  %13 = load i64, ptr %7, align 8
  %14 = inttoptr i64 %13 to ptr
  %15 = load i64, ptr %8, align 8
  %16 = load i64, ptr %9, align 8
  %17 = load i64, ptr %10, align 8
  %18 = load i64, ptr %11, align 8
  %19 = load i64, ptr %12, align 8
  %20 = call i64 %14(i64 noundef %15, i64 noundef %16, i64 noundef %17, i64 noundef %18, i64 noundef %19)
  ret i64 %20
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall6(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6) #0 {
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i64, align 8
  store i64 %0, ptr %8, align 8
  store i64 %1, ptr %9, align 8
  store i64 %2, ptr %10, align 8
  store i64 %3, ptr %11, align 8
  store i64 %4, ptr %12, align 8
  store i64 %5, ptr %13, align 8
  store i64 %6, ptr %14, align 8
  %15 = load i64, ptr %8, align 8
  %16 = inttoptr i64 %15 to ptr
  %17 = load i64, ptr %9, align 8
  %18 = load i64, ptr %10, align 8
  %19 = load i64, ptr %11, align 8
  %20 = load i64, ptr %12, align 8
  %21 = load i64, ptr %13, align 8
  %22 = load i64, ptr %14, align 8
  %23 = call i64 %16(i64 noundef %17, i64 noundef %18, i64 noundef %19, i64 noundef %20, i64 noundef %21, i64 noundef %22)
  ret i64 %23
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall7(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7) #0 {
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca i64, align 8
  store i64 %0, ptr %9, align 8
  store i64 %1, ptr %10, align 8
  store i64 %2, ptr %11, align 8
  store i64 %3, ptr %12, align 8
  store i64 %4, ptr %13, align 8
  store i64 %5, ptr %14, align 8
  store i64 %6, ptr %15, align 8
  store i64 %7, ptr %16, align 8
  %17 = load i64, ptr %9, align 8
  %18 = inttoptr i64 %17 to ptr
  %19 = load i64, ptr %10, align 8
  %20 = load i64, ptr %11, align 8
  %21 = load i64, ptr %12, align 8
  %22 = load i64, ptr %13, align 8
  %23 = load i64, ptr %14, align 8
  %24 = load i64, ptr %15, align 8
  %25 = load i64, ptr %16, align 8
  %26 = call i64 %18(i64 noundef %19, i64 noundef %20, i64 noundef %21, i64 noundef %22, i64 noundef %23, i64 noundef %24, i64 noundef %25)
  ret i64 %26
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall8(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7, i64 noundef %8) #0 {
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca i64, align 8
  %17 = alloca i64, align 8
  %18 = alloca i64, align 8
  store i64 %0, ptr %10, align 8
  store i64 %1, ptr %11, align 8
  store i64 %2, ptr %12, align 8
  store i64 %3, ptr %13, align 8
  store i64 %4, ptr %14, align 8
  store i64 %5, ptr %15, align 8
  store i64 %6, ptr %16, align 8
  store i64 %7, ptr %17, align 8
  store i64 %8, ptr %18, align 8
  %19 = load i64, ptr %10, align 8
  %20 = inttoptr i64 %19 to ptr
  %21 = load i64, ptr %11, align 8
  %22 = load i64, ptr %12, align 8
  %23 = load i64, ptr %13, align 8
  %24 = load i64, ptr %14, align 8
  %25 = load i64, ptr %15, align 8
  %26 = load i64, ptr %16, align 8
  %27 = load i64, ptr %17, align 8
  %28 = load i64, ptr %18, align 8
  %29 = call i64 %20(i64 noundef %21, i64 noundef %22, i64 noundef %23, i64 noundef %24, i64 noundef %25, i64 noundef %26, i64 noundef %27, i64 noundef %28)
  ret i64 %29
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall9(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7, i64 noundef %8, i64 noundef %9) #0 {
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca i64, align 8
  %17 = alloca i64, align 8
  %18 = alloca i64, align 8
  %19 = alloca i64, align 8
  %20 = alloca i64, align 8
  store i64 %0, ptr %11, align 8
  store i64 %1, ptr %12, align 8
  store i64 %2, ptr %13, align 8
  store i64 %3, ptr %14, align 8
  store i64 %4, ptr %15, align 8
  store i64 %5, ptr %16, align 8
  store i64 %6, ptr %17, align 8
  store i64 %7, ptr %18, align 8
  store i64 %8, ptr %19, align 8
  store i64 %9, ptr %20, align 8
  %21 = load i64, ptr %11, align 8
  %22 = inttoptr i64 %21 to ptr
  %23 = load i64, ptr %12, align 8
  %24 = load i64, ptr %13, align 8
  %25 = load i64, ptr %14, align 8
  %26 = load i64, ptr %15, align 8
  %27 = load i64, ptr %16, align 8
  %28 = load i64, ptr %17, align 8
  %29 = load i64, ptr %18, align 8
  %30 = load i64, ptr %19, align 8
  %31 = load i64, ptr %20, align 8
  %32 = call i64 %22(i64 noundef %23, i64 noundef %24, i64 noundef %25, i64 noundef %26, i64 noundef %27, i64 noundef %28, i64 noundef %29, i64 noundef %30, i64 noundef %31)
  ret i64 %32
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall10(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7, i64 noundef %8, i64 noundef %9, i64 noundef %10) #0 {
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca i64, align 8
  %17 = alloca i64, align 8
  %18 = alloca i64, align 8
  %19 = alloca i64, align 8
  %20 = alloca i64, align 8
  %21 = alloca i64, align 8
  %22 = alloca i64, align 8
  store i64 %0, ptr %12, align 8
  store i64 %1, ptr %13, align 8
  store i64 %2, ptr %14, align 8
  store i64 %3, ptr %15, align 8
  store i64 %4, ptr %16, align 8
  store i64 %5, ptr %17, align 8
  store i64 %6, ptr %18, align 8
  store i64 %7, ptr %19, align 8
  store i64 %8, ptr %20, align 8
  store i64 %9, ptr %21, align 8
  store i64 %10, ptr %22, align 8
  %23 = load i64, ptr %12, align 8
  %24 = inttoptr i64 %23 to ptr
  %25 = load i64, ptr %13, align 8
  %26 = load i64, ptr %14, align 8
  %27 = load i64, ptr %15, align 8
  %28 = load i64, ptr %16, align 8
  %29 = load i64, ptr %17, align 8
  %30 = load i64, ptr %18, align 8
  %31 = load i64, ptr %19, align 8
  %32 = load i64, ptr %20, align 8
  %33 = load i64, ptr %21, align 8
  %34 = load i64, ptr %22, align 8
  %35 = call i64 %24(i64 noundef %25, i64 noundef %26, i64 noundef %27, i64 noundef %28, i64 noundef %29, i64 noundef %30, i64 noundef %31, i64 noundef %32, i64 noundef %33, i64 noundef %34)
  ret i64 %35
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_double_to_bits(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  ret i64 %3
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_bits_to_double(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  ret i64 %3
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_f0(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call double %4()
  %6 = call i64 @ep_double_to_ll(double noundef %5)
  ret i64 %6
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal i64 @ep_double_to_ll(double noundef %0) #0 {
  %2 = alloca double, align 8
  %3 = alloca %union.ep_float_bits, align 8
  store double %0, ptr %2, align 8
  %4 = load double, ptr %2, align 8
  store double %4, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  ret i64 %5
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_f1(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  %7 = load i64, ptr %4, align 8
  %8 = call double @ep_ll_to_double(i64 noundef %7)
  %9 = call double %6(double noundef %8)
  %10 = call i64 @ep_double_to_ll(double noundef %9)
  ret i64 %10
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal double @ep_ll_to_double(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca %union.ep_float_bits, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  store i64 %4, ptr %3, align 8
  %5 = load double, ptr %3, align 8
  ret double %5
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_f2(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = load i64, ptr %5, align 8
  %10 = call double @ep_ll_to_double(i64 noundef %9)
  %11 = load i64, ptr %6, align 8
  %12 = call double @ep_ll_to_double(i64 noundef %11)
  %13 = call double %8(double noundef %10, double noundef %12)
  %14 = call i64 @ep_double_to_ll(double noundef %13)
  ret i64 %14
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_f3(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3) #0 {
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  store i64 %0, ptr %5, align 8
  store i64 %1, ptr %6, align 8
  store i64 %2, ptr %7, align 8
  store i64 %3, ptr %8, align 8
  %9 = load i64, ptr %5, align 8
  %10 = inttoptr i64 %9 to ptr
  %11 = load i64, ptr %6, align 8
  %12 = call double @ep_ll_to_double(i64 noundef %11)
  %13 = load i64, ptr %7, align 8
  %14 = call double @ep_ll_to_double(i64 noundef %13)
  %15 = load i64, ptr %8, align 8
  %16 = call double @ep_ll_to_double(i64 noundef %15)
  %17 = call double %10(double noundef %12, double noundef %14, double noundef %16)
  %18 = call i64 @ep_double_to_ll(double noundef %17)
  ret i64 %18
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_f4(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4) #0 {
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  store i64 %0, ptr %6, align 8
  store i64 %1, ptr %7, align 8
  store i64 %2, ptr %8, align 8
  store i64 %3, ptr %9, align 8
  store i64 %4, ptr %10, align 8
  %11 = load i64, ptr %6, align 8
  %12 = inttoptr i64 %11 to ptr
  %13 = load i64, ptr %7, align 8
  %14 = call double @ep_ll_to_double(i64 noundef %13)
  %15 = load i64, ptr %8, align 8
  %16 = call double @ep_ll_to_double(i64 noundef %15)
  %17 = load i64, ptr %9, align 8
  %18 = call double @ep_ll_to_double(i64 noundef %17)
  %19 = load i64, ptr %10, align 8
  %20 = call double @ep_ll_to_double(i64 noundef %19)
  %21 = call double %12(double noundef %14, double noundef %16, double noundef %18, double noundef %20)
  %22 = call i64 @ep_double_to_ll(double noundef %21)
  ret i64 %22
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_f5(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5) #0 {
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  store i64 %0, ptr %7, align 8
  store i64 %1, ptr %8, align 8
  store i64 %2, ptr %9, align 8
  store i64 %3, ptr %10, align 8
  store i64 %4, ptr %11, align 8
  store i64 %5, ptr %12, align 8
  %13 = load i64, ptr %7, align 8
  %14 = inttoptr i64 %13 to ptr
  %15 = load i64, ptr %8, align 8
  %16 = call double @ep_ll_to_double(i64 noundef %15)
  %17 = load i64, ptr %9, align 8
  %18 = call double @ep_ll_to_double(i64 noundef %17)
  %19 = load i64, ptr %10, align 8
  %20 = call double @ep_ll_to_double(i64 noundef %19)
  %21 = load i64, ptr %11, align 8
  %22 = call double @ep_ll_to_double(i64 noundef %21)
  %23 = load i64, ptr %12, align 8
  %24 = call double @ep_ll_to_double(i64 noundef %23)
  %25 = call double %14(double noundef %16, double noundef %18, double noundef %20, double noundef %22, double noundef %24)
  %26 = call i64 @ep_double_to_ll(double noundef %25)
  ret i64 %26
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_f6(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6) #0 {
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i64, align 8
  store i64 %0, ptr %8, align 8
  store i64 %1, ptr %9, align 8
  store i64 %2, ptr %10, align 8
  store i64 %3, ptr %11, align 8
  store i64 %4, ptr %12, align 8
  store i64 %5, ptr %13, align 8
  store i64 %6, ptr %14, align 8
  %15 = load i64, ptr %8, align 8
  %16 = inttoptr i64 %15 to ptr
  %17 = load i64, ptr %9, align 8
  %18 = call double @ep_ll_to_double(i64 noundef %17)
  %19 = load i64, ptr %10, align 8
  %20 = call double @ep_ll_to_double(i64 noundef %19)
  %21 = load i64, ptr %11, align 8
  %22 = call double @ep_ll_to_double(i64 noundef %21)
  %23 = load i64, ptr %12, align 8
  %24 = call double @ep_ll_to_double(i64 noundef %23)
  %25 = load i64, ptr %13, align 8
  %26 = call double @ep_ll_to_double(i64 noundef %25)
  %27 = load i64, ptr %14, align 8
  %28 = call double @ep_ll_to_double(i64 noundef %27)
  %29 = call double %16(double noundef %18, double noundef %20, double noundef %22, double noundef %24, double noundef %26, double noundef %28)
  %30 = call i64 @ep_double_to_ll(double noundef %29)
  ret i64 %30
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_fd1(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  %7 = load i64, ptr %4, align 8
  %8 = call double @ep_ll_to_double(i64 noundef %7)
  %9 = call i64 %6(double noundef %8)
  ret i64 %9
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_fd2(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = load i64, ptr %5, align 8
  %10 = call double @ep_ll_to_double(i64 noundef %9)
  %11 = load i64, ptr %6, align 8
  %12 = call double @ep_ll_to_double(i64 noundef %11)
  %13 = call i64 %8(double noundef %10, double noundef %12)
  ret i64 %13
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_dlcall_fd3(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3) #0 {
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  store i64 %0, ptr %5, align 8
  store i64 %1, ptr %6, align 8
  store i64 %2, ptr %7, align 8
  store i64 %3, ptr %8, align 8
  %9 = load i64, ptr %5, align 8
  %10 = inttoptr i64 %9 to ptr
  %11 = load i64, ptr %6, align 8
  %12 = call double @ep_ll_to_double(i64 noundef %11)
  %13 = load i64, ptr %7, align 8
  %14 = call double @ep_ll_to_double(i64 noundef %13)
  %15 = load i64, ptr %8, align 8
  %16 = call double @ep_ll_to_double(i64 noundef %15)
  %17 = call i64 %10(double noundef %12, double noundef %14, double noundef %16)
  ret i64 %17
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @hash_string(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca i64, align 8
  %4 = alloca i32, align 4
  store ptr %0, ptr %2, align 8
  store i64 5381, ptr %3, align 8
  br label %5

5:                                                ; preds = %11, %1
  %6 = load ptr, ptr %2, align 8
  %7 = getelementptr inbounds i8, ptr %6, i32 1
  store ptr %7, ptr %2, align 8
  %8 = load i8, ptr %6, align 1
  %9 = sext i8 %8 to i32
  store i32 %9, ptr %4, align 4
  %10 = icmp ne i32 %9, 0
  br i1 %10, label %11, label %19

11:                                               ; preds = %5
  %12 = load i64, ptr %3, align 8
  %13 = shl i64 %12, 5
  %14 = load i64, ptr %3, align 8
  %15 = add i64 %13, %14
  %16 = load i32, ptr %4, align 4
  %17 = sext i32 %16 to i64
  %18 = add i64 %15, %17
  store i64 %18, ptr %3, align 8
  br label %5, !llvm.loop !10

19:                                               ; preds = %5
  %20 = load i64, ptr %3, align 8
  ret i64 %20
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @create_map() #0 {
  %1 = alloca i64, align 8
  %2 = alloca ptr, align 8
  %3 = call ptr @malloc(i64 noundef 24) #14
  store ptr %3, ptr %2, align 8
  %4 = load ptr, ptr %2, align 8
  %5 = icmp ne ptr %4, null
  br i1 %5, label %7, label %6

6:                                                ; preds = %0
  store i64 0, ptr %1, align 8
  br label %27

7:                                                ; preds = %0
  %8 = load ptr, ptr %2, align 8
  %9 = getelementptr inbounds %struct.EpMap, ptr %8, i32 0, i32 1
  store i64 16, ptr %9, align 8
  %10 = load ptr, ptr %2, align 8
  %11 = getelementptr inbounds %struct.EpMap, ptr %10, i32 0, i32 2
  store i64 0, ptr %11, align 8
  %12 = load ptr, ptr %2, align 8
  %13 = getelementptr inbounds %struct.EpMap, ptr %12, i32 0, i32 1
  %14 = load i64, ptr %13, align 8
  %15 = call ptr @calloc(i64 noundef %14, i64 noundef 24) #15
  %16 = load ptr, ptr %2, align 8
  %17 = getelementptr inbounds %struct.EpMap, ptr %16, i32 0, i32 0
  store ptr %15, ptr %17, align 8
  %18 = load ptr, ptr %2, align 8
  %19 = getelementptr inbounds %struct.EpMap, ptr %18, i32 0, i32 0
  %20 = load ptr, ptr %19, align 8
  %21 = icmp ne ptr %20, null
  br i1 %21, label %24, label %22

22:                                               ; preds = %7
  %23 = load ptr, ptr %2, align 8
  call void @free(ptr noundef %23)
  store i64 0, ptr %1, align 8
  br label %27

24:                                               ; preds = %7
  %25 = load ptr, ptr %2, align 8
  %26 = ptrtoint ptr %25 to i64
  store i64 %26, ptr %1, align 8
  br label %27

27:                                               ; preds = %24, %22, %6
  %28 = load i64, ptr %1, align 8
  ret i64 %28
}

; Function Attrs: allocsize(0,1)
declare ptr @calloc(i64 noundef, i64 noundef) #6

declare void @free(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_insert(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca ptr, align 8
  %9 = alloca [32 x i8], align 1
  %10 = alloca ptr, align 8
  %11 = alloca i64, align 8
  store i64 %0, ptr %5, align 8
  store i64 %1, ptr %6, align 8
  store i64 %2, ptr %7, align 8
  %12 = load i64, ptr %5, align 8
  %13 = inttoptr i64 %12 to ptr
  store ptr %13, ptr %8, align 8
  %14 = load i64, ptr %6, align 8
  %15 = getelementptr inbounds [32 x i8], ptr %9, i64 0, i64 0
  %16 = call ptr @ep_map_key_str(i64 noundef %14, ptr noundef %15, i32 noundef 32)
  store ptr %16, ptr %10, align 8
  %17 = load ptr, ptr %8, align 8
  %18 = icmp ne ptr %17, null
  br i1 %18, label %20, label %19

19:                                               ; preds = %3
  store i64 0, ptr %4, align 8
  br label %111

20:                                               ; preds = %3
  %21 = load ptr, ptr %8, align 8
  %22 = getelementptr inbounds %struct.EpMap, ptr %21, i32 0, i32 2
  %23 = load i64, ptr %22, align 8
  %24 = mul nsw i64 %23, 2
  %25 = load ptr, ptr %8, align 8
  %26 = getelementptr inbounds %struct.EpMap, ptr %25, i32 0, i32 1
  %27 = load i64, ptr %26, align 8
  %28 = icmp sge i64 %24, %27
  br i1 %28, label %29, label %35

29:                                               ; preds = %20
  %30 = load ptr, ptr %8, align 8
  %31 = load ptr, ptr %8, align 8
  %32 = getelementptr inbounds %struct.EpMap, ptr %31, i32 0, i32 1
  %33 = load i64, ptr %32, align 8
  %34 = mul nsw i64 %33, 2
  call void @map_resize(ptr noundef %30, i64 noundef %34)
  br label %35

35:                                               ; preds = %29, %20
  %36 = load ptr, ptr %10, align 8
  %37 = call i64 @hash_string(ptr noundef %36)
  %38 = load ptr, ptr %8, align 8
  %39 = getelementptr inbounds %struct.EpMap, ptr %38, i32 0, i32 1
  %40 = load i64, ptr %39, align 8
  %41 = urem i64 %37, %40
  store i64 %41, ptr %11, align 8
  br label %42

42:                                               ; preds = %74, %35
  %43 = load ptr, ptr %8, align 8
  %44 = getelementptr inbounds %struct.EpMap, ptr %43, i32 0, i32 0
  %45 = load ptr, ptr %44, align 8
  %46 = load i64, ptr %11, align 8
  %47 = getelementptr inbounds %struct.EpMapEntry, ptr %45, i64 %46
  %48 = getelementptr inbounds %struct.EpMapEntry, ptr %47, i32 0, i32 2
  %49 = load i32, ptr %48, align 8
  %50 = icmp ne i32 %49, 0
  br i1 %50, label %51, label %81

51:                                               ; preds = %42
  %52 = load ptr, ptr %8, align 8
  %53 = getelementptr inbounds %struct.EpMap, ptr %52, i32 0, i32 0
  %54 = load ptr, ptr %53, align 8
  %55 = load i64, ptr %11, align 8
  %56 = getelementptr inbounds %struct.EpMapEntry, ptr %54, i64 %55
  %57 = getelementptr inbounds %struct.EpMapEntry, ptr %56, i32 0, i32 0
  %58 = load ptr, ptr %57, align 8
  %59 = load ptr, ptr %10, align 8
  %60 = call i32 @strcmp(ptr noundef %58, ptr noundef %59) #13
  %61 = icmp eq i32 %60, 0
  br i1 %61, label %62, label %74

62:                                               ; preds = %51
  %63 = load i64, ptr %7, align 8
  %64 = load ptr, ptr %8, align 8
  %65 = getelementptr inbounds %struct.EpMap, ptr %64, i32 0, i32 0
  %66 = load ptr, ptr %65, align 8
  %67 = load i64, ptr %11, align 8
  %68 = getelementptr inbounds %struct.EpMapEntry, ptr %66, i64 %67
  %69 = getelementptr inbounds %struct.EpMapEntry, ptr %68, i32 0, i32 1
  store i64 %63, ptr %69, align 8
  %70 = load i64, ptr %5, align 8
  %71 = inttoptr i64 %70 to ptr
  %72 = load i64, ptr %7, align 8
  call void @ep_gc_write_barrier(ptr noundef %71, i64 noundef %72)
  %73 = load i64, ptr %7, align 8
  store i64 %73, ptr %4, align 8
  br label %111

74:                                               ; preds = %51
  %75 = load i64, ptr %11, align 8
  %76 = add i64 %75, 1
  %77 = load ptr, ptr %8, align 8
  %78 = getelementptr inbounds %struct.EpMap, ptr %77, i32 0, i32 1
  %79 = load i64, ptr %78, align 8
  %80 = urem i64 %76, %79
  store i64 %80, ptr %11, align 8
  br label %42, !llvm.loop !11

81:                                               ; preds = %42
  %82 = load ptr, ptr %10, align 8
  %83 = call ptr @strdup(ptr noundef %82) #13
  %84 = load ptr, ptr %8, align 8
  %85 = getelementptr inbounds %struct.EpMap, ptr %84, i32 0, i32 0
  %86 = load ptr, ptr %85, align 8
  %87 = load i64, ptr %11, align 8
  %88 = getelementptr inbounds %struct.EpMapEntry, ptr %86, i64 %87
  %89 = getelementptr inbounds %struct.EpMapEntry, ptr %88, i32 0, i32 0
  store ptr %83, ptr %89, align 8
  %90 = load i64, ptr %7, align 8
  %91 = load ptr, ptr %8, align 8
  %92 = getelementptr inbounds %struct.EpMap, ptr %91, i32 0, i32 0
  %93 = load ptr, ptr %92, align 8
  %94 = load i64, ptr %11, align 8
  %95 = getelementptr inbounds %struct.EpMapEntry, ptr %93, i64 %94
  %96 = getelementptr inbounds %struct.EpMapEntry, ptr %95, i32 0, i32 1
  store i64 %90, ptr %96, align 8
  %97 = load ptr, ptr %8, align 8
  %98 = getelementptr inbounds %struct.EpMap, ptr %97, i32 0, i32 0
  %99 = load ptr, ptr %98, align 8
  %100 = load i64, ptr %11, align 8
  %101 = getelementptr inbounds %struct.EpMapEntry, ptr %99, i64 %100
  %102 = getelementptr inbounds %struct.EpMapEntry, ptr %101, i32 0, i32 2
  store i32 1, ptr %102, align 8
  %103 = load ptr, ptr %8, align 8
  %104 = getelementptr inbounds %struct.EpMap, ptr %103, i32 0, i32 2
  %105 = load i64, ptr %104, align 8
  %106 = add nsw i64 %105, 1
  store i64 %106, ptr %104, align 8
  %107 = load i64, ptr %5, align 8
  %108 = inttoptr i64 %107 to ptr
  %109 = load i64, ptr %7, align 8
  call void @ep_gc_write_barrier(ptr noundef %108, i64 noundef %109)
  %110 = load i64, ptr %7, align 8
  store i64 %110, ptr %4, align 8
  br label %111

111:                                              ; preds = %81, %62, %19
  %112 = load i64, ptr %4, align 8
  ret i64 %112
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal ptr @ep_map_key_str(i64 noundef %0, ptr noundef %1, i32 noundef %2) #0 {
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i32, align 4
  %8 = alloca ptr, align 8
  %9 = alloca i8, align 1
  store i64 %0, ptr %5, align 8
  store ptr %1, ptr %6, align 8
  store i32 %2, ptr %7, align 4
  %10 = load i64, ptr %5, align 8
  %11 = icmp eq i64 %10, 0
  br i1 %11, label %12, label %18

12:                                               ; preds = %3
  %13 = load ptr, ptr %6, align 8
  %14 = getelementptr inbounds i8, ptr %13, i64 0
  store i8 48, ptr %14, align 1
  %15 = load ptr, ptr %6, align 8
  %16 = getelementptr inbounds i8, ptr %15, i64 1
  store i8 0, ptr %16, align 1
  %17 = load ptr, ptr %6, align 8
  store ptr %17, ptr %4, align 8
  br label %53

18:                                               ; preds = %3
  %19 = load i64, ptr %5, align 8
  %20 = icmp sgt i64 %19, 1048576
  br i1 %20, label %21, label %44

21:                                               ; preds = %18
  %22 = load i64, ptr %5, align 8
  %23 = inttoptr i64 %22 to ptr
  store ptr %23, ptr %8, align 8
  %24 = load ptr, ptr %8, align 8
  %25 = load i8, ptr %24, align 1
  store i8 %25, ptr %9, align 1
  %26 = load i8, ptr %9, align 1
  %27 = zext i8 %26 to i32
  %28 = icmp sge i32 %27, 32
  br i1 %28, label %29, label %33

29:                                               ; preds = %21
  %30 = load i8, ptr %9, align 1
  %31 = zext i8 %30 to i32
  %32 = icmp slt i32 %31, 127
  br i1 %32, label %41, label %33

33:                                               ; preds = %29, %21
  %34 = load i8, ptr %9, align 1
  %35 = zext i8 %34 to i32
  %36 = icmp sge i32 %35, 192
  br i1 %36, label %41, label %37

37:                                               ; preds = %33
  %38 = load i8, ptr %9, align 1
  %39 = zext i8 %38 to i32
  %40 = icmp eq i32 %39, 0
  br i1 %40, label %41, label %43

41:                                               ; preds = %37, %33, %29
  %42 = load ptr, ptr %8, align 8
  store ptr %42, ptr %4, align 8
  br label %53

43:                                               ; preds = %37
  br label %44

44:                                               ; preds = %43, %18
  %45 = load ptr, ptr %6, align 8
  %46 = load i32, ptr %7, align 4
  %47 = sext i32 %46 to i64
  %48 = load ptr, ptr %6, align 8
  %49 = call i64 @llvm.objectsize.i64.p0(ptr %48, i1 false, i1 true, i1 false)
  %50 = load i64, ptr %5, align 8
  %51 = call i32 (ptr, i64, i32, i64, ptr, ...) @__snprintf_chk(ptr noundef %45, i64 noundef %47, i32 noundef 0, i64 noundef %49, ptr noundef @.str.26, i64 noundef %50)
  %52 = load ptr, ptr %6, align 8
  store ptr %52, ptr %4, align 8
  br label %53

53:                                               ; preds = %44, %41, %12
  %54 = load ptr, ptr %4, align 8
  ret ptr %54
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @map_resize(ptr noundef %0, i64 noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca i64, align 8
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca ptr, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  store ptr %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %11 = load ptr, ptr %3, align 8
  %12 = getelementptr inbounds %struct.EpMap, ptr %11, i32 0, i32 0
  %13 = load ptr, ptr %12, align 8
  store ptr %13, ptr %5, align 8
  %14 = load ptr, ptr %3, align 8
  %15 = getelementptr inbounds %struct.EpMap, ptr %14, i32 0, i32 1
  %16 = load i64, ptr %15, align 8
  store i64 %16, ptr %6, align 8
  %17 = load i64, ptr %4, align 8
  %18 = load ptr, ptr %3, align 8
  %19 = getelementptr inbounds %struct.EpMap, ptr %18, i32 0, i32 1
  store i64 %17, ptr %19, align 8
  %20 = load i64, ptr %4, align 8
  %21 = call ptr @calloc(i64 noundef %20, i64 noundef 24) #15
  %22 = load ptr, ptr %3, align 8
  %23 = getelementptr inbounds %struct.EpMap, ptr %22, i32 0, i32 0
  store ptr %21, ptr %23, align 8
  %24 = load ptr, ptr %3, align 8
  %25 = getelementptr inbounds %struct.EpMap, ptr %24, i32 0, i32 2
  store i64 0, ptr %25, align 8
  store i64 0, ptr %7, align 8
  br label %26

26:                                               ; preds = %99, %2
  %27 = load i64, ptr %7, align 8
  %28 = load i64, ptr %6, align 8
  %29 = icmp slt i64 %27, %28
  br i1 %29, label %30, label %102

30:                                               ; preds = %26
  %31 = load ptr, ptr %5, align 8
  %32 = load i64, ptr %7, align 8
  %33 = getelementptr inbounds %struct.EpMapEntry, ptr %31, i64 %32
  %34 = getelementptr inbounds %struct.EpMapEntry, ptr %33, i32 0, i32 2
  %35 = load i32, ptr %34, align 8
  %36 = icmp ne i32 %35, 0
  br i1 %36, label %37, label %98

37:                                               ; preds = %30
  %38 = load ptr, ptr %5, align 8
  %39 = load i64, ptr %7, align 8
  %40 = getelementptr inbounds %struct.EpMapEntry, ptr %38, i64 %39
  %41 = getelementptr inbounds %struct.EpMapEntry, ptr %40, i32 0, i32 0
  %42 = load ptr, ptr %41, align 8
  %43 = icmp ne ptr %42, null
  br i1 %43, label %44, label %98

44:                                               ; preds = %37
  %45 = load ptr, ptr %5, align 8
  %46 = load i64, ptr %7, align 8
  %47 = getelementptr inbounds %struct.EpMapEntry, ptr %45, i64 %46
  %48 = getelementptr inbounds %struct.EpMapEntry, ptr %47, i32 0, i32 0
  %49 = load ptr, ptr %48, align 8
  store ptr %49, ptr %8, align 8
  %50 = load ptr, ptr %5, align 8
  %51 = load i64, ptr %7, align 8
  %52 = getelementptr inbounds %struct.EpMapEntry, ptr %50, i64 %51
  %53 = getelementptr inbounds %struct.EpMapEntry, ptr %52, i32 0, i32 1
  %54 = load i64, ptr %53, align 8
  store i64 %54, ptr %9, align 8
  %55 = load ptr, ptr %8, align 8
  %56 = call i64 @hash_string(ptr noundef %55)
  %57 = load i64, ptr %4, align 8
  %58 = urem i64 %56, %57
  store i64 %58, ptr %10, align 8
  br label %59

59:                                               ; preds = %68, %44
  %60 = load ptr, ptr %3, align 8
  %61 = getelementptr inbounds %struct.EpMap, ptr %60, i32 0, i32 0
  %62 = load ptr, ptr %61, align 8
  %63 = load i64, ptr %10, align 8
  %64 = getelementptr inbounds %struct.EpMapEntry, ptr %62, i64 %63
  %65 = getelementptr inbounds %struct.EpMapEntry, ptr %64, i32 0, i32 2
  %66 = load i32, ptr %65, align 8
  %67 = icmp ne i32 %66, 0
  br i1 %67, label %68, label %73

68:                                               ; preds = %59
  %69 = load i64, ptr %10, align 8
  %70 = add i64 %69, 1
  %71 = load i64, ptr %4, align 8
  %72 = urem i64 %70, %71
  store i64 %72, ptr %10, align 8
  br label %59, !llvm.loop !12

73:                                               ; preds = %59
  %74 = load ptr, ptr %8, align 8
  %75 = load ptr, ptr %3, align 8
  %76 = getelementptr inbounds %struct.EpMap, ptr %75, i32 0, i32 0
  %77 = load ptr, ptr %76, align 8
  %78 = load i64, ptr %10, align 8
  %79 = getelementptr inbounds %struct.EpMapEntry, ptr %77, i64 %78
  %80 = getelementptr inbounds %struct.EpMapEntry, ptr %79, i32 0, i32 0
  store ptr %74, ptr %80, align 8
  %81 = load i64, ptr %9, align 8
  %82 = load ptr, ptr %3, align 8
  %83 = getelementptr inbounds %struct.EpMap, ptr %82, i32 0, i32 0
  %84 = load ptr, ptr %83, align 8
  %85 = load i64, ptr %10, align 8
  %86 = getelementptr inbounds %struct.EpMapEntry, ptr %84, i64 %85
  %87 = getelementptr inbounds %struct.EpMapEntry, ptr %86, i32 0, i32 1
  store i64 %81, ptr %87, align 8
  %88 = load ptr, ptr %3, align 8
  %89 = getelementptr inbounds %struct.EpMap, ptr %88, i32 0, i32 0
  %90 = load ptr, ptr %89, align 8
  %91 = load i64, ptr %10, align 8
  %92 = getelementptr inbounds %struct.EpMapEntry, ptr %90, i64 %91
  %93 = getelementptr inbounds %struct.EpMapEntry, ptr %92, i32 0, i32 2
  store i32 1, ptr %93, align 8
  %94 = load ptr, ptr %3, align 8
  %95 = getelementptr inbounds %struct.EpMap, ptr %94, i32 0, i32 2
  %96 = load i64, ptr %95, align 8
  %97 = add nsw i64 %96, 1
  store i64 %97, ptr %95, align 8
  br label %98

98:                                               ; preds = %73, %37, %30
  br label %99

99:                                               ; preds = %98
  %100 = load i64, ptr %7, align 8
  %101 = add nsw i64 %100, 1
  store i64 %101, ptr %7, align 8
  br label %26, !llvm.loop !13

102:                                              ; preds = %26
  %103 = load ptr, ptr %5, align 8
  call void @free(ptr noundef %103)
  ret void
}

; Function Attrs: nounwind
declare i32 @strcmp(ptr noundef, ptr noundef) #4

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_write_barrier(ptr noundef %0, i64 noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca i64, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i32, align 4
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca ptr, align 8
  store ptr %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %11 = load i64, ptr %4, align 8
  %12 = icmp eq i64 %11, 0
  br i1 %12, label %13, label %14

13:                                               ; preds = %2
  br label %94

14:                                               ; preds = %2
  %15 = load ptr, ptr %3, align 8
  %16 = call ptr @ep_gc_find(ptr noundef %15)
  store ptr %16, ptr %5, align 8
  %17 = load i64, ptr %4, align 8
  %18 = inttoptr i64 %17 to ptr
  %19 = call ptr @ep_gc_find(ptr noundef %18)
  store ptr %19, ptr %6, align 8
  %20 = load ptr, ptr %5, align 8
  %21 = icmp ne ptr %20, null
  br i1 %21, label %22, label %94

22:                                               ; preds = %14
  %23 = load ptr, ptr %6, align 8
  %24 = icmp ne ptr %23, null
  br i1 %24, label %25, label %94

25:                                               ; preds = %22
  %26 = load ptr, ptr %5, align 8
  %27 = getelementptr inbounds %struct.EpGCObject, ptr %26, i32 0, i32 5
  %28 = load i32, ptr %27, align 8
  %29 = icmp eq i32 %28, 1
  br i1 %29, label %30, label %94

30:                                               ; preds = %25
  %31 = load ptr, ptr %6, align 8
  %32 = getelementptr inbounds %struct.EpGCObject, ptr %31, i32 0, i32 5
  %33 = load i32, ptr %32, align 8
  %34 = icmp eq i32 %33, 0
  br i1 %34, label %35, label %94

35:                                               ; preds = %30
  %36 = call i32 @pthread_mutex_lock(ptr noundef @ep_gc_mutex)
  store i32 0, ptr %7, align 4
  store i64 0, ptr %8, align 8
  br label %37

37:                                               ; preds = %51, %35
  %38 = load i64, ptr %8, align 8
  %39 = load i64, ptr @ep_gc_remembered_size, align 8
  %40 = icmp slt i64 %38, %39
  br i1 %40, label %41, label %54

41:                                               ; preds = %37
  %42 = load ptr, ptr @ep_gc_remembered_set, align 8
  %43 = load i64, ptr %8, align 8
  %44 = getelementptr inbounds ptr, ptr %42, i64 %43
  %45 = load ptr, ptr %44, align 8
  %46 = load i64, ptr %4, align 8
  %47 = inttoptr i64 %46 to ptr
  %48 = icmp eq ptr %45, %47
  br i1 %48, label %49, label %50

49:                                               ; preds = %41
  store i32 1, ptr %7, align 4
  br label %54

50:                                               ; preds = %41
  br label %51

51:                                               ; preds = %50
  %52 = load i64, ptr %8, align 8
  %53 = add nsw i64 %52, 1
  store i64 %53, ptr %8, align 8
  br label %37, !llvm.loop !14

54:                                               ; preds = %49, %37
  %55 = load i32, ptr %7, align 4
  %56 = icmp ne i32 %55, 0
  br i1 %56, label %92, label %57

57:                                               ; preds = %54
  %58 = load i64, ptr @ep_gc_remembered_size, align 8
  %59 = load i64, ptr @ep_gc_remembered_cap, align 8
  %60 = icmp sge i64 %58, %59
  br i1 %60, label %61, label %80

61:                                               ; preds = %57
  %62 = load i64, ptr @ep_gc_remembered_cap, align 8
  %63 = icmp eq i64 %62, 0
  br i1 %63, label %64, label %65

64:                                               ; preds = %61
  br label %68

65:                                               ; preds = %61
  %66 = load i64, ptr @ep_gc_remembered_cap, align 8
  %67 = mul nsw i64 %66, 2
  br label %68

68:                                               ; preds = %65, %64
  %69 = phi i64 [ 128, %64 ], [ %67, %65 ]
  store i64 %69, ptr %9, align 8
  %70 = load ptr, ptr @ep_gc_remembered_set, align 8
  %71 = load i64, ptr %9, align 8
  %72 = mul i64 %71, 8
  %73 = call ptr @realloc(ptr noundef %70, i64 noundef %72) #16
  store ptr %73, ptr %10, align 8
  %74 = load ptr, ptr %10, align 8
  %75 = icmp ne ptr %74, null
  br i1 %75, label %76, label %79

76:                                               ; preds = %68
  %77 = load ptr, ptr %10, align 8
  store ptr %77, ptr @ep_gc_remembered_set, align 8
  %78 = load i64, ptr %9, align 8
  store i64 %78, ptr @ep_gc_remembered_cap, align 8
  br label %79

79:                                               ; preds = %76, %68
  br label %80

80:                                               ; preds = %79, %57
  %81 = load i64, ptr @ep_gc_remembered_size, align 8
  %82 = load i64, ptr @ep_gc_remembered_cap, align 8
  %83 = icmp slt i64 %81, %82
  br i1 %83, label %84, label %91

84:                                               ; preds = %80
  %85 = load i64, ptr %4, align 8
  %86 = inttoptr i64 %85 to ptr
  %87 = load ptr, ptr @ep_gc_remembered_set, align 8
  %88 = load i64, ptr @ep_gc_remembered_size, align 8
  %89 = add nsw i64 %88, 1
  store i64 %89, ptr @ep_gc_remembered_size, align 8
  %90 = getelementptr inbounds ptr, ptr %87, i64 %88
  store ptr %86, ptr %90, align 8
  br label %91

91:                                               ; preds = %84, %80
  br label %92

92:                                               ; preds = %91, %54
  %93 = call i32 @pthread_mutex_unlock(ptr noundef @ep_gc_mutex)
  br label %94

94:                                               ; preds = %13, %92, %30, %25, %22, %14
  ret void
}

; Function Attrs: nounwind
declare ptr @strdup(ptr noundef) #4

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_get_val(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca [32 x i8], align 1
  %8 = alloca ptr, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %11 = load i64, ptr %4, align 8
  %12 = inttoptr i64 %11 to ptr
  store ptr %12, ptr %6, align 8
  %13 = load i64, ptr %5, align 8
  %14 = getelementptr inbounds [32 x i8], ptr %7, i64 0, i64 0
  %15 = call ptr @ep_map_key_str(i64 noundef %13, ptr noundef %14, i32 noundef 32)
  store ptr %15, ptr %8, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %19, label %18

18:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %77

19:                                               ; preds = %2
  %20 = load ptr, ptr %8, align 8
  %21 = call i64 @hash_string(ptr noundef %20)
  %22 = load ptr, ptr %6, align 8
  %23 = getelementptr inbounds %struct.EpMap, ptr %22, i32 0, i32 1
  %24 = load i64, ptr %23, align 8
  %25 = urem i64 %21, %24
  store i64 %25, ptr %9, align 8
  %26 = load i64, ptr %9, align 8
  store i64 %26, ptr %10, align 8
  br label %27

27:                                               ; preds = %75, %19
  %28 = load ptr, ptr %6, align 8
  %29 = getelementptr inbounds %struct.EpMap, ptr %28, i32 0, i32 0
  %30 = load ptr, ptr %29, align 8
  %31 = load i64, ptr %9, align 8
  %32 = getelementptr inbounds %struct.EpMapEntry, ptr %30, i64 %31
  %33 = getelementptr inbounds %struct.EpMapEntry, ptr %32, i32 0, i32 2
  %34 = load i32, ptr %33, align 8
  %35 = icmp ne i32 %34, 0
  br i1 %35, label %36, label %76

36:                                               ; preds = %27
  %37 = load ptr, ptr %6, align 8
  %38 = getelementptr inbounds %struct.EpMap, ptr %37, i32 0, i32 0
  %39 = load ptr, ptr %38, align 8
  %40 = load i64, ptr %9, align 8
  %41 = getelementptr inbounds %struct.EpMapEntry, ptr %39, i64 %40
  %42 = getelementptr inbounds %struct.EpMapEntry, ptr %41, i32 0, i32 0
  %43 = load ptr, ptr %42, align 8
  %44 = icmp ne ptr %43, null
  br i1 %44, label %45, label %64

45:                                               ; preds = %36
  %46 = load ptr, ptr %6, align 8
  %47 = getelementptr inbounds %struct.EpMap, ptr %46, i32 0, i32 0
  %48 = load ptr, ptr %47, align 8
  %49 = load i64, ptr %9, align 8
  %50 = getelementptr inbounds %struct.EpMapEntry, ptr %48, i64 %49
  %51 = getelementptr inbounds %struct.EpMapEntry, ptr %50, i32 0, i32 0
  %52 = load ptr, ptr %51, align 8
  %53 = load ptr, ptr %8, align 8
  %54 = call i32 @strcmp(ptr noundef %52, ptr noundef %53) #13
  %55 = icmp eq i32 %54, 0
  br i1 %55, label %56, label %64

56:                                               ; preds = %45
  %57 = load ptr, ptr %6, align 8
  %58 = getelementptr inbounds %struct.EpMap, ptr %57, i32 0, i32 0
  %59 = load ptr, ptr %58, align 8
  %60 = load i64, ptr %9, align 8
  %61 = getelementptr inbounds %struct.EpMapEntry, ptr %59, i64 %60
  %62 = getelementptr inbounds %struct.EpMapEntry, ptr %61, i32 0, i32 1
  %63 = load i64, ptr %62, align 8
  store i64 %63, ptr %3, align 8
  br label %77

64:                                               ; preds = %45, %36
  %65 = load i64, ptr %9, align 8
  %66 = add i64 %65, 1
  %67 = load ptr, ptr %6, align 8
  %68 = getelementptr inbounds %struct.EpMap, ptr %67, i32 0, i32 1
  %69 = load i64, ptr %68, align 8
  %70 = urem i64 %66, %69
  store i64 %70, ptr %9, align 8
  %71 = load i64, ptr %9, align 8
  %72 = load i64, ptr %10, align 8
  %73 = icmp eq i64 %71, %72
  br i1 %73, label %74, label %75

74:                                               ; preds = %64
  br label %76

75:                                               ; preds = %64
  br label %27, !llvm.loop !15

76:                                               ; preds = %74, %27
  store i64 0, ptr %3, align 8
  br label %77

77:                                               ; preds = %76, %56, %18
  %78 = load i64, ptr %3, align 8
  ret i64 %78
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_set_str(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  %7 = load i64, ptr %4, align 8
  %8 = load i64, ptr %5, align 8
  %9 = load i64, ptr %6, align 8
  %10 = call i64 @map_insert(i64 noundef %7, i64 noundef %8, i64 noundef %9)
  ret i64 %10
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_get_str(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = load i64, ptr %4, align 8
  %7 = call i64 @map_get_val(i64 noundef %5, i64 noundef %6)
  ret i64 %7
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_contains(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca [32 x i8], align 1
  %8 = alloca ptr, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %11 = load i64, ptr %4, align 8
  %12 = inttoptr i64 %11 to ptr
  store ptr %12, ptr %6, align 8
  %13 = load i64, ptr %5, align 8
  %14 = getelementptr inbounds [32 x i8], ptr %7, i64 0, i64 0
  %15 = call ptr @ep_map_key_str(i64 noundef %13, ptr noundef %14, i32 noundef 32)
  store ptr %15, ptr %8, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %19, label %18

18:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %70

19:                                               ; preds = %2
  %20 = load ptr, ptr %8, align 8
  %21 = call i64 @hash_string(ptr noundef %20)
  %22 = load ptr, ptr %6, align 8
  %23 = getelementptr inbounds %struct.EpMap, ptr %22, i32 0, i32 1
  %24 = load i64, ptr %23, align 8
  %25 = urem i64 %21, %24
  store i64 %25, ptr %9, align 8
  %26 = load i64, ptr %9, align 8
  store i64 %26, ptr %10, align 8
  br label %27

27:                                               ; preds = %68, %19
  %28 = load ptr, ptr %6, align 8
  %29 = getelementptr inbounds %struct.EpMap, ptr %28, i32 0, i32 0
  %30 = load ptr, ptr %29, align 8
  %31 = load i64, ptr %9, align 8
  %32 = getelementptr inbounds %struct.EpMapEntry, ptr %30, i64 %31
  %33 = getelementptr inbounds %struct.EpMapEntry, ptr %32, i32 0, i32 2
  %34 = load i32, ptr %33, align 8
  %35 = icmp ne i32 %34, 0
  br i1 %35, label %36, label %69

36:                                               ; preds = %27
  %37 = load ptr, ptr %6, align 8
  %38 = getelementptr inbounds %struct.EpMap, ptr %37, i32 0, i32 0
  %39 = load ptr, ptr %38, align 8
  %40 = load i64, ptr %9, align 8
  %41 = getelementptr inbounds %struct.EpMapEntry, ptr %39, i64 %40
  %42 = getelementptr inbounds %struct.EpMapEntry, ptr %41, i32 0, i32 0
  %43 = load ptr, ptr %42, align 8
  %44 = icmp ne ptr %43, null
  br i1 %44, label %45, label %57

45:                                               ; preds = %36
  %46 = load ptr, ptr %6, align 8
  %47 = getelementptr inbounds %struct.EpMap, ptr %46, i32 0, i32 0
  %48 = load ptr, ptr %47, align 8
  %49 = load i64, ptr %9, align 8
  %50 = getelementptr inbounds %struct.EpMapEntry, ptr %48, i64 %49
  %51 = getelementptr inbounds %struct.EpMapEntry, ptr %50, i32 0, i32 0
  %52 = load ptr, ptr %51, align 8
  %53 = load ptr, ptr %8, align 8
  %54 = call i32 @strcmp(ptr noundef %52, ptr noundef %53) #13
  %55 = icmp eq i32 %54, 0
  br i1 %55, label %56, label %57

56:                                               ; preds = %45
  store i64 1, ptr %3, align 8
  br label %70

57:                                               ; preds = %45, %36
  %58 = load i64, ptr %9, align 8
  %59 = add i64 %58, 1
  %60 = load ptr, ptr %6, align 8
  %61 = getelementptr inbounds %struct.EpMap, ptr %60, i32 0, i32 1
  %62 = load i64, ptr %61, align 8
  %63 = urem i64 %59, %62
  store i64 %63, ptr %9, align 8
  %64 = load i64, ptr %9, align 8
  %65 = load i64, ptr %10, align 8
  %66 = icmp eq i64 %64, %65
  br i1 %66, label %67, label %68

67:                                               ; preds = %57
  br label %69

68:                                               ; preds = %57
  br label %27, !llvm.loop !16

69:                                               ; preds = %67, %27
  store i64 0, ptr %3, align 8
  br label %70

70:                                               ; preds = %69, %56, %18
  %71 = load i64, ptr %3, align 8
  ret i64 %71
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_delete(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca [32 x i8], align 1
  %8 = alloca ptr, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca ptr, align 8
  %13 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %14 = load i64, ptr %4, align 8
  %15 = inttoptr i64 %14 to ptr
  store ptr %15, ptr %6, align 8
  %16 = load i64, ptr %5, align 8
  %17 = getelementptr inbounds [32 x i8], ptr %7, i64 0, i64 0
  %18 = call ptr @ep_map_key_str(i64 noundef %16, ptr noundef %17, i32 noundef 32)
  store ptr %18, ptr %8, align 8
  %19 = load ptr, ptr %6, align 8
  %20 = icmp ne ptr %19, null
  br i1 %20, label %22, label %21

21:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %167

22:                                               ; preds = %2
  %23 = load ptr, ptr %8, align 8
  %24 = call i64 @hash_string(ptr noundef %23)
  %25 = load ptr, ptr %6, align 8
  %26 = getelementptr inbounds %struct.EpMap, ptr %25, i32 0, i32 1
  %27 = load i64, ptr %26, align 8
  %28 = urem i64 %24, %27
  store i64 %28, ptr %9, align 8
  %29 = load i64, ptr %9, align 8
  store i64 %29, ptr %10, align 8
  br label %30

30:                                               ; preds = %165, %22
  %31 = load ptr, ptr %6, align 8
  %32 = getelementptr inbounds %struct.EpMap, ptr %31, i32 0, i32 0
  %33 = load ptr, ptr %32, align 8
  %34 = load i64, ptr %9, align 8
  %35 = getelementptr inbounds %struct.EpMapEntry, ptr %33, i64 %34
  %36 = getelementptr inbounds %struct.EpMapEntry, ptr %35, i32 0, i32 2
  %37 = load i32, ptr %36, align 8
  %38 = icmp ne i32 %37, 0
  br i1 %38, label %39, label %166

39:                                               ; preds = %30
  %40 = load ptr, ptr %6, align 8
  %41 = getelementptr inbounds %struct.EpMap, ptr %40, i32 0, i32 0
  %42 = load ptr, ptr %41, align 8
  %43 = load i64, ptr %9, align 8
  %44 = getelementptr inbounds %struct.EpMapEntry, ptr %42, i64 %43
  %45 = getelementptr inbounds %struct.EpMapEntry, ptr %44, i32 0, i32 0
  %46 = load ptr, ptr %45, align 8
  %47 = icmp ne ptr %46, null
  br i1 %47, label %48, label %154

48:                                               ; preds = %39
  %49 = load ptr, ptr %6, align 8
  %50 = getelementptr inbounds %struct.EpMap, ptr %49, i32 0, i32 0
  %51 = load ptr, ptr %50, align 8
  %52 = load i64, ptr %9, align 8
  %53 = getelementptr inbounds %struct.EpMapEntry, ptr %51, i64 %52
  %54 = getelementptr inbounds %struct.EpMapEntry, ptr %53, i32 0, i32 0
  %55 = load ptr, ptr %54, align 8
  %56 = load ptr, ptr %8, align 8
  %57 = call i32 @strcmp(ptr noundef %55, ptr noundef %56) #13
  %58 = icmp eq i32 %57, 0
  br i1 %58, label %59, label %154

59:                                               ; preds = %48
  %60 = load ptr, ptr %6, align 8
  %61 = getelementptr inbounds %struct.EpMap, ptr %60, i32 0, i32 0
  %62 = load ptr, ptr %61, align 8
  %63 = load i64, ptr %9, align 8
  %64 = getelementptr inbounds %struct.EpMapEntry, ptr %62, i64 %63
  %65 = getelementptr inbounds %struct.EpMapEntry, ptr %64, i32 0, i32 0
  %66 = load ptr, ptr %65, align 8
  call void @free(ptr noundef %66)
  %67 = load ptr, ptr %6, align 8
  %68 = getelementptr inbounds %struct.EpMap, ptr %67, i32 0, i32 0
  %69 = load ptr, ptr %68, align 8
  %70 = load i64, ptr %9, align 8
  %71 = getelementptr inbounds %struct.EpMapEntry, ptr %69, i64 %70
  %72 = getelementptr inbounds %struct.EpMapEntry, ptr %71, i32 0, i32 0
  store ptr null, ptr %72, align 8
  %73 = load ptr, ptr %6, align 8
  %74 = getelementptr inbounds %struct.EpMap, ptr %73, i32 0, i32 0
  %75 = load ptr, ptr %74, align 8
  %76 = load i64, ptr %9, align 8
  %77 = getelementptr inbounds %struct.EpMapEntry, ptr %75, i64 %76
  %78 = getelementptr inbounds %struct.EpMapEntry, ptr %77, i32 0, i32 1
  store i64 0, ptr %78, align 8
  %79 = load ptr, ptr %6, align 8
  %80 = getelementptr inbounds %struct.EpMap, ptr %79, i32 0, i32 0
  %81 = load ptr, ptr %80, align 8
  %82 = load i64, ptr %9, align 8
  %83 = getelementptr inbounds %struct.EpMapEntry, ptr %81, i64 %82
  %84 = getelementptr inbounds %struct.EpMapEntry, ptr %83, i32 0, i32 2
  store i32 0, ptr %84, align 8
  %85 = load ptr, ptr %6, align 8
  %86 = getelementptr inbounds %struct.EpMap, ptr %85, i32 0, i32 2
  %87 = load i64, ptr %86, align 8
  %88 = add nsw i64 %87, -1
  store i64 %88, ptr %86, align 8
  %89 = load i64, ptr %9, align 8
  %90 = add i64 %89, 1
  %91 = load ptr, ptr %6, align 8
  %92 = getelementptr inbounds %struct.EpMap, ptr %91, i32 0, i32 1
  %93 = load i64, ptr %92, align 8
  %94 = urem i64 %90, %93
  store i64 %94, ptr %11, align 8
  br label %95

95:                                               ; preds = %104, %59
  %96 = load ptr, ptr %6, align 8
  %97 = getelementptr inbounds %struct.EpMap, ptr %96, i32 0, i32 0
  %98 = load ptr, ptr %97, align 8
  %99 = load i64, ptr %11, align 8
  %100 = getelementptr inbounds %struct.EpMapEntry, ptr %98, i64 %99
  %101 = getelementptr inbounds %struct.EpMapEntry, ptr %100, i32 0, i32 2
  %102 = load i32, ptr %101, align 8
  %103 = icmp ne i32 %102, 0
  br i1 %103, label %104, label %153

104:                                              ; preds = %95
  %105 = load ptr, ptr %6, align 8
  %106 = getelementptr inbounds %struct.EpMap, ptr %105, i32 0, i32 0
  %107 = load ptr, ptr %106, align 8
  %108 = load i64, ptr %11, align 8
  %109 = getelementptr inbounds %struct.EpMapEntry, ptr %107, i64 %108
  %110 = getelementptr inbounds %struct.EpMapEntry, ptr %109, i32 0, i32 0
  %111 = load ptr, ptr %110, align 8
  store ptr %111, ptr %12, align 8
  %112 = load ptr, ptr %6, align 8
  %113 = getelementptr inbounds %struct.EpMap, ptr %112, i32 0, i32 0
  %114 = load ptr, ptr %113, align 8
  %115 = load i64, ptr %11, align 8
  %116 = getelementptr inbounds %struct.EpMapEntry, ptr %114, i64 %115
  %117 = getelementptr inbounds %struct.EpMapEntry, ptr %116, i32 0, i32 1
  %118 = load i64, ptr %117, align 8
  store i64 %118, ptr %13, align 8
  %119 = load ptr, ptr %6, align 8
  %120 = getelementptr inbounds %struct.EpMap, ptr %119, i32 0, i32 0
  %121 = load ptr, ptr %120, align 8
  %122 = load i64, ptr %11, align 8
  %123 = getelementptr inbounds %struct.EpMapEntry, ptr %121, i64 %122
  %124 = getelementptr inbounds %struct.EpMapEntry, ptr %123, i32 0, i32 0
  store ptr null, ptr %124, align 8
  %125 = load ptr, ptr %6, align 8
  %126 = getelementptr inbounds %struct.EpMap, ptr %125, i32 0, i32 0
  %127 = load ptr, ptr %126, align 8
  %128 = load i64, ptr %11, align 8
  %129 = getelementptr inbounds %struct.EpMapEntry, ptr %127, i64 %128
  %130 = getelementptr inbounds %struct.EpMapEntry, ptr %129, i32 0, i32 1
  store i64 0, ptr %130, align 8
  %131 = load ptr, ptr %6, align 8
  %132 = getelementptr inbounds %struct.EpMap, ptr %131, i32 0, i32 0
  %133 = load ptr, ptr %132, align 8
  %134 = load i64, ptr %11, align 8
  %135 = getelementptr inbounds %struct.EpMapEntry, ptr %133, i64 %134
  %136 = getelementptr inbounds %struct.EpMapEntry, ptr %135, i32 0, i32 2
  store i32 0, ptr %136, align 8
  %137 = load ptr, ptr %6, align 8
  %138 = getelementptr inbounds %struct.EpMap, ptr %137, i32 0, i32 2
  %139 = load i64, ptr %138, align 8
  %140 = add nsw i64 %139, -1
  store i64 %140, ptr %138, align 8
  %141 = load i64, ptr %4, align 8
  %142 = load ptr, ptr %12, align 8
  %143 = ptrtoint ptr %142 to i64
  %144 = load i64, ptr %13, align 8
  %145 = call i64 @map_insert(i64 noundef %141, i64 noundef %143, i64 noundef %144)
  %146 = load ptr, ptr %12, align 8
  call void @free(ptr noundef %146)
  %147 = load i64, ptr %11, align 8
  %148 = add nsw i64 %147, 1
  %149 = load ptr, ptr %6, align 8
  %150 = getelementptr inbounds %struct.EpMap, ptr %149, i32 0, i32 1
  %151 = load i64, ptr %150, align 8
  %152 = srem i64 %148, %151
  store i64 %152, ptr %11, align 8
  br label %95, !llvm.loop !17

153:                                              ; preds = %95
  store i64 1, ptr %3, align 8
  br label %167

154:                                              ; preds = %48, %39
  %155 = load i64, ptr %9, align 8
  %156 = add i64 %155, 1
  %157 = load ptr, ptr %6, align 8
  %158 = getelementptr inbounds %struct.EpMap, ptr %157, i32 0, i32 1
  %159 = load i64, ptr %158, align 8
  %160 = urem i64 %156, %159
  store i64 %160, ptr %9, align 8
  %161 = load i64, ptr %9, align 8
  %162 = load i64, ptr %10, align 8
  %163 = icmp eq i64 %161, %162
  br i1 %163, label %164, label %165

164:                                              ; preds = %154
  br label %166

165:                                              ; preds = %154
  br label %30, !llvm.loop !18

166:                                              ; preds = %164, %30
  store i64 0, ptr %3, align 8
  br label %167

167:                                              ; preds = %166, %153, %21
  %168 = load i64, ptr %3, align 8
  ret i64 %168
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_keys(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %7 = load i64, ptr %3, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %4, align 8
  %9 = load ptr, ptr %4, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %13, label %11

11:                                               ; preds = %1
  %12 = call i64 @create_list()
  store i64 %12, ptr %2, align 8
  br label %57

13:                                               ; preds = %1
  %14 = call i64 @create_list()
  store i64 %14, ptr %5, align 8
  store i64 0, ptr %6, align 8
  br label %15

15:                                               ; preds = %52, %13
  %16 = load i64, ptr %6, align 8
  %17 = load ptr, ptr %4, align 8
  %18 = getelementptr inbounds %struct.EpMap, ptr %17, i32 0, i32 1
  %19 = load i64, ptr %18, align 8
  %20 = icmp slt i64 %16, %19
  br i1 %20, label %21, label %55

21:                                               ; preds = %15
  %22 = load ptr, ptr %4, align 8
  %23 = getelementptr inbounds %struct.EpMap, ptr %22, i32 0, i32 0
  %24 = load ptr, ptr %23, align 8
  %25 = load i64, ptr %6, align 8
  %26 = getelementptr inbounds %struct.EpMapEntry, ptr %24, i64 %25
  %27 = getelementptr inbounds %struct.EpMapEntry, ptr %26, i32 0, i32 2
  %28 = load i32, ptr %27, align 8
  %29 = icmp ne i32 %28, 0
  br i1 %29, label %30, label %51

30:                                               ; preds = %21
  %31 = load ptr, ptr %4, align 8
  %32 = getelementptr inbounds %struct.EpMap, ptr %31, i32 0, i32 0
  %33 = load ptr, ptr %32, align 8
  %34 = load i64, ptr %6, align 8
  %35 = getelementptr inbounds %struct.EpMapEntry, ptr %33, i64 %34
  %36 = getelementptr inbounds %struct.EpMapEntry, ptr %35, i32 0, i32 0
  %37 = load ptr, ptr %36, align 8
  %38 = icmp ne ptr %37, null
  br i1 %38, label %39, label %51

39:                                               ; preds = %30
  %40 = load i64, ptr %5, align 8
  %41 = load ptr, ptr %4, align 8
  %42 = getelementptr inbounds %struct.EpMap, ptr %41, i32 0, i32 0
  %43 = load ptr, ptr %42, align 8
  %44 = load i64, ptr %6, align 8
  %45 = getelementptr inbounds %struct.EpMapEntry, ptr %43, i64 %44
  %46 = getelementptr inbounds %struct.EpMapEntry, ptr %45, i32 0, i32 0
  %47 = load ptr, ptr %46, align 8
  %48 = call ptr @strdup(ptr noundef %47) #13
  %49 = ptrtoint ptr %48 to i64
  %50 = call i64 @append_list(i64 noundef %40, i64 noundef %49)
  br label %51

51:                                               ; preds = %39, %30, %21
  br label %52

52:                                               ; preds = %51
  %53 = load i64, ptr %6, align 8
  %54 = add nsw i64 %53, 1
  store i64 %54, ptr %6, align 8
  br label %15, !llvm.loop !19

55:                                               ; preds = %15
  %56 = load i64, ptr %5, align 8
  store i64 %56, ptr %2, align 8
  br label %57

57:                                               ; preds = %55, %11
  %58 = load i64, ptr %2, align 8
  ret i64 %58
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @create_list() #0 {
  %1 = alloca i64, align 8
  %2 = alloca ptr, align 8
  %3 = call ptr @malloc(i64 noundef 24) #14
  store ptr %3, ptr %2, align 8
  %4 = load ptr, ptr %2, align 8
  %5 = icmp ne ptr %4, null
  br i1 %5, label %7, label %6

6:                                                ; preds = %0
  store i64 0, ptr %1, align 8
  br label %23

7:                                                ; preds = %0
  %8 = load ptr, ptr %2, align 8
  %9 = getelementptr inbounds %struct.EpList, ptr %8, i32 0, i32 2
  store i64 4, ptr %9, align 8
  %10 = load ptr, ptr %2, align 8
  %11 = getelementptr inbounds %struct.EpList, ptr %10, i32 0, i32 1
  store i64 0, ptr %11, align 8
  %12 = load ptr, ptr %2, align 8
  %13 = getelementptr inbounds %struct.EpList, ptr %12, i32 0, i32 2
  %14 = load i64, ptr %13, align 8
  %15 = mul i64 %14, 8
  %16 = call ptr @malloc(i64 noundef %15) #14
  %17 = load ptr, ptr %2, align 8
  %18 = getelementptr inbounds %struct.EpList, ptr %17, i32 0, i32 0
  store ptr %16, ptr %18, align 8
  %19 = load ptr, ptr %2, align 8
  %20 = call ptr @ep_gc_register(ptr noundef %19, i32 noundef 0)
  %21 = load ptr, ptr %2, align 8
  %22 = ptrtoint ptr %21 to i64
  store i64 %22, ptr %1, align 8
  br label %23

23:                                               ; preds = %7, %6
  %24 = load i64, ptr %1, align 8
  ret i64 %24
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @append_list(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %6, align 8
  %9 = load ptr, ptr %6, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %12, label %11

11:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %52

12:                                               ; preds = %2
  %13 = load ptr, ptr %6, align 8
  %14 = getelementptr inbounds %struct.EpList, ptr %13, i32 0, i32 1
  %15 = load i64, ptr %14, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = getelementptr inbounds %struct.EpList, ptr %16, i32 0, i32 2
  %18 = load i64, ptr %17, align 8
  %19 = icmp sge i64 %15, %18
  br i1 %19, label %20, label %35

20:                                               ; preds = %12
  %21 = load ptr, ptr %6, align 8
  %22 = getelementptr inbounds %struct.EpList, ptr %21, i32 0, i32 2
  %23 = load i64, ptr %22, align 8
  %24 = mul nsw i64 %23, 2
  store i64 %24, ptr %22, align 8
  %25 = load ptr, ptr %6, align 8
  %26 = getelementptr inbounds %struct.EpList, ptr %25, i32 0, i32 0
  %27 = load ptr, ptr %26, align 8
  %28 = load ptr, ptr %6, align 8
  %29 = getelementptr inbounds %struct.EpList, ptr %28, i32 0, i32 2
  %30 = load i64, ptr %29, align 8
  %31 = mul i64 %30, 8
  %32 = call ptr @realloc(ptr noundef %27, i64 noundef %31) #16
  %33 = load ptr, ptr %6, align 8
  %34 = getelementptr inbounds %struct.EpList, ptr %33, i32 0, i32 0
  store ptr %32, ptr %34, align 8
  br label %35

35:                                               ; preds = %20, %12
  %36 = load i64, ptr %5, align 8
  %37 = load ptr, ptr %6, align 8
  %38 = getelementptr inbounds %struct.EpList, ptr %37, i32 0, i32 0
  %39 = load ptr, ptr %38, align 8
  %40 = load ptr, ptr %6, align 8
  %41 = getelementptr inbounds %struct.EpList, ptr %40, i32 0, i32 1
  %42 = load i64, ptr %41, align 8
  %43 = getelementptr inbounds i64, ptr %39, i64 %42
  store i64 %36, ptr %43, align 8
  %44 = load ptr, ptr %6, align 8
  %45 = getelementptr inbounds %struct.EpList, ptr %44, i32 0, i32 1
  %46 = load i64, ptr %45, align 8
  %47 = add nsw i64 %46, 1
  store i64 %47, ptr %45, align 8
  %48 = load i64, ptr %4, align 8
  %49 = inttoptr i64 %48 to ptr
  %50 = load i64, ptr %5, align 8
  call void @ep_gc_write_barrier(ptr noundef %49, i64 noundef %50)
  %51 = load i64, ptr %5, align 8
  store i64 %51, ptr %3, align 8
  br label %52

52:                                               ; preds = %35, %11
  %53 = load i64, ptr %3, align 8
  ret i64 %53
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_values(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %7 = load i64, ptr %3, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %4, align 8
  %9 = load ptr, ptr %4, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %13, label %11

11:                                               ; preds = %1
  %12 = call i64 @create_list()
  store i64 %12, ptr %2, align 8
  br label %55

13:                                               ; preds = %1
  %14 = call i64 @create_list()
  store i64 %14, ptr %5, align 8
  store i64 0, ptr %6, align 8
  br label %15

15:                                               ; preds = %50, %13
  %16 = load i64, ptr %6, align 8
  %17 = load ptr, ptr %4, align 8
  %18 = getelementptr inbounds %struct.EpMap, ptr %17, i32 0, i32 1
  %19 = load i64, ptr %18, align 8
  %20 = icmp slt i64 %16, %19
  br i1 %20, label %21, label %53

21:                                               ; preds = %15
  %22 = load ptr, ptr %4, align 8
  %23 = getelementptr inbounds %struct.EpMap, ptr %22, i32 0, i32 0
  %24 = load ptr, ptr %23, align 8
  %25 = load i64, ptr %6, align 8
  %26 = getelementptr inbounds %struct.EpMapEntry, ptr %24, i64 %25
  %27 = getelementptr inbounds %struct.EpMapEntry, ptr %26, i32 0, i32 2
  %28 = load i32, ptr %27, align 8
  %29 = icmp ne i32 %28, 0
  br i1 %29, label %30, label %49

30:                                               ; preds = %21
  %31 = load ptr, ptr %4, align 8
  %32 = getelementptr inbounds %struct.EpMap, ptr %31, i32 0, i32 0
  %33 = load ptr, ptr %32, align 8
  %34 = load i64, ptr %6, align 8
  %35 = getelementptr inbounds %struct.EpMapEntry, ptr %33, i64 %34
  %36 = getelementptr inbounds %struct.EpMapEntry, ptr %35, i32 0, i32 0
  %37 = load ptr, ptr %36, align 8
  %38 = icmp ne ptr %37, null
  br i1 %38, label %39, label %49

39:                                               ; preds = %30
  %40 = load i64, ptr %5, align 8
  %41 = load ptr, ptr %4, align 8
  %42 = getelementptr inbounds %struct.EpMap, ptr %41, i32 0, i32 0
  %43 = load ptr, ptr %42, align 8
  %44 = load i64, ptr %6, align 8
  %45 = getelementptr inbounds %struct.EpMapEntry, ptr %43, i64 %44
  %46 = getelementptr inbounds %struct.EpMapEntry, ptr %45, i32 0, i32 1
  %47 = load i64, ptr %46, align 8
  %48 = call i64 @append_list(i64 noundef %40, i64 noundef %47)
  br label %49

49:                                               ; preds = %39, %30, %21
  br label %50

50:                                               ; preds = %49
  %51 = load i64, ptr %6, align 8
  %52 = add nsw i64 %51, 1
  store i64 %52, ptr %6, align 8
  br label %15, !llvm.loop !20

53:                                               ; preds = %15
  %54 = load i64, ptr %5, align 8
  store i64 %54, ptr %2, align 8
  br label %55

55:                                               ; preds = %53, %11
  %56 = load i64, ptr %2, align 8
  ret i64 %56
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @map_size(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %14

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = getelementptr inbounds %struct.EpMap, ptr %11, i32 0, i32 2
  %13 = load i64, ptr %12, align 8
  store i64 %13, ptr %2, align 8
  br label %14

14:                                               ; preds = %10, %9
  %15 = load i64, ptr %2, align 8
  ret i64 %15
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @free_map(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %11, label %10

10:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %59

11:                                               ; preds = %1
  %12 = load ptr, ptr %4, align 8
  %13 = call ptr @ep_gc_find(ptr noundef %12)
  %14 = icmp ne ptr %13, null
  br i1 %14, label %16, label %15

15:                                               ; preds = %11
  store i64 0, ptr %2, align 8
  br label %59

16:                                               ; preds = %11
  %17 = load ptr, ptr %4, align 8
  call void @ep_gc_unregister(ptr noundef %17)
  store i64 0, ptr %5, align 8
  br label %18

18:                                               ; preds = %51, %16
  %19 = load i64, ptr %5, align 8
  %20 = load ptr, ptr %4, align 8
  %21 = getelementptr inbounds %struct.EpMap, ptr %20, i32 0, i32 1
  %22 = load i64, ptr %21, align 8
  %23 = icmp slt i64 %19, %22
  br i1 %23, label %24, label %54

24:                                               ; preds = %18
  %25 = load ptr, ptr %4, align 8
  %26 = getelementptr inbounds %struct.EpMap, ptr %25, i32 0, i32 0
  %27 = load ptr, ptr %26, align 8
  %28 = load i64, ptr %5, align 8
  %29 = getelementptr inbounds %struct.EpMapEntry, ptr %27, i64 %28
  %30 = getelementptr inbounds %struct.EpMapEntry, ptr %29, i32 0, i32 2
  %31 = load i32, ptr %30, align 8
  %32 = icmp ne i32 %31, 0
  br i1 %32, label %33, label %50

33:                                               ; preds = %24
  %34 = load ptr, ptr %4, align 8
  %35 = getelementptr inbounds %struct.EpMap, ptr %34, i32 0, i32 0
  %36 = load ptr, ptr %35, align 8
  %37 = load i64, ptr %5, align 8
  %38 = getelementptr inbounds %struct.EpMapEntry, ptr %36, i64 %37
  %39 = getelementptr inbounds %struct.EpMapEntry, ptr %38, i32 0, i32 0
  %40 = load ptr, ptr %39, align 8
  %41 = icmp ne ptr %40, null
  br i1 %41, label %42, label %50

42:                                               ; preds = %33
  %43 = load ptr, ptr %4, align 8
  %44 = getelementptr inbounds %struct.EpMap, ptr %43, i32 0, i32 0
  %45 = load ptr, ptr %44, align 8
  %46 = load i64, ptr %5, align 8
  %47 = getelementptr inbounds %struct.EpMapEntry, ptr %45, i64 %46
  %48 = getelementptr inbounds %struct.EpMapEntry, ptr %47, i32 0, i32 0
  %49 = load ptr, ptr %48, align 8
  call void @free(ptr noundef %49)
  br label %50

50:                                               ; preds = %42, %33, %24
  br label %51

51:                                               ; preds = %50
  %52 = load i64, ptr %5, align 8
  %53 = add nsw i64 %52, 1
  store i64 %53, ptr %5, align 8
  br label %18, !llvm.loop !21

54:                                               ; preds = %18
  %55 = load ptr, ptr %4, align 8
  %56 = getelementptr inbounds %struct.EpMap, ptr %55, i32 0, i32 0
  %57 = load ptr, ptr %56, align 8
  call void @free(ptr noundef %57)
  %58 = load ptr, ptr %4, align 8
  call void @free(ptr noundef %58)
  store i64 0, ptr %2, align 8
  br label %59

59:                                               ; preds = %54, %15, %10
  %60 = load i64, ptr %2, align 8
  ret i64 %60
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal ptr @ep_gc_find(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  %4 = call ptr @ep_gc_table_get(ptr noundef %3)
  ret ptr %4
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_unregister(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %5 = load ptr, ptr %2, align 8
  %6 = icmp ne ptr %5, null
  br i1 %6, label %8, label %7

7:                                                ; preds = %1
  br label %47

8:                                                ; preds = %1
  %9 = call i32 @pthread_mutex_lock(ptr noundef @ep_gc_mutex)
  %10 = load ptr, ptr %2, align 8
  call void @ep_gc_table_remove(ptr noundef %10)
  store ptr @ep_gc_head, ptr %3, align 8
  br label %11

11:                                               ; preds = %41, %8
  %12 = load ptr, ptr %3, align 8
  %13 = load ptr, ptr %12, align 8
  %14 = icmp ne ptr %13, null
  br i1 %14, label %15, label %45

15:                                               ; preds = %11
  %16 = load ptr, ptr %3, align 8
  %17 = load ptr, ptr %16, align 8
  %18 = getelementptr inbounds %struct.EpGCObject, ptr %17, i32 0, i32 2
  %19 = load ptr, ptr %18, align 8
  %20 = load ptr, ptr %2, align 8
  %21 = icmp eq ptr %19, %20
  br i1 %21, label %22, label %41

22:                                               ; preds = %15
  %23 = load ptr, ptr %3, align 8
  %24 = load ptr, ptr %23, align 8
  store ptr %24, ptr %4, align 8
  %25 = load ptr, ptr %4, align 8
  %26 = getelementptr inbounds %struct.EpGCObject, ptr %25, i32 0, i32 6
  %27 = load ptr, ptr %26, align 8
  %28 = load ptr, ptr %3, align 8
  store ptr %27, ptr %28, align 8
  %29 = load ptr, ptr %4, align 8
  %30 = getelementptr inbounds %struct.EpGCObject, ptr %29, i32 0, i32 5
  %31 = load i32, ptr %30, align 8
  %32 = icmp eq i32 %31, 0
  br i1 %32, label %33, label %36

33:                                               ; preds = %22
  %34 = load i64, ptr @ep_gc_nursery_count, align 8
  %35 = add nsw i64 %34, -1
  store i64 %35, ptr @ep_gc_nursery_count, align 8
  br label %36

36:                                               ; preds = %33, %22
  %37 = load ptr, ptr %4, align 8
  call void @free(ptr noundef %37)
  %38 = load i64, ptr @ep_gc_count, align 8
  %39 = add nsw i64 %38, -1
  store i64 %39, ptr @ep_gc_count, align 8
  %40 = call i32 @pthread_mutex_unlock(ptr noundef @ep_gc_mutex)
  br label %47

41:                                               ; preds = %15
  %42 = load ptr, ptr %3, align 8
  %43 = load ptr, ptr %42, align 8
  %44 = getelementptr inbounds %struct.EpGCObject, ptr %43, i32 0, i32 6
  store ptr %44, ptr %3, align 8
  br label %11, !llvm.loop !22

45:                                               ; preds = %11
  %46 = call i32 @pthread_mutex_unlock(ptr noundef @ep_gc_mutex)
  br label %47

47:                                               ; preds = %45, %36, %7
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @create_deque() #0 {
  %1 = alloca i64, align 8
  %2 = alloca ptr, align 8
  %3 = call ptr @malloc(i64 noundef 40) #14
  store ptr %3, ptr %2, align 8
  %4 = load ptr, ptr %2, align 8
  %5 = icmp ne ptr %4, null
  br i1 %5, label %7, label %6

6:                                                ; preds = %0
  store i64 0, ptr %1, align 8
  br label %32

7:                                                ; preds = %0
  %8 = load ptr, ptr %2, align 8
  %9 = getelementptr inbounds %struct.EpDeque, ptr %8, i32 0, i32 1
  store i64 16, ptr %9, align 8
  %10 = load ptr, ptr %2, align 8
  %11 = getelementptr inbounds %struct.EpDeque, ptr %10, i32 0, i32 4
  store i64 0, ptr %11, align 8
  %12 = load ptr, ptr %2, align 8
  %13 = getelementptr inbounds %struct.EpDeque, ptr %12, i32 0, i32 2
  store i64 0, ptr %13, align 8
  %14 = load ptr, ptr %2, align 8
  %15 = getelementptr inbounds %struct.EpDeque, ptr %14, i32 0, i32 3
  store i64 0, ptr %15, align 8
  %16 = load ptr, ptr %2, align 8
  %17 = getelementptr inbounds %struct.EpDeque, ptr %16, i32 0, i32 1
  %18 = load i64, ptr %17, align 8
  %19 = mul i64 %18, 8
  %20 = call ptr @malloc(i64 noundef %19) #14
  %21 = load ptr, ptr %2, align 8
  %22 = getelementptr inbounds %struct.EpDeque, ptr %21, i32 0, i32 0
  store ptr %20, ptr %22, align 8
  %23 = load ptr, ptr %2, align 8
  %24 = getelementptr inbounds %struct.EpDeque, ptr %23, i32 0, i32 0
  %25 = load ptr, ptr %24, align 8
  %26 = icmp ne ptr %25, null
  br i1 %26, label %29, label %27

27:                                               ; preds = %7
  %28 = load ptr, ptr %2, align 8
  call void @free(ptr noundef %28)
  store i64 0, ptr %1, align 8
  br label %32

29:                                               ; preds = %7
  %30 = load ptr, ptr %2, align 8
  %31 = ptrtoint ptr %30 to i64
  store i64 %31, ptr %1, align 8
  br label %32

32:                                               ; preds = %29, %27, %6
  %33 = load i64, ptr %1, align 8
  ret i64 %33
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @deque_push_back(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %6, align 8
  %9 = load ptr, ptr %6, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %12, label %11

11:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %50

12:                                               ; preds = %2
  %13 = load ptr, ptr %6, align 8
  %14 = getelementptr inbounds %struct.EpDeque, ptr %13, i32 0, i32 4
  %15 = load i64, ptr %14, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = getelementptr inbounds %struct.EpDeque, ptr %16, i32 0, i32 1
  %18 = load i64, ptr %17, align 8
  %19 = icmp sge i64 %15, %18
  br i1 %19, label %20, label %26

20:                                               ; preds = %12
  %21 = load ptr, ptr %6, align 8
  %22 = load ptr, ptr %6, align 8
  %23 = getelementptr inbounds %struct.EpDeque, ptr %22, i32 0, i32 1
  %24 = load i64, ptr %23, align 8
  %25 = mul nsw i64 %24, 2
  call void @deque_resize(ptr noundef %21, i64 noundef %25)
  br label %26

26:                                               ; preds = %20, %12
  %27 = load i64, ptr %5, align 8
  %28 = load ptr, ptr %6, align 8
  %29 = getelementptr inbounds %struct.EpDeque, ptr %28, i32 0, i32 0
  %30 = load ptr, ptr %29, align 8
  %31 = load ptr, ptr %6, align 8
  %32 = getelementptr inbounds %struct.EpDeque, ptr %31, i32 0, i32 3
  %33 = load i64, ptr %32, align 8
  %34 = getelementptr inbounds i64, ptr %30, i64 %33
  store i64 %27, ptr %34, align 8
  %35 = load ptr, ptr %6, align 8
  %36 = getelementptr inbounds %struct.EpDeque, ptr %35, i32 0, i32 3
  %37 = load i64, ptr %36, align 8
  %38 = add nsw i64 %37, 1
  %39 = load ptr, ptr %6, align 8
  %40 = getelementptr inbounds %struct.EpDeque, ptr %39, i32 0, i32 1
  %41 = load i64, ptr %40, align 8
  %42 = srem i64 %38, %41
  %43 = load ptr, ptr %6, align 8
  %44 = getelementptr inbounds %struct.EpDeque, ptr %43, i32 0, i32 3
  store i64 %42, ptr %44, align 8
  %45 = load ptr, ptr %6, align 8
  %46 = getelementptr inbounds %struct.EpDeque, ptr %45, i32 0, i32 4
  %47 = load i64, ptr %46, align 8
  %48 = add nsw i64 %47, 1
  store i64 %48, ptr %46, align 8
  %49 = load i64, ptr %5, align 8
  store i64 %49, ptr %3, align 8
  br label %50

50:                                               ; preds = %26, %11
  %51 = load i64, ptr %3, align 8
  ret i64 %51
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @deque_resize(ptr noundef %0, i64 noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca i64, align 8
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  store ptr %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %7 = load i64, ptr %4, align 8
  %8 = mul i64 %7, 8
  %9 = call ptr @malloc(i64 noundef %8) #14
  store ptr %9, ptr %5, align 8
  store i64 0, ptr %6, align 8
  br label %10

10:                                               ; preds = %34, %2
  %11 = load i64, ptr %6, align 8
  %12 = load ptr, ptr %3, align 8
  %13 = getelementptr inbounds %struct.EpDeque, ptr %12, i32 0, i32 4
  %14 = load i64, ptr %13, align 8
  %15 = icmp slt i64 %11, %14
  br i1 %15, label %16, label %37

16:                                               ; preds = %10
  %17 = load ptr, ptr %3, align 8
  %18 = getelementptr inbounds %struct.EpDeque, ptr %17, i32 0, i32 0
  %19 = load ptr, ptr %18, align 8
  %20 = load ptr, ptr %3, align 8
  %21 = getelementptr inbounds %struct.EpDeque, ptr %20, i32 0, i32 2
  %22 = load i64, ptr %21, align 8
  %23 = load i64, ptr %6, align 8
  %24 = add nsw i64 %22, %23
  %25 = load ptr, ptr %3, align 8
  %26 = getelementptr inbounds %struct.EpDeque, ptr %25, i32 0, i32 1
  %27 = load i64, ptr %26, align 8
  %28 = srem i64 %24, %27
  %29 = getelementptr inbounds i64, ptr %19, i64 %28
  %30 = load i64, ptr %29, align 8
  %31 = load ptr, ptr %5, align 8
  %32 = load i64, ptr %6, align 8
  %33 = getelementptr inbounds i64, ptr %31, i64 %32
  store i64 %30, ptr %33, align 8
  br label %34

34:                                               ; preds = %16
  %35 = load i64, ptr %6, align 8
  %36 = add nsw i64 %35, 1
  store i64 %36, ptr %6, align 8
  br label %10, !llvm.loop !23

37:                                               ; preds = %10
  %38 = load ptr, ptr %3, align 8
  %39 = getelementptr inbounds %struct.EpDeque, ptr %38, i32 0, i32 0
  %40 = load ptr, ptr %39, align 8
  call void @free(ptr noundef %40)
  %41 = load ptr, ptr %5, align 8
  %42 = load ptr, ptr %3, align 8
  %43 = getelementptr inbounds %struct.EpDeque, ptr %42, i32 0, i32 0
  store ptr %41, ptr %43, align 8
  %44 = load i64, ptr %4, align 8
  %45 = load ptr, ptr %3, align 8
  %46 = getelementptr inbounds %struct.EpDeque, ptr %45, i32 0, i32 1
  store i64 %44, ptr %46, align 8
  %47 = load ptr, ptr %3, align 8
  %48 = getelementptr inbounds %struct.EpDeque, ptr %47, i32 0, i32 2
  store i64 0, ptr %48, align 8
  %49 = load ptr, ptr %3, align 8
  %50 = getelementptr inbounds %struct.EpDeque, ptr %49, i32 0, i32 4
  %51 = load i64, ptr %50, align 8
  %52 = load ptr, ptr %3, align 8
  %53 = getelementptr inbounds %struct.EpDeque, ptr %52, i32 0, i32 3
  store i64 %51, ptr %53, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @deque_push_front(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %6, align 8
  %9 = load ptr, ptr %6, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %12, label %11

11:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %54

12:                                               ; preds = %2
  %13 = load ptr, ptr %6, align 8
  %14 = getelementptr inbounds %struct.EpDeque, ptr %13, i32 0, i32 4
  %15 = load i64, ptr %14, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = getelementptr inbounds %struct.EpDeque, ptr %16, i32 0, i32 1
  %18 = load i64, ptr %17, align 8
  %19 = icmp sge i64 %15, %18
  br i1 %19, label %20, label %26

20:                                               ; preds = %12
  %21 = load ptr, ptr %6, align 8
  %22 = load ptr, ptr %6, align 8
  %23 = getelementptr inbounds %struct.EpDeque, ptr %22, i32 0, i32 1
  %24 = load i64, ptr %23, align 8
  %25 = mul nsw i64 %24, 2
  call void @deque_resize(ptr noundef %21, i64 noundef %25)
  br label %26

26:                                               ; preds = %20, %12
  %27 = load ptr, ptr %6, align 8
  %28 = getelementptr inbounds %struct.EpDeque, ptr %27, i32 0, i32 2
  %29 = load i64, ptr %28, align 8
  %30 = sub nsw i64 %29, 1
  %31 = load ptr, ptr %6, align 8
  %32 = getelementptr inbounds %struct.EpDeque, ptr %31, i32 0, i32 1
  %33 = load i64, ptr %32, align 8
  %34 = add nsw i64 %30, %33
  %35 = load ptr, ptr %6, align 8
  %36 = getelementptr inbounds %struct.EpDeque, ptr %35, i32 0, i32 1
  %37 = load i64, ptr %36, align 8
  %38 = srem i64 %34, %37
  %39 = load ptr, ptr %6, align 8
  %40 = getelementptr inbounds %struct.EpDeque, ptr %39, i32 0, i32 2
  store i64 %38, ptr %40, align 8
  %41 = load i64, ptr %5, align 8
  %42 = load ptr, ptr %6, align 8
  %43 = getelementptr inbounds %struct.EpDeque, ptr %42, i32 0, i32 0
  %44 = load ptr, ptr %43, align 8
  %45 = load ptr, ptr %6, align 8
  %46 = getelementptr inbounds %struct.EpDeque, ptr %45, i32 0, i32 2
  %47 = load i64, ptr %46, align 8
  %48 = getelementptr inbounds i64, ptr %44, i64 %47
  store i64 %41, ptr %48, align 8
  %49 = load ptr, ptr %6, align 8
  %50 = getelementptr inbounds %struct.EpDeque, ptr %49, i32 0, i32 4
  %51 = load i64, ptr %50, align 8
  %52 = add nsw i64 %51, 1
  store i64 %52, ptr %50, align 8
  %53 = load i64, ptr %5, align 8
  store i64 %53, ptr %3, align 8
  br label %54

54:                                               ; preds = %26, %11
  %55 = load i64, ptr %3, align 8
  ret i64 %55
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @deque_pop_back(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %10, label %15

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = getelementptr inbounds %struct.EpDeque, ptr %11, i32 0, i32 4
  %13 = load i64, ptr %12, align 8
  %14 = icmp eq i64 %13, 0
  br i1 %14, label %15, label %16

15:                                               ; preds = %10, %1
  store i64 0, ptr %2, align 8
  br label %44

16:                                               ; preds = %10
  %17 = load ptr, ptr %4, align 8
  %18 = getelementptr inbounds %struct.EpDeque, ptr %17, i32 0, i32 3
  %19 = load i64, ptr %18, align 8
  %20 = sub nsw i64 %19, 1
  %21 = load ptr, ptr %4, align 8
  %22 = getelementptr inbounds %struct.EpDeque, ptr %21, i32 0, i32 1
  %23 = load i64, ptr %22, align 8
  %24 = add nsw i64 %20, %23
  %25 = load ptr, ptr %4, align 8
  %26 = getelementptr inbounds %struct.EpDeque, ptr %25, i32 0, i32 1
  %27 = load i64, ptr %26, align 8
  %28 = srem i64 %24, %27
  %29 = load ptr, ptr %4, align 8
  %30 = getelementptr inbounds %struct.EpDeque, ptr %29, i32 0, i32 3
  store i64 %28, ptr %30, align 8
  %31 = load ptr, ptr %4, align 8
  %32 = getelementptr inbounds %struct.EpDeque, ptr %31, i32 0, i32 0
  %33 = load ptr, ptr %32, align 8
  %34 = load ptr, ptr %4, align 8
  %35 = getelementptr inbounds %struct.EpDeque, ptr %34, i32 0, i32 3
  %36 = load i64, ptr %35, align 8
  %37 = getelementptr inbounds i64, ptr %33, i64 %36
  %38 = load i64, ptr %37, align 8
  store i64 %38, ptr %5, align 8
  %39 = load ptr, ptr %4, align 8
  %40 = getelementptr inbounds %struct.EpDeque, ptr %39, i32 0, i32 4
  %41 = load i64, ptr %40, align 8
  %42 = add nsw i64 %41, -1
  store i64 %42, ptr %40, align 8
  %43 = load i64, ptr %5, align 8
  store i64 %43, ptr %2, align 8
  br label %44

44:                                               ; preds = %16, %15
  %45 = load i64, ptr %2, align 8
  ret i64 %45
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @deque_pop_front(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %10, label %15

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = getelementptr inbounds %struct.EpDeque, ptr %11, i32 0, i32 4
  %13 = load i64, ptr %12, align 8
  %14 = icmp eq i64 %13, 0
  br i1 %14, label %15, label %16

15:                                               ; preds = %10, %1
  store i64 0, ptr %2, align 8
  br label %40

16:                                               ; preds = %10
  %17 = load ptr, ptr %4, align 8
  %18 = getelementptr inbounds %struct.EpDeque, ptr %17, i32 0, i32 0
  %19 = load ptr, ptr %18, align 8
  %20 = load ptr, ptr %4, align 8
  %21 = getelementptr inbounds %struct.EpDeque, ptr %20, i32 0, i32 2
  %22 = load i64, ptr %21, align 8
  %23 = getelementptr inbounds i64, ptr %19, i64 %22
  %24 = load i64, ptr %23, align 8
  store i64 %24, ptr %5, align 8
  %25 = load ptr, ptr %4, align 8
  %26 = getelementptr inbounds %struct.EpDeque, ptr %25, i32 0, i32 2
  %27 = load i64, ptr %26, align 8
  %28 = add nsw i64 %27, 1
  %29 = load ptr, ptr %4, align 8
  %30 = getelementptr inbounds %struct.EpDeque, ptr %29, i32 0, i32 1
  %31 = load i64, ptr %30, align 8
  %32 = srem i64 %28, %31
  %33 = load ptr, ptr %4, align 8
  %34 = getelementptr inbounds %struct.EpDeque, ptr %33, i32 0, i32 2
  store i64 %32, ptr %34, align 8
  %35 = load ptr, ptr %4, align 8
  %36 = getelementptr inbounds %struct.EpDeque, ptr %35, i32 0, i32 4
  %37 = load i64, ptr %36, align 8
  %38 = add nsw i64 %37, -1
  store i64 %38, ptr %36, align 8
  %39 = load i64, ptr %5, align 8
  store i64 %39, ptr %2, align 8
  br label %40

40:                                               ; preds = %16, %15
  %41 = load i64, ptr %2, align 8
  ret i64 %41
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @deque_length(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %14

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = getelementptr inbounds %struct.EpDeque, ptr %11, i32 0, i32 4
  %13 = load i64, ptr %12, align 8
  store i64 %13, ptr %2, align 8
  br label %14

14:                                               ; preds = %10, %9
  %15 = load i64, ptr %2, align 8
  ret i64 %15
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @free_deque(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %15

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = getelementptr inbounds %struct.EpDeque, ptr %11, i32 0, i32 0
  %13 = load ptr, ptr %12, align 8
  call void @free(ptr noundef %13)
  %14 = load ptr, ptr %4, align 8
  call void @free(ptr noundef %14)
  store i64 0, ptr %2, align 8
  br label %15

15:                                               ; preds = %10, %9
  %16 = load i64, ptr %2, align 8
  ret i64 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @fs_scan_dir(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %9 = load i64, ptr %3, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %4, align 8
  %11 = call i64 @create_list()
  store i64 %11, ptr %5, align 8
  %12 = load ptr, ptr %4, align 8
  %13 = icmp ne ptr %12, null
  br i1 %13, label %16, label %14

14:                                               ; preds = %1
  %15 = load i64, ptr %5, align 8
  store i64 %15, ptr %2, align 8
  br label %54

16:                                               ; preds = %1
  %17 = load ptr, ptr %4, align 8
  %18 = call ptr @"\01_opendir"(ptr noundef %17)
  store ptr %18, ptr %6, align 8
  %19 = load ptr, ptr %6, align 8
  %20 = icmp ne ptr %19, null
  br i1 %20, label %23, label %21

21:                                               ; preds = %16
  %22 = load i64, ptr %5, align 8
  store i64 %22, ptr %2, align 8
  br label %54

23:                                               ; preds = %16
  br label %24

24:                                               ; preds = %41, %40, %23
  %25 = load ptr, ptr %6, align 8
  %26 = call ptr @"\01_readdir"(ptr noundef %25)
  store ptr %26, ptr %7, align 8
  %27 = icmp ne ptr %26, null
  br i1 %27, label %28, label %50

28:                                               ; preds = %24
  %29 = load ptr, ptr %7, align 8
  %30 = getelementptr inbounds %struct.dirent, ptr %29, i32 0, i32 5
  %31 = getelementptr inbounds [1024 x i8], ptr %30, i64 0, i64 0
  %32 = call i32 @strcmp(ptr noundef %31, ptr noundef @.str.1) #13
  %33 = icmp eq i32 %32, 0
  br i1 %33, label %40, label %34

34:                                               ; preds = %28
  %35 = load ptr, ptr %7, align 8
  %36 = getelementptr inbounds %struct.dirent, ptr %35, i32 0, i32 5
  %37 = getelementptr inbounds [1024 x i8], ptr %36, i64 0, i64 0
  %38 = call i32 @strcmp(ptr noundef %37, ptr noundef @.str.2) #13
  %39 = icmp eq i32 %38, 0
  br i1 %39, label %40, label %41

40:                                               ; preds = %34, %28
  br label %24, !llvm.loop !24

41:                                               ; preds = %34
  %42 = load ptr, ptr %7, align 8
  %43 = getelementptr inbounds %struct.dirent, ptr %42, i32 0, i32 5
  %44 = getelementptr inbounds [1024 x i8], ptr %43, i64 0, i64 0
  %45 = call ptr @strdup(ptr noundef %44) #13
  store ptr %45, ptr %8, align 8
  %46 = load i64, ptr %5, align 8
  %47 = load ptr, ptr %8, align 8
  %48 = ptrtoint ptr %47 to i64
  %49 = call i64 @append_list(i64 noundef %46, i64 noundef %48)
  br label %24, !llvm.loop !24

50:                                               ; preds = %24
  %51 = load ptr, ptr %6, align 8
  %52 = call i32 @"\01_closedir"(ptr noundef %51)
  %53 = load i64, ptr %5, align 8
  store i64 %53, ptr %2, align 8
  br label %54

54:                                               ; preds = %50, %21, %14
  %55 = load i64, ptr %2, align 8
  ret i64 %55
}

declare ptr @"\01_opendir"(ptr noundef) #1

declare ptr @"\01_readdir"(ptr noundef) #1

declare i32 @"\01_closedir"(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @fs_copy_file(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  %9 = alloca ptr, align 8
  %10 = alloca [4096 x i8], align 1
  %11 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %12 = load i64, ptr %4, align 8
  %13 = inttoptr i64 %12 to ptr
  store ptr %13, ptr %6, align 8
  %14 = load i64, ptr %5, align 8
  %15 = inttoptr i64 %14 to ptr
  store ptr %15, ptr %7, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %18, label %21

18:                                               ; preds = %2
  %19 = load ptr, ptr %7, align 8
  %20 = icmp ne ptr %19, null
  br i1 %20, label %22, label %21

21:                                               ; preds = %18, %2
  store i64 0, ptr %3, align 8
  br label %52

22:                                               ; preds = %18
  %23 = load ptr, ptr %6, align 8
  %24 = call ptr @"\01_fopen"(ptr noundef %23, ptr noundef @.str.3)
  store ptr %24, ptr %8, align 8
  %25 = load ptr, ptr %8, align 8
  %26 = icmp ne ptr %25, null
  br i1 %26, label %28, label %27

27:                                               ; preds = %22
  store i64 0, ptr %3, align 8
  br label %52

28:                                               ; preds = %22
  %29 = load ptr, ptr %7, align 8
  %30 = call ptr @"\01_fopen"(ptr noundef %29, ptr noundef @.str.4)
  store ptr %30, ptr %9, align 8
  %31 = load ptr, ptr %9, align 8
  %32 = icmp ne ptr %31, null
  br i1 %32, label %36, label %33

33:                                               ; preds = %28
  %34 = load ptr, ptr %8, align 8
  %35 = call i32 @fclose(ptr noundef %34)
  store i64 0, ptr %3, align 8
  br label %52

36:                                               ; preds = %28
  br label %37

37:                                               ; preds = %42, %36
  %38 = getelementptr inbounds [4096 x i8], ptr %10, i64 0, i64 0
  %39 = load ptr, ptr %8, align 8
  %40 = call i64 @fread(ptr noundef %38, i64 noundef 1, i64 noundef 4096, ptr noundef %39)
  store i64 %40, ptr %11, align 8
  %41 = icmp ugt i64 %40, 0
  br i1 %41, label %42, label %47

42:                                               ; preds = %37
  %43 = getelementptr inbounds [4096 x i8], ptr %10, i64 0, i64 0
  %44 = load i64, ptr %11, align 8
  %45 = load ptr, ptr %9, align 8
  %46 = call i64 @"\01_fwrite"(ptr noundef %43, i64 noundef 1, i64 noundef %44, ptr noundef %45)
  br label %37, !llvm.loop !25

47:                                               ; preds = %37
  %48 = load ptr, ptr %8, align 8
  %49 = call i32 @fclose(ptr noundef %48)
  %50 = load ptr, ptr %9, align 8
  %51 = call i32 @fclose(ptr noundef %50)
  store i64 1, ptr %3, align 8
  br label %52

52:                                               ; preds = %47, %33, %27, %21
  %53 = load i64, ptr %3, align 8
  ret i64 %53
}

declare ptr @"\01_fopen"(ptr noundef, ptr noundef) #1

declare i32 @fclose(ptr noundef) #1

declare i64 @fread(ptr noundef, i64 noundef, i64 noundef, ptr noundef) #1

declare i64 @"\01_fwrite"(ptr noundef, i64 noundef, i64 noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @fs_delete_file(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %17

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = call i32 @remove(ptr noundef %11)
  %13 = icmp eq i32 %12, 0
  %14 = zext i1 %13 to i64
  %15 = select i1 %13, i32 1, i32 0
  %16 = sext i32 %15 to i64
  store i64 %16, ptr %2, align 8
  br label %17

17:                                               ; preds = %10, %9
  %18 = load i64, ptr %2, align 8
  ret i64 %18
}

declare i32 @remove(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @fs_move_file(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %8 = load i64, ptr %4, align 8
  %9 = inttoptr i64 %8 to ptr
  store ptr %9, ptr %6, align 8
  %10 = load i64, ptr %5, align 8
  %11 = inttoptr i64 %10 to ptr
  store ptr %11, ptr %7, align 8
  %12 = load ptr, ptr %6, align 8
  %13 = icmp ne ptr %12, null
  br i1 %13, label %14, label %17

14:                                               ; preds = %2
  %15 = load ptr, ptr %7, align 8
  %16 = icmp ne ptr %15, null
  br i1 %16, label %18, label %17

17:                                               ; preds = %14, %2
  store i64 0, ptr %3, align 8
  br label %26

18:                                               ; preds = %14
  %19 = load ptr, ptr %6, align 8
  %20 = load ptr, ptr %7, align 8
  %21 = call i32 @rename(ptr noundef %19, ptr noundef %20)
  %22 = icmp eq i32 %21, 0
  %23 = zext i1 %22 to i64
  %24 = select i1 %22, i32 1, i32 0
  %25 = sext i32 %24 to i64
  store i64 %25, ptr %3, align 8
  br label %26

26:                                               ; preds = %18, %17
  %27 = load i64, ptr %3, align 8
  ret i64 %27
}

declare i32 @rename(ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @fs_exists(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca %struct.stat, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %11, label %10

10:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %18

11:                                               ; preds = %1
  %12 = load ptr, ptr %4, align 8
  %13 = call i32 @"\01_stat"(ptr noundef %12, ptr noundef %5)
  %14 = icmp eq i32 %13, 0
  %15 = zext i1 %14 to i64
  %16 = select i1 %14, i32 1, i32 0
  %17 = sext i32 %16 to i64
  store i64 %17, ptr %2, align 8
  br label %18

18:                                               ; preds = %11, %10
  %19 = load i64, ptr %2, align 8
  ret i64 %19
}

declare i32 @"\01_stat"(ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @fs_is_dir(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca %struct.stat, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %11, label %10

10:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %25

11:                                               ; preds = %1
  %12 = load ptr, ptr %4, align 8
  %13 = call i32 @"\01_stat"(ptr noundef %12, ptr noundef %5)
  %14 = icmp ne i32 %13, 0
  br i1 %14, label %15, label %16

15:                                               ; preds = %11
  store i64 0, ptr %2, align 8
  br label %25

16:                                               ; preds = %11
  %17 = getelementptr inbounds %struct.stat, ptr %5, i32 0, i32 1
  %18 = load i16, ptr %17, align 4
  %19 = zext i16 %18 to i32
  %20 = and i32 %19, 61440
  %21 = icmp eq i32 %20, 16384
  %22 = zext i1 %21 to i64
  %23 = select i1 %21, i32 1, i32 0
  %24 = sext i32 %23 to i64
  store i64 %24, ptr %2, align 8
  br label %25

25:                                               ; preds = %16, %15, %10
  %26 = load i64, ptr %2, align 8
  ret i64 %26
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @fs_is_file(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca %struct.stat, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %11, label %10

10:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %25

11:                                               ; preds = %1
  %12 = load ptr, ptr %4, align 8
  %13 = call i32 @"\01_stat"(ptr noundef %12, ptr noundef %5)
  %14 = icmp ne i32 %13, 0
  br i1 %14, label %15, label %16

15:                                               ; preds = %11
  store i64 0, ptr %2, align 8
  br label %25

16:                                               ; preds = %11
  %17 = getelementptr inbounds %struct.stat, ptr %5, i32 0, i32 1
  %18 = load i16, ptr %17, align 4
  %19 = zext i16 %18 to i32
  %20 = and i32 %19, 61440
  %21 = icmp eq i32 %20, 32768
  %22 = zext i1 %21 to i64
  %23 = select i1 %21, i32 1, i32 0
  %24 = sext i32 %23 to i64
  store i64 %24, ptr %2, align 8
  br label %25

25:                                               ; preds = %16, %15, %10
  %26 = load i64, ptr %2, align 8
  ret i64 %26
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @fs_get_size(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca %struct.stat, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %11, label %10

10:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %19

11:                                               ; preds = %1
  %12 = load ptr, ptr %4, align 8
  %13 = call i32 @"\01_stat"(ptr noundef %12, ptr noundef %5)
  %14 = icmp ne i32 %13, 0
  br i1 %14, label %15, label %16

15:                                               ; preds = %11
  store i64 0, ptr %2, align 8
  br label %19

16:                                               ; preds = %11
  %17 = getelementptr inbounds %struct.stat, ptr %5, i32 0, i32 11
  %18 = load i64, ptr %17, align 8
  store i64 %18, ptr %2, align 8
  br label %19

19:                                               ; preds = %16, %15, %10
  %20 = load i64, ptr %2, align 8
  ret i64 %20
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_http_request(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3) #0 {
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca ptr, align 8
  %11 = alloca ptr, align 8
  %12 = alloca ptr, align 8
  %13 = alloca ptr, align 8
  %14 = alloca ptr, align 8
  %15 = alloca ptr, align 8
  %16 = alloca [256 x i8], align 1
  %17 = alloca [1024 x i8], align 1
  %18 = alloca i64, align 8
  %19 = alloca i32, align 4
  %20 = alloca ptr, align 8
  %21 = alloca i32, align 4
  %22 = alloca ptr, align 8
  %23 = alloca %struct.sockaddr_in, align 4
  %24 = alloca [4096 x i8], align 1
  %25 = alloca i64, align 8
  %26 = alloca i32, align 4
  %27 = alloca i64, align 8
  %28 = alloca i64, align 8
  %29 = alloca ptr, align 8
  %30 = alloca [4096 x i8], align 1
  %31 = alloca i64, align 8
  %32 = alloca ptr, align 8
  %33 = alloca ptr, align 8
  %34 = alloca ptr, align 8
  store i64 %0, ptr %6, align 8
  store i64 %1, ptr %7, align 8
  store i64 %2, ptr %8, align 8
  store i64 %3, ptr %9, align 8
  %35 = load i64, ptr %6, align 8
  %36 = inttoptr i64 %35 to ptr
  store ptr %36, ptr %10, align 8
  %37 = load i64, ptr %7, align 8
  %38 = inttoptr i64 %37 to ptr
  store ptr %38, ptr %11, align 8
  %39 = load i64, ptr %8, align 8
  %40 = inttoptr i64 %39 to ptr
  store ptr %40, ptr %12, align 8
  %41 = load i64, ptr %9, align 8
  %42 = inttoptr i64 %41 to ptr
  store ptr %42, ptr %13, align 8
  %43 = load ptr, ptr %10, align 8
  %44 = icmp ne ptr %43, null
  br i1 %44, label %45, label %48

45:                                               ; preds = %4
  %46 = load ptr, ptr %11, align 8
  %47 = icmp ne ptr %46, null
  br i1 %47, label %51, label %48

48:                                               ; preds = %45, %4
  %49 = call ptr @strdup(ptr noundef @.str.5) #13
  %50 = ptrtoint ptr %49 to i64
  store i64 %50, ptr %5, align 8
  br label %306

51:                                               ; preds = %45
  %52 = load ptr, ptr %11, align 8
  %53 = call i32 @strncmp(ptr noundef %52, ptr noundef @.str.6, i64 noundef 7) #13
  %54 = icmp ne i32 %53, 0
  br i1 %54, label %55, label %58

55:                                               ; preds = %51
  %56 = call ptr @strdup(ptr noundef @.str.7) #13
  %57 = ptrtoint ptr %56 to i64
  store i64 %57, ptr %5, align 8
  br label %306

58:                                               ; preds = %51
  %59 = load ptr, ptr %11, align 8
  %60 = getelementptr inbounds i8, ptr %59, i64 7
  store ptr %60, ptr %14, align 8
  %61 = load ptr, ptr %14, align 8
  %62 = call ptr @strchr(ptr noundef %61, i32 noundef 47) #13
  store ptr %62, ptr %15, align 8
  %63 = load ptr, ptr %15, align 8
  %64 = icmp ne ptr %63, null
  br i1 %64, label %65, label %85

65:                                               ; preds = %58
  %66 = load ptr, ptr %15, align 8
  %67 = load ptr, ptr %14, align 8
  %68 = ptrtoint ptr %66 to i64
  %69 = ptrtoint ptr %67 to i64
  %70 = sub i64 %68, %69
  store i64 %70, ptr %18, align 8
  %71 = load i64, ptr %18, align 8
  %72 = icmp uge i64 %71, 256
  br i1 %72, label %73, label %74

73:                                               ; preds = %65
  store i64 255, ptr %18, align 8
  br label %74

74:                                               ; preds = %73, %65
  %75 = getelementptr inbounds [256 x i8], ptr %16, i64 0, i64 0
  %76 = load ptr, ptr %14, align 8
  %77 = load i64, ptr %18, align 8
  %78 = call ptr @__strncpy_chk(ptr noundef %75, ptr noundef %76, i64 noundef %77, i64 noundef 256) #13
  %79 = load i64, ptr %18, align 8
  %80 = getelementptr inbounds [256 x i8], ptr %16, i64 0, i64 %79
  store i8 0, ptr %80, align 1
  %81 = getelementptr inbounds [1024 x i8], ptr %17, i64 0, i64 0
  %82 = load ptr, ptr %15, align 8
  %83 = call ptr @__strncpy_chk(ptr noundef %81, ptr noundef %82, i64 noundef 1023, i64 noundef 1024) #13
  %84 = getelementptr inbounds [1024 x i8], ptr %17, i64 0, i64 1023
  store i8 0, ptr %84, align 1
  br label %92

85:                                               ; preds = %58
  %86 = getelementptr inbounds [256 x i8], ptr %16, i64 0, i64 0
  %87 = load ptr, ptr %14, align 8
  %88 = call ptr @__strncpy_chk(ptr noundef %86, ptr noundef %87, i64 noundef 255, i64 noundef 256) #13
  %89 = getelementptr inbounds [256 x i8], ptr %16, i64 0, i64 255
  store i8 0, ptr %89, align 1
  %90 = getelementptr inbounds [1024 x i8], ptr %17, i64 0, i64 0
  %91 = call ptr @__strcpy_chk(ptr noundef %90, ptr noundef @.str.8, i64 noundef 1024) #13
  br label %92

92:                                               ; preds = %85, %74
  store i32 80, ptr %19, align 4
  %93 = getelementptr inbounds [256 x i8], ptr %16, i64 0, i64 0
  %94 = call ptr @strchr(ptr noundef %93, i32 noundef 58) #13
  store ptr %94, ptr %20, align 8
  %95 = load ptr, ptr %20, align 8
  %96 = icmp ne ptr %95, null
  br i1 %96, label %97, label %102

97:                                               ; preds = %92
  %98 = load ptr, ptr %20, align 8
  store i8 0, ptr %98, align 1
  %99 = load ptr, ptr %20, align 8
  %100 = getelementptr inbounds i8, ptr %99, i64 1
  %101 = call i32 @atoi(ptr noundef %100)
  store i32 %101, ptr %19, align 4
  br label %102

102:                                              ; preds = %97, %92
  %103 = call i32 @socket(i32 noundef 2, i32 noundef 1, i32 noundef 0)
  store i32 %103, ptr %21, align 4
  %104 = load i32, ptr %21, align 4
  %105 = icmp slt i32 %104, 0
  br i1 %105, label %106, label %109

106:                                              ; preds = %102
  %107 = call ptr @strdup(ptr noundef @.str.9) #13
  %108 = ptrtoint ptr %107 to i64
  store i64 %108, ptr %5, align 8
  br label %306

109:                                              ; preds = %102
  %110 = getelementptr inbounds [256 x i8], ptr %16, i64 0, i64 0
  %111 = call ptr @gethostbyname(ptr noundef %110)
  store ptr %111, ptr %22, align 8
  %112 = load ptr, ptr %22, align 8
  %113 = icmp ne ptr %112, null
  br i1 %113, label %119, label %114

114:                                              ; preds = %109
  %115 = load i32, ptr %21, align 4
  %116 = call i32 @"\01_close"(i32 noundef %115)
  %117 = call ptr @strdup(ptr noundef @.str.10) #13
  %118 = ptrtoint ptr %117 to i64
  store i64 %118, ptr %5, align 8
  br label %306

119:                                              ; preds = %109
  call void @llvm.memset.p0.i64(ptr align 4 %23, i8 0, i64 16, i1 false)
  %120 = getelementptr inbounds %struct.sockaddr_in, ptr %23, i32 0, i32 1
  store i8 2, ptr %120, align 1
  %121 = getelementptr inbounds %struct.sockaddr_in, ptr %23, i32 0, i32 3
  %122 = getelementptr inbounds %struct.in_addr, ptr %121, i32 0, i32 0
  %123 = load ptr, ptr %22, align 8
  %124 = getelementptr inbounds %struct.hostent, ptr %123, i32 0, i32 4
  %125 = load ptr, ptr %124, align 8
  %126 = getelementptr inbounds ptr, ptr %125, i64 0
  %127 = load ptr, ptr %126, align 8
  %128 = load ptr, ptr %22, align 8
  %129 = getelementptr inbounds %struct.hostent, ptr %128, i32 0, i32 3
  %130 = load i32, ptr %129, align 4
  %131 = sext i32 %130 to i64
  %132 = call ptr @__memcpy_chk(ptr noundef %122, ptr noundef %127, i64 noundef %131, i64 noundef 12) #13
  %133 = load i32, ptr %19, align 4
  %134 = call i1 @llvm.is.constant.i32(i32 %133)
  br i1 %134, label %135, label %149

135:                                              ; preds = %119
  %136 = load i32, ptr %19, align 4
  %137 = trunc i32 %136 to i16
  %138 = zext i16 %137 to i32
  %139 = and i32 %138, 65280
  %140 = lshr i32 %139, 8
  %141 = load i32, ptr %19, align 4
  %142 = trunc i32 %141 to i16
  %143 = zext i16 %142 to i32
  %144 = and i32 %143, 255
  %145 = shl i32 %144, 8
  %146 = or i32 %140, %145
  %147 = trunc i32 %146 to i16
  %148 = zext i16 %147 to i32
  br label %154

149:                                              ; preds = %119
  %150 = load i32, ptr %19, align 4
  %151 = trunc i32 %150 to i16
  %152 = call zeroext i16 @_OSSwapInt16(i16 noundef zeroext %151)
  %153 = zext i16 %152 to i32
  br label %154

154:                                              ; preds = %149, %135
  %155 = phi i32 [ %148, %135 ], [ %153, %149 ]
  %156 = trunc i32 %155 to i16
  %157 = getelementptr inbounds %struct.sockaddr_in, ptr %23, i32 0, i32 2
  store i16 %156, ptr %157, align 2
  %158 = load i32, ptr %21, align 4
  %159 = call i32 @"\01_connect"(i32 noundef %158, ptr noundef %23, i32 noundef 16)
  %160 = icmp slt i32 %159, 0
  br i1 %160, label %161, label %166

161:                                              ; preds = %154
  %162 = load i32, ptr %21, align 4
  %163 = call i32 @"\01_close"(i32 noundef %162)
  %164 = call ptr @strdup(ptr noundef @.str.11) #13
  %165 = ptrtoint ptr %164 to i64
  store i64 %165, ptr %5, align 8
  br label %306

166:                                              ; preds = %154
  %167 = load ptr, ptr %13, align 8
  %168 = icmp ne ptr %167, null
  br i1 %168, label %169, label %172

169:                                              ; preds = %166
  %170 = load ptr, ptr %13, align 8
  %171 = call i64 @strlen(ptr noundef %170) #13
  br label %173

172:                                              ; preds = %166
  br label %173

173:                                              ; preds = %172, %169
  %174 = phi i64 [ %171, %169 ], [ 0, %172 ]
  store i64 %174, ptr %25, align 8
  %175 = getelementptr inbounds [4096 x i8], ptr %24, i64 0, i64 0
  %176 = load ptr, ptr %10, align 8
  %177 = getelementptr inbounds [1024 x i8], ptr %17, i64 0, i64 0
  %178 = getelementptr inbounds [256 x i8], ptr %16, i64 0, i64 0
  %179 = load i64, ptr %25, align 8
  %180 = load ptr, ptr %12, align 8
  %181 = icmp ne ptr %180, null
  br i1 %181, label %182, label %184

182:                                              ; preds = %173
  %183 = load ptr, ptr %12, align 8
  br label %185

184:                                              ; preds = %173
  br label %185

185:                                              ; preds = %184, %182
  %186 = phi ptr [ %183, %182 ], [ @.str.5, %184 ]
  %187 = load ptr, ptr %12, align 8
  %188 = icmp ne ptr %187, null
  br i1 %188, label %189, label %202

189:                                              ; preds = %185
  %190 = load ptr, ptr %12, align 8
  %191 = call i64 @strlen(ptr noundef %190) #13
  %192 = icmp ugt i64 %191, 0
  br i1 %192, label %193, label %202

193:                                              ; preds = %189
  %194 = load ptr, ptr %12, align 8
  %195 = load ptr, ptr %12, align 8
  %196 = call i64 @strlen(ptr noundef %195) #13
  %197 = sub i64 %196, 1
  %198 = getelementptr inbounds i8, ptr %194, i64 %197
  %199 = load i8, ptr %198, align 1
  %200 = sext i8 %199 to i32
  %201 = icmp ne i32 %200, 10
  br label %202

202:                                              ; preds = %193, %189, %185
  %203 = phi i1 [ false, %189 ], [ false, %185 ], [ %201, %193 ]
  %204 = zext i1 %203 to i64
  %205 = select i1 %203, ptr @.str.13, ptr @.str.5
  %206 = call i32 (ptr, i64, i32, i64, ptr, ...) @__snprintf_chk(ptr noundef %175, i64 noundef 4096, i32 noundef 0, i64 noundef 4096, ptr noundef @.str.12, ptr noundef %176, ptr noundef %177, ptr noundef %178, i64 noundef %179, ptr noundef %186, ptr noundef %205)
  store i32 %206, ptr %26, align 4
  %207 = load i32, ptr %21, align 4
  %208 = getelementptr inbounds [4096 x i8], ptr %24, i64 0, i64 0
  %209 = load i32, ptr %26, align 4
  %210 = sext i32 %209 to i64
  %211 = call i64 @"\01_send"(i32 noundef %207, ptr noundef %208, i64 noundef %210, i32 noundef 0)
  %212 = icmp slt i64 %211, 0
  br i1 %212, label %213, label %218

213:                                              ; preds = %202
  %214 = load i32, ptr %21, align 4
  %215 = call i32 @"\01_close"(i32 noundef %214)
  %216 = call ptr @strdup(ptr noundef @.str.14) #13
  %217 = ptrtoint ptr %216 to i64
  store i64 %217, ptr %5, align 8
  br label %306

218:                                              ; preds = %202
  %219 = load i64, ptr %25, align 8
  %220 = icmp ugt i64 %219, 0
  br i1 %220, label %221, label %233

221:                                              ; preds = %218
  %222 = load i32, ptr %21, align 4
  %223 = load ptr, ptr %13, align 8
  %224 = load i64, ptr %25, align 8
  %225 = call i64 @"\01_send"(i32 noundef %222, ptr noundef %223, i64 noundef %224, i32 noundef 0)
  %226 = icmp slt i64 %225, 0
  br i1 %226, label %227, label %232

227:                                              ; preds = %221
  %228 = load i32, ptr %21, align 4
  %229 = call i32 @"\01_close"(i32 noundef %228)
  %230 = call ptr @strdup(ptr noundef @.str.15) #13
  %231 = ptrtoint ptr %230 to i64
  store i64 %231, ptr %5, align 8
  br label %306

232:                                              ; preds = %221
  br label %233

233:                                              ; preds = %232, %218
  store i64 4096, ptr %27, align 8
  store i64 0, ptr %28, align 8
  %234 = load i64, ptr %27, align 8
  %235 = call ptr @malloc(i64 noundef %234) #14
  store ptr %235, ptr %29, align 8
  %236 = load ptr, ptr %29, align 8
  %237 = icmp ne ptr %236, null
  br i1 %237, label %243, label %238

238:                                              ; preds = %233
  %239 = load i32, ptr %21, align 4
  %240 = call i32 @"\01_close"(i32 noundef %239)
  %241 = call ptr @strdup(ptr noundef @.str.5) #13
  %242 = ptrtoint ptr %241 to i64
  store i64 %242, ptr %5, align 8
  br label %306

243:                                              ; preds = %233
  br label %244

244:                                              ; preds = %271, %243
  %245 = load i32, ptr %21, align 4
  %246 = getelementptr inbounds [4096 x i8], ptr %30, i64 0, i64 0
  %247 = call i64 @"\01_recv"(i32 noundef %245, ptr noundef %246, i64 noundef 4096, i32 noundef 0)
  store i64 %247, ptr %31, align 8
  %248 = icmp sgt i64 %247, 0
  br i1 %248, label %249, label %285

249:                                              ; preds = %244
  %250 = load i64, ptr %28, align 8
  %251 = load i64, ptr %31, align 8
  %252 = add i64 %250, %251
  %253 = load i64, ptr %27, align 8
  %254 = icmp uge i64 %252, %253
  br i1 %254, label %255, label %271

255:                                              ; preds = %249
  %256 = load i64, ptr %27, align 8
  %257 = mul i64 %256, 2
  store i64 %257, ptr %27, align 8
  %258 = load ptr, ptr %29, align 8
  %259 = load i64, ptr %27, align 8
  %260 = call ptr @realloc(ptr noundef %258, i64 noundef %259) #16
  store ptr %260, ptr %32, align 8
  %261 = load ptr, ptr %32, align 8
  %262 = icmp ne ptr %261, null
  br i1 %262, label %269, label %263

263:                                              ; preds = %255
  %264 = load ptr, ptr %29, align 8
  call void @free(ptr noundef %264)
  %265 = load i32, ptr %21, align 4
  %266 = call i32 @"\01_close"(i32 noundef %265)
  %267 = call ptr @strdup(ptr noundef @.str.16) #13
  %268 = ptrtoint ptr %267 to i64
  store i64 %268, ptr %5, align 8
  br label %306

269:                                              ; preds = %255
  %270 = load ptr, ptr %32, align 8
  store ptr %270, ptr %29, align 8
  br label %271

271:                                              ; preds = %269, %249
  %272 = load ptr, ptr %29, align 8
  %273 = load i64, ptr %28, align 8
  %274 = getelementptr inbounds i8, ptr %272, i64 %273
  %275 = getelementptr inbounds [4096 x i8], ptr %30, i64 0, i64 0
  %276 = load i64, ptr %31, align 8
  %277 = load ptr, ptr %29, align 8
  %278 = load i64, ptr %28, align 8
  %279 = getelementptr inbounds i8, ptr %277, i64 %278
  %280 = call i64 @llvm.objectsize.i64.p0(ptr %279, i1 false, i1 true, i1 false)
  %281 = call ptr @__memcpy_chk(ptr noundef %274, ptr noundef %275, i64 noundef %276, i64 noundef %280) #13
  %282 = load i64, ptr %31, align 8
  %283 = load i64, ptr %28, align 8
  %284 = add i64 %283, %282
  store i64 %284, ptr %28, align 8
  br label %244, !llvm.loop !26

285:                                              ; preds = %244
  %286 = load ptr, ptr %29, align 8
  %287 = load i64, ptr %28, align 8
  %288 = getelementptr inbounds i8, ptr %286, i64 %287
  store i8 0, ptr %288, align 1
  %289 = load i32, ptr %21, align 4
  %290 = call i32 @"\01_close"(i32 noundef %289)
  %291 = load ptr, ptr %29, align 8
  %292 = call ptr @strstr(ptr noundef %291, ptr noundef @.str.17) #13
  store ptr %292, ptr %33, align 8
  %293 = load ptr, ptr %33, align 8
  %294 = icmp ne ptr %293, null
  br i1 %294, label %295, label %303

295:                                              ; preds = %285
  %296 = load ptr, ptr %33, align 8
  %297 = getelementptr inbounds i8, ptr %296, i64 4
  store ptr %297, ptr %33, align 8
  %298 = load ptr, ptr %33, align 8
  %299 = call ptr @strdup(ptr noundef %298) #13
  store ptr %299, ptr %34, align 8
  %300 = load ptr, ptr %29, align 8
  call void @free(ptr noundef %300)
  %301 = load ptr, ptr %34, align 8
  %302 = ptrtoint ptr %301 to i64
  store i64 %302, ptr %5, align 8
  br label %306

303:                                              ; preds = %285
  %304 = load ptr, ptr %29, align 8
  %305 = ptrtoint ptr %304 to i64
  store i64 %305, ptr %5, align 8
  br label %306

306:                                              ; preds = %303, %295, %263, %238, %227, %213, %161, %114, %106, %55, %48
  %307 = load i64, ptr %5, align 8
  ret i64 %307
}

; Function Attrs: nounwind
declare i32 @strncmp(ptr noundef, ptr noundef, i64 noundef) #4

; Function Attrs: nounwind
declare ptr @strchr(ptr noundef, i32 noundef) #4

; Function Attrs: nounwind
declare ptr @__strncpy_chk(ptr noundef, ptr noundef, i64 noundef, i64 noundef) #4

; Function Attrs: nounwind
declare ptr @__strcpy_chk(ptr noundef, ptr noundef, i64 noundef) #4

declare i32 @atoi(ptr noundef) #1

; Function Attrs: convergent nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.is.constant.i32(i32) #5

; Function Attrs: allocsize(1)
declare ptr @realloc(ptr noundef, i64 noundef) #7

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.objectsize.i64.p0(ptr, i1 immarg, i1 immarg, i1 immarg) #8

; Function Attrs: nounwind
declare ptr @strstr(ptr noundef, ptr noundef) #4

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @ep_sha256_transform(ptr noundef %0, ptr noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  %7 = alloca i32, align 4
  %8 = alloca i32, align 4
  %9 = alloca i32, align 4
  %10 = alloca i32, align 4
  %11 = alloca i32, align 4
  %12 = alloca i32, align 4
  %13 = alloca i32, align 4
  %14 = alloca i32, align 4
  %15 = alloca i32, align 4
  %16 = alloca i32, align 4
  %17 = alloca [64 x i32], align 4
  store ptr %0, ptr %3, align 8
  store ptr %1, ptr %4, align 8
  store i32 0, ptr %13, align 4
  store i32 0, ptr %14, align 4
  br label %18

18:                                               ; preds = %58, %2
  %19 = load i32, ptr %13, align 4
  %20 = icmp ult i32 %19, 16
  br i1 %20, label %21, label %63

21:                                               ; preds = %18
  %22 = load ptr, ptr %4, align 8
  %23 = load i32, ptr %14, align 4
  %24 = zext i32 %23 to i64
  %25 = getelementptr inbounds i8, ptr %22, i64 %24
  %26 = load i8, ptr %25, align 1
  %27 = zext i8 %26 to i32
  %28 = shl i32 %27, 24
  %29 = load ptr, ptr %4, align 8
  %30 = load i32, ptr %14, align 4
  %31 = add i32 %30, 1
  %32 = zext i32 %31 to i64
  %33 = getelementptr inbounds i8, ptr %29, i64 %32
  %34 = load i8, ptr %33, align 1
  %35 = zext i8 %34 to i32
  %36 = shl i32 %35, 16
  %37 = or i32 %28, %36
  %38 = load ptr, ptr %4, align 8
  %39 = load i32, ptr %14, align 4
  %40 = add i32 %39, 2
  %41 = zext i32 %40 to i64
  %42 = getelementptr inbounds i8, ptr %38, i64 %41
  %43 = load i8, ptr %42, align 1
  %44 = zext i8 %43 to i32
  %45 = shl i32 %44, 8
  %46 = or i32 %37, %45
  %47 = load ptr, ptr %4, align 8
  %48 = load i32, ptr %14, align 4
  %49 = add i32 %48, 3
  %50 = zext i32 %49 to i64
  %51 = getelementptr inbounds i8, ptr %47, i64 %50
  %52 = load i8, ptr %51, align 1
  %53 = zext i8 %52 to i32
  %54 = or i32 %46, %53
  %55 = load i32, ptr %13, align 4
  %56 = zext i32 %55 to i64
  %57 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %56
  store i32 %54, ptr %57, align 4
  br label %58

58:                                               ; preds = %21
  %59 = load i32, ptr %13, align 4
  %60 = add i32 %59, 1
  store i32 %60, ptr %13, align 4
  %61 = load i32, ptr %14, align 4
  %62 = add i32 %61, 4
  store i32 %62, ptr %14, align 4
  br label %18, !llvm.loop !27

63:                                               ; preds = %18
  br label %64

64:                                               ; preds = %152, %63
  %65 = load i32, ptr %13, align 4
  %66 = icmp ult i32 %65, 64
  br i1 %66, label %67, label %155

67:                                               ; preds = %64
  %68 = load i32, ptr %13, align 4
  %69 = sub i32 %68, 2
  %70 = zext i32 %69 to i64
  %71 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %70
  %72 = load i32, ptr %71, align 4
  %73 = lshr i32 %72, 17
  %74 = load i32, ptr %13, align 4
  %75 = sub i32 %74, 2
  %76 = zext i32 %75 to i64
  %77 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %76
  %78 = load i32, ptr %77, align 4
  %79 = shl i32 %78, 15
  %80 = or i32 %73, %79
  %81 = load i32, ptr %13, align 4
  %82 = sub i32 %81, 2
  %83 = zext i32 %82 to i64
  %84 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %83
  %85 = load i32, ptr %84, align 4
  %86 = lshr i32 %85, 19
  %87 = load i32, ptr %13, align 4
  %88 = sub i32 %87, 2
  %89 = zext i32 %88 to i64
  %90 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %89
  %91 = load i32, ptr %90, align 4
  %92 = shl i32 %91, 13
  %93 = or i32 %86, %92
  %94 = xor i32 %80, %93
  %95 = load i32, ptr %13, align 4
  %96 = sub i32 %95, 2
  %97 = zext i32 %96 to i64
  %98 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %97
  %99 = load i32, ptr %98, align 4
  %100 = lshr i32 %99, 10
  %101 = xor i32 %94, %100
  %102 = load i32, ptr %13, align 4
  %103 = sub i32 %102, 7
  %104 = zext i32 %103 to i64
  %105 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %104
  %106 = load i32, ptr %105, align 4
  %107 = add i32 %101, %106
  %108 = load i32, ptr %13, align 4
  %109 = sub i32 %108, 15
  %110 = zext i32 %109 to i64
  %111 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %110
  %112 = load i32, ptr %111, align 4
  %113 = lshr i32 %112, 7
  %114 = load i32, ptr %13, align 4
  %115 = sub i32 %114, 15
  %116 = zext i32 %115 to i64
  %117 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %116
  %118 = load i32, ptr %117, align 4
  %119 = shl i32 %118, 25
  %120 = or i32 %113, %119
  %121 = load i32, ptr %13, align 4
  %122 = sub i32 %121, 15
  %123 = zext i32 %122 to i64
  %124 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %123
  %125 = load i32, ptr %124, align 4
  %126 = lshr i32 %125, 18
  %127 = load i32, ptr %13, align 4
  %128 = sub i32 %127, 15
  %129 = zext i32 %128 to i64
  %130 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %129
  %131 = load i32, ptr %130, align 4
  %132 = shl i32 %131, 14
  %133 = or i32 %126, %132
  %134 = xor i32 %120, %133
  %135 = load i32, ptr %13, align 4
  %136 = sub i32 %135, 15
  %137 = zext i32 %136 to i64
  %138 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %137
  %139 = load i32, ptr %138, align 4
  %140 = lshr i32 %139, 3
  %141 = xor i32 %134, %140
  %142 = add i32 %107, %141
  %143 = load i32, ptr %13, align 4
  %144 = sub i32 %143, 16
  %145 = zext i32 %144 to i64
  %146 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %145
  %147 = load i32, ptr %146, align 4
  %148 = add i32 %142, %147
  %149 = load i32, ptr %13, align 4
  %150 = zext i32 %149 to i64
  %151 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %150
  store i32 %148, ptr %151, align 4
  br label %152

152:                                              ; preds = %67
  %153 = load i32, ptr %13, align 4
  %154 = add i32 %153, 1
  store i32 %154, ptr %13, align 4
  br label %64, !llvm.loop !28

155:                                              ; preds = %64
  %156 = load ptr, ptr %3, align 8
  %157 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %156, i32 0, i32 3
  %158 = getelementptr inbounds [8 x i32], ptr %157, i64 0, i64 0
  %159 = load i32, ptr %158, align 8
  store i32 %159, ptr %5, align 4
  %160 = load ptr, ptr %3, align 8
  %161 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %160, i32 0, i32 3
  %162 = getelementptr inbounds [8 x i32], ptr %161, i64 0, i64 1
  %163 = load i32, ptr %162, align 4
  store i32 %163, ptr %6, align 4
  %164 = load ptr, ptr %3, align 8
  %165 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %164, i32 0, i32 3
  %166 = getelementptr inbounds [8 x i32], ptr %165, i64 0, i64 2
  %167 = load i32, ptr %166, align 8
  store i32 %167, ptr %7, align 4
  %168 = load ptr, ptr %3, align 8
  %169 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %168, i32 0, i32 3
  %170 = getelementptr inbounds [8 x i32], ptr %169, i64 0, i64 3
  %171 = load i32, ptr %170, align 4
  store i32 %171, ptr %8, align 4
  %172 = load ptr, ptr %3, align 8
  %173 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %172, i32 0, i32 3
  %174 = getelementptr inbounds [8 x i32], ptr %173, i64 0, i64 4
  %175 = load i32, ptr %174, align 8
  store i32 %175, ptr %9, align 4
  %176 = load ptr, ptr %3, align 8
  %177 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %176, i32 0, i32 3
  %178 = getelementptr inbounds [8 x i32], ptr %177, i64 0, i64 5
  %179 = load i32, ptr %178, align 4
  store i32 %179, ptr %10, align 4
  %180 = load ptr, ptr %3, align 8
  %181 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %180, i32 0, i32 3
  %182 = getelementptr inbounds [8 x i32], ptr %181, i64 0, i64 6
  %183 = load i32, ptr %182, align 8
  store i32 %183, ptr %11, align 4
  %184 = load ptr, ptr %3, align 8
  %185 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %184, i32 0, i32 3
  %186 = getelementptr inbounds [8 x i32], ptr %185, i64 0, i64 7
  %187 = load i32, ptr %186, align 4
  store i32 %187, ptr %12, align 4
  store i32 0, ptr %13, align 4
  br label %188

188:                                              ; preds = %271, %155
  %189 = load i32, ptr %13, align 4
  %190 = icmp ult i32 %189, 64
  br i1 %190, label %191, label %274

191:                                              ; preds = %188
  %192 = load i32, ptr %12, align 4
  %193 = load i32, ptr %9, align 4
  %194 = lshr i32 %193, 6
  %195 = load i32, ptr %9, align 4
  %196 = shl i32 %195, 26
  %197 = or i32 %194, %196
  %198 = load i32, ptr %9, align 4
  %199 = lshr i32 %198, 11
  %200 = load i32, ptr %9, align 4
  %201 = shl i32 %200, 21
  %202 = or i32 %199, %201
  %203 = xor i32 %197, %202
  %204 = load i32, ptr %9, align 4
  %205 = lshr i32 %204, 25
  %206 = load i32, ptr %9, align 4
  %207 = shl i32 %206, 7
  %208 = or i32 %205, %207
  %209 = xor i32 %203, %208
  %210 = add i32 %192, %209
  %211 = load i32, ptr %9, align 4
  %212 = load i32, ptr %10, align 4
  %213 = and i32 %211, %212
  %214 = load i32, ptr %9, align 4
  %215 = xor i32 %214, -1
  %216 = load i32, ptr %11, align 4
  %217 = and i32 %215, %216
  %218 = xor i32 %213, %217
  %219 = add i32 %210, %218
  %220 = load i32, ptr %13, align 4
  %221 = zext i32 %220 to i64
  %222 = getelementptr inbounds [64 x i32], ptr @sha256_k, i64 0, i64 %221
  %223 = load i32, ptr %222, align 4
  %224 = add i32 %219, %223
  %225 = load i32, ptr %13, align 4
  %226 = zext i32 %225 to i64
  %227 = getelementptr inbounds [64 x i32], ptr %17, i64 0, i64 %226
  %228 = load i32, ptr %227, align 4
  %229 = add i32 %224, %228
  store i32 %229, ptr %15, align 4
  %230 = load i32, ptr %5, align 4
  %231 = lshr i32 %230, 2
  %232 = load i32, ptr %5, align 4
  %233 = shl i32 %232, 30
  %234 = or i32 %231, %233
  %235 = load i32, ptr %5, align 4
  %236 = lshr i32 %235, 13
  %237 = load i32, ptr %5, align 4
  %238 = shl i32 %237, 19
  %239 = or i32 %236, %238
  %240 = xor i32 %234, %239
  %241 = load i32, ptr %5, align 4
  %242 = lshr i32 %241, 22
  %243 = load i32, ptr %5, align 4
  %244 = shl i32 %243, 10
  %245 = or i32 %242, %244
  %246 = xor i32 %240, %245
  %247 = load i32, ptr %5, align 4
  %248 = load i32, ptr %6, align 4
  %249 = and i32 %247, %248
  %250 = load i32, ptr %5, align 4
  %251 = load i32, ptr %7, align 4
  %252 = and i32 %250, %251
  %253 = xor i32 %249, %252
  %254 = load i32, ptr %6, align 4
  %255 = load i32, ptr %7, align 4
  %256 = and i32 %254, %255
  %257 = xor i32 %253, %256
  %258 = add i32 %246, %257
  store i32 %258, ptr %16, align 4
  %259 = load i32, ptr %11, align 4
  store i32 %259, ptr %12, align 4
  %260 = load i32, ptr %10, align 4
  store i32 %260, ptr %11, align 4
  %261 = load i32, ptr %9, align 4
  store i32 %261, ptr %10, align 4
  %262 = load i32, ptr %8, align 4
  %263 = load i32, ptr %15, align 4
  %264 = add i32 %262, %263
  store i32 %264, ptr %9, align 4
  %265 = load i32, ptr %7, align 4
  store i32 %265, ptr %8, align 4
  %266 = load i32, ptr %6, align 4
  store i32 %266, ptr %7, align 4
  %267 = load i32, ptr %5, align 4
  store i32 %267, ptr %6, align 4
  %268 = load i32, ptr %15, align 4
  %269 = load i32, ptr %16, align 4
  %270 = add i32 %268, %269
  store i32 %270, ptr %5, align 4
  br label %271

271:                                              ; preds = %191
  %272 = load i32, ptr %13, align 4
  %273 = add i32 %272, 1
  store i32 %273, ptr %13, align 4
  br label %188, !llvm.loop !29

274:                                              ; preds = %188
  %275 = load i32, ptr %5, align 4
  %276 = load ptr, ptr %3, align 8
  %277 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %276, i32 0, i32 3
  %278 = getelementptr inbounds [8 x i32], ptr %277, i64 0, i64 0
  %279 = load i32, ptr %278, align 8
  %280 = add i32 %279, %275
  store i32 %280, ptr %278, align 8
  %281 = load i32, ptr %6, align 4
  %282 = load ptr, ptr %3, align 8
  %283 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %282, i32 0, i32 3
  %284 = getelementptr inbounds [8 x i32], ptr %283, i64 0, i64 1
  %285 = load i32, ptr %284, align 4
  %286 = add i32 %285, %281
  store i32 %286, ptr %284, align 4
  %287 = load i32, ptr %7, align 4
  %288 = load ptr, ptr %3, align 8
  %289 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %288, i32 0, i32 3
  %290 = getelementptr inbounds [8 x i32], ptr %289, i64 0, i64 2
  %291 = load i32, ptr %290, align 8
  %292 = add i32 %291, %287
  store i32 %292, ptr %290, align 8
  %293 = load i32, ptr %8, align 4
  %294 = load ptr, ptr %3, align 8
  %295 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %294, i32 0, i32 3
  %296 = getelementptr inbounds [8 x i32], ptr %295, i64 0, i64 3
  %297 = load i32, ptr %296, align 4
  %298 = add i32 %297, %293
  store i32 %298, ptr %296, align 4
  %299 = load i32, ptr %9, align 4
  %300 = load ptr, ptr %3, align 8
  %301 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %300, i32 0, i32 3
  %302 = getelementptr inbounds [8 x i32], ptr %301, i64 0, i64 4
  %303 = load i32, ptr %302, align 8
  %304 = add i32 %303, %299
  store i32 %304, ptr %302, align 8
  %305 = load i32, ptr %10, align 4
  %306 = load ptr, ptr %3, align 8
  %307 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %306, i32 0, i32 3
  %308 = getelementptr inbounds [8 x i32], ptr %307, i64 0, i64 5
  %309 = load i32, ptr %308, align 4
  %310 = add i32 %309, %305
  store i32 %310, ptr %308, align 4
  %311 = load i32, ptr %11, align 4
  %312 = load ptr, ptr %3, align 8
  %313 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %312, i32 0, i32 3
  %314 = getelementptr inbounds [8 x i32], ptr %313, i64 0, i64 6
  %315 = load i32, ptr %314, align 8
  %316 = add i32 %315, %311
  store i32 %316, ptr %314, align 8
  %317 = load i32, ptr %12, align 4
  %318 = load ptr, ptr %3, align 8
  %319 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %318, i32 0, i32 3
  %320 = getelementptr inbounds [8 x i32], ptr %319, i64 0, i64 7
  %321 = load i32, ptr %320, align 4
  %322 = add i32 %321, %317
  store i32 %322, ptr %320, align 4
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @ep_sha256_init(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  %4 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %3, i32 0, i32 1
  store i32 0, ptr %4, align 8
  %5 = load ptr, ptr %2, align 8
  %6 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %5, i32 0, i32 2
  store i64 0, ptr %6, align 8
  %7 = load ptr, ptr %2, align 8
  %8 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %7, i32 0, i32 3
  %9 = getelementptr inbounds [8 x i32], ptr %8, i64 0, i64 0
  store i32 1779033703, ptr %9, align 8
  %10 = load ptr, ptr %2, align 8
  %11 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %10, i32 0, i32 3
  %12 = getelementptr inbounds [8 x i32], ptr %11, i64 0, i64 1
  store i32 -1150833019, ptr %12, align 4
  %13 = load ptr, ptr %2, align 8
  %14 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %13, i32 0, i32 3
  %15 = getelementptr inbounds [8 x i32], ptr %14, i64 0, i64 2
  store i32 1013904242, ptr %15, align 8
  %16 = load ptr, ptr %2, align 8
  %17 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %16, i32 0, i32 3
  %18 = getelementptr inbounds [8 x i32], ptr %17, i64 0, i64 3
  store i32 -1521486534, ptr %18, align 4
  %19 = load ptr, ptr %2, align 8
  %20 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %19, i32 0, i32 3
  %21 = getelementptr inbounds [8 x i32], ptr %20, i64 0, i64 4
  store i32 1359893119, ptr %21, align 8
  %22 = load ptr, ptr %2, align 8
  %23 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %22, i32 0, i32 3
  %24 = getelementptr inbounds [8 x i32], ptr %23, i64 0, i64 5
  store i32 -1694144372, ptr %24, align 4
  %25 = load ptr, ptr %2, align 8
  %26 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %25, i32 0, i32 3
  %27 = getelementptr inbounds [8 x i32], ptr %26, i64 0, i64 6
  store i32 528734635, ptr %27, align 8
  %28 = load ptr, ptr %2, align 8
  %29 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %28, i32 0, i32 3
  %30 = getelementptr inbounds [8 x i32], ptr %29, i64 0, i64 7
  store i32 1541459225, ptr %30, align 4
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @ep_sha256_update(ptr noundef %0, ptr noundef %1, i64 noundef %2) #0 {
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  store ptr %0, ptr %4, align 8
  store ptr %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  store i64 0, ptr %7, align 8
  br label %8

8:                                                ; preds = %44, %3
  %9 = load i64, ptr %7, align 8
  %10 = load i64, ptr %6, align 8
  %11 = icmp ult i64 %9, %10
  br i1 %11, label %12, label %47

12:                                               ; preds = %8
  %13 = load ptr, ptr %5, align 8
  %14 = load i64, ptr %7, align 8
  %15 = getelementptr inbounds i8, ptr %13, i64 %14
  %16 = load i8, ptr %15, align 1
  %17 = load ptr, ptr %4, align 8
  %18 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %17, i32 0, i32 0
  %19 = load ptr, ptr %4, align 8
  %20 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %19, i32 0, i32 1
  %21 = load i32, ptr %20, align 8
  %22 = zext i32 %21 to i64
  %23 = getelementptr inbounds [64 x i8], ptr %18, i64 0, i64 %22
  store i8 %16, ptr %23, align 1
  %24 = load ptr, ptr %4, align 8
  %25 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %24, i32 0, i32 1
  %26 = load i32, ptr %25, align 8
  %27 = add i32 %26, 1
  store i32 %27, ptr %25, align 8
  %28 = load ptr, ptr %4, align 8
  %29 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %28, i32 0, i32 1
  %30 = load i32, ptr %29, align 8
  %31 = icmp eq i32 %30, 64
  br i1 %31, label %32, label %43

32:                                               ; preds = %12
  %33 = load ptr, ptr %4, align 8
  %34 = load ptr, ptr %4, align 8
  %35 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %34, i32 0, i32 0
  %36 = getelementptr inbounds [64 x i8], ptr %35, i64 0, i64 0
  call void @ep_sha256_transform(ptr noundef %33, ptr noundef %36)
  %37 = load ptr, ptr %4, align 8
  %38 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %37, i32 0, i32 2
  %39 = load i64, ptr %38, align 8
  %40 = add i64 %39, 512
  store i64 %40, ptr %38, align 8
  %41 = load ptr, ptr %4, align 8
  %42 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %41, i32 0, i32 1
  store i32 0, ptr %42, align 8
  br label %43

43:                                               ; preds = %32, %12
  br label %44

44:                                               ; preds = %43
  %45 = load i64, ptr %7, align 8
  %46 = add i64 %45, 1
  store i64 %46, ptr %7, align 8
  br label %8, !llvm.loop !30

47:                                               ; preds = %8
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @ep_sha256_final(ptr noundef %0, ptr noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i32, align 4
  store ptr %0, ptr %3, align 8
  store ptr %1, ptr %4, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %6, i32 0, i32 1
  %8 = load i32, ptr %7, align 8
  store i32 %8, ptr %5, align 4
  %9 = load ptr, ptr %3, align 8
  %10 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %9, i32 0, i32 1
  %11 = load i32, ptr %10, align 8
  %12 = icmp ult i32 %11, 56
  br i1 %12, label %13, label %31

13:                                               ; preds = %2
  %14 = load ptr, ptr %3, align 8
  %15 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %14, i32 0, i32 0
  %16 = load i32, ptr %5, align 4
  %17 = add i32 %16, 1
  store i32 %17, ptr %5, align 4
  %18 = zext i32 %16 to i64
  %19 = getelementptr inbounds [64 x i8], ptr %15, i64 0, i64 %18
  store i8 -128, ptr %19, align 1
  br label %20

20:                                               ; preds = %23, %13
  %21 = load i32, ptr %5, align 4
  %22 = icmp ult i32 %21, 56
  br i1 %22, label %23, label %30

23:                                               ; preds = %20
  %24 = load ptr, ptr %3, align 8
  %25 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %24, i32 0, i32 0
  %26 = load i32, ptr %5, align 4
  %27 = add i32 %26, 1
  store i32 %27, ptr %5, align 4
  %28 = zext i32 %26 to i64
  %29 = getelementptr inbounds [64 x i8], ptr %25, i64 0, i64 %28
  store i8 0, ptr %29, align 1
  br label %20, !llvm.loop !31

30:                                               ; preds = %20
  br label %61

31:                                               ; preds = %2
  %32 = load ptr, ptr %3, align 8
  %33 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %32, i32 0, i32 0
  %34 = load i32, ptr %5, align 4
  %35 = add i32 %34, 1
  store i32 %35, ptr %5, align 4
  %36 = zext i32 %34 to i64
  %37 = getelementptr inbounds [64 x i8], ptr %33, i64 0, i64 %36
  store i8 -128, ptr %37, align 1
  br label %38

38:                                               ; preds = %41, %31
  %39 = load i32, ptr %5, align 4
  %40 = icmp ult i32 %39, 64
  br i1 %40, label %41, label %48

41:                                               ; preds = %38
  %42 = load ptr, ptr %3, align 8
  %43 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %42, i32 0, i32 0
  %44 = load i32, ptr %5, align 4
  %45 = add i32 %44, 1
  store i32 %45, ptr %5, align 4
  %46 = zext i32 %44 to i64
  %47 = getelementptr inbounds [64 x i8], ptr %43, i64 0, i64 %46
  store i8 0, ptr %47, align 1
  br label %38, !llvm.loop !32

48:                                               ; preds = %38
  %49 = load ptr, ptr %3, align 8
  %50 = load ptr, ptr %3, align 8
  %51 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %50, i32 0, i32 0
  %52 = getelementptr inbounds [64 x i8], ptr %51, i64 0, i64 0
  call void @ep_sha256_transform(ptr noundef %49, ptr noundef %52)
  %53 = load ptr, ptr %3, align 8
  %54 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %53, i32 0, i32 0
  %55 = getelementptr inbounds [64 x i8], ptr %54, i64 0, i64 0
  %56 = load ptr, ptr %3, align 8
  %57 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %56, i32 0, i32 0
  %58 = getelementptr inbounds [64 x i8], ptr %57, i64 0, i64 0
  %59 = call i64 @llvm.objectsize.i64.p0(ptr %58, i1 false, i1 true, i1 false)
  %60 = call ptr @__memset_chk(ptr noundef %55, i32 noundef 0, i64 noundef 56, i64 noundef %59) #13
  br label %61

61:                                               ; preds = %48, %30
  %62 = load ptr, ptr %3, align 8
  %63 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %62, i32 0, i32 1
  %64 = load i32, ptr %63, align 8
  %65 = mul i32 %64, 8
  %66 = zext i32 %65 to i64
  %67 = load ptr, ptr %3, align 8
  %68 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %67, i32 0, i32 2
  %69 = load i64, ptr %68, align 8
  %70 = add i64 %69, %66
  store i64 %70, ptr %68, align 8
  %71 = load ptr, ptr %3, align 8
  %72 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %71, i32 0, i32 2
  %73 = load i64, ptr %72, align 8
  %74 = trunc i64 %73 to i8
  %75 = load ptr, ptr %3, align 8
  %76 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %75, i32 0, i32 0
  %77 = getelementptr inbounds [64 x i8], ptr %76, i64 0, i64 63
  store i8 %74, ptr %77, align 1
  %78 = load ptr, ptr %3, align 8
  %79 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %78, i32 0, i32 2
  %80 = load i64, ptr %79, align 8
  %81 = lshr i64 %80, 8
  %82 = trunc i64 %81 to i8
  %83 = load ptr, ptr %3, align 8
  %84 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %83, i32 0, i32 0
  %85 = getelementptr inbounds [64 x i8], ptr %84, i64 0, i64 62
  store i8 %82, ptr %85, align 2
  %86 = load ptr, ptr %3, align 8
  %87 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %86, i32 0, i32 2
  %88 = load i64, ptr %87, align 8
  %89 = lshr i64 %88, 16
  %90 = trunc i64 %89 to i8
  %91 = load ptr, ptr %3, align 8
  %92 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %91, i32 0, i32 0
  %93 = getelementptr inbounds [64 x i8], ptr %92, i64 0, i64 61
  store i8 %90, ptr %93, align 1
  %94 = load ptr, ptr %3, align 8
  %95 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %94, i32 0, i32 2
  %96 = load i64, ptr %95, align 8
  %97 = lshr i64 %96, 24
  %98 = trunc i64 %97 to i8
  %99 = load ptr, ptr %3, align 8
  %100 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %99, i32 0, i32 0
  %101 = getelementptr inbounds [64 x i8], ptr %100, i64 0, i64 60
  store i8 %98, ptr %101, align 4
  %102 = load ptr, ptr %3, align 8
  %103 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %102, i32 0, i32 2
  %104 = load i64, ptr %103, align 8
  %105 = lshr i64 %104, 32
  %106 = trunc i64 %105 to i8
  %107 = load ptr, ptr %3, align 8
  %108 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %107, i32 0, i32 0
  %109 = getelementptr inbounds [64 x i8], ptr %108, i64 0, i64 59
  store i8 %106, ptr %109, align 1
  %110 = load ptr, ptr %3, align 8
  %111 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %110, i32 0, i32 2
  %112 = load i64, ptr %111, align 8
  %113 = lshr i64 %112, 40
  %114 = trunc i64 %113 to i8
  %115 = load ptr, ptr %3, align 8
  %116 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %115, i32 0, i32 0
  %117 = getelementptr inbounds [64 x i8], ptr %116, i64 0, i64 58
  store i8 %114, ptr %117, align 2
  %118 = load ptr, ptr %3, align 8
  %119 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %118, i32 0, i32 2
  %120 = load i64, ptr %119, align 8
  %121 = lshr i64 %120, 48
  %122 = trunc i64 %121 to i8
  %123 = load ptr, ptr %3, align 8
  %124 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %123, i32 0, i32 0
  %125 = getelementptr inbounds [64 x i8], ptr %124, i64 0, i64 57
  store i8 %122, ptr %125, align 1
  %126 = load ptr, ptr %3, align 8
  %127 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %126, i32 0, i32 2
  %128 = load i64, ptr %127, align 8
  %129 = lshr i64 %128, 56
  %130 = trunc i64 %129 to i8
  %131 = load ptr, ptr %3, align 8
  %132 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %131, i32 0, i32 0
  %133 = getelementptr inbounds [64 x i8], ptr %132, i64 0, i64 56
  store i8 %130, ptr %133, align 8
  %134 = load ptr, ptr %3, align 8
  %135 = load ptr, ptr %3, align 8
  %136 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %135, i32 0, i32 0
  %137 = getelementptr inbounds [64 x i8], ptr %136, i64 0, i64 0
  call void @ep_sha256_transform(ptr noundef %134, ptr noundef %137)
  store i32 0, ptr %5, align 4
  br label %138

138:                                              ; preds = %261, %61
  %139 = load i32, ptr %5, align 4
  %140 = icmp ult i32 %139, 4
  br i1 %140, label %141, label %264

141:                                              ; preds = %138
  %142 = load ptr, ptr %3, align 8
  %143 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %142, i32 0, i32 3
  %144 = getelementptr inbounds [8 x i32], ptr %143, i64 0, i64 0
  %145 = load i32, ptr %144, align 8
  %146 = load i32, ptr %5, align 4
  %147 = mul i32 %146, 8
  %148 = sub i32 24, %147
  %149 = lshr i32 %145, %148
  %150 = and i32 %149, 255
  %151 = trunc i32 %150 to i8
  %152 = load ptr, ptr %4, align 8
  %153 = load i32, ptr %5, align 4
  %154 = zext i32 %153 to i64
  %155 = getelementptr inbounds i8, ptr %152, i64 %154
  store i8 %151, ptr %155, align 1
  %156 = load ptr, ptr %3, align 8
  %157 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %156, i32 0, i32 3
  %158 = getelementptr inbounds [8 x i32], ptr %157, i64 0, i64 1
  %159 = load i32, ptr %158, align 4
  %160 = load i32, ptr %5, align 4
  %161 = mul i32 %160, 8
  %162 = sub i32 24, %161
  %163 = lshr i32 %159, %162
  %164 = and i32 %163, 255
  %165 = trunc i32 %164 to i8
  %166 = load ptr, ptr %4, align 8
  %167 = load i32, ptr %5, align 4
  %168 = add i32 %167, 4
  %169 = zext i32 %168 to i64
  %170 = getelementptr inbounds i8, ptr %166, i64 %169
  store i8 %165, ptr %170, align 1
  %171 = load ptr, ptr %3, align 8
  %172 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %171, i32 0, i32 3
  %173 = getelementptr inbounds [8 x i32], ptr %172, i64 0, i64 2
  %174 = load i32, ptr %173, align 8
  %175 = load i32, ptr %5, align 4
  %176 = mul i32 %175, 8
  %177 = sub i32 24, %176
  %178 = lshr i32 %174, %177
  %179 = and i32 %178, 255
  %180 = trunc i32 %179 to i8
  %181 = load ptr, ptr %4, align 8
  %182 = load i32, ptr %5, align 4
  %183 = add i32 %182, 8
  %184 = zext i32 %183 to i64
  %185 = getelementptr inbounds i8, ptr %181, i64 %184
  store i8 %180, ptr %185, align 1
  %186 = load ptr, ptr %3, align 8
  %187 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %186, i32 0, i32 3
  %188 = getelementptr inbounds [8 x i32], ptr %187, i64 0, i64 3
  %189 = load i32, ptr %188, align 4
  %190 = load i32, ptr %5, align 4
  %191 = mul i32 %190, 8
  %192 = sub i32 24, %191
  %193 = lshr i32 %189, %192
  %194 = and i32 %193, 255
  %195 = trunc i32 %194 to i8
  %196 = load ptr, ptr %4, align 8
  %197 = load i32, ptr %5, align 4
  %198 = add i32 %197, 12
  %199 = zext i32 %198 to i64
  %200 = getelementptr inbounds i8, ptr %196, i64 %199
  store i8 %195, ptr %200, align 1
  %201 = load ptr, ptr %3, align 8
  %202 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %201, i32 0, i32 3
  %203 = getelementptr inbounds [8 x i32], ptr %202, i64 0, i64 4
  %204 = load i32, ptr %203, align 8
  %205 = load i32, ptr %5, align 4
  %206 = mul i32 %205, 8
  %207 = sub i32 24, %206
  %208 = lshr i32 %204, %207
  %209 = and i32 %208, 255
  %210 = trunc i32 %209 to i8
  %211 = load ptr, ptr %4, align 8
  %212 = load i32, ptr %5, align 4
  %213 = add i32 %212, 16
  %214 = zext i32 %213 to i64
  %215 = getelementptr inbounds i8, ptr %211, i64 %214
  store i8 %210, ptr %215, align 1
  %216 = load ptr, ptr %3, align 8
  %217 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %216, i32 0, i32 3
  %218 = getelementptr inbounds [8 x i32], ptr %217, i64 0, i64 5
  %219 = load i32, ptr %218, align 4
  %220 = load i32, ptr %5, align 4
  %221 = mul i32 %220, 8
  %222 = sub i32 24, %221
  %223 = lshr i32 %219, %222
  %224 = and i32 %223, 255
  %225 = trunc i32 %224 to i8
  %226 = load ptr, ptr %4, align 8
  %227 = load i32, ptr %5, align 4
  %228 = add i32 %227, 20
  %229 = zext i32 %228 to i64
  %230 = getelementptr inbounds i8, ptr %226, i64 %229
  store i8 %225, ptr %230, align 1
  %231 = load ptr, ptr %3, align 8
  %232 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %231, i32 0, i32 3
  %233 = getelementptr inbounds [8 x i32], ptr %232, i64 0, i64 6
  %234 = load i32, ptr %233, align 8
  %235 = load i32, ptr %5, align 4
  %236 = mul i32 %235, 8
  %237 = sub i32 24, %236
  %238 = lshr i32 %234, %237
  %239 = and i32 %238, 255
  %240 = trunc i32 %239 to i8
  %241 = load ptr, ptr %4, align 8
  %242 = load i32, ptr %5, align 4
  %243 = add i32 %242, 24
  %244 = zext i32 %243 to i64
  %245 = getelementptr inbounds i8, ptr %241, i64 %244
  store i8 %240, ptr %245, align 1
  %246 = load ptr, ptr %3, align 8
  %247 = getelementptr inbounds %struct.EP_SHA256_CTX, ptr %246, i32 0, i32 3
  %248 = getelementptr inbounds [8 x i32], ptr %247, i64 0, i64 7
  %249 = load i32, ptr %248, align 4
  %250 = load i32, ptr %5, align 4
  %251 = mul i32 %250, 8
  %252 = sub i32 24, %251
  %253 = lshr i32 %249, %252
  %254 = and i32 %253, 255
  %255 = trunc i32 %254 to i8
  %256 = load ptr, ptr %4, align 8
  %257 = load i32, ptr %5, align 4
  %258 = add i32 %257, 28
  %259 = zext i32 %258 to i64
  %260 = getelementptr inbounds i8, ptr %256, i64 %259
  store i8 %255, ptr %260, align 1
  br label %261

261:                                              ; preds = %141
  %262 = load i32, ptr %5, align 4
  %263 = add i32 %262, 1
  store i32 %263, ptr %5, align 4
  br label %138, !llvm.loop !33

264:                                              ; preds = %138
  ret void
}

; Function Attrs: nounwind
declare ptr @__memset_chk(ptr noundef, i32 noundef, i64 noundef, i64 noundef) #4

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define ptr @ep_sha256(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca %struct.EP_SHA256_CTX, align 8
  %4 = alloca [32 x i8], align 1
  %5 = alloca ptr, align 8
  %6 = alloca i32, align 4
  store ptr %0, ptr %2, align 8
  %7 = load ptr, ptr %2, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store ptr @.str.5, ptr %2, align 8
  br label %10

10:                                               ; preds = %9, %1
  call void @ep_sha256_init(ptr noundef %3)
  %11 = load ptr, ptr %2, align 8
  %12 = load ptr, ptr %2, align 8
  %13 = call i64 @strlen(ptr noundef %12) #13
  call void @ep_sha256_update(ptr noundef %3, ptr noundef %11, i64 noundef %13)
  %14 = getelementptr inbounds [32 x i8], ptr %4, i64 0, i64 0
  call void @ep_sha256_final(ptr noundef %3, ptr noundef %14)
  %15 = call ptr @malloc(i64 noundef 65) #14
  store ptr %15, ptr %5, align 8
  %16 = load ptr, ptr %5, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %18, label %46

18:                                               ; preds = %10
  store i32 0, ptr %6, align 4
  br label %19

19:                                               ; preds = %40, %18
  %20 = load i32, ptr %6, align 4
  %21 = icmp slt i32 %20, 32
  br i1 %21, label %22, label %43

22:                                               ; preds = %19
  %23 = load ptr, ptr %5, align 8
  %24 = load i32, ptr %6, align 4
  %25 = mul nsw i32 %24, 2
  %26 = sext i32 %25 to i64
  %27 = getelementptr inbounds i8, ptr %23, i64 %26
  %28 = load ptr, ptr %5, align 8
  %29 = load i32, ptr %6, align 4
  %30 = mul nsw i32 %29, 2
  %31 = sext i32 %30 to i64
  %32 = getelementptr inbounds i8, ptr %28, i64 %31
  %33 = call i64 @llvm.objectsize.i64.p0(ptr %32, i1 false, i1 true, i1 false)
  %34 = load i32, ptr %6, align 4
  %35 = sext i32 %34 to i64
  %36 = getelementptr inbounds [32 x i8], ptr %4, i64 0, i64 %35
  %37 = load i8, ptr %36, align 1
  %38 = zext i8 %37 to i32
  %39 = call i32 (ptr, i64, i32, i64, ptr, ...) @__snprintf_chk(ptr noundef %27, i64 noundef 3, i32 noundef 0, i64 noundef %33, ptr noundef @.str.18, i32 noundef %38)
  br label %40

40:                                               ; preds = %22
  %41 = load i32, ptr %6, align 4
  %42 = add nsw i32 %41, 1
  store i32 %42, ptr %6, align 4
  br label %19, !llvm.loop !34

43:                                               ; preds = %19
  %44 = load ptr, ptr %5, align 8
  %45 = getelementptr inbounds i8, ptr %44, i64 64
  store i8 0, ptr %45, align 1
  br label %46

46:                                               ; preds = %43, %10
  %47 = load ptr, ptr %5, align 8
  ret ptr %47
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @ep_md5_init(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  %4 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %3, i32 0, i32 0
  %5 = getelementptr inbounds [2 x i32], ptr %4, i64 0, i64 1
  store i32 0, ptr %5, align 4
  %6 = load ptr, ptr %2, align 8
  %7 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %6, i32 0, i32 0
  %8 = getelementptr inbounds [2 x i32], ptr %7, i64 0, i64 0
  store i32 0, ptr %8, align 4
  %9 = load ptr, ptr %2, align 8
  %10 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %9, i32 0, i32 1
  %11 = getelementptr inbounds [4 x i32], ptr %10, i64 0, i64 0
  store i32 1732584193, ptr %11, align 4
  %12 = load ptr, ptr %2, align 8
  %13 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %12, i32 0, i32 1
  %14 = getelementptr inbounds [4 x i32], ptr %13, i64 0, i64 1
  store i32 -271733879, ptr %14, align 4
  %15 = load ptr, ptr %2, align 8
  %16 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %15, i32 0, i32 1
  %17 = getelementptr inbounds [4 x i32], ptr %16, i64 0, i64 2
  store i32 -1732584194, ptr %17, align 4
  %18 = load ptr, ptr %2, align 8
  %19 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %18, i32 0, i32 1
  %20 = getelementptr inbounds [4 x i32], ptr %19, i64 0, i64 3
  store i32 271733878, ptr %20, align 4
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @ep_md5_transform(ptr noundef %0, ptr noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  %7 = alloca i32, align 4
  %8 = alloca i32, align 4
  %9 = alloca [16 x i32], align 4
  %10 = alloca i32, align 4
  %11 = alloca i32, align 4
  store ptr %0, ptr %3, align 8
  store ptr %1, ptr %4, align 8
  %12 = load ptr, ptr %3, align 8
  %13 = getelementptr inbounds i32, ptr %12, i64 0
  %14 = load i32, ptr %13, align 4
  store i32 %14, ptr %5, align 4
  %15 = load ptr, ptr %3, align 8
  %16 = getelementptr inbounds i32, ptr %15, i64 1
  %17 = load i32, ptr %16, align 4
  store i32 %17, ptr %6, align 4
  %18 = load ptr, ptr %3, align 8
  %19 = getelementptr inbounds i32, ptr %18, i64 2
  %20 = load i32, ptr %19, align 4
  store i32 %20, ptr %7, align 4
  %21 = load ptr, ptr %3, align 8
  %22 = getelementptr inbounds i32, ptr %21, i64 3
  %23 = load i32, ptr %22, align 4
  store i32 %23, ptr %8, align 4
  store i32 0, ptr %10, align 4
  store i32 0, ptr %11, align 4
  br label %24

24:                                               ; preds = %64, %2
  %25 = load i32, ptr %10, align 4
  %26 = icmp slt i32 %25, 16
  br i1 %26, label %27, label %69

27:                                               ; preds = %24
  %28 = load ptr, ptr %4, align 8
  %29 = load i32, ptr %11, align 4
  %30 = sext i32 %29 to i64
  %31 = getelementptr inbounds i8, ptr %28, i64 %30
  %32 = load i8, ptr %31, align 1
  %33 = zext i8 %32 to i32
  %34 = load ptr, ptr %4, align 8
  %35 = load i32, ptr %11, align 4
  %36 = add nsw i32 %35, 1
  %37 = sext i32 %36 to i64
  %38 = getelementptr inbounds i8, ptr %34, i64 %37
  %39 = load i8, ptr %38, align 1
  %40 = zext i8 %39 to i32
  %41 = shl i32 %40, 8
  %42 = or i32 %33, %41
  %43 = load ptr, ptr %4, align 8
  %44 = load i32, ptr %11, align 4
  %45 = add nsw i32 %44, 2
  %46 = sext i32 %45 to i64
  %47 = getelementptr inbounds i8, ptr %43, i64 %46
  %48 = load i8, ptr %47, align 1
  %49 = zext i8 %48 to i32
  %50 = shl i32 %49, 16
  %51 = or i32 %42, %50
  %52 = load ptr, ptr %4, align 8
  %53 = load i32, ptr %11, align 4
  %54 = add nsw i32 %53, 3
  %55 = sext i32 %54 to i64
  %56 = getelementptr inbounds i8, ptr %52, i64 %55
  %57 = load i8, ptr %56, align 1
  %58 = zext i8 %57 to i32
  %59 = shl i32 %58, 24
  %60 = or i32 %51, %59
  %61 = load i32, ptr %10, align 4
  %62 = sext i32 %61 to i64
  %63 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 %62
  store i32 %60, ptr %63, align 4
  br label %64

64:                                               ; preds = %27
  %65 = load i32, ptr %10, align 4
  %66 = add nsw i32 %65, 1
  store i32 %66, ptr %10, align 4
  %67 = load i32, ptr %11, align 4
  %68 = add nsw i32 %67, 4
  store i32 %68, ptr %11, align 4
  br label %24, !llvm.loop !35

69:                                               ; preds = %24
  %70 = load i32, ptr %6, align 4
  %71 = load i32, ptr %7, align 4
  %72 = and i32 %70, %71
  %73 = load i32, ptr %6, align 4
  %74 = xor i32 %73, -1
  %75 = load i32, ptr %8, align 4
  %76 = and i32 %74, %75
  %77 = or i32 %72, %76
  %78 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 0
  %79 = load i32, ptr %78, align 4
  %80 = add i32 %77, %79
  %81 = add i32 %80, -680876936
  %82 = load i32, ptr %5, align 4
  %83 = add i32 %82, %81
  store i32 %83, ptr %5, align 4
  %84 = load i32, ptr %5, align 4
  %85 = shl i32 %84, 7
  %86 = load i32, ptr %5, align 4
  %87 = lshr i32 %86, 25
  %88 = or i32 %85, %87
  store i32 %88, ptr %5, align 4
  %89 = load i32, ptr %6, align 4
  %90 = load i32, ptr %5, align 4
  %91 = add i32 %90, %89
  store i32 %91, ptr %5, align 4
  %92 = load i32, ptr %5, align 4
  %93 = load i32, ptr %6, align 4
  %94 = and i32 %92, %93
  %95 = load i32, ptr %5, align 4
  %96 = xor i32 %95, -1
  %97 = load i32, ptr %7, align 4
  %98 = and i32 %96, %97
  %99 = or i32 %94, %98
  %100 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 1
  %101 = load i32, ptr %100, align 4
  %102 = add i32 %99, %101
  %103 = add i32 %102, -389564586
  %104 = load i32, ptr %8, align 4
  %105 = add i32 %104, %103
  store i32 %105, ptr %8, align 4
  %106 = load i32, ptr %8, align 4
  %107 = shl i32 %106, 12
  %108 = load i32, ptr %8, align 4
  %109 = lshr i32 %108, 20
  %110 = or i32 %107, %109
  store i32 %110, ptr %8, align 4
  %111 = load i32, ptr %5, align 4
  %112 = load i32, ptr %8, align 4
  %113 = add i32 %112, %111
  store i32 %113, ptr %8, align 4
  %114 = load i32, ptr %8, align 4
  %115 = load i32, ptr %5, align 4
  %116 = and i32 %114, %115
  %117 = load i32, ptr %8, align 4
  %118 = xor i32 %117, -1
  %119 = load i32, ptr %6, align 4
  %120 = and i32 %118, %119
  %121 = or i32 %116, %120
  %122 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 2
  %123 = load i32, ptr %122, align 4
  %124 = add i32 %121, %123
  %125 = add i32 %124, 606105819
  %126 = load i32, ptr %7, align 4
  %127 = add i32 %126, %125
  store i32 %127, ptr %7, align 4
  %128 = load i32, ptr %7, align 4
  %129 = shl i32 %128, 17
  %130 = load i32, ptr %7, align 4
  %131 = lshr i32 %130, 15
  %132 = or i32 %129, %131
  store i32 %132, ptr %7, align 4
  %133 = load i32, ptr %8, align 4
  %134 = load i32, ptr %7, align 4
  %135 = add i32 %134, %133
  store i32 %135, ptr %7, align 4
  %136 = load i32, ptr %7, align 4
  %137 = load i32, ptr %8, align 4
  %138 = and i32 %136, %137
  %139 = load i32, ptr %7, align 4
  %140 = xor i32 %139, -1
  %141 = load i32, ptr %5, align 4
  %142 = and i32 %140, %141
  %143 = or i32 %138, %142
  %144 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 3
  %145 = load i32, ptr %144, align 4
  %146 = add i32 %143, %145
  %147 = add i32 %146, -1044525330
  %148 = load i32, ptr %6, align 4
  %149 = add i32 %148, %147
  store i32 %149, ptr %6, align 4
  %150 = load i32, ptr %6, align 4
  %151 = shl i32 %150, 22
  %152 = load i32, ptr %6, align 4
  %153 = lshr i32 %152, 10
  %154 = or i32 %151, %153
  store i32 %154, ptr %6, align 4
  %155 = load i32, ptr %7, align 4
  %156 = load i32, ptr %6, align 4
  %157 = add i32 %156, %155
  store i32 %157, ptr %6, align 4
  %158 = load i32, ptr %6, align 4
  %159 = load i32, ptr %7, align 4
  %160 = and i32 %158, %159
  %161 = load i32, ptr %6, align 4
  %162 = xor i32 %161, -1
  %163 = load i32, ptr %8, align 4
  %164 = and i32 %162, %163
  %165 = or i32 %160, %164
  %166 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 4
  %167 = load i32, ptr %166, align 4
  %168 = add i32 %165, %167
  %169 = add i32 %168, -176418897
  %170 = load i32, ptr %5, align 4
  %171 = add i32 %170, %169
  store i32 %171, ptr %5, align 4
  %172 = load i32, ptr %5, align 4
  %173 = shl i32 %172, 7
  %174 = load i32, ptr %5, align 4
  %175 = lshr i32 %174, 25
  %176 = or i32 %173, %175
  store i32 %176, ptr %5, align 4
  %177 = load i32, ptr %6, align 4
  %178 = load i32, ptr %5, align 4
  %179 = add i32 %178, %177
  store i32 %179, ptr %5, align 4
  %180 = load i32, ptr %5, align 4
  %181 = load i32, ptr %6, align 4
  %182 = and i32 %180, %181
  %183 = load i32, ptr %5, align 4
  %184 = xor i32 %183, -1
  %185 = load i32, ptr %7, align 4
  %186 = and i32 %184, %185
  %187 = or i32 %182, %186
  %188 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 5
  %189 = load i32, ptr %188, align 4
  %190 = add i32 %187, %189
  %191 = add i32 %190, 1200080426
  %192 = load i32, ptr %8, align 4
  %193 = add i32 %192, %191
  store i32 %193, ptr %8, align 4
  %194 = load i32, ptr %8, align 4
  %195 = shl i32 %194, 12
  %196 = load i32, ptr %8, align 4
  %197 = lshr i32 %196, 20
  %198 = or i32 %195, %197
  store i32 %198, ptr %8, align 4
  %199 = load i32, ptr %5, align 4
  %200 = load i32, ptr %8, align 4
  %201 = add i32 %200, %199
  store i32 %201, ptr %8, align 4
  %202 = load i32, ptr %8, align 4
  %203 = load i32, ptr %5, align 4
  %204 = and i32 %202, %203
  %205 = load i32, ptr %8, align 4
  %206 = xor i32 %205, -1
  %207 = load i32, ptr %6, align 4
  %208 = and i32 %206, %207
  %209 = or i32 %204, %208
  %210 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 6
  %211 = load i32, ptr %210, align 4
  %212 = add i32 %209, %211
  %213 = add i32 %212, -1473231341
  %214 = load i32, ptr %7, align 4
  %215 = add i32 %214, %213
  store i32 %215, ptr %7, align 4
  %216 = load i32, ptr %7, align 4
  %217 = shl i32 %216, 17
  %218 = load i32, ptr %7, align 4
  %219 = lshr i32 %218, 15
  %220 = or i32 %217, %219
  store i32 %220, ptr %7, align 4
  %221 = load i32, ptr %8, align 4
  %222 = load i32, ptr %7, align 4
  %223 = add i32 %222, %221
  store i32 %223, ptr %7, align 4
  %224 = load i32, ptr %7, align 4
  %225 = load i32, ptr %8, align 4
  %226 = and i32 %224, %225
  %227 = load i32, ptr %7, align 4
  %228 = xor i32 %227, -1
  %229 = load i32, ptr %5, align 4
  %230 = and i32 %228, %229
  %231 = or i32 %226, %230
  %232 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 7
  %233 = load i32, ptr %232, align 4
  %234 = add i32 %231, %233
  %235 = add i32 %234, -45705983
  %236 = load i32, ptr %6, align 4
  %237 = add i32 %236, %235
  store i32 %237, ptr %6, align 4
  %238 = load i32, ptr %6, align 4
  %239 = shl i32 %238, 22
  %240 = load i32, ptr %6, align 4
  %241 = lshr i32 %240, 10
  %242 = or i32 %239, %241
  store i32 %242, ptr %6, align 4
  %243 = load i32, ptr %7, align 4
  %244 = load i32, ptr %6, align 4
  %245 = add i32 %244, %243
  store i32 %245, ptr %6, align 4
  %246 = load i32, ptr %6, align 4
  %247 = load i32, ptr %7, align 4
  %248 = and i32 %246, %247
  %249 = load i32, ptr %6, align 4
  %250 = xor i32 %249, -1
  %251 = load i32, ptr %8, align 4
  %252 = and i32 %250, %251
  %253 = or i32 %248, %252
  %254 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 8
  %255 = load i32, ptr %254, align 4
  %256 = add i32 %253, %255
  %257 = add i32 %256, 1770035416
  %258 = load i32, ptr %5, align 4
  %259 = add i32 %258, %257
  store i32 %259, ptr %5, align 4
  %260 = load i32, ptr %5, align 4
  %261 = shl i32 %260, 7
  %262 = load i32, ptr %5, align 4
  %263 = lshr i32 %262, 25
  %264 = or i32 %261, %263
  store i32 %264, ptr %5, align 4
  %265 = load i32, ptr %6, align 4
  %266 = load i32, ptr %5, align 4
  %267 = add i32 %266, %265
  store i32 %267, ptr %5, align 4
  %268 = load i32, ptr %5, align 4
  %269 = load i32, ptr %6, align 4
  %270 = and i32 %268, %269
  %271 = load i32, ptr %5, align 4
  %272 = xor i32 %271, -1
  %273 = load i32, ptr %7, align 4
  %274 = and i32 %272, %273
  %275 = or i32 %270, %274
  %276 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 9
  %277 = load i32, ptr %276, align 4
  %278 = add i32 %275, %277
  %279 = add i32 %278, -1958414417
  %280 = load i32, ptr %8, align 4
  %281 = add i32 %280, %279
  store i32 %281, ptr %8, align 4
  %282 = load i32, ptr %8, align 4
  %283 = shl i32 %282, 12
  %284 = load i32, ptr %8, align 4
  %285 = lshr i32 %284, 20
  %286 = or i32 %283, %285
  store i32 %286, ptr %8, align 4
  %287 = load i32, ptr %5, align 4
  %288 = load i32, ptr %8, align 4
  %289 = add i32 %288, %287
  store i32 %289, ptr %8, align 4
  %290 = load i32, ptr %8, align 4
  %291 = load i32, ptr %5, align 4
  %292 = and i32 %290, %291
  %293 = load i32, ptr %8, align 4
  %294 = xor i32 %293, -1
  %295 = load i32, ptr %6, align 4
  %296 = and i32 %294, %295
  %297 = or i32 %292, %296
  %298 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 10
  %299 = load i32, ptr %298, align 4
  %300 = add i32 %297, %299
  %301 = add i32 %300, -42063
  %302 = load i32, ptr %7, align 4
  %303 = add i32 %302, %301
  store i32 %303, ptr %7, align 4
  %304 = load i32, ptr %7, align 4
  %305 = shl i32 %304, 17
  %306 = load i32, ptr %7, align 4
  %307 = lshr i32 %306, 15
  %308 = or i32 %305, %307
  store i32 %308, ptr %7, align 4
  %309 = load i32, ptr %8, align 4
  %310 = load i32, ptr %7, align 4
  %311 = add i32 %310, %309
  store i32 %311, ptr %7, align 4
  %312 = load i32, ptr %7, align 4
  %313 = load i32, ptr %8, align 4
  %314 = and i32 %312, %313
  %315 = load i32, ptr %7, align 4
  %316 = xor i32 %315, -1
  %317 = load i32, ptr %5, align 4
  %318 = and i32 %316, %317
  %319 = or i32 %314, %318
  %320 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 11
  %321 = load i32, ptr %320, align 4
  %322 = add i32 %319, %321
  %323 = add i32 %322, -1990404162
  %324 = load i32, ptr %6, align 4
  %325 = add i32 %324, %323
  store i32 %325, ptr %6, align 4
  %326 = load i32, ptr %6, align 4
  %327 = shl i32 %326, 22
  %328 = load i32, ptr %6, align 4
  %329 = lshr i32 %328, 10
  %330 = or i32 %327, %329
  store i32 %330, ptr %6, align 4
  %331 = load i32, ptr %7, align 4
  %332 = load i32, ptr %6, align 4
  %333 = add i32 %332, %331
  store i32 %333, ptr %6, align 4
  %334 = load i32, ptr %6, align 4
  %335 = load i32, ptr %7, align 4
  %336 = and i32 %334, %335
  %337 = load i32, ptr %6, align 4
  %338 = xor i32 %337, -1
  %339 = load i32, ptr %8, align 4
  %340 = and i32 %338, %339
  %341 = or i32 %336, %340
  %342 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 12
  %343 = load i32, ptr %342, align 4
  %344 = add i32 %341, %343
  %345 = add i32 %344, 1804603682
  %346 = load i32, ptr %5, align 4
  %347 = add i32 %346, %345
  store i32 %347, ptr %5, align 4
  %348 = load i32, ptr %5, align 4
  %349 = shl i32 %348, 7
  %350 = load i32, ptr %5, align 4
  %351 = lshr i32 %350, 25
  %352 = or i32 %349, %351
  store i32 %352, ptr %5, align 4
  %353 = load i32, ptr %6, align 4
  %354 = load i32, ptr %5, align 4
  %355 = add i32 %354, %353
  store i32 %355, ptr %5, align 4
  %356 = load i32, ptr %5, align 4
  %357 = load i32, ptr %6, align 4
  %358 = and i32 %356, %357
  %359 = load i32, ptr %5, align 4
  %360 = xor i32 %359, -1
  %361 = load i32, ptr %7, align 4
  %362 = and i32 %360, %361
  %363 = or i32 %358, %362
  %364 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 13
  %365 = load i32, ptr %364, align 4
  %366 = add i32 %363, %365
  %367 = add i32 %366, -40341101
  %368 = load i32, ptr %8, align 4
  %369 = add i32 %368, %367
  store i32 %369, ptr %8, align 4
  %370 = load i32, ptr %8, align 4
  %371 = shl i32 %370, 12
  %372 = load i32, ptr %8, align 4
  %373 = lshr i32 %372, 20
  %374 = or i32 %371, %373
  store i32 %374, ptr %8, align 4
  %375 = load i32, ptr %5, align 4
  %376 = load i32, ptr %8, align 4
  %377 = add i32 %376, %375
  store i32 %377, ptr %8, align 4
  %378 = load i32, ptr %8, align 4
  %379 = load i32, ptr %5, align 4
  %380 = and i32 %378, %379
  %381 = load i32, ptr %8, align 4
  %382 = xor i32 %381, -1
  %383 = load i32, ptr %6, align 4
  %384 = and i32 %382, %383
  %385 = or i32 %380, %384
  %386 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 14
  %387 = load i32, ptr %386, align 4
  %388 = add i32 %385, %387
  %389 = add i32 %388, -1502002290
  %390 = load i32, ptr %7, align 4
  %391 = add i32 %390, %389
  store i32 %391, ptr %7, align 4
  %392 = load i32, ptr %7, align 4
  %393 = shl i32 %392, 17
  %394 = load i32, ptr %7, align 4
  %395 = lshr i32 %394, 15
  %396 = or i32 %393, %395
  store i32 %396, ptr %7, align 4
  %397 = load i32, ptr %8, align 4
  %398 = load i32, ptr %7, align 4
  %399 = add i32 %398, %397
  store i32 %399, ptr %7, align 4
  %400 = load i32, ptr %7, align 4
  %401 = load i32, ptr %8, align 4
  %402 = and i32 %400, %401
  %403 = load i32, ptr %7, align 4
  %404 = xor i32 %403, -1
  %405 = load i32, ptr %5, align 4
  %406 = and i32 %404, %405
  %407 = or i32 %402, %406
  %408 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 15
  %409 = load i32, ptr %408, align 4
  %410 = add i32 %407, %409
  %411 = add i32 %410, 1236535329
  %412 = load i32, ptr %6, align 4
  %413 = add i32 %412, %411
  store i32 %413, ptr %6, align 4
  %414 = load i32, ptr %6, align 4
  %415 = shl i32 %414, 22
  %416 = load i32, ptr %6, align 4
  %417 = lshr i32 %416, 10
  %418 = or i32 %415, %417
  store i32 %418, ptr %6, align 4
  %419 = load i32, ptr %7, align 4
  %420 = load i32, ptr %6, align 4
  %421 = add i32 %420, %419
  store i32 %421, ptr %6, align 4
  %422 = load i32, ptr %6, align 4
  %423 = load i32, ptr %8, align 4
  %424 = and i32 %422, %423
  %425 = load i32, ptr %7, align 4
  %426 = load i32, ptr %8, align 4
  %427 = xor i32 %426, -1
  %428 = and i32 %425, %427
  %429 = or i32 %424, %428
  %430 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 1
  %431 = load i32, ptr %430, align 4
  %432 = add i32 %429, %431
  %433 = add i32 %432, -165796510
  %434 = load i32, ptr %5, align 4
  %435 = add i32 %434, %433
  store i32 %435, ptr %5, align 4
  %436 = load i32, ptr %5, align 4
  %437 = shl i32 %436, 5
  %438 = load i32, ptr %5, align 4
  %439 = lshr i32 %438, 27
  %440 = or i32 %437, %439
  store i32 %440, ptr %5, align 4
  %441 = load i32, ptr %6, align 4
  %442 = load i32, ptr %5, align 4
  %443 = add i32 %442, %441
  store i32 %443, ptr %5, align 4
  %444 = load i32, ptr %5, align 4
  %445 = load i32, ptr %7, align 4
  %446 = and i32 %444, %445
  %447 = load i32, ptr %6, align 4
  %448 = load i32, ptr %7, align 4
  %449 = xor i32 %448, -1
  %450 = and i32 %447, %449
  %451 = or i32 %446, %450
  %452 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 6
  %453 = load i32, ptr %452, align 4
  %454 = add i32 %451, %453
  %455 = add i32 %454, -1069501632
  %456 = load i32, ptr %8, align 4
  %457 = add i32 %456, %455
  store i32 %457, ptr %8, align 4
  %458 = load i32, ptr %8, align 4
  %459 = shl i32 %458, 9
  %460 = load i32, ptr %8, align 4
  %461 = lshr i32 %460, 23
  %462 = or i32 %459, %461
  store i32 %462, ptr %8, align 4
  %463 = load i32, ptr %5, align 4
  %464 = load i32, ptr %8, align 4
  %465 = add i32 %464, %463
  store i32 %465, ptr %8, align 4
  %466 = load i32, ptr %8, align 4
  %467 = load i32, ptr %6, align 4
  %468 = and i32 %466, %467
  %469 = load i32, ptr %5, align 4
  %470 = load i32, ptr %6, align 4
  %471 = xor i32 %470, -1
  %472 = and i32 %469, %471
  %473 = or i32 %468, %472
  %474 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 11
  %475 = load i32, ptr %474, align 4
  %476 = add i32 %473, %475
  %477 = add i32 %476, 643717713
  %478 = load i32, ptr %7, align 4
  %479 = add i32 %478, %477
  store i32 %479, ptr %7, align 4
  %480 = load i32, ptr %7, align 4
  %481 = shl i32 %480, 14
  %482 = load i32, ptr %7, align 4
  %483 = lshr i32 %482, 18
  %484 = or i32 %481, %483
  store i32 %484, ptr %7, align 4
  %485 = load i32, ptr %8, align 4
  %486 = load i32, ptr %7, align 4
  %487 = add i32 %486, %485
  store i32 %487, ptr %7, align 4
  %488 = load i32, ptr %7, align 4
  %489 = load i32, ptr %5, align 4
  %490 = and i32 %488, %489
  %491 = load i32, ptr %8, align 4
  %492 = load i32, ptr %5, align 4
  %493 = xor i32 %492, -1
  %494 = and i32 %491, %493
  %495 = or i32 %490, %494
  %496 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 0
  %497 = load i32, ptr %496, align 4
  %498 = add i32 %495, %497
  %499 = add i32 %498, -373897302
  %500 = load i32, ptr %6, align 4
  %501 = add i32 %500, %499
  store i32 %501, ptr %6, align 4
  %502 = load i32, ptr %6, align 4
  %503 = shl i32 %502, 20
  %504 = load i32, ptr %6, align 4
  %505 = lshr i32 %504, 12
  %506 = or i32 %503, %505
  store i32 %506, ptr %6, align 4
  %507 = load i32, ptr %7, align 4
  %508 = load i32, ptr %6, align 4
  %509 = add i32 %508, %507
  store i32 %509, ptr %6, align 4
  %510 = load i32, ptr %6, align 4
  %511 = load i32, ptr %8, align 4
  %512 = and i32 %510, %511
  %513 = load i32, ptr %7, align 4
  %514 = load i32, ptr %8, align 4
  %515 = xor i32 %514, -1
  %516 = and i32 %513, %515
  %517 = or i32 %512, %516
  %518 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 5
  %519 = load i32, ptr %518, align 4
  %520 = add i32 %517, %519
  %521 = add i32 %520, -701558691
  %522 = load i32, ptr %5, align 4
  %523 = add i32 %522, %521
  store i32 %523, ptr %5, align 4
  %524 = load i32, ptr %5, align 4
  %525 = shl i32 %524, 5
  %526 = load i32, ptr %5, align 4
  %527 = lshr i32 %526, 27
  %528 = or i32 %525, %527
  store i32 %528, ptr %5, align 4
  %529 = load i32, ptr %6, align 4
  %530 = load i32, ptr %5, align 4
  %531 = add i32 %530, %529
  store i32 %531, ptr %5, align 4
  %532 = load i32, ptr %5, align 4
  %533 = load i32, ptr %7, align 4
  %534 = and i32 %532, %533
  %535 = load i32, ptr %6, align 4
  %536 = load i32, ptr %7, align 4
  %537 = xor i32 %536, -1
  %538 = and i32 %535, %537
  %539 = or i32 %534, %538
  %540 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 10
  %541 = load i32, ptr %540, align 4
  %542 = add i32 %539, %541
  %543 = add i32 %542, 38016083
  %544 = load i32, ptr %8, align 4
  %545 = add i32 %544, %543
  store i32 %545, ptr %8, align 4
  %546 = load i32, ptr %8, align 4
  %547 = shl i32 %546, 9
  %548 = load i32, ptr %8, align 4
  %549 = lshr i32 %548, 23
  %550 = or i32 %547, %549
  store i32 %550, ptr %8, align 4
  %551 = load i32, ptr %5, align 4
  %552 = load i32, ptr %8, align 4
  %553 = add i32 %552, %551
  store i32 %553, ptr %8, align 4
  %554 = load i32, ptr %8, align 4
  %555 = load i32, ptr %6, align 4
  %556 = and i32 %554, %555
  %557 = load i32, ptr %5, align 4
  %558 = load i32, ptr %6, align 4
  %559 = xor i32 %558, -1
  %560 = and i32 %557, %559
  %561 = or i32 %556, %560
  %562 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 15
  %563 = load i32, ptr %562, align 4
  %564 = add i32 %561, %563
  %565 = add i32 %564, -660478335
  %566 = load i32, ptr %7, align 4
  %567 = add i32 %566, %565
  store i32 %567, ptr %7, align 4
  %568 = load i32, ptr %7, align 4
  %569 = shl i32 %568, 14
  %570 = load i32, ptr %7, align 4
  %571 = lshr i32 %570, 18
  %572 = or i32 %569, %571
  store i32 %572, ptr %7, align 4
  %573 = load i32, ptr %8, align 4
  %574 = load i32, ptr %7, align 4
  %575 = add i32 %574, %573
  store i32 %575, ptr %7, align 4
  %576 = load i32, ptr %7, align 4
  %577 = load i32, ptr %5, align 4
  %578 = and i32 %576, %577
  %579 = load i32, ptr %8, align 4
  %580 = load i32, ptr %5, align 4
  %581 = xor i32 %580, -1
  %582 = and i32 %579, %581
  %583 = or i32 %578, %582
  %584 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 4
  %585 = load i32, ptr %584, align 4
  %586 = add i32 %583, %585
  %587 = add i32 %586, -405537848
  %588 = load i32, ptr %6, align 4
  %589 = add i32 %588, %587
  store i32 %589, ptr %6, align 4
  %590 = load i32, ptr %6, align 4
  %591 = shl i32 %590, 20
  %592 = load i32, ptr %6, align 4
  %593 = lshr i32 %592, 12
  %594 = or i32 %591, %593
  store i32 %594, ptr %6, align 4
  %595 = load i32, ptr %7, align 4
  %596 = load i32, ptr %6, align 4
  %597 = add i32 %596, %595
  store i32 %597, ptr %6, align 4
  %598 = load i32, ptr %6, align 4
  %599 = load i32, ptr %8, align 4
  %600 = and i32 %598, %599
  %601 = load i32, ptr %7, align 4
  %602 = load i32, ptr %8, align 4
  %603 = xor i32 %602, -1
  %604 = and i32 %601, %603
  %605 = or i32 %600, %604
  %606 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 9
  %607 = load i32, ptr %606, align 4
  %608 = add i32 %605, %607
  %609 = add i32 %608, 568446438
  %610 = load i32, ptr %5, align 4
  %611 = add i32 %610, %609
  store i32 %611, ptr %5, align 4
  %612 = load i32, ptr %5, align 4
  %613 = shl i32 %612, 5
  %614 = load i32, ptr %5, align 4
  %615 = lshr i32 %614, 27
  %616 = or i32 %613, %615
  store i32 %616, ptr %5, align 4
  %617 = load i32, ptr %6, align 4
  %618 = load i32, ptr %5, align 4
  %619 = add i32 %618, %617
  store i32 %619, ptr %5, align 4
  %620 = load i32, ptr %5, align 4
  %621 = load i32, ptr %7, align 4
  %622 = and i32 %620, %621
  %623 = load i32, ptr %6, align 4
  %624 = load i32, ptr %7, align 4
  %625 = xor i32 %624, -1
  %626 = and i32 %623, %625
  %627 = or i32 %622, %626
  %628 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 14
  %629 = load i32, ptr %628, align 4
  %630 = add i32 %627, %629
  %631 = add i32 %630, -1019803690
  %632 = load i32, ptr %8, align 4
  %633 = add i32 %632, %631
  store i32 %633, ptr %8, align 4
  %634 = load i32, ptr %8, align 4
  %635 = shl i32 %634, 9
  %636 = load i32, ptr %8, align 4
  %637 = lshr i32 %636, 23
  %638 = or i32 %635, %637
  store i32 %638, ptr %8, align 4
  %639 = load i32, ptr %5, align 4
  %640 = load i32, ptr %8, align 4
  %641 = add i32 %640, %639
  store i32 %641, ptr %8, align 4
  %642 = load i32, ptr %8, align 4
  %643 = load i32, ptr %6, align 4
  %644 = and i32 %642, %643
  %645 = load i32, ptr %5, align 4
  %646 = load i32, ptr %6, align 4
  %647 = xor i32 %646, -1
  %648 = and i32 %645, %647
  %649 = or i32 %644, %648
  %650 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 3
  %651 = load i32, ptr %650, align 4
  %652 = add i32 %649, %651
  %653 = add i32 %652, -187363961
  %654 = load i32, ptr %7, align 4
  %655 = add i32 %654, %653
  store i32 %655, ptr %7, align 4
  %656 = load i32, ptr %7, align 4
  %657 = shl i32 %656, 14
  %658 = load i32, ptr %7, align 4
  %659 = lshr i32 %658, 18
  %660 = or i32 %657, %659
  store i32 %660, ptr %7, align 4
  %661 = load i32, ptr %8, align 4
  %662 = load i32, ptr %7, align 4
  %663 = add i32 %662, %661
  store i32 %663, ptr %7, align 4
  %664 = load i32, ptr %7, align 4
  %665 = load i32, ptr %5, align 4
  %666 = and i32 %664, %665
  %667 = load i32, ptr %8, align 4
  %668 = load i32, ptr %5, align 4
  %669 = xor i32 %668, -1
  %670 = and i32 %667, %669
  %671 = or i32 %666, %670
  %672 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 8
  %673 = load i32, ptr %672, align 4
  %674 = add i32 %671, %673
  %675 = add i32 %674, 1163531501
  %676 = load i32, ptr %6, align 4
  %677 = add i32 %676, %675
  store i32 %677, ptr %6, align 4
  %678 = load i32, ptr %6, align 4
  %679 = shl i32 %678, 20
  %680 = load i32, ptr %6, align 4
  %681 = lshr i32 %680, 12
  %682 = or i32 %679, %681
  store i32 %682, ptr %6, align 4
  %683 = load i32, ptr %7, align 4
  %684 = load i32, ptr %6, align 4
  %685 = add i32 %684, %683
  store i32 %685, ptr %6, align 4
  %686 = load i32, ptr %6, align 4
  %687 = load i32, ptr %8, align 4
  %688 = and i32 %686, %687
  %689 = load i32, ptr %7, align 4
  %690 = load i32, ptr %8, align 4
  %691 = xor i32 %690, -1
  %692 = and i32 %689, %691
  %693 = or i32 %688, %692
  %694 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 13
  %695 = load i32, ptr %694, align 4
  %696 = add i32 %693, %695
  %697 = add i32 %696, -1444681467
  %698 = load i32, ptr %5, align 4
  %699 = add i32 %698, %697
  store i32 %699, ptr %5, align 4
  %700 = load i32, ptr %5, align 4
  %701 = shl i32 %700, 5
  %702 = load i32, ptr %5, align 4
  %703 = lshr i32 %702, 27
  %704 = or i32 %701, %703
  store i32 %704, ptr %5, align 4
  %705 = load i32, ptr %6, align 4
  %706 = load i32, ptr %5, align 4
  %707 = add i32 %706, %705
  store i32 %707, ptr %5, align 4
  %708 = load i32, ptr %5, align 4
  %709 = load i32, ptr %7, align 4
  %710 = and i32 %708, %709
  %711 = load i32, ptr %6, align 4
  %712 = load i32, ptr %7, align 4
  %713 = xor i32 %712, -1
  %714 = and i32 %711, %713
  %715 = or i32 %710, %714
  %716 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 2
  %717 = load i32, ptr %716, align 4
  %718 = add i32 %715, %717
  %719 = add i32 %718, -51403784
  %720 = load i32, ptr %8, align 4
  %721 = add i32 %720, %719
  store i32 %721, ptr %8, align 4
  %722 = load i32, ptr %8, align 4
  %723 = shl i32 %722, 9
  %724 = load i32, ptr %8, align 4
  %725 = lshr i32 %724, 23
  %726 = or i32 %723, %725
  store i32 %726, ptr %8, align 4
  %727 = load i32, ptr %5, align 4
  %728 = load i32, ptr %8, align 4
  %729 = add i32 %728, %727
  store i32 %729, ptr %8, align 4
  %730 = load i32, ptr %8, align 4
  %731 = load i32, ptr %6, align 4
  %732 = and i32 %730, %731
  %733 = load i32, ptr %5, align 4
  %734 = load i32, ptr %6, align 4
  %735 = xor i32 %734, -1
  %736 = and i32 %733, %735
  %737 = or i32 %732, %736
  %738 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 7
  %739 = load i32, ptr %738, align 4
  %740 = add i32 %737, %739
  %741 = add i32 %740, 1735328473
  %742 = load i32, ptr %7, align 4
  %743 = add i32 %742, %741
  store i32 %743, ptr %7, align 4
  %744 = load i32, ptr %7, align 4
  %745 = shl i32 %744, 14
  %746 = load i32, ptr %7, align 4
  %747 = lshr i32 %746, 18
  %748 = or i32 %745, %747
  store i32 %748, ptr %7, align 4
  %749 = load i32, ptr %8, align 4
  %750 = load i32, ptr %7, align 4
  %751 = add i32 %750, %749
  store i32 %751, ptr %7, align 4
  %752 = load i32, ptr %7, align 4
  %753 = load i32, ptr %5, align 4
  %754 = and i32 %752, %753
  %755 = load i32, ptr %8, align 4
  %756 = load i32, ptr %5, align 4
  %757 = xor i32 %756, -1
  %758 = and i32 %755, %757
  %759 = or i32 %754, %758
  %760 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 12
  %761 = load i32, ptr %760, align 4
  %762 = add i32 %759, %761
  %763 = add i32 %762, -1926607734
  %764 = load i32, ptr %6, align 4
  %765 = add i32 %764, %763
  store i32 %765, ptr %6, align 4
  %766 = load i32, ptr %6, align 4
  %767 = shl i32 %766, 20
  %768 = load i32, ptr %6, align 4
  %769 = lshr i32 %768, 12
  %770 = or i32 %767, %769
  store i32 %770, ptr %6, align 4
  %771 = load i32, ptr %7, align 4
  %772 = load i32, ptr %6, align 4
  %773 = add i32 %772, %771
  store i32 %773, ptr %6, align 4
  %774 = load i32, ptr %6, align 4
  %775 = load i32, ptr %7, align 4
  %776 = xor i32 %774, %775
  %777 = load i32, ptr %8, align 4
  %778 = xor i32 %776, %777
  %779 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 5
  %780 = load i32, ptr %779, align 4
  %781 = add i32 %778, %780
  %782 = add i32 %781, -378558
  %783 = load i32, ptr %5, align 4
  %784 = add i32 %783, %782
  store i32 %784, ptr %5, align 4
  %785 = load i32, ptr %5, align 4
  %786 = shl i32 %785, 4
  %787 = load i32, ptr %5, align 4
  %788 = lshr i32 %787, 28
  %789 = or i32 %786, %788
  store i32 %789, ptr %5, align 4
  %790 = load i32, ptr %6, align 4
  %791 = load i32, ptr %5, align 4
  %792 = add i32 %791, %790
  store i32 %792, ptr %5, align 4
  %793 = load i32, ptr %5, align 4
  %794 = load i32, ptr %6, align 4
  %795 = xor i32 %793, %794
  %796 = load i32, ptr %7, align 4
  %797 = xor i32 %795, %796
  %798 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 8
  %799 = load i32, ptr %798, align 4
  %800 = add i32 %797, %799
  %801 = add i32 %800, -2022574463
  %802 = load i32, ptr %8, align 4
  %803 = add i32 %802, %801
  store i32 %803, ptr %8, align 4
  %804 = load i32, ptr %8, align 4
  %805 = shl i32 %804, 11
  %806 = load i32, ptr %8, align 4
  %807 = lshr i32 %806, 21
  %808 = or i32 %805, %807
  store i32 %808, ptr %8, align 4
  %809 = load i32, ptr %5, align 4
  %810 = load i32, ptr %8, align 4
  %811 = add i32 %810, %809
  store i32 %811, ptr %8, align 4
  %812 = load i32, ptr %8, align 4
  %813 = load i32, ptr %5, align 4
  %814 = xor i32 %812, %813
  %815 = load i32, ptr %6, align 4
  %816 = xor i32 %814, %815
  %817 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 11
  %818 = load i32, ptr %817, align 4
  %819 = add i32 %816, %818
  %820 = add i32 %819, 1839030562
  %821 = load i32, ptr %7, align 4
  %822 = add i32 %821, %820
  store i32 %822, ptr %7, align 4
  %823 = load i32, ptr %7, align 4
  %824 = shl i32 %823, 16
  %825 = load i32, ptr %7, align 4
  %826 = lshr i32 %825, 16
  %827 = or i32 %824, %826
  store i32 %827, ptr %7, align 4
  %828 = load i32, ptr %8, align 4
  %829 = load i32, ptr %7, align 4
  %830 = add i32 %829, %828
  store i32 %830, ptr %7, align 4
  %831 = load i32, ptr %7, align 4
  %832 = load i32, ptr %8, align 4
  %833 = xor i32 %831, %832
  %834 = load i32, ptr %5, align 4
  %835 = xor i32 %833, %834
  %836 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 14
  %837 = load i32, ptr %836, align 4
  %838 = add i32 %835, %837
  %839 = add i32 %838, -35309556
  %840 = load i32, ptr %6, align 4
  %841 = add i32 %840, %839
  store i32 %841, ptr %6, align 4
  %842 = load i32, ptr %6, align 4
  %843 = shl i32 %842, 23
  %844 = load i32, ptr %6, align 4
  %845 = lshr i32 %844, 9
  %846 = or i32 %843, %845
  store i32 %846, ptr %6, align 4
  %847 = load i32, ptr %7, align 4
  %848 = load i32, ptr %6, align 4
  %849 = add i32 %848, %847
  store i32 %849, ptr %6, align 4
  %850 = load i32, ptr %6, align 4
  %851 = load i32, ptr %7, align 4
  %852 = xor i32 %850, %851
  %853 = load i32, ptr %8, align 4
  %854 = xor i32 %852, %853
  %855 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 1
  %856 = load i32, ptr %855, align 4
  %857 = add i32 %854, %856
  %858 = add i32 %857, -1530992060
  %859 = load i32, ptr %5, align 4
  %860 = add i32 %859, %858
  store i32 %860, ptr %5, align 4
  %861 = load i32, ptr %5, align 4
  %862 = shl i32 %861, 4
  %863 = load i32, ptr %5, align 4
  %864 = lshr i32 %863, 28
  %865 = or i32 %862, %864
  store i32 %865, ptr %5, align 4
  %866 = load i32, ptr %6, align 4
  %867 = load i32, ptr %5, align 4
  %868 = add i32 %867, %866
  store i32 %868, ptr %5, align 4
  %869 = load i32, ptr %5, align 4
  %870 = load i32, ptr %6, align 4
  %871 = xor i32 %869, %870
  %872 = load i32, ptr %7, align 4
  %873 = xor i32 %871, %872
  %874 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 4
  %875 = load i32, ptr %874, align 4
  %876 = add i32 %873, %875
  %877 = add i32 %876, 1272893353
  %878 = load i32, ptr %8, align 4
  %879 = add i32 %878, %877
  store i32 %879, ptr %8, align 4
  %880 = load i32, ptr %8, align 4
  %881 = shl i32 %880, 11
  %882 = load i32, ptr %8, align 4
  %883 = lshr i32 %882, 21
  %884 = or i32 %881, %883
  store i32 %884, ptr %8, align 4
  %885 = load i32, ptr %5, align 4
  %886 = load i32, ptr %8, align 4
  %887 = add i32 %886, %885
  store i32 %887, ptr %8, align 4
  %888 = load i32, ptr %8, align 4
  %889 = load i32, ptr %5, align 4
  %890 = xor i32 %888, %889
  %891 = load i32, ptr %6, align 4
  %892 = xor i32 %890, %891
  %893 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 7
  %894 = load i32, ptr %893, align 4
  %895 = add i32 %892, %894
  %896 = add i32 %895, -155497632
  %897 = load i32, ptr %7, align 4
  %898 = add i32 %897, %896
  store i32 %898, ptr %7, align 4
  %899 = load i32, ptr %7, align 4
  %900 = shl i32 %899, 16
  %901 = load i32, ptr %7, align 4
  %902 = lshr i32 %901, 16
  %903 = or i32 %900, %902
  store i32 %903, ptr %7, align 4
  %904 = load i32, ptr %8, align 4
  %905 = load i32, ptr %7, align 4
  %906 = add i32 %905, %904
  store i32 %906, ptr %7, align 4
  %907 = load i32, ptr %7, align 4
  %908 = load i32, ptr %8, align 4
  %909 = xor i32 %907, %908
  %910 = load i32, ptr %5, align 4
  %911 = xor i32 %909, %910
  %912 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 10
  %913 = load i32, ptr %912, align 4
  %914 = add i32 %911, %913
  %915 = add i32 %914, -1094730640
  %916 = load i32, ptr %6, align 4
  %917 = add i32 %916, %915
  store i32 %917, ptr %6, align 4
  %918 = load i32, ptr %6, align 4
  %919 = shl i32 %918, 23
  %920 = load i32, ptr %6, align 4
  %921 = lshr i32 %920, 9
  %922 = or i32 %919, %921
  store i32 %922, ptr %6, align 4
  %923 = load i32, ptr %7, align 4
  %924 = load i32, ptr %6, align 4
  %925 = add i32 %924, %923
  store i32 %925, ptr %6, align 4
  %926 = load i32, ptr %6, align 4
  %927 = load i32, ptr %7, align 4
  %928 = xor i32 %926, %927
  %929 = load i32, ptr %8, align 4
  %930 = xor i32 %928, %929
  %931 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 13
  %932 = load i32, ptr %931, align 4
  %933 = add i32 %930, %932
  %934 = add i32 %933, 681279174
  %935 = load i32, ptr %5, align 4
  %936 = add i32 %935, %934
  store i32 %936, ptr %5, align 4
  %937 = load i32, ptr %5, align 4
  %938 = shl i32 %937, 4
  %939 = load i32, ptr %5, align 4
  %940 = lshr i32 %939, 28
  %941 = or i32 %938, %940
  store i32 %941, ptr %5, align 4
  %942 = load i32, ptr %6, align 4
  %943 = load i32, ptr %5, align 4
  %944 = add i32 %943, %942
  store i32 %944, ptr %5, align 4
  %945 = load i32, ptr %5, align 4
  %946 = load i32, ptr %6, align 4
  %947 = xor i32 %945, %946
  %948 = load i32, ptr %7, align 4
  %949 = xor i32 %947, %948
  %950 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 0
  %951 = load i32, ptr %950, align 4
  %952 = add i32 %949, %951
  %953 = add i32 %952, -358537222
  %954 = load i32, ptr %8, align 4
  %955 = add i32 %954, %953
  store i32 %955, ptr %8, align 4
  %956 = load i32, ptr %8, align 4
  %957 = shl i32 %956, 11
  %958 = load i32, ptr %8, align 4
  %959 = lshr i32 %958, 21
  %960 = or i32 %957, %959
  store i32 %960, ptr %8, align 4
  %961 = load i32, ptr %5, align 4
  %962 = load i32, ptr %8, align 4
  %963 = add i32 %962, %961
  store i32 %963, ptr %8, align 4
  %964 = load i32, ptr %8, align 4
  %965 = load i32, ptr %5, align 4
  %966 = xor i32 %964, %965
  %967 = load i32, ptr %6, align 4
  %968 = xor i32 %966, %967
  %969 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 3
  %970 = load i32, ptr %969, align 4
  %971 = add i32 %968, %970
  %972 = add i32 %971, -722521979
  %973 = load i32, ptr %7, align 4
  %974 = add i32 %973, %972
  store i32 %974, ptr %7, align 4
  %975 = load i32, ptr %7, align 4
  %976 = shl i32 %975, 16
  %977 = load i32, ptr %7, align 4
  %978 = lshr i32 %977, 16
  %979 = or i32 %976, %978
  store i32 %979, ptr %7, align 4
  %980 = load i32, ptr %8, align 4
  %981 = load i32, ptr %7, align 4
  %982 = add i32 %981, %980
  store i32 %982, ptr %7, align 4
  %983 = load i32, ptr %7, align 4
  %984 = load i32, ptr %8, align 4
  %985 = xor i32 %983, %984
  %986 = load i32, ptr %5, align 4
  %987 = xor i32 %985, %986
  %988 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 6
  %989 = load i32, ptr %988, align 4
  %990 = add i32 %987, %989
  %991 = add i32 %990, 76029189
  %992 = load i32, ptr %6, align 4
  %993 = add i32 %992, %991
  store i32 %993, ptr %6, align 4
  %994 = load i32, ptr %6, align 4
  %995 = shl i32 %994, 23
  %996 = load i32, ptr %6, align 4
  %997 = lshr i32 %996, 9
  %998 = or i32 %995, %997
  store i32 %998, ptr %6, align 4
  %999 = load i32, ptr %7, align 4
  %1000 = load i32, ptr %6, align 4
  %1001 = add i32 %1000, %999
  store i32 %1001, ptr %6, align 4
  %1002 = load i32, ptr %6, align 4
  %1003 = load i32, ptr %7, align 4
  %1004 = xor i32 %1002, %1003
  %1005 = load i32, ptr %8, align 4
  %1006 = xor i32 %1004, %1005
  %1007 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 9
  %1008 = load i32, ptr %1007, align 4
  %1009 = add i32 %1006, %1008
  %1010 = add i32 %1009, -640364487
  %1011 = load i32, ptr %5, align 4
  %1012 = add i32 %1011, %1010
  store i32 %1012, ptr %5, align 4
  %1013 = load i32, ptr %5, align 4
  %1014 = shl i32 %1013, 4
  %1015 = load i32, ptr %5, align 4
  %1016 = lshr i32 %1015, 28
  %1017 = or i32 %1014, %1016
  store i32 %1017, ptr %5, align 4
  %1018 = load i32, ptr %6, align 4
  %1019 = load i32, ptr %5, align 4
  %1020 = add i32 %1019, %1018
  store i32 %1020, ptr %5, align 4
  %1021 = load i32, ptr %5, align 4
  %1022 = load i32, ptr %6, align 4
  %1023 = xor i32 %1021, %1022
  %1024 = load i32, ptr %7, align 4
  %1025 = xor i32 %1023, %1024
  %1026 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 12
  %1027 = load i32, ptr %1026, align 4
  %1028 = add i32 %1025, %1027
  %1029 = add i32 %1028, -421815835
  %1030 = load i32, ptr %8, align 4
  %1031 = add i32 %1030, %1029
  store i32 %1031, ptr %8, align 4
  %1032 = load i32, ptr %8, align 4
  %1033 = shl i32 %1032, 11
  %1034 = load i32, ptr %8, align 4
  %1035 = lshr i32 %1034, 21
  %1036 = or i32 %1033, %1035
  store i32 %1036, ptr %8, align 4
  %1037 = load i32, ptr %5, align 4
  %1038 = load i32, ptr %8, align 4
  %1039 = add i32 %1038, %1037
  store i32 %1039, ptr %8, align 4
  %1040 = load i32, ptr %8, align 4
  %1041 = load i32, ptr %5, align 4
  %1042 = xor i32 %1040, %1041
  %1043 = load i32, ptr %6, align 4
  %1044 = xor i32 %1042, %1043
  %1045 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 15
  %1046 = load i32, ptr %1045, align 4
  %1047 = add i32 %1044, %1046
  %1048 = add i32 %1047, 530742520
  %1049 = load i32, ptr %7, align 4
  %1050 = add i32 %1049, %1048
  store i32 %1050, ptr %7, align 4
  %1051 = load i32, ptr %7, align 4
  %1052 = shl i32 %1051, 16
  %1053 = load i32, ptr %7, align 4
  %1054 = lshr i32 %1053, 16
  %1055 = or i32 %1052, %1054
  store i32 %1055, ptr %7, align 4
  %1056 = load i32, ptr %8, align 4
  %1057 = load i32, ptr %7, align 4
  %1058 = add i32 %1057, %1056
  store i32 %1058, ptr %7, align 4
  %1059 = load i32, ptr %7, align 4
  %1060 = load i32, ptr %8, align 4
  %1061 = xor i32 %1059, %1060
  %1062 = load i32, ptr %5, align 4
  %1063 = xor i32 %1061, %1062
  %1064 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 2
  %1065 = load i32, ptr %1064, align 4
  %1066 = add i32 %1063, %1065
  %1067 = add i32 %1066, -995338651
  %1068 = load i32, ptr %6, align 4
  %1069 = add i32 %1068, %1067
  store i32 %1069, ptr %6, align 4
  %1070 = load i32, ptr %6, align 4
  %1071 = shl i32 %1070, 23
  %1072 = load i32, ptr %6, align 4
  %1073 = lshr i32 %1072, 9
  %1074 = or i32 %1071, %1073
  store i32 %1074, ptr %6, align 4
  %1075 = load i32, ptr %7, align 4
  %1076 = load i32, ptr %6, align 4
  %1077 = add i32 %1076, %1075
  store i32 %1077, ptr %6, align 4
  %1078 = load i32, ptr %7, align 4
  %1079 = load i32, ptr %6, align 4
  %1080 = load i32, ptr %8, align 4
  %1081 = xor i32 %1080, -1
  %1082 = or i32 %1079, %1081
  %1083 = xor i32 %1078, %1082
  %1084 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 0
  %1085 = load i32, ptr %1084, align 4
  %1086 = add i32 %1083, %1085
  %1087 = add i32 %1086, -198630844
  %1088 = load i32, ptr %5, align 4
  %1089 = add i32 %1088, %1087
  store i32 %1089, ptr %5, align 4
  %1090 = load i32, ptr %5, align 4
  %1091 = shl i32 %1090, 6
  %1092 = load i32, ptr %5, align 4
  %1093 = lshr i32 %1092, 26
  %1094 = or i32 %1091, %1093
  store i32 %1094, ptr %5, align 4
  %1095 = load i32, ptr %6, align 4
  %1096 = load i32, ptr %5, align 4
  %1097 = add i32 %1096, %1095
  store i32 %1097, ptr %5, align 4
  %1098 = load i32, ptr %6, align 4
  %1099 = load i32, ptr %5, align 4
  %1100 = load i32, ptr %7, align 4
  %1101 = xor i32 %1100, -1
  %1102 = or i32 %1099, %1101
  %1103 = xor i32 %1098, %1102
  %1104 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 7
  %1105 = load i32, ptr %1104, align 4
  %1106 = add i32 %1103, %1105
  %1107 = add i32 %1106, 1126891415
  %1108 = load i32, ptr %8, align 4
  %1109 = add i32 %1108, %1107
  store i32 %1109, ptr %8, align 4
  %1110 = load i32, ptr %8, align 4
  %1111 = shl i32 %1110, 10
  %1112 = load i32, ptr %8, align 4
  %1113 = lshr i32 %1112, 22
  %1114 = or i32 %1111, %1113
  store i32 %1114, ptr %8, align 4
  %1115 = load i32, ptr %5, align 4
  %1116 = load i32, ptr %8, align 4
  %1117 = add i32 %1116, %1115
  store i32 %1117, ptr %8, align 4
  %1118 = load i32, ptr %5, align 4
  %1119 = load i32, ptr %8, align 4
  %1120 = load i32, ptr %6, align 4
  %1121 = xor i32 %1120, -1
  %1122 = or i32 %1119, %1121
  %1123 = xor i32 %1118, %1122
  %1124 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 14
  %1125 = load i32, ptr %1124, align 4
  %1126 = add i32 %1123, %1125
  %1127 = add i32 %1126, -1416354905
  %1128 = load i32, ptr %7, align 4
  %1129 = add i32 %1128, %1127
  store i32 %1129, ptr %7, align 4
  %1130 = load i32, ptr %7, align 4
  %1131 = shl i32 %1130, 15
  %1132 = load i32, ptr %7, align 4
  %1133 = lshr i32 %1132, 17
  %1134 = or i32 %1131, %1133
  store i32 %1134, ptr %7, align 4
  %1135 = load i32, ptr %8, align 4
  %1136 = load i32, ptr %7, align 4
  %1137 = add i32 %1136, %1135
  store i32 %1137, ptr %7, align 4
  %1138 = load i32, ptr %8, align 4
  %1139 = load i32, ptr %7, align 4
  %1140 = load i32, ptr %5, align 4
  %1141 = xor i32 %1140, -1
  %1142 = or i32 %1139, %1141
  %1143 = xor i32 %1138, %1142
  %1144 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 5
  %1145 = load i32, ptr %1144, align 4
  %1146 = add i32 %1143, %1145
  %1147 = add i32 %1146, -57434055
  %1148 = load i32, ptr %6, align 4
  %1149 = add i32 %1148, %1147
  store i32 %1149, ptr %6, align 4
  %1150 = load i32, ptr %6, align 4
  %1151 = shl i32 %1150, 21
  %1152 = load i32, ptr %6, align 4
  %1153 = lshr i32 %1152, 11
  %1154 = or i32 %1151, %1153
  store i32 %1154, ptr %6, align 4
  %1155 = load i32, ptr %7, align 4
  %1156 = load i32, ptr %6, align 4
  %1157 = add i32 %1156, %1155
  store i32 %1157, ptr %6, align 4
  %1158 = load i32, ptr %7, align 4
  %1159 = load i32, ptr %6, align 4
  %1160 = load i32, ptr %8, align 4
  %1161 = xor i32 %1160, -1
  %1162 = or i32 %1159, %1161
  %1163 = xor i32 %1158, %1162
  %1164 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 12
  %1165 = load i32, ptr %1164, align 4
  %1166 = add i32 %1163, %1165
  %1167 = add i32 %1166, 1700485571
  %1168 = load i32, ptr %5, align 4
  %1169 = add i32 %1168, %1167
  store i32 %1169, ptr %5, align 4
  %1170 = load i32, ptr %5, align 4
  %1171 = shl i32 %1170, 6
  %1172 = load i32, ptr %5, align 4
  %1173 = lshr i32 %1172, 26
  %1174 = or i32 %1171, %1173
  store i32 %1174, ptr %5, align 4
  %1175 = load i32, ptr %6, align 4
  %1176 = load i32, ptr %5, align 4
  %1177 = add i32 %1176, %1175
  store i32 %1177, ptr %5, align 4
  %1178 = load i32, ptr %6, align 4
  %1179 = load i32, ptr %5, align 4
  %1180 = load i32, ptr %7, align 4
  %1181 = xor i32 %1180, -1
  %1182 = or i32 %1179, %1181
  %1183 = xor i32 %1178, %1182
  %1184 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 3
  %1185 = load i32, ptr %1184, align 4
  %1186 = add i32 %1183, %1185
  %1187 = add i32 %1186, -1894986606
  %1188 = load i32, ptr %8, align 4
  %1189 = add i32 %1188, %1187
  store i32 %1189, ptr %8, align 4
  %1190 = load i32, ptr %8, align 4
  %1191 = shl i32 %1190, 10
  %1192 = load i32, ptr %8, align 4
  %1193 = lshr i32 %1192, 22
  %1194 = or i32 %1191, %1193
  store i32 %1194, ptr %8, align 4
  %1195 = load i32, ptr %5, align 4
  %1196 = load i32, ptr %8, align 4
  %1197 = add i32 %1196, %1195
  store i32 %1197, ptr %8, align 4
  %1198 = load i32, ptr %5, align 4
  %1199 = load i32, ptr %8, align 4
  %1200 = load i32, ptr %6, align 4
  %1201 = xor i32 %1200, -1
  %1202 = or i32 %1199, %1201
  %1203 = xor i32 %1198, %1202
  %1204 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 10
  %1205 = load i32, ptr %1204, align 4
  %1206 = add i32 %1203, %1205
  %1207 = add i32 %1206, -1051523
  %1208 = load i32, ptr %7, align 4
  %1209 = add i32 %1208, %1207
  store i32 %1209, ptr %7, align 4
  %1210 = load i32, ptr %7, align 4
  %1211 = shl i32 %1210, 15
  %1212 = load i32, ptr %7, align 4
  %1213 = lshr i32 %1212, 17
  %1214 = or i32 %1211, %1213
  store i32 %1214, ptr %7, align 4
  %1215 = load i32, ptr %8, align 4
  %1216 = load i32, ptr %7, align 4
  %1217 = add i32 %1216, %1215
  store i32 %1217, ptr %7, align 4
  %1218 = load i32, ptr %8, align 4
  %1219 = load i32, ptr %7, align 4
  %1220 = load i32, ptr %5, align 4
  %1221 = xor i32 %1220, -1
  %1222 = or i32 %1219, %1221
  %1223 = xor i32 %1218, %1222
  %1224 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 1
  %1225 = load i32, ptr %1224, align 4
  %1226 = add i32 %1223, %1225
  %1227 = add i32 %1226, -2054922799
  %1228 = load i32, ptr %6, align 4
  %1229 = add i32 %1228, %1227
  store i32 %1229, ptr %6, align 4
  %1230 = load i32, ptr %6, align 4
  %1231 = shl i32 %1230, 21
  %1232 = load i32, ptr %6, align 4
  %1233 = lshr i32 %1232, 11
  %1234 = or i32 %1231, %1233
  store i32 %1234, ptr %6, align 4
  %1235 = load i32, ptr %7, align 4
  %1236 = load i32, ptr %6, align 4
  %1237 = add i32 %1236, %1235
  store i32 %1237, ptr %6, align 4
  %1238 = load i32, ptr %7, align 4
  %1239 = load i32, ptr %6, align 4
  %1240 = load i32, ptr %8, align 4
  %1241 = xor i32 %1240, -1
  %1242 = or i32 %1239, %1241
  %1243 = xor i32 %1238, %1242
  %1244 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 8
  %1245 = load i32, ptr %1244, align 4
  %1246 = add i32 %1243, %1245
  %1247 = add i32 %1246, 1873313359
  %1248 = load i32, ptr %5, align 4
  %1249 = add i32 %1248, %1247
  store i32 %1249, ptr %5, align 4
  %1250 = load i32, ptr %5, align 4
  %1251 = shl i32 %1250, 6
  %1252 = load i32, ptr %5, align 4
  %1253 = lshr i32 %1252, 26
  %1254 = or i32 %1251, %1253
  store i32 %1254, ptr %5, align 4
  %1255 = load i32, ptr %6, align 4
  %1256 = load i32, ptr %5, align 4
  %1257 = add i32 %1256, %1255
  store i32 %1257, ptr %5, align 4
  %1258 = load i32, ptr %6, align 4
  %1259 = load i32, ptr %5, align 4
  %1260 = load i32, ptr %7, align 4
  %1261 = xor i32 %1260, -1
  %1262 = or i32 %1259, %1261
  %1263 = xor i32 %1258, %1262
  %1264 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 15
  %1265 = load i32, ptr %1264, align 4
  %1266 = add i32 %1263, %1265
  %1267 = add i32 %1266, -30611744
  %1268 = load i32, ptr %8, align 4
  %1269 = add i32 %1268, %1267
  store i32 %1269, ptr %8, align 4
  %1270 = load i32, ptr %8, align 4
  %1271 = shl i32 %1270, 10
  %1272 = load i32, ptr %8, align 4
  %1273 = lshr i32 %1272, 22
  %1274 = or i32 %1271, %1273
  store i32 %1274, ptr %8, align 4
  %1275 = load i32, ptr %5, align 4
  %1276 = load i32, ptr %8, align 4
  %1277 = add i32 %1276, %1275
  store i32 %1277, ptr %8, align 4
  %1278 = load i32, ptr %5, align 4
  %1279 = load i32, ptr %8, align 4
  %1280 = load i32, ptr %6, align 4
  %1281 = xor i32 %1280, -1
  %1282 = or i32 %1279, %1281
  %1283 = xor i32 %1278, %1282
  %1284 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 6
  %1285 = load i32, ptr %1284, align 4
  %1286 = add i32 %1283, %1285
  %1287 = add i32 %1286, -1560198380
  %1288 = load i32, ptr %7, align 4
  %1289 = add i32 %1288, %1287
  store i32 %1289, ptr %7, align 4
  %1290 = load i32, ptr %7, align 4
  %1291 = shl i32 %1290, 15
  %1292 = load i32, ptr %7, align 4
  %1293 = lshr i32 %1292, 17
  %1294 = or i32 %1291, %1293
  store i32 %1294, ptr %7, align 4
  %1295 = load i32, ptr %8, align 4
  %1296 = load i32, ptr %7, align 4
  %1297 = add i32 %1296, %1295
  store i32 %1297, ptr %7, align 4
  %1298 = load i32, ptr %8, align 4
  %1299 = load i32, ptr %7, align 4
  %1300 = load i32, ptr %5, align 4
  %1301 = xor i32 %1300, -1
  %1302 = or i32 %1299, %1301
  %1303 = xor i32 %1298, %1302
  %1304 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 13
  %1305 = load i32, ptr %1304, align 4
  %1306 = add i32 %1303, %1305
  %1307 = add i32 %1306, 1309151649
  %1308 = load i32, ptr %6, align 4
  %1309 = add i32 %1308, %1307
  store i32 %1309, ptr %6, align 4
  %1310 = load i32, ptr %6, align 4
  %1311 = shl i32 %1310, 21
  %1312 = load i32, ptr %6, align 4
  %1313 = lshr i32 %1312, 11
  %1314 = or i32 %1311, %1313
  store i32 %1314, ptr %6, align 4
  %1315 = load i32, ptr %7, align 4
  %1316 = load i32, ptr %6, align 4
  %1317 = add i32 %1316, %1315
  store i32 %1317, ptr %6, align 4
  %1318 = load i32, ptr %7, align 4
  %1319 = load i32, ptr %6, align 4
  %1320 = load i32, ptr %8, align 4
  %1321 = xor i32 %1320, -1
  %1322 = or i32 %1319, %1321
  %1323 = xor i32 %1318, %1322
  %1324 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 4
  %1325 = load i32, ptr %1324, align 4
  %1326 = add i32 %1323, %1325
  %1327 = add i32 %1326, -145523070
  %1328 = load i32, ptr %5, align 4
  %1329 = add i32 %1328, %1327
  store i32 %1329, ptr %5, align 4
  %1330 = load i32, ptr %5, align 4
  %1331 = shl i32 %1330, 6
  %1332 = load i32, ptr %5, align 4
  %1333 = lshr i32 %1332, 26
  %1334 = or i32 %1331, %1333
  store i32 %1334, ptr %5, align 4
  %1335 = load i32, ptr %6, align 4
  %1336 = load i32, ptr %5, align 4
  %1337 = add i32 %1336, %1335
  store i32 %1337, ptr %5, align 4
  %1338 = load i32, ptr %6, align 4
  %1339 = load i32, ptr %5, align 4
  %1340 = load i32, ptr %7, align 4
  %1341 = xor i32 %1340, -1
  %1342 = or i32 %1339, %1341
  %1343 = xor i32 %1338, %1342
  %1344 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 11
  %1345 = load i32, ptr %1344, align 4
  %1346 = add i32 %1343, %1345
  %1347 = add i32 %1346, -1120210379
  %1348 = load i32, ptr %8, align 4
  %1349 = add i32 %1348, %1347
  store i32 %1349, ptr %8, align 4
  %1350 = load i32, ptr %8, align 4
  %1351 = shl i32 %1350, 10
  %1352 = load i32, ptr %8, align 4
  %1353 = lshr i32 %1352, 22
  %1354 = or i32 %1351, %1353
  store i32 %1354, ptr %8, align 4
  %1355 = load i32, ptr %5, align 4
  %1356 = load i32, ptr %8, align 4
  %1357 = add i32 %1356, %1355
  store i32 %1357, ptr %8, align 4
  %1358 = load i32, ptr %5, align 4
  %1359 = load i32, ptr %8, align 4
  %1360 = load i32, ptr %6, align 4
  %1361 = xor i32 %1360, -1
  %1362 = or i32 %1359, %1361
  %1363 = xor i32 %1358, %1362
  %1364 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 2
  %1365 = load i32, ptr %1364, align 4
  %1366 = add i32 %1363, %1365
  %1367 = add i32 %1366, 718787259
  %1368 = load i32, ptr %7, align 4
  %1369 = add i32 %1368, %1367
  store i32 %1369, ptr %7, align 4
  %1370 = load i32, ptr %7, align 4
  %1371 = shl i32 %1370, 15
  %1372 = load i32, ptr %7, align 4
  %1373 = lshr i32 %1372, 17
  %1374 = or i32 %1371, %1373
  store i32 %1374, ptr %7, align 4
  %1375 = load i32, ptr %8, align 4
  %1376 = load i32, ptr %7, align 4
  %1377 = add i32 %1376, %1375
  store i32 %1377, ptr %7, align 4
  %1378 = load i32, ptr %8, align 4
  %1379 = load i32, ptr %7, align 4
  %1380 = load i32, ptr %5, align 4
  %1381 = xor i32 %1380, -1
  %1382 = or i32 %1379, %1381
  %1383 = xor i32 %1378, %1382
  %1384 = getelementptr inbounds [16 x i32], ptr %9, i64 0, i64 9
  %1385 = load i32, ptr %1384, align 4
  %1386 = add i32 %1383, %1385
  %1387 = add i32 %1386, -343485551
  %1388 = load i32, ptr %6, align 4
  %1389 = add i32 %1388, %1387
  store i32 %1389, ptr %6, align 4
  %1390 = load i32, ptr %6, align 4
  %1391 = shl i32 %1390, 21
  %1392 = load i32, ptr %6, align 4
  %1393 = lshr i32 %1392, 11
  %1394 = or i32 %1391, %1393
  store i32 %1394, ptr %6, align 4
  %1395 = load i32, ptr %7, align 4
  %1396 = load i32, ptr %6, align 4
  %1397 = add i32 %1396, %1395
  store i32 %1397, ptr %6, align 4
  %1398 = load i32, ptr %5, align 4
  %1399 = load ptr, ptr %3, align 8
  %1400 = getelementptr inbounds i32, ptr %1399, i64 0
  %1401 = load i32, ptr %1400, align 4
  %1402 = add i32 %1401, %1398
  store i32 %1402, ptr %1400, align 4
  %1403 = load i32, ptr %6, align 4
  %1404 = load ptr, ptr %3, align 8
  %1405 = getelementptr inbounds i32, ptr %1404, i64 1
  %1406 = load i32, ptr %1405, align 4
  %1407 = add i32 %1406, %1403
  store i32 %1407, ptr %1405, align 4
  %1408 = load i32, ptr %7, align 4
  %1409 = load ptr, ptr %3, align 8
  %1410 = getelementptr inbounds i32, ptr %1409, i64 2
  %1411 = load i32, ptr %1410, align 4
  %1412 = add i32 %1411, %1408
  store i32 %1412, ptr %1410, align 4
  %1413 = load i32, ptr %8, align 4
  %1414 = load ptr, ptr %3, align 8
  %1415 = getelementptr inbounds i32, ptr %1414, i64 3
  %1416 = load i32, ptr %1415, align 4
  %1417 = add i32 %1416, %1413
  store i32 %1417, ptr %1415, align 4
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @ep_md5_update(ptr noundef %0, ptr noundef %1, i64 noundef %2) #0 {
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca i32, align 4
  %8 = alloca i32, align 4
  %9 = alloca i32, align 4
  store ptr %0, ptr %4, align 8
  store ptr %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  store i32 0, ptr %7, align 4
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %10, i32 0, i32 0
  %12 = getelementptr inbounds [2 x i32], ptr %11, i64 0, i64 0
  %13 = load i32, ptr %12, align 4
  %14 = lshr i32 %13, 3
  %15 = and i32 %14, 63
  store i32 %15, ptr %8, align 4
  %16 = load i32, ptr %8, align 4
  %17 = sub i32 64, %16
  store i32 %17, ptr %9, align 4
  %18 = load i64, ptr %6, align 8
  %19 = shl i64 %18, 3
  %20 = load ptr, ptr %4, align 8
  %21 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %20, i32 0, i32 0
  %22 = getelementptr inbounds [2 x i32], ptr %21, i64 0, i64 0
  %23 = load i32, ptr %22, align 4
  %24 = zext i32 %23 to i64
  %25 = add i64 %24, %19
  %26 = trunc i64 %25 to i32
  store i32 %26, ptr %22, align 4
  %27 = load ptr, ptr %4, align 8
  %28 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %27, i32 0, i32 0
  %29 = getelementptr inbounds [2 x i32], ptr %28, i64 0, i64 0
  %30 = load i32, ptr %29, align 4
  %31 = zext i32 %30 to i64
  %32 = load i64, ptr %6, align 8
  %33 = shl i64 %32, 3
  %34 = icmp ult i64 %31, %33
  br i1 %34, label %35, label %41

35:                                               ; preds = %3
  %36 = load ptr, ptr %4, align 8
  %37 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %36, i32 0, i32 0
  %38 = getelementptr inbounds [2 x i32], ptr %37, i64 0, i64 1
  %39 = load i32, ptr %38, align 4
  %40 = add i32 %39, 1
  store i32 %40, ptr %38, align 4
  br label %41

41:                                               ; preds = %35, %3
  %42 = load i64, ptr %6, align 8
  %43 = lshr i64 %42, 29
  %44 = load ptr, ptr %4, align 8
  %45 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %44, i32 0, i32 0
  %46 = getelementptr inbounds [2 x i32], ptr %45, i64 0, i64 1
  %47 = load i32, ptr %46, align 4
  %48 = zext i32 %47 to i64
  %49 = add i64 %48, %43
  %50 = trunc i64 %49 to i32
  store i32 %50, ptr %46, align 4
  %51 = load i64, ptr %6, align 8
  %52 = load i32, ptr %9, align 4
  %53 = zext i32 %52 to i64
  %54 = icmp uge i64 %51, %53
  br i1 %54, label %55, label %96

55:                                               ; preds = %41
  %56 = load ptr, ptr %4, align 8
  %57 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %56, i32 0, i32 2
  %58 = load i32, ptr %8, align 4
  %59 = zext i32 %58 to i64
  %60 = getelementptr inbounds [64 x i8], ptr %57, i64 0, i64 %59
  %61 = load ptr, ptr %5, align 8
  %62 = load i32, ptr %9, align 4
  %63 = zext i32 %62 to i64
  %64 = load ptr, ptr %4, align 8
  %65 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %64, i32 0, i32 2
  %66 = load i32, ptr %8, align 4
  %67 = zext i32 %66 to i64
  %68 = getelementptr inbounds [64 x i8], ptr %65, i64 0, i64 %67
  %69 = call i64 @llvm.objectsize.i64.p0(ptr %68, i1 false, i1 true, i1 false)
  %70 = call ptr @__memcpy_chk(ptr noundef %60, ptr noundef %61, i64 noundef %63, i64 noundef %69) #13
  %71 = load ptr, ptr %4, align 8
  %72 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %71, i32 0, i32 1
  %73 = getelementptr inbounds [4 x i32], ptr %72, i64 0, i64 0
  %74 = load ptr, ptr %4, align 8
  %75 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %74, i32 0, i32 2
  %76 = getelementptr inbounds [64 x i8], ptr %75, i64 0, i64 0
  call void @ep_md5_transform(ptr noundef %73, ptr noundef %76)
  %77 = load i32, ptr %9, align 4
  store i32 %77, ptr %7, align 4
  br label %78

78:                                               ; preds = %92, %55
  %79 = load i32, ptr %7, align 4
  %80 = add i32 %79, 63
  %81 = zext i32 %80 to i64
  %82 = load i64, ptr %6, align 8
  %83 = icmp ult i64 %81, %82
  br i1 %83, label %84, label %95

84:                                               ; preds = %78
  %85 = load ptr, ptr %4, align 8
  %86 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %85, i32 0, i32 1
  %87 = getelementptr inbounds [4 x i32], ptr %86, i64 0, i64 0
  %88 = load ptr, ptr %5, align 8
  %89 = load i32, ptr %7, align 4
  %90 = zext i32 %89 to i64
  %91 = getelementptr inbounds i8, ptr %88, i64 %90
  call void @ep_md5_transform(ptr noundef %87, ptr noundef %91)
  br label %92

92:                                               ; preds = %84
  %93 = load i32, ptr %7, align 4
  %94 = add i32 %93, 64
  store i32 %94, ptr %7, align 4
  br label %78, !llvm.loop !36

95:                                               ; preds = %78
  store i32 0, ptr %8, align 4
  br label %96

96:                                               ; preds = %95, %41
  %97 = load ptr, ptr %4, align 8
  %98 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %97, i32 0, i32 2
  %99 = load i32, ptr %8, align 4
  %100 = zext i32 %99 to i64
  %101 = getelementptr inbounds [64 x i8], ptr %98, i64 0, i64 %100
  %102 = load ptr, ptr %5, align 8
  %103 = load i32, ptr %7, align 4
  %104 = zext i32 %103 to i64
  %105 = getelementptr inbounds i8, ptr %102, i64 %104
  %106 = load i64, ptr %6, align 8
  %107 = load i32, ptr %7, align 4
  %108 = zext i32 %107 to i64
  %109 = sub i64 %106, %108
  %110 = load ptr, ptr %4, align 8
  %111 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %110, i32 0, i32 2
  %112 = load i32, ptr %8, align 4
  %113 = zext i32 %112 to i64
  %114 = getelementptr inbounds [64 x i8], ptr %111, i64 0, i64 %113
  %115 = call i64 @llvm.objectsize.i64.p0(ptr %114, i1 false, i1 true, i1 false)
  %116 = call ptr @__memcpy_chk(ptr noundef %101, ptr noundef %105, i64 noundef %109, i64 noundef %115) #13
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @ep_md5_final(ptr noundef %0, ptr noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca [8 x i8], align 1
  %6 = alloca i32, align 4
  %7 = alloca i32, align 4
  %8 = alloca [64 x i8], align 1
  %9 = alloca i32, align 4
  store ptr %0, ptr %3, align 8
  store ptr %1, ptr %4, align 8
  %10 = load ptr, ptr %3, align 8
  %11 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %10, i32 0, i32 0
  %12 = getelementptr inbounds [2 x i32], ptr %11, i64 0, i64 0
  %13 = load i32, ptr %12, align 4
  %14 = trunc i32 %13 to i8
  %15 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 0
  store i8 %14, ptr %15, align 1
  %16 = load ptr, ptr %3, align 8
  %17 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %16, i32 0, i32 0
  %18 = getelementptr inbounds [2 x i32], ptr %17, i64 0, i64 0
  %19 = load i32, ptr %18, align 4
  %20 = lshr i32 %19, 8
  %21 = trunc i32 %20 to i8
  %22 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 1
  store i8 %21, ptr %22, align 1
  %23 = load ptr, ptr %3, align 8
  %24 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %23, i32 0, i32 0
  %25 = getelementptr inbounds [2 x i32], ptr %24, i64 0, i64 0
  %26 = load i32, ptr %25, align 4
  %27 = lshr i32 %26, 16
  %28 = trunc i32 %27 to i8
  %29 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 2
  store i8 %28, ptr %29, align 1
  %30 = load ptr, ptr %3, align 8
  %31 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %30, i32 0, i32 0
  %32 = getelementptr inbounds [2 x i32], ptr %31, i64 0, i64 0
  %33 = load i32, ptr %32, align 4
  %34 = lshr i32 %33, 24
  %35 = trunc i32 %34 to i8
  %36 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 3
  store i8 %35, ptr %36, align 1
  %37 = load ptr, ptr %3, align 8
  %38 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %37, i32 0, i32 0
  %39 = getelementptr inbounds [2 x i32], ptr %38, i64 0, i64 1
  %40 = load i32, ptr %39, align 4
  %41 = trunc i32 %40 to i8
  %42 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 4
  store i8 %41, ptr %42, align 1
  %43 = load ptr, ptr %3, align 8
  %44 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %43, i32 0, i32 0
  %45 = getelementptr inbounds [2 x i32], ptr %44, i64 0, i64 1
  %46 = load i32, ptr %45, align 4
  %47 = lshr i32 %46, 8
  %48 = trunc i32 %47 to i8
  %49 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 5
  store i8 %48, ptr %49, align 1
  %50 = load ptr, ptr %3, align 8
  %51 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %50, i32 0, i32 0
  %52 = getelementptr inbounds [2 x i32], ptr %51, i64 0, i64 1
  %53 = load i32, ptr %52, align 4
  %54 = lshr i32 %53, 16
  %55 = trunc i32 %54 to i8
  %56 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 6
  store i8 %55, ptr %56, align 1
  %57 = load ptr, ptr %3, align 8
  %58 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %57, i32 0, i32 0
  %59 = getelementptr inbounds [2 x i32], ptr %58, i64 0, i64 1
  %60 = load i32, ptr %59, align 4
  %61 = lshr i32 %60, 24
  %62 = trunc i32 %61 to i8
  %63 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 7
  store i8 %62, ptr %63, align 1
  %64 = load ptr, ptr %3, align 8
  %65 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %64, i32 0, i32 0
  %66 = getelementptr inbounds [2 x i32], ptr %65, i64 0, i64 0
  %67 = load i32, ptr %66, align 4
  %68 = lshr i32 %67, 3
  %69 = and i32 %68, 63
  store i32 %69, ptr %6, align 4
  %70 = load i32, ptr %6, align 4
  %71 = icmp ult i32 %70, 56
  br i1 %71, label %72, label %75

72:                                               ; preds = %2
  %73 = load i32, ptr %6, align 4
  %74 = sub i32 56, %73
  br label %78

75:                                               ; preds = %2
  %76 = load i32, ptr %6, align 4
  %77 = sub i32 120, %76
  br label %78

78:                                               ; preds = %75, %72
  %79 = phi i32 [ %74, %72 ], [ %77, %75 ]
  store i32 %79, ptr %7, align 4
  %80 = getelementptr inbounds [64 x i8], ptr %8, i64 0, i64 0
  call void @llvm.memset.p0.i64(ptr align 1 %80, i8 0, i64 64, i1 false)
  %81 = getelementptr inbounds [64 x i8], ptr %8, i64 0, i64 0
  store i8 -128, ptr %81, align 1
  %82 = load ptr, ptr %3, align 8
  %83 = getelementptr inbounds [64 x i8], ptr %8, i64 0, i64 0
  %84 = load i32, ptr %7, align 4
  %85 = zext i32 %84 to i64
  call void @ep_md5_update(ptr noundef %82, ptr noundef %83, i64 noundef %85)
  %86 = load ptr, ptr %3, align 8
  %87 = getelementptr inbounds [8 x i8], ptr %5, i64 0, i64 0
  call void @ep_md5_update(ptr noundef %86, ptr noundef %87, i64 noundef 8)
  store i32 0, ptr %9, align 4
  br label %88

88:                                               ; preds = %146, %78
  %89 = load i32, ptr %9, align 4
  %90 = icmp slt i32 %89, 4
  br i1 %90, label %91, label %149

91:                                               ; preds = %88
  %92 = load ptr, ptr %3, align 8
  %93 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %92, i32 0, i32 1
  %94 = load i32, ptr %9, align 4
  %95 = sext i32 %94 to i64
  %96 = getelementptr inbounds [4 x i32], ptr %93, i64 0, i64 %95
  %97 = load i32, ptr %96, align 4
  %98 = trunc i32 %97 to i8
  %99 = load ptr, ptr %4, align 8
  %100 = load i32, ptr %9, align 4
  %101 = mul nsw i32 %100, 4
  %102 = sext i32 %101 to i64
  %103 = getelementptr inbounds i8, ptr %99, i64 %102
  store i8 %98, ptr %103, align 1
  %104 = load ptr, ptr %3, align 8
  %105 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %104, i32 0, i32 1
  %106 = load i32, ptr %9, align 4
  %107 = sext i32 %106 to i64
  %108 = getelementptr inbounds [4 x i32], ptr %105, i64 0, i64 %107
  %109 = load i32, ptr %108, align 4
  %110 = lshr i32 %109, 8
  %111 = trunc i32 %110 to i8
  %112 = load ptr, ptr %4, align 8
  %113 = load i32, ptr %9, align 4
  %114 = mul nsw i32 %113, 4
  %115 = add nsw i32 %114, 1
  %116 = sext i32 %115 to i64
  %117 = getelementptr inbounds i8, ptr %112, i64 %116
  store i8 %111, ptr %117, align 1
  %118 = load ptr, ptr %3, align 8
  %119 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %118, i32 0, i32 1
  %120 = load i32, ptr %9, align 4
  %121 = sext i32 %120 to i64
  %122 = getelementptr inbounds [4 x i32], ptr %119, i64 0, i64 %121
  %123 = load i32, ptr %122, align 4
  %124 = lshr i32 %123, 16
  %125 = trunc i32 %124 to i8
  %126 = load ptr, ptr %4, align 8
  %127 = load i32, ptr %9, align 4
  %128 = mul nsw i32 %127, 4
  %129 = add nsw i32 %128, 2
  %130 = sext i32 %129 to i64
  %131 = getelementptr inbounds i8, ptr %126, i64 %130
  store i8 %125, ptr %131, align 1
  %132 = load ptr, ptr %3, align 8
  %133 = getelementptr inbounds %struct.EP_MD5_CTX, ptr %132, i32 0, i32 1
  %134 = load i32, ptr %9, align 4
  %135 = sext i32 %134 to i64
  %136 = getelementptr inbounds [4 x i32], ptr %133, i64 0, i64 %135
  %137 = load i32, ptr %136, align 4
  %138 = lshr i32 %137, 24
  %139 = trunc i32 %138 to i8
  %140 = load ptr, ptr %4, align 8
  %141 = load i32, ptr %9, align 4
  %142 = mul nsw i32 %141, 4
  %143 = add nsw i32 %142, 3
  %144 = sext i32 %143 to i64
  %145 = getelementptr inbounds i8, ptr %140, i64 %144
  store i8 %139, ptr %145, align 1
  br label %146

146:                                              ; preds = %91
  %147 = load i32, ptr %9, align 4
  %148 = add nsw i32 %147, 1
  store i32 %148, ptr %9, align 4
  br label %88, !llvm.loop !37

149:                                              ; preds = %88
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define ptr @ep_md5(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca %struct.EP_MD5_CTX, align 4
  %4 = alloca [16 x i8], align 1
  %5 = alloca ptr, align 8
  %6 = alloca i32, align 4
  store ptr %0, ptr %2, align 8
  %7 = load ptr, ptr %2, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store ptr @.str.5, ptr %2, align 8
  br label %10

10:                                               ; preds = %9, %1
  call void @ep_md5_init(ptr noundef %3)
  %11 = load ptr, ptr %2, align 8
  %12 = load ptr, ptr %2, align 8
  %13 = call i64 @strlen(ptr noundef %12) #13
  call void @ep_md5_update(ptr noundef %3, ptr noundef %11, i64 noundef %13)
  %14 = getelementptr inbounds [16 x i8], ptr %4, i64 0, i64 0
  call void @ep_md5_final(ptr noundef %3, ptr noundef %14)
  %15 = call ptr @malloc(i64 noundef 33) #14
  store ptr %15, ptr %5, align 8
  %16 = load ptr, ptr %5, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %18, label %46

18:                                               ; preds = %10
  store i32 0, ptr %6, align 4
  br label %19

19:                                               ; preds = %40, %18
  %20 = load i32, ptr %6, align 4
  %21 = icmp slt i32 %20, 16
  br i1 %21, label %22, label %43

22:                                               ; preds = %19
  %23 = load ptr, ptr %5, align 8
  %24 = load i32, ptr %6, align 4
  %25 = mul nsw i32 %24, 2
  %26 = sext i32 %25 to i64
  %27 = getelementptr inbounds i8, ptr %23, i64 %26
  %28 = load ptr, ptr %5, align 8
  %29 = load i32, ptr %6, align 4
  %30 = mul nsw i32 %29, 2
  %31 = sext i32 %30 to i64
  %32 = getelementptr inbounds i8, ptr %28, i64 %31
  %33 = call i64 @llvm.objectsize.i64.p0(ptr %32, i1 false, i1 true, i1 false)
  %34 = load i32, ptr %6, align 4
  %35 = sext i32 %34 to i64
  %36 = getelementptr inbounds [16 x i8], ptr %4, i64 0, i64 %35
  %37 = load i8, ptr %36, align 1
  %38 = zext i8 %37 to i32
  %39 = call i32 (ptr, i64, i32, i64, ptr, ...) @__snprintf_chk(ptr noundef %27, i64 noundef 3, i32 noundef 0, i64 noundef %33, ptr noundef @.str.18, i32 noundef %38)
  br label %40

40:                                               ; preds = %22
  %41 = load i32, ptr %6, align 4
  %42 = add nsw i32 %41, 1
  store i32 %42, ptr %6, align 4
  br label %19, !llvm.loop !38

43:                                               ; preds = %19
  %44 = load ptr, ptr %5, align 8
  %45 = getelementptr inbounds i8, ptr %44, i64 32
  store i8 0, ptr %45, align 1
  br label %46

46:                                               ; preds = %43, %10
  %47 = load ptr, ptr %5, align 8
  ret ptr %47
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define ptr @read_file_content(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca ptr, align 8
  %4 = alloca [3 x i8], align 1
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  %8 = alloca ptr, align 8
  %9 = alloca ptr, align 8
  %10 = alloca i64, align 8
  store ptr %0, ptr %3, align 8
  %11 = getelementptr inbounds [3 x i8], ptr %4, i64 0, i64 0
  store i8 114, ptr %11, align 1
  %12 = getelementptr inbounds [3 x i8], ptr %4, i64 0, i64 1
  store i8 98, ptr %12, align 1
  %13 = getelementptr inbounds [3 x i8], ptr %4, i64 0, i64 2
  store i8 0, ptr %13, align 1
  %14 = load ptr, ptr %3, align 8
  %15 = getelementptr inbounds [3 x i8], ptr %4, i64 0, i64 0
  %16 = call ptr @"\01_fopen"(ptr noundef %14, ptr noundef %15)
  store ptr %16, ptr %5, align 8
  %17 = load ptr, ptr %5, align 8
  %18 = icmp ne ptr %17, null
  br i1 %18, label %30, label %19

19:                                               ; preds = %1
  %20 = call ptr @malloc(i64 noundef 1) #14
  store ptr %20, ptr %6, align 8
  %21 = load ptr, ptr %6, align 8
  %22 = icmp ne ptr %21, null
  br i1 %22, label %23, label %26

23:                                               ; preds = %19
  %24 = load ptr, ptr %6, align 8
  %25 = getelementptr inbounds i8, ptr %24, i64 0
  store i8 0, ptr %25, align 1
  br label %26

26:                                               ; preds = %23, %19
  %27 = load ptr, ptr %6, align 8
  %28 = call ptr @ep_gc_register(ptr noundef %27, i32 noundef 1)
  %29 = load ptr, ptr %6, align 8
  store ptr %29, ptr %2, align 8
  br label %68

30:                                               ; preds = %1
  %31 = load ptr, ptr %5, align 8
  %32 = call i32 @fseek(ptr noundef %31, i64 noundef 0, i32 noundef 2)
  %33 = load ptr, ptr %5, align 8
  %34 = call i64 @ftell(ptr noundef %33)
  store i64 %34, ptr %7, align 8
  %35 = load ptr, ptr %5, align 8
  %36 = call i32 @fseek(ptr noundef %35, i64 noundef 0, i32 noundef 0)
  %37 = load i64, ptr %7, align 8
  %38 = add nsw i64 %37, 1
  %39 = call ptr @malloc(i64 noundef %38) #14
  store ptr %39, ptr %8, align 8
  %40 = load ptr, ptr %8, align 8
  %41 = icmp ne ptr %40, null
  br i1 %41, label %55, label %42

42:                                               ; preds = %30
  %43 = load ptr, ptr %5, align 8
  %44 = call i32 @fclose(ptr noundef %43)
  %45 = call ptr @malloc(i64 noundef 1) #14
  store ptr %45, ptr %9, align 8
  %46 = load ptr, ptr %9, align 8
  %47 = icmp ne ptr %46, null
  br i1 %47, label %48, label %51

48:                                               ; preds = %42
  %49 = load ptr, ptr %9, align 8
  %50 = getelementptr inbounds i8, ptr %49, i64 0
  store i8 0, ptr %50, align 1
  br label %51

51:                                               ; preds = %48, %42
  %52 = load ptr, ptr %9, align 8
  %53 = call ptr @ep_gc_register(ptr noundef %52, i32 noundef 1)
  %54 = load ptr, ptr %9, align 8
  store ptr %54, ptr %2, align 8
  br label %68

55:                                               ; preds = %30
  %56 = load ptr, ptr %8, align 8
  %57 = load i64, ptr %7, align 8
  %58 = load ptr, ptr %5, align 8
  %59 = call i64 @fread(ptr noundef %56, i64 noundef 1, i64 noundef %57, ptr noundef %58)
  store i64 %59, ptr %10, align 8
  %60 = load ptr, ptr %8, align 8
  %61 = load i64, ptr %10, align 8
  %62 = getelementptr inbounds i8, ptr %60, i64 %61
  store i8 0, ptr %62, align 1
  %63 = load ptr, ptr %5, align 8
  %64 = call i32 @fclose(ptr noundef %63)
  %65 = load ptr, ptr %8, align 8
  %66 = call ptr @ep_gc_register(ptr noundef %65, i32 noundef 1)
  %67 = load ptr, ptr %8, align 8
  store ptr %67, ptr %2, align 8
  br label %68

68:                                               ; preds = %55, %51, %26
  %69 = load ptr, ptr %2, align 8
  ret ptr %69
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal ptr @ep_gc_register(ptr noundef %0, i32 noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i32, align 4
  %6 = alloca ptr, align 8
  store ptr %0, ptr %4, align 8
  store i32 %1, ptr %5, align 4
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %2
  store ptr null, ptr %3, align 8
  br label %44

10:                                               ; preds = %2
  %11 = call i32 @pthread_mutex_lock(ptr noundef @ep_gc_mutex)
  %12 = call ptr @malloc(i64 noundef 48) #14
  store ptr %12, ptr %6, align 8
  %13 = load ptr, ptr %6, align 8
  %14 = icmp ne ptr %13, null
  br i1 %14, label %17, label %15

15:                                               ; preds = %10
  %16 = call i32 @pthread_mutex_unlock(ptr noundef @ep_gc_mutex)
  store ptr null, ptr %3, align 8
  br label %44

17:                                               ; preds = %10
  %18 = load i32, ptr %5, align 4
  %19 = load ptr, ptr %6, align 8
  %20 = getelementptr inbounds %struct.EpGCObject, ptr %19, i32 0, i32 0
  store i32 %18, ptr %20, align 8
  %21 = load ptr, ptr %6, align 8
  %22 = getelementptr inbounds %struct.EpGCObject, ptr %21, i32 0, i32 1
  store i32 0, ptr %22, align 4
  %23 = load ptr, ptr %4, align 8
  %24 = load ptr, ptr %6, align 8
  %25 = getelementptr inbounds %struct.EpGCObject, ptr %24, i32 0, i32 2
  store ptr %23, ptr %25, align 8
  %26 = load ptr, ptr %6, align 8
  %27 = getelementptr inbounds %struct.EpGCObject, ptr %26, i32 0, i32 3
  store i64 0, ptr %27, align 8
  %28 = load ptr, ptr %6, align 8
  %29 = getelementptr inbounds %struct.EpGCObject, ptr %28, i32 0, i32 4
  store i64 0, ptr %29, align 8
  %30 = load ptr, ptr %6, align 8
  %31 = getelementptr inbounds %struct.EpGCObject, ptr %30, i32 0, i32 5
  store i32 0, ptr %31, align 8
  %32 = load ptr, ptr @ep_gc_head, align 8
  %33 = load ptr, ptr %6, align 8
  %34 = getelementptr inbounds %struct.EpGCObject, ptr %33, i32 0, i32 6
  store ptr %32, ptr %34, align 8
  %35 = load ptr, ptr %6, align 8
  store ptr %35, ptr @ep_gc_head, align 8
  %36 = load i64, ptr @ep_gc_count, align 8
  %37 = add nsw i64 %36, 1
  store i64 %37, ptr @ep_gc_count, align 8
  %38 = load i64, ptr @ep_gc_nursery_count, align 8
  %39 = add nsw i64 %38, 1
  store i64 %39, ptr @ep_gc_nursery_count, align 8
  %40 = load ptr, ptr %4, align 8
  %41 = load ptr, ptr %6, align 8
  call void @ep_gc_table_insert(ptr noundef %40, ptr noundef %41)
  %42 = call i32 @pthread_mutex_unlock(ptr noundef @ep_gc_mutex)
  %43 = load ptr, ptr %6, align 8
  store ptr %43, ptr %3, align 8
  br label %44

44:                                               ; preds = %17, %15, %9
  %45 = load ptr, ptr %3, align 8
  ret ptr %45
}

declare i32 @fseek(ptr noundef, i64 noundef, i32 noundef) #1

declare i64 @ftell(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_length(ptr noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store ptr %0, ptr %3, align 8
  %4 = load ptr, ptr %3, align 8
  %5 = icmp ne ptr %4, null
  br i1 %5, label %7, label %6

6:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %10

7:                                                ; preds = %1
  %8 = load ptr, ptr %3, align 8
  %9 = call i64 @strlen(ptr noundef %8) #13
  store i64 %9, ptr %2, align 8
  br label %10

10:                                               ; preds = %7, %6
  %11 = load i64, ptr %2, align 8
  ret i64 %11
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @get_character(ptr noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store ptr %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %2
  store i64 0, ptr %3, align 8
  br label %26

10:                                               ; preds = %2
  %11 = load ptr, ptr %4, align 8
  %12 = call i64 @strlen(ptr noundef %11) #13
  store i64 %12, ptr %6, align 8
  %13 = load i64, ptr %5, align 8
  %14 = icmp slt i64 %13, 0
  br i1 %14, label %19, label %15

15:                                               ; preds = %10
  %16 = load i64, ptr %5, align 8
  %17 = load i64, ptr %6, align 8
  %18 = icmp sge i64 %16, %17
  br i1 %18, label %19, label %20

19:                                               ; preds = %15, %10
  store i64 0, ptr %3, align 8
  br label %26

20:                                               ; preds = %15
  %21 = load ptr, ptr %4, align 8
  %22 = load i64, ptr %5, align 8
  %23 = getelementptr inbounds i8, ptr %21, i64 %22
  %24 = load i8, ptr %23, align 1
  %25 = zext i8 %24 to i64
  store i64 %25, ptr %3, align 8
  br label %26

26:                                               ; preds = %20, %19, %9
  %27 = load i64, ptr %3, align 8
  ret i64 %27
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @get_list_data_ptr(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %15

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = getelementptr inbounds %struct.EpList, ptr %11, i32 0, i32 0
  %13 = load ptr, ptr %12, align 8
  %14 = ptrtoint ptr %13 to i64
  store i64 %14, ptr %2, align 8
  br label %15

15:                                               ; preds = %10, %9
  %16 = load i64, ptr %2, align 8
  ret i64 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @get_list(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %6, align 8
  %9 = load ptr, ptr %6, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %11, label %20

11:                                               ; preds = %2
  %12 = load i64, ptr %5, align 8
  %13 = icmp slt i64 %12, 0
  br i1 %13, label %20, label %14

14:                                               ; preds = %11
  %15 = load i64, ptr %5, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = getelementptr inbounds %struct.EpList, ptr %16, i32 0, i32 1
  %18 = load i64, ptr %17, align 8
  %19 = icmp sge i64 %15, %18
  br i1 %19, label %20, label %21

20:                                               ; preds = %14, %11, %2
  store i64 0, ptr %3, align 8
  br label %28

21:                                               ; preds = %14
  %22 = load ptr, ptr %6, align 8
  %23 = getelementptr inbounds %struct.EpList, ptr %22, i32 0, i32 0
  %24 = load ptr, ptr %23, align 8
  %25 = load i64, ptr %5, align 8
  %26 = getelementptr inbounds i64, ptr %24, i64 %25
  %27 = load i64, ptr %26, align 8
  store i64 %27, ptr %3, align 8
  br label %28

28:                                               ; preds = %21, %20
  %29 = load i64, ptr %3, align 8
  ret i64 %29
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @set_list(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %5, align 8
  store i64 %1, ptr %6, align 8
  store i64 %2, ptr %7, align 8
  %9 = load i64, ptr %5, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %8, align 8
  %11 = load ptr, ptr %8, align 8
  %12 = icmp ne ptr %11, null
  br i1 %12, label %13, label %22

13:                                               ; preds = %3
  %14 = load i64, ptr %6, align 8
  %15 = icmp slt i64 %14, 0
  br i1 %15, label %22, label %16

16:                                               ; preds = %13
  %17 = load i64, ptr %6, align 8
  %18 = load ptr, ptr %8, align 8
  %19 = getelementptr inbounds %struct.EpList, ptr %18, i32 0, i32 1
  %20 = load i64, ptr %19, align 8
  %21 = icmp sge i64 %17, %20
  br i1 %21, label %22, label %23

22:                                               ; preds = %16, %13, %3
  store i64 0, ptr %4, align 8
  br label %34

23:                                               ; preds = %16
  %24 = load i64, ptr %7, align 8
  %25 = load ptr, ptr %8, align 8
  %26 = getelementptr inbounds %struct.EpList, ptr %25, i32 0, i32 0
  %27 = load ptr, ptr %26, align 8
  %28 = load i64, ptr %6, align 8
  %29 = getelementptr inbounds i64, ptr %27, i64 %28
  store i64 %24, ptr %29, align 8
  %30 = load i64, ptr %5, align 8
  %31 = inttoptr i64 %30 to ptr
  %32 = load i64, ptr %7, align 8
  call void @ep_gc_write_barrier(ptr noundef %31, i64 noundef %32)
  %33 = load i64, ptr %7, align 8
  store i64 %33, ptr %4, align 8
  br label %34

34:                                               ; preds = %23, %22
  %35 = load i64, ptr %4, align 8
  ret i64 %35
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @length_list(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %14

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = getelementptr inbounds %struct.EpList, ptr %11, i32 0, i32 1
  %13 = load i64, ptr %12, align 8
  store i64 %13, ptr %2, align 8
  br label %14

14:                                               ; preds = %10, %9
  %15 = load i64, ptr %2, align 8
  ret i64 %15
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @free_list(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %10, label %9

9:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %21

10:                                               ; preds = %1
  %11 = load ptr, ptr %4, align 8
  %12 = call ptr @ep_gc_find(ptr noundef %11)
  %13 = icmp ne ptr %12, null
  br i1 %13, label %15, label %14

14:                                               ; preds = %10
  store i64 0, ptr %2, align 8
  br label %21

15:                                               ; preds = %10
  %16 = load ptr, ptr %4, align 8
  call void @ep_gc_unregister(ptr noundef %16)
  %17 = load ptr, ptr %4, align 8
  %18 = getelementptr inbounds %struct.EpList, ptr %17, i32 0, i32 0
  %19 = load ptr, ptr %18, align 8
  call void @free(ptr noundef %19)
  %20 = load ptr, ptr %4, align 8
  call void @free(ptr noundef %20)
  store i64 0, ptr %2, align 8
  br label %21

21:                                               ; preds = %15, %14, %9
  %22 = load i64, ptr %2, align 8
  ret i64 %22
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @sqlite_get_callback_ptr(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  ret i64 ptrtoint (ptr @sqlite_list_callback to i64)
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal i32 @sqlite_list_callback(ptr noundef %0, i32 noundef %1, ptr noundef %2, ptr noundef %3) #0 {
  %5 = alloca ptr, align 8
  %6 = alloca i32, align 4
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  %9 = alloca ptr, align 8
  %10 = alloca ptr, align 8
  %11 = alloca i32, align 4
  %12 = alloca ptr, align 8
  store ptr %0, ptr %5, align 8
  store i32 %1, ptr %6, align 4
  store ptr %2, ptr %7, align 8
  store ptr %3, ptr %8, align 8
  %13 = load ptr, ptr %5, align 8
  store ptr %13, ptr %9, align 8
  %14 = call i64 @create_list()
  %15 = inttoptr i64 %14 to ptr
  store ptr %15, ptr %10, align 8
  store i32 0, ptr %11, align 4
  br label %16

16:                                               ; preds = %43, %4
  %17 = load i32, ptr %11, align 4
  %18 = load i32, ptr %6, align 4
  %19 = icmp slt i32 %17, %18
  br i1 %19, label %20, label %46

20:                                               ; preds = %16
  %21 = load ptr, ptr %7, align 8
  %22 = load i32, ptr %11, align 4
  %23 = sext i32 %22 to i64
  %24 = getelementptr inbounds ptr, ptr %21, i64 %23
  %25 = load ptr, ptr %24, align 8
  %26 = icmp ne ptr %25, null
  br i1 %26, label %27, label %34

27:                                               ; preds = %20
  %28 = load ptr, ptr %7, align 8
  %29 = load i32, ptr %11, align 4
  %30 = sext i32 %29 to i64
  %31 = getelementptr inbounds ptr, ptr %28, i64 %30
  %32 = load ptr, ptr %31, align 8
  %33 = call ptr @strdup(ptr noundef %32) #13
  br label %36

34:                                               ; preds = %20
  %35 = call ptr @strdup(ptr noundef @.str.5) #13
  br label %36

36:                                               ; preds = %34, %27
  %37 = phi ptr [ %33, %27 ], [ %35, %34 ]
  store ptr %37, ptr %12, align 8
  %38 = load ptr, ptr %10, align 8
  %39 = ptrtoint ptr %38 to i64
  %40 = load ptr, ptr %12, align 8
  %41 = ptrtoint ptr %40 to i64
  %42 = call i64 @append_list(i64 noundef %39, i64 noundef %41)
  br label %43

43:                                               ; preds = %36
  %44 = load i32, ptr %11, align 4
  %45 = add nsw i32 %44, 1
  store i32 %45, ptr %11, align 4
  br label %16, !llvm.loop !39

46:                                               ; preds = %16
  %47 = load ptr, ptr %9, align 8
  %48 = ptrtoint ptr %47 to i64
  %49 = load ptr, ptr %10, align 8
  %50 = ptrtoint ptr %49 to i64
  %51 = call i64 @append_list(i64 noundef %48, i64 noundef %50)
  ret i32 0
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @init_ep_args(i32 noundef %0, ptr noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca ptr, align 8
  store i32 %0, ptr %3, align 4
  store ptr %1, ptr %4, align 8
  %5 = load i32, ptr %3, align 4
  store i32 %5, ptr @ep_argc, align 4
  %6 = load ptr, ptr %4, align 8
  store ptr %6, ptr @ep_argv, align 8
  call void @ep_gc_register_thread(ptr noundef %3)
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_register_thread(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store ptr %0, ptr %2, align 8
  %5 = load ptr, ptr %2, align 8
  %6 = call align 8 ptr @llvm.threadlocal.address.p0(ptr align 8 @ep_thread_local_bottom)
  store ptr %5, ptr %6, align 8
  %7 = load ptr, ptr %2, align 8
  %8 = call align 8 ptr @llvm.threadlocal.address.p0(ptr align 8 @ep_thread_local_top)
  store volatile ptr %7, ptr %8, align 8
  %9 = call i32 @pthread_mutex_lock(ptr noundef @ep_gc_mutex)
  store i32 -1, ptr %3, align 4
  store i32 0, ptr %4, align 4
  br label %10

10:                                               ; preds = %23, %1
  %11 = load i32, ptr %4, align 4
  %12 = load i32, ptr @ep_num_threads, align 4
  %13 = icmp slt i32 %11, %12
  br i1 %13, label %14, label %26

14:                                               ; preds = %10
  %15 = load i32, ptr %4, align 4
  %16 = sext i32 %15 to i64
  %17 = getelementptr inbounds [256 x i32], ptr @ep_thread_active, i64 0, i64 %16
  %18 = load volatile i32, ptr %17, align 4
  %19 = icmp ne i32 %18, 0
  br i1 %19, label %22, label %20

20:                                               ; preds = %14
  %21 = load i32, ptr %4, align 4
  store i32 %21, ptr %3, align 4
  br label %26

22:                                               ; preds = %14
  br label %23

23:                                               ; preds = %22
  %24 = load i32, ptr %4, align 4
  %25 = add nsw i32 %24, 1
  store i32 %25, ptr %4, align 4
  br label %10, !llvm.loop !40

26:                                               ; preds = %20, %10
  %27 = load i32, ptr %3, align 4
  %28 = icmp eq i32 %27, -1
  br i1 %28, label %29, label %35

29:                                               ; preds = %26
  %30 = load i32, ptr @ep_num_threads, align 4
  %31 = icmp slt i32 %30, 256
  br i1 %31, label %32, label %35

32:                                               ; preds = %29
  %33 = load i32, ptr @ep_num_threads, align 4
  %34 = add nsw i32 %33, 1
  store i32 %34, ptr @ep_num_threads, align 4
  store i32 %33, ptr %3, align 4
  br label %35

35:                                               ; preds = %32, %29, %26
  %36 = load i32, ptr %3, align 4
  %37 = icmp ne i32 %36, -1
  br i1 %37, label %38, label %68

38:                                               ; preds = %35
  %39 = call align 8 ptr @llvm.threadlocal.address.p0(ptr align 8 @ep_thread_local_top)
  %40 = load i32, ptr %3, align 4
  %41 = sext i32 %40 to i64
  %42 = getelementptr inbounds [256 x ptr], ptr @ep_thread_tops, i64 0, i64 %41
  store ptr %39, ptr %42, align 8
  %43 = load ptr, ptr %2, align 8
  %44 = load i32, ptr %3, align 4
  %45 = sext i32 %44 to i64
  %46 = getelementptr inbounds [256 x ptr], ptr @ep_thread_bottoms, i64 0, i64 %45
  store ptr %43, ptr %46, align 8
  %47 = load i32, ptr %3, align 4
  %48 = sext i32 %47 to i64
  %49 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %48
  %50 = load ptr, ptr %49, align 8
  %51 = icmp ne ptr %50, null
  br i1 %51, label %57, label %52

52:                                               ; preds = %38
  %53 = call ptr @calloc(i64 noundef 1, i64 noundef 32776) #15
  %54 = load i32, ptr %3, align 4
  %55 = sext i32 %54 to i64
  %56 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %55
  store ptr %53, ptr %56, align 8
  br label %57

57:                                               ; preds = %52, %38
  %58 = load i32, ptr %3, align 4
  %59 = sext i32 %58 to i64
  %60 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %59
  %61 = load ptr, ptr %60, align 8
  %62 = getelementptr inbounds %struct.EpThreadGCState, ptr %61, i32 0, i32 1
  store volatile i32 0, ptr %62, align 8
  %63 = load i32, ptr %3, align 4
  %64 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_thread_slot)
  store i32 %63, ptr %64, align 4
  fence seq_cst
  %65 = load i32, ptr %3, align 4
  %66 = sext i32 %65 to i64
  %67 = getelementptr inbounds [256 x i32], ptr @ep_thread_active, i64 0, i64 %66
  store volatile i32 1, ptr %67, align 4
  br label %68

68:                                               ; preds = %57, %35
  %69 = call i32 @pthread_mutex_unlock(ptr noundef @ep_gc_mutex)
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @get_argument_count() #0 {
  %1 = load i32, ptr @ep_argc, align 4
  %2 = sext i32 %1 to i64
  ret i64 %2
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define ptr @get_argument(i64 noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = icmp slt i64 %5, 0
  br i1 %6, label %12, label %7

7:                                                ; preds = %1
  %8 = load i64, ptr %3, align 8
  %9 = load i32, ptr @ep_argc, align 4
  %10 = sext i32 %9 to i64
  %11 = icmp sge i64 %8, %10
  br i1 %11, label %12, label %21

12:                                               ; preds = %7, %1
  %13 = call ptr @malloc(i64 noundef 1) #14
  store ptr %13, ptr %4, align 8
  %14 = load ptr, ptr %4, align 8
  %15 = icmp ne ptr %14, null
  br i1 %15, label %16, label %19

16:                                               ; preds = %12
  %17 = load ptr, ptr %4, align 8
  %18 = getelementptr inbounds i8, ptr %17, i64 0
  store i8 0, ptr %18, align 1
  br label %19

19:                                               ; preds = %16, %12
  %20 = load ptr, ptr %4, align 8
  store ptr %20, ptr %2, align 8
  br label %26

21:                                               ; preds = %7
  %22 = load ptr, ptr @ep_argv, align 8
  %23 = load i64, ptr %3, align 8
  %24 = getelementptr inbounds ptr, ptr %22, i64 %23
  %25 = load ptr, ptr %24, align 8
  store ptr %25, ptr %2, align 8
  br label %26

26:                                               ; preds = %21, %19
  %27 = load ptr, ptr %2, align 8
  ret ptr %27
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @write_file_content(ptr noundef %0, ptr noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca [3 x i8], align 1
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  store ptr %0, ptr %4, align 8
  store ptr %1, ptr %5, align 8
  %10 = getelementptr inbounds [3 x i8], ptr %6, i64 0, i64 0
  store i8 119, ptr %10, align 1
  %11 = getelementptr inbounds [3 x i8], ptr %6, i64 0, i64 1
  store i8 98, ptr %11, align 1
  %12 = getelementptr inbounds [3 x i8], ptr %6, i64 0, i64 2
  store i8 0, ptr %12, align 1
  %13 = load ptr, ptr %4, align 8
  %14 = getelementptr inbounds [3 x i8], ptr %6, i64 0, i64 0
  %15 = call ptr @"\01_fopen"(ptr noundef %13, ptr noundef %14)
  store ptr %15, ptr %7, align 8
  %16 = load ptr, ptr %7, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %19, label %18

18:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %34

19:                                               ; preds = %2
  %20 = load ptr, ptr %5, align 8
  %21 = call i64 @strlen(ptr noundef %20) #13
  store i64 %21, ptr %8, align 8
  %22 = load ptr, ptr %5, align 8
  %23 = load i64, ptr %8, align 8
  %24 = load ptr, ptr %7, align 8
  %25 = call i64 @"\01_fwrite"(ptr noundef %22, i64 noundef 1, i64 noundef %23, ptr noundef %24)
  store i64 %25, ptr %9, align 8
  %26 = load ptr, ptr %7, align 8
  %27 = call i32 @fclose(ptr noundef %26)
  %28 = load i64, ptr %9, align 8
  %29 = load i64, ptr %8, align 8
  %30 = icmp eq i64 %28, %29
  %31 = zext i1 %30 to i64
  %32 = select i1 %30, i32 1, i32 0
  %33 = sext i32 %32 to i64
  store i64 %33, ptr %3, align 8
  br label %34

34:                                               ; preds = %19, %18
  %35 = load i64, ptr %3, align 8
  ret i64 %35
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @run_command(ptr noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store ptr %0, ptr %3, align 8
  %4 = load ptr, ptr %3, align 8
  %5 = icmp ne ptr %4, null
  br i1 %5, label %7, label %6

6:                                                ; preds = %1
  store i64 -1, ptr %2, align 8
  br label %11

7:                                                ; preds = %1
  %8 = load ptr, ptr %3, align 8
  %9 = call i32 @"\01_system"(ptr noundef %8)
  %10 = sext i32 %9 to i64
  store i64 %10, ptr %2, align 8
  br label %11

11:                                               ; preds = %7, %6
  %12 = load i64, ptr %2, align 8
  ret i64 %12
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define ptr @substring(ptr noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca ptr, align 8
  %9 = alloca i64, align 8
  %10 = alloca ptr, align 8
  %11 = alloca ptr, align 8
  %12 = alloca ptr, align 8
  store ptr %0, ptr %5, align 8
  store i64 %1, ptr %6, align 8
  store i64 %2, ptr %7, align 8
  %13 = load ptr, ptr %5, align 8
  %14 = icmp ne ptr %13, null
  br i1 %14, label %26, label %15

15:                                               ; preds = %3
  %16 = call ptr @malloc(i64 noundef 1) #14
  store ptr %16, ptr %8, align 8
  %17 = load ptr, ptr %8, align 8
  %18 = icmp ne ptr %17, null
  br i1 %18, label %19, label %22

19:                                               ; preds = %15
  %20 = load ptr, ptr %8, align 8
  %21 = getelementptr inbounds i8, ptr %20, i64 0
  store i8 0, ptr %21, align 1
  br label %22

22:                                               ; preds = %19, %15
  %23 = load ptr, ptr %8, align 8
  %24 = call ptr @ep_gc_register(ptr noundef %23, i32 noundef 1)
  %25 = load ptr, ptr %8, align 8
  store ptr %25, ptr %4, align 8
  br label %91

26:                                               ; preds = %3
  %27 = load ptr, ptr %5, align 8
  %28 = call i64 @strlen(ptr noundef %27) #13
  store i64 %28, ptr %9, align 8
  %29 = load i64, ptr %6, align 8
  %30 = icmp slt i64 %29, 0
  br i1 %30, label %38, label %31

31:                                               ; preds = %26
  %32 = load i64, ptr %6, align 8
  %33 = load i64, ptr %9, align 8
  %34 = icmp sge i64 %32, %33
  br i1 %34, label %38, label %35

35:                                               ; preds = %31
  %36 = load i64, ptr %7, align 8
  %37 = icmp sle i64 %36, 0
  br i1 %37, label %38, label %49

38:                                               ; preds = %35, %31, %26
  %39 = call ptr @malloc(i64 noundef 1) #14
  store ptr %39, ptr %10, align 8
  %40 = load ptr, ptr %10, align 8
  %41 = icmp ne ptr %40, null
  br i1 %41, label %42, label %45

42:                                               ; preds = %38
  %43 = load ptr, ptr %10, align 8
  %44 = getelementptr inbounds i8, ptr %43, i64 0
  store i8 0, ptr %44, align 1
  br label %45

45:                                               ; preds = %42, %38
  %46 = load ptr, ptr %10, align 8
  %47 = call ptr @ep_gc_register(ptr noundef %46, i32 noundef 1)
  %48 = load ptr, ptr %10, align 8
  store ptr %48, ptr %4, align 8
  br label %91

49:                                               ; preds = %35
  %50 = load i64, ptr %6, align 8
  %51 = load i64, ptr %7, align 8
  %52 = add nsw i64 %50, %51
  %53 = load i64, ptr %9, align 8
  %54 = icmp sgt i64 %52, %53
  br i1 %54, label %55, label %59

55:                                               ; preds = %49
  %56 = load i64, ptr %9, align 8
  %57 = load i64, ptr %6, align 8
  %58 = sub nsw i64 %56, %57
  store i64 %58, ptr %7, align 8
  br label %59

59:                                               ; preds = %55, %49
  %60 = load i64, ptr %7, align 8
  %61 = add nsw i64 %60, 1
  %62 = call ptr @malloc(i64 noundef %61) #14
  store ptr %62, ptr %11, align 8
  %63 = load ptr, ptr %11, align 8
  %64 = icmp ne ptr %63, null
  br i1 %64, label %76, label %65

65:                                               ; preds = %59
  %66 = call ptr @malloc(i64 noundef 1) #14
  store ptr %66, ptr %12, align 8
  %67 = load ptr, ptr %12, align 8
  %68 = icmp ne ptr %67, null
  br i1 %68, label %69, label %72

69:                                               ; preds = %65
  %70 = load ptr, ptr %12, align 8
  %71 = getelementptr inbounds i8, ptr %70, i64 0
  store i8 0, ptr %71, align 1
  br label %72

72:                                               ; preds = %69, %65
  %73 = load ptr, ptr %12, align 8
  %74 = call ptr @ep_gc_register(ptr noundef %73, i32 noundef 1)
  %75 = load ptr, ptr %12, align 8
  store ptr %75, ptr %4, align 8
  br label %91

76:                                               ; preds = %59
  %77 = load ptr, ptr %11, align 8
  %78 = load ptr, ptr %5, align 8
  %79 = load i64, ptr %6, align 8
  %80 = getelementptr inbounds i8, ptr %78, i64 %79
  %81 = load i64, ptr %7, align 8
  %82 = load ptr, ptr %11, align 8
  %83 = call i64 @llvm.objectsize.i64.p0(ptr %82, i1 false, i1 true, i1 false)
  %84 = call ptr @__strncpy_chk(ptr noundef %77, ptr noundef %80, i64 noundef %81, i64 noundef %83) #13
  %85 = load ptr, ptr %11, align 8
  %86 = load i64, ptr %7, align 8
  %87 = getelementptr inbounds i8, ptr %85, i64 %86
  store i8 0, ptr %87, align 1
  %88 = load ptr, ptr %11, align 8
  %89 = call ptr @ep_gc_register(ptr noundef %88, i32 noundef 1)
  %90 = load ptr, ptr %11, align 8
  store ptr %90, ptr %4, align 8
  br label %91

91:                                               ; preds = %76, %72, %45, %22
  %92 = load ptr, ptr %4, align 8
  ret ptr %92
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define ptr @string_from_list(i64 noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %9 = load i64, ptr %3, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %4, align 8
  %11 = load ptr, ptr %4, align 8
  %12 = icmp ne ptr %11, null
  br i1 %12, label %24, label %13

13:                                               ; preds = %1
  %14 = call ptr @malloc(i64 noundef 1) #14
  store ptr %14, ptr %5, align 8
  %15 = load ptr, ptr %5, align 8
  %16 = icmp ne ptr %15, null
  br i1 %16, label %17, label %20

17:                                               ; preds = %13
  %18 = load ptr, ptr %5, align 8
  %19 = getelementptr inbounds i8, ptr %18, i64 0
  store i8 0, ptr %19, align 1
  br label %20

20:                                               ; preds = %17, %13
  %21 = load ptr, ptr %5, align 8
  %22 = call ptr @ep_gc_register(ptr noundef %21, i32 noundef 1)
  %23 = load ptr, ptr %5, align 8
  store ptr %23, ptr %2, align 8
  br label %73

24:                                               ; preds = %1
  %25 = load ptr, ptr %4, align 8
  %26 = getelementptr inbounds %struct.EpList, ptr %25, i32 0, i32 1
  %27 = load i64, ptr %26, align 8
  %28 = add nsw i64 %27, 1
  %29 = call ptr @malloc(i64 noundef %28) #14
  store ptr %29, ptr %6, align 8
  %30 = load ptr, ptr %6, align 8
  %31 = icmp ne ptr %30, null
  br i1 %31, label %43, label %32

32:                                               ; preds = %24
  %33 = call ptr @malloc(i64 noundef 1) #14
  store ptr %33, ptr %7, align 8
  %34 = load ptr, ptr %7, align 8
  %35 = icmp ne ptr %34, null
  br i1 %35, label %36, label %39

36:                                               ; preds = %32
  %37 = load ptr, ptr %7, align 8
  %38 = getelementptr inbounds i8, ptr %37, i64 0
  store i8 0, ptr %38, align 1
  br label %39

39:                                               ; preds = %36, %32
  %40 = load ptr, ptr %7, align 8
  %41 = call ptr @ep_gc_register(ptr noundef %40, i32 noundef 1)
  %42 = load ptr, ptr %7, align 8
  store ptr %42, ptr %2, align 8
  br label %73

43:                                               ; preds = %24
  store i64 0, ptr %8, align 8
  br label %44

44:                                               ; preds = %61, %43
  %45 = load i64, ptr %8, align 8
  %46 = load ptr, ptr %4, align 8
  %47 = getelementptr inbounds %struct.EpList, ptr %46, i32 0, i32 1
  %48 = load i64, ptr %47, align 8
  %49 = icmp slt i64 %45, %48
  br i1 %49, label %50, label %64

50:                                               ; preds = %44
  %51 = load ptr, ptr %4, align 8
  %52 = getelementptr inbounds %struct.EpList, ptr %51, i32 0, i32 0
  %53 = load ptr, ptr %52, align 8
  %54 = load i64, ptr %8, align 8
  %55 = getelementptr inbounds i64, ptr %53, i64 %54
  %56 = load i64, ptr %55, align 8
  %57 = trunc i64 %56 to i8
  %58 = load ptr, ptr %6, align 8
  %59 = load i64, ptr %8, align 8
  %60 = getelementptr inbounds i8, ptr %58, i64 %59
  store i8 %57, ptr %60, align 1
  br label %61

61:                                               ; preds = %50
  %62 = load i64, ptr %8, align 8
  %63 = add nsw i64 %62, 1
  store i64 %63, ptr %8, align 8
  br label %44, !llvm.loop !41

64:                                               ; preds = %44
  %65 = load ptr, ptr %6, align 8
  %66 = load ptr, ptr %4, align 8
  %67 = getelementptr inbounds %struct.EpList, ptr %66, i32 0, i32 1
  %68 = load i64, ptr %67, align 8
  %69 = getelementptr inbounds i8, ptr %65, i64 %68
  store i8 0, ptr %69, align 1
  %70 = load ptr, ptr %6, align 8
  %71 = call ptr @ep_gc_register(ptr noundef %70, i32 noundef 1)
  %72 = load ptr, ptr %6, align 8
  store ptr %72, ptr %2, align 8
  br label %73

73:                                               ; preds = %64, %39, %20
  %74 = load ptr, ptr %2, align 8
  ret ptr %74
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @pop_list(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %14

9:                                                ; preds = %1
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.EpList, ptr %10, i32 0, i32 1
  %12 = load i64, ptr %11, align 8
  %13 = icmp sle i64 %12, 0
  br i1 %13, label %14, label %15

14:                                               ; preds = %9, %1
  store i64 0, ptr %2, align 8
  br label %28

15:                                               ; preds = %9
  %16 = load ptr, ptr %4, align 8
  %17 = getelementptr inbounds %struct.EpList, ptr %16, i32 0, i32 1
  %18 = load i64, ptr %17, align 8
  %19 = sub nsw i64 %18, 1
  store i64 %19, ptr %17, align 8
  %20 = load ptr, ptr %4, align 8
  %21 = getelementptr inbounds %struct.EpList, ptr %20, i32 0, i32 0
  %22 = load ptr, ptr %21, align 8
  %23 = load ptr, ptr %4, align 8
  %24 = getelementptr inbounds %struct.EpList, ptr %23, i32 0, i32 1
  %25 = load i64, ptr %24, align 8
  %26 = getelementptr inbounds i64, ptr %22, i64 %25
  %27 = load i64, ptr %26, align 8
  store i64 %27, ptr %2, align 8
  br label %28

28:                                               ; preds = %15, %14
  %29 = load i64, ptr %2, align 8
  ret i64 %29
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @remove_list(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %4, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %6, align 8
  %11 = load ptr, ptr %6, align 8
  %12 = icmp ne ptr %11, null
  br i1 %12, label %13, label %22

13:                                               ; preds = %2
  %14 = load i64, ptr %5, align 8
  %15 = icmp slt i64 %14, 0
  br i1 %15, label %22, label %16

16:                                               ; preds = %13
  %17 = load i64, ptr %5, align 8
  %18 = load ptr, ptr %6, align 8
  %19 = getelementptr inbounds %struct.EpList, ptr %18, i32 0, i32 1
  %20 = load i64, ptr %19, align 8
  %21 = icmp sge i64 %17, %20
  br i1 %21, label %22, label %23

22:                                               ; preds = %16, %13, %2
  store i64 0, ptr %3, align 8
  br label %60

23:                                               ; preds = %16
  %24 = load ptr, ptr %6, align 8
  %25 = getelementptr inbounds %struct.EpList, ptr %24, i32 0, i32 0
  %26 = load ptr, ptr %25, align 8
  %27 = load i64, ptr %5, align 8
  %28 = getelementptr inbounds i64, ptr %26, i64 %27
  %29 = load i64, ptr %28, align 8
  store i64 %29, ptr %7, align 8
  %30 = load i64, ptr %5, align 8
  store i64 %30, ptr %8, align 8
  br label %31

31:                                               ; preds = %51, %23
  %32 = load i64, ptr %8, align 8
  %33 = load ptr, ptr %6, align 8
  %34 = getelementptr inbounds %struct.EpList, ptr %33, i32 0, i32 1
  %35 = load i64, ptr %34, align 8
  %36 = sub nsw i64 %35, 1
  %37 = icmp slt i64 %32, %36
  br i1 %37, label %38, label %54

38:                                               ; preds = %31
  %39 = load ptr, ptr %6, align 8
  %40 = getelementptr inbounds %struct.EpList, ptr %39, i32 0, i32 0
  %41 = load ptr, ptr %40, align 8
  %42 = load i64, ptr %8, align 8
  %43 = add nsw i64 %42, 1
  %44 = getelementptr inbounds i64, ptr %41, i64 %43
  %45 = load i64, ptr %44, align 8
  %46 = load ptr, ptr %6, align 8
  %47 = getelementptr inbounds %struct.EpList, ptr %46, i32 0, i32 0
  %48 = load ptr, ptr %47, align 8
  %49 = load i64, ptr %8, align 8
  %50 = getelementptr inbounds i64, ptr %48, i64 %49
  store i64 %45, ptr %50, align 8
  br label %51

51:                                               ; preds = %38
  %52 = load i64, ptr %8, align 8
  %53 = add nsw i64 %52, 1
  store i64 %53, ptr %8, align 8
  br label %31, !llvm.loop !42

54:                                               ; preds = %31
  %55 = load ptr, ptr %6, align 8
  %56 = getelementptr inbounds %struct.EpList, ptr %55, i32 0, i32 1
  %57 = load i64, ptr %56, align 8
  %58 = sub nsw i64 %57, 1
  store i64 %58, ptr %56, align 8
  %59 = load i64, ptr %7, align 8
  store i64 %59, ptr %3, align 8
  br label %60

60:                                               ; preds = %54, %22
  %61 = load i64, ptr %3, align 8
  ret i64 %61
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @display_string(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  %4 = icmp ne ptr %3, null
  br i1 %4, label %5, label %8

5:                                                ; preds = %1
  %6 = load ptr, ptr %2, align 8
  %7 = call i32 @puts(ptr noundef %6)
  br label %8

8:                                                ; preds = %5, %1
  ret i64 0
}

declare i32 @puts(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_read_file(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %8 = load i64, ptr %3, align 8
  %9 = inttoptr i64 %8 to ptr
  store ptr %9, ptr %4, align 8
  %10 = load ptr, ptr %4, align 8
  %11 = call ptr @"\01_fopen"(ptr noundef %10, ptr noundef @.str.3)
  store ptr %11, ptr %5, align 8
  %12 = load ptr, ptr %5, align 8
  %13 = icmp ne ptr %12, null
  br i1 %13, label %15, label %14

14:                                               ; preds = %1
  store i64 ptrtoint (ptr @.str.5 to i64), ptr %2, align 8
  br label %36

15:                                               ; preds = %1
  %16 = load ptr, ptr %5, align 8
  %17 = call i32 @fseek(ptr noundef %16, i64 noundef 0, i32 noundef 2)
  %18 = load ptr, ptr %5, align 8
  %19 = call i64 @ftell(ptr noundef %18)
  store i64 %19, ptr %6, align 8
  %20 = load ptr, ptr %5, align 8
  %21 = call i32 @fseek(ptr noundef %20, i64 noundef 0, i32 noundef 0)
  %22 = load i64, ptr %6, align 8
  %23 = add nsw i64 %22, 1
  %24 = call ptr @malloc(i64 noundef %23) #14
  store ptr %24, ptr %7, align 8
  %25 = load ptr, ptr %7, align 8
  %26 = load i64, ptr %6, align 8
  %27 = load ptr, ptr %5, align 8
  %28 = call i64 @fread(ptr noundef %25, i64 noundef 1, i64 noundef %26, ptr noundef %27)
  %29 = load ptr, ptr %7, align 8
  %30 = load i64, ptr %6, align 8
  %31 = getelementptr inbounds i8, ptr %29, i64 %30
  store i8 0, ptr %31, align 1
  %32 = load ptr, ptr %5, align 8
  %33 = call i32 @fclose(ptr noundef %32)
  %34 = load ptr, ptr %7, align 8
  %35 = ptrtoint ptr %34 to i64
  store i64 %35, ptr %2, align 8
  br label %36

36:                                               ; preds = %15, %14
  %37 = load i64, ptr %2, align 8
  ret i64 %37
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_write_file(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %4, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %6, align 8
  %11 = load i64, ptr %5, align 8
  %12 = inttoptr i64 %11 to ptr
  store ptr %12, ptr %7, align 8
  %13 = load ptr, ptr %6, align 8
  %14 = call ptr @"\01_fopen"(ptr noundef %13, ptr noundef @.str.4)
  store ptr %14, ptr %8, align 8
  %15 = load ptr, ptr %8, align 8
  %16 = icmp ne ptr %15, null
  br i1 %16, label %18, label %17

17:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %24

18:                                               ; preds = %2
  %19 = load ptr, ptr %7, align 8
  %20 = load ptr, ptr %8, align 8
  %21 = call i32 @"\01_fputs"(ptr noundef %19, ptr noundef %20)
  %22 = load ptr, ptr %8, align 8
  %23 = call i32 @fclose(ptr noundef %22)
  store i64 1, ptr %3, align 8
  br label %24

24:                                               ; preds = %18, %17
  %25 = load i64, ptr %3, align 8
  ret i64 %25
}

declare i32 @"\01_fputs"(ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_append_file(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %4, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %6, align 8
  %11 = load i64, ptr %5, align 8
  %12 = inttoptr i64 %11 to ptr
  store ptr %12, ptr %7, align 8
  %13 = load ptr, ptr %6, align 8
  %14 = call ptr @"\01_fopen"(ptr noundef %13, ptr noundef @.str.19)
  store ptr %14, ptr %8, align 8
  %15 = load ptr, ptr %8, align 8
  %16 = icmp ne ptr %15, null
  br i1 %16, label %18, label %17

17:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %24

18:                                               ; preds = %2
  %19 = load ptr, ptr %7, align 8
  %20 = load ptr, ptr %8, align 8
  %21 = call i32 @"\01_fputs"(ptr noundef %19, ptr noundef %20)
  %22 = load ptr, ptr %8, align 8
  %23 = call i32 @fclose(ptr noundef %22)
  store i64 1, ptr %3, align 8
  br label %24

24:                                               ; preds = %18, %17
  %25 = load i64, ptr %3, align 8
  ret i64 %25
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_file_exists(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  %4 = alloca %struct.stat, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %3, align 8
  %7 = load ptr, ptr %3, align 8
  %8 = call i32 @"\01_stat"(ptr noundef %7, ptr noundef %4)
  %9 = icmp eq i32 %8, 0
  %10 = zext i1 %9 to i64
  %11 = select i1 %9, i32 1, i32 0
  %12 = sext i32 %11 to i64
  ret i64 %12
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_is_directory(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca %struct.stat, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = call i32 @"\01_stat"(ptr noundef %8, ptr noundef %5)
  %10 = icmp ne i32 %9, 0
  br i1 %10, label %11, label %12

11:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %21

12:                                               ; preds = %1
  %13 = getelementptr inbounds %struct.stat, ptr %5, i32 0, i32 1
  %14 = load i16, ptr %13, align 4
  %15 = zext i16 %14 to i32
  %16 = and i32 %15, 61440
  %17 = icmp eq i32 %16, 16384
  %18 = zext i1 %17 to i64
  %19 = select i1 %17, i32 1, i32 0
  %20 = sext i32 %19 to i64
  store i64 %20, ptr %2, align 8
  br label %21

21:                                               ; preds = %12, %11
  %22 = load i64, ptr %2, align 8
  ret i64 %22
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_file_size(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca %struct.stat, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = call i32 @"\01_stat"(ptr noundef %8, ptr noundef %5)
  %10 = icmp ne i32 %9, 0
  br i1 %10, label %11, label %12

11:                                               ; preds = %1
  store i64 -1, ptr %2, align 8
  br label %15

12:                                               ; preds = %1
  %13 = getelementptr inbounds %struct.stat, ptr %5, i32 0, i32 11
  %14 = load i64, ptr %13, align 8
  store i64 %14, ptr %2, align 8
  br label %15

15:                                               ; preds = %12, %11
  %16 = load i64, ptr %2, align 8
  ret i64 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_list_directory(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %9 = load i64, ptr %3, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %4, align 8
  %11 = load ptr, ptr %4, align 8
  %12 = call ptr @"\01_opendir"(ptr noundef %11)
  store ptr %12, ptr %5, align 8
  %13 = load ptr, ptr %5, align 8
  %14 = icmp ne ptr %13, null
  br i1 %14, label %17, label %15

15:                                               ; preds = %1
  %16 = call i64 @create_list()
  store i64 %16, ptr %2, align 8
  br label %65

17:                                               ; preds = %1
  %18 = call i64 @create_list()
  store i64 %18, ptr %6, align 8
  br label %19

19:                                               ; preds = %52, %51, %17
  %20 = load ptr, ptr %5, align 8
  %21 = call ptr @"\01_readdir"(ptr noundef %20)
  store ptr %21, ptr %7, align 8
  %22 = icmp ne ptr %21, null
  br i1 %22, label %23, label %61

23:                                               ; preds = %19
  %24 = load ptr, ptr %7, align 8
  %25 = getelementptr inbounds %struct.dirent, ptr %24, i32 0, i32 5
  %26 = getelementptr inbounds [1024 x i8], ptr %25, i64 0, i64 0
  %27 = load i8, ptr %26, align 1
  %28 = sext i8 %27 to i32
  %29 = icmp eq i32 %28, 46
  br i1 %29, label %30, label %52

30:                                               ; preds = %23
  %31 = load ptr, ptr %7, align 8
  %32 = getelementptr inbounds %struct.dirent, ptr %31, i32 0, i32 5
  %33 = getelementptr inbounds [1024 x i8], ptr %32, i64 0, i64 1
  %34 = load i8, ptr %33, align 1
  %35 = sext i8 %34 to i32
  %36 = icmp eq i32 %35, 0
  br i1 %36, label %51, label %37

37:                                               ; preds = %30
  %38 = load ptr, ptr %7, align 8
  %39 = getelementptr inbounds %struct.dirent, ptr %38, i32 0, i32 5
  %40 = getelementptr inbounds [1024 x i8], ptr %39, i64 0, i64 1
  %41 = load i8, ptr %40, align 1
  %42 = sext i8 %41 to i32
  %43 = icmp eq i32 %42, 46
  br i1 %43, label %44, label %52

44:                                               ; preds = %37
  %45 = load ptr, ptr %7, align 8
  %46 = getelementptr inbounds %struct.dirent, ptr %45, i32 0, i32 5
  %47 = getelementptr inbounds [1024 x i8], ptr %46, i64 0, i64 2
  %48 = load i8, ptr %47, align 1
  %49 = sext i8 %48 to i32
  %50 = icmp eq i32 %49, 0
  br i1 %50, label %51, label %52

51:                                               ; preds = %44, %30
  br label %19, !llvm.loop !43

52:                                               ; preds = %44, %37, %23
  %53 = load ptr, ptr %7, align 8
  %54 = getelementptr inbounds %struct.dirent, ptr %53, i32 0, i32 5
  %55 = getelementptr inbounds [1024 x i8], ptr %54, i64 0, i64 0
  %56 = call ptr @strdup(ptr noundef %55) #13
  store ptr %56, ptr %8, align 8
  %57 = load i64, ptr %6, align 8
  %58 = load ptr, ptr %8, align 8
  %59 = ptrtoint ptr %58 to i64
  %60 = call i64 @append_list(i64 noundef %57, i64 noundef %59)
  br label %19, !llvm.loop !43

61:                                               ; preds = %19
  %62 = load ptr, ptr %5, align 8
  %63 = call i32 @"\01_closedir"(ptr noundef %62)
  %64 = load i64, ptr %6, align 8
  store i64 %64, ptr %2, align 8
  br label %65

65:                                               ; preds = %61, %15
  %66 = load i64, ptr %2, align 8
  ret i64 %66
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_create_directory(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  store ptr %5, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = call i32 @mkdir(ptr noundef %6, i16 noundef zeroext 493)
  %8 = icmp eq i32 %7, 0
  %9 = zext i1 %8 to i64
  %10 = select i1 %8, i32 1, i32 0
  %11 = sext i32 %10 to i64
  ret i64 %11
}

declare i32 @mkdir(ptr noundef, i16 noundef zeroext) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_remove_file(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  store ptr %5, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = call i32 @remove(ptr noundef %6)
  %8 = icmp eq i32 %7, 0
  %9 = zext i1 %8 to i64
  %10 = select i1 %8, i32 1, i32 0
  %11 = sext i32 %10 to i64
  ret i64 %11
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_remove_directory(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  store ptr %5, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = call i32 @rmdir(ptr noundef %6)
  %8 = icmp eq i32 %7, 0
  %9 = zext i1 %8 to i64
  %10 = select i1 %8, i32 1, i32 0
  %11 = sext i32 %10 to i64
  ret i64 %11
}

declare i32 @rmdir(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_rename_file(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = call i32 @rename(ptr noundef %6, ptr noundef %8)
  %10 = icmp eq i32 %9, 0
  %11 = zext i1 %10 to i64
  %12 = select i1 %10, i32 1, i32 0
  %13 = sext i32 %12 to i64
  ret i64 %13
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_copy_file(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  %9 = alloca ptr, align 8
  %10 = alloca [8192 x i8], align 1
  %11 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %12 = load i64, ptr %4, align 8
  %13 = inttoptr i64 %12 to ptr
  store ptr %13, ptr %6, align 8
  %14 = load i64, ptr %5, align 8
  %15 = inttoptr i64 %14 to ptr
  store ptr %15, ptr %7, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = call ptr @"\01_fopen"(ptr noundef %16, ptr noundef @.str.3)
  store ptr %17, ptr %8, align 8
  %18 = load ptr, ptr %8, align 8
  %19 = icmp ne ptr %18, null
  br i1 %19, label %21, label %20

20:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %45

21:                                               ; preds = %2
  %22 = load ptr, ptr %7, align 8
  %23 = call ptr @"\01_fopen"(ptr noundef %22, ptr noundef @.str.4)
  store ptr %23, ptr %9, align 8
  %24 = load ptr, ptr %9, align 8
  %25 = icmp ne ptr %24, null
  br i1 %25, label %29, label %26

26:                                               ; preds = %21
  %27 = load ptr, ptr %8, align 8
  %28 = call i32 @fclose(ptr noundef %27)
  store i64 0, ptr %3, align 8
  br label %45

29:                                               ; preds = %21
  br label %30

30:                                               ; preds = %35, %29
  %31 = getelementptr inbounds [8192 x i8], ptr %10, i64 0, i64 0
  %32 = load ptr, ptr %8, align 8
  %33 = call i64 @fread(ptr noundef %31, i64 noundef 1, i64 noundef 8192, ptr noundef %32)
  store i64 %33, ptr %11, align 8
  %34 = icmp ugt i64 %33, 0
  br i1 %34, label %35, label %40

35:                                               ; preds = %30
  %36 = getelementptr inbounds [8192 x i8], ptr %10, i64 0, i64 0
  %37 = load i64, ptr %11, align 8
  %38 = load ptr, ptr %9, align 8
  %39 = call i64 @"\01_fwrite"(ptr noundef %36, i64 noundef 1, i64 noundef %37, ptr noundef %38)
  br label %30, !llvm.loop !44

40:                                               ; preds = %30
  %41 = load ptr, ptr %8, align 8
  %42 = call i32 @fclose(ptr noundef %41)
  %43 = load ptr, ptr %9, align 8
  %44 = call i32 @fclose(ptr noundef %43)
  store i64 1, ptr %3, align 8
  br label %45

45:                                               ; preds = %40, %26, %20
  %46 = load i64, ptr %3, align 8
  ret i64 %46
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_now_ms() #0 {
  %1 = alloca %struct.timeval, align 8
  %2 = call i32 @gettimeofday(ptr noundef %1, ptr noundef null)
  %3 = getelementptr inbounds %struct.timeval, ptr %1, i32 0, i32 0
  %4 = load i64, ptr %3, align 8
  %5 = mul nsw i64 %4, 1000
  %6 = getelementptr inbounds %struct.timeval, ptr %1, i32 0, i32 1
  %7 = load i32, ptr %6, align 8
  %8 = sext i32 %7 to i64
  %9 = sdiv i64 %8, 1000
  %10 = add nsw i64 %5, %9
  ret i64 %10
}

declare i32 @gettimeofday(ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_now_sec() #0 {
  %1 = call i64 @time(ptr noundef null)
  ret i64 %1
}

declare i64 @time(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_year(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  store i64 %5, ptr %3, align 8
  %6 = call ptr @localtime(ptr noundef %3)
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %14

9:                                                ; preds = %1
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.tm, ptr %10, i32 0, i32 5
  %12 = load i32, ptr %11, align 4
  %13 = add nsw i32 %12, 1900
  br label %15

14:                                               ; preds = %1
  br label %15

15:                                               ; preds = %14, %9
  %16 = phi i32 [ %13, %9 ], [ 0, %14 ]
  %17 = sext i32 %16 to i64
  ret i64 %17
}

declare ptr @localtime(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_month(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  store i64 %5, ptr %3, align 8
  %6 = call ptr @localtime(ptr noundef %3)
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %14

9:                                                ; preds = %1
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.tm, ptr %10, i32 0, i32 4
  %12 = load i32, ptr %11, align 8
  %13 = add nsw i32 %12, 1
  br label %15

14:                                               ; preds = %1
  br label %15

15:                                               ; preds = %14, %9
  %16 = phi i32 [ %13, %9 ], [ 0, %14 ]
  %17 = sext i32 %16 to i64
  ret i64 %17
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_day(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  store i64 %5, ptr %3, align 8
  %6 = call ptr @localtime(ptr noundef %3)
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %13

9:                                                ; preds = %1
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.tm, ptr %10, i32 0, i32 3
  %12 = load i32, ptr %11, align 4
  br label %14

13:                                               ; preds = %1
  br label %14

14:                                               ; preds = %13, %9
  %15 = phi i32 [ %12, %9 ], [ 0, %13 ]
  %16 = sext i32 %15 to i64
  ret i64 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_hour(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  store i64 %5, ptr %3, align 8
  %6 = call ptr @localtime(ptr noundef %3)
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %13

9:                                                ; preds = %1
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.tm, ptr %10, i32 0, i32 2
  %12 = load i32, ptr %11, align 8
  br label %14

13:                                               ; preds = %1
  br label %14

14:                                               ; preds = %13, %9
  %15 = phi i32 [ %12, %9 ], [ 0, %13 ]
  %16 = sext i32 %15 to i64
  ret i64 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_minute(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  store i64 %5, ptr %3, align 8
  %6 = call ptr @localtime(ptr noundef %3)
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %13

9:                                                ; preds = %1
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.tm, ptr %10, i32 0, i32 1
  %12 = load i32, ptr %11, align 4
  br label %14

13:                                               ; preds = %1
  br label %14

14:                                               ; preds = %13, %9
  %15 = phi i32 [ %12, %9 ], [ 0, %13 ]
  %16 = sext i32 %15 to i64
  ret i64 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_second(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  store i64 %5, ptr %3, align 8
  %6 = call ptr @localtime(ptr noundef %3)
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %13

9:                                                ; preds = %1
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.tm, ptr %10, i32 0, i32 0
  %12 = load i32, ptr %11, align 8
  br label %14

13:                                               ; preds = %1
  br label %14

14:                                               ; preds = %13, %9
  %15 = phi i32 [ %12, %9 ], [ 0, %13 ]
  %16 = sext i32 %15 to i64
  ret i64 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_time_weekday(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  store i64 %5, ptr %3, align 8
  %6 = call ptr @localtime(ptr noundef %3)
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %13

9:                                                ; preds = %1
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.tm, ptr %10, i32 0, i32 6
  %12 = load i32, ptr %11, align 8
  br label %14

13:                                               ; preds = %1
  br label %14

14:                                               ; preds = %13, %9
  %15 = phi i32 [ %12, %9 ], [ 0, %13 ]
  %16 = sext i32 %15 to i64
  ret i64 %16
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_format_time(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %4, align 8
  store i64 %9, ptr %6, align 8
  %10 = call ptr @localtime(ptr noundef %6)
  store ptr %10, ptr %7, align 8
  %11 = load ptr, ptr %7, align 8
  %12 = icmp ne ptr %11, null
  br i1 %12, label %14, label %13

13:                                               ; preds = %2
  store i64 ptrtoint (ptr @.str.5 to i64), ptr %3, align 8
  br label %23

14:                                               ; preds = %2
  %15 = call ptr @malloc(i64 noundef 256) #14
  store ptr %15, ptr %8, align 8
  %16 = load ptr, ptr %8, align 8
  %17 = load i64, ptr %5, align 8
  %18 = inttoptr i64 %17 to ptr
  %19 = load ptr, ptr %7, align 8
  %20 = call i64 @"\01_strftime"(ptr noundef %16, i64 noundef 256, ptr noundef %18, ptr noundef %19)
  %21 = load ptr, ptr %8, align 8
  %22 = ptrtoint ptr %21 to i64
  store i64 %22, ptr %3, align 8
  br label %23

23:                                               ; preds = %14, %13
  %24 = load i64, ptr %3, align 8
  ret i64 %24
}

declare i64 @"\01_strftime"(ptr noundef, i64 noundef, ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_getenv(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  %6 = call ptr @getenv(ptr noundef %5)
  store ptr %6, ptr %3, align 8
  %7 = load ptr, ptr %3, align 8
  %8 = icmp ne ptr %7, null
  br i1 %8, label %9, label %12

9:                                                ; preds = %1
  %10 = load ptr, ptr %3, align 8
  %11 = ptrtoint ptr %10 to i64
  br label %13

12:                                               ; preds = %1
  br label %13

13:                                               ; preds = %12, %9
  %14 = phi i64 [ %11, %9 ], [ ptrtoint (ptr @.str.5 to i64), %12 ]
  ret i64 %14
}

declare ptr @getenv(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_setenv(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = call i32 @"\01_setenv"(ptr noundef %6, ptr noundef %8, i32 noundef 1)
  %10 = icmp eq i32 %9, 0
  %11 = zext i1 %10 to i64
  %12 = select i1 %10, i32 1, i32 0
  %13 = sext i32 %12 to i64
  ret i64 %13
}

declare i32 @"\01_setenv"(ptr noundef, ptr noundef, i32 noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_get_cwd() #0 {
  %1 = alloca i64, align 8
  %2 = alloca ptr, align 8
  %3 = call ptr @malloc(i64 noundef 4096) #14
  store ptr %3, ptr %2, align 8
  %4 = load ptr, ptr %2, align 8
  %5 = call ptr @getcwd(ptr noundef %4, i64 noundef 4096)
  %6 = icmp ne ptr %5, null
  br i1 %6, label %7, label %10

7:                                                ; preds = %0
  %8 = load ptr, ptr %2, align 8
  %9 = ptrtoint ptr %8 to i64
  store i64 %9, ptr %1, align 8
  br label %12

10:                                               ; preds = %0
  %11 = load ptr, ptr %2, align 8
  call void @free(ptr noundef %11)
  store i64 ptrtoint (ptr @.str.5 to i64), ptr %1, align 8
  br label %12

12:                                               ; preds = %10, %7
  %13 = load i64, ptr %1, align 8
  ret i64 %13
}

declare ptr @getcwd(ptr noundef, i64 noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_os_name() #0 {
  ret i64 ptrtoint (ptr @.str.20 to i64)
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_arch_name() #0 {
  ret i64 ptrtoint (ptr @.str.21 to i64)
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_exit(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = trunc i64 %3 to i32
  call void @exit(i32 noundef %4) #12
  unreachable
}

; Function Attrs: noreturn
declare void @exit(i32 noundef) #9

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_get_pid() #0 {
  %1 = call i32 @getpid()
  %2 = sext i32 %1 to i64
  ret i64 %2
}

declare i32 @getpid() #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_get_home_dir() #0 {
  %1 = alloca ptr, align 8
  %2 = call ptr @getenv(ptr noundef @.str.22)
  store ptr %2, ptr %1, align 8
  %3 = load ptr, ptr %1, align 8
  %4 = icmp ne ptr %3, null
  br i1 %4, label %5, label %8

5:                                                ; preds = %0
  %6 = load ptr, ptr %1, align 8
  %7 = ptrtoint ptr %6 to i64
  br label %9

8:                                                ; preds = %0
  br label %9

9:                                                ; preds = %8, %5
  %10 = phi i64 [ %7, %5 ], [ ptrtoint (ptr @.str.5 to i64), %8 ]
  ret i64 %10
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_run_command(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  %8 = alloca [4096 x i8], align 1
  %9 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %10 = load i64, ptr %3, align 8
  %11 = inttoptr i64 %10 to ptr
  store ptr %11, ptr %4, align 8
  %12 = load ptr, ptr %4, align 8
  %13 = call ptr @"\01_popen"(ptr noundef %12, ptr noundef @.str.23)
  store ptr %13, ptr %5, align 8
  %14 = load ptr, ptr %5, align 8
  %15 = icmp ne ptr %14, null
  br i1 %15, label %17, label %16

16:                                               ; preds = %1
  store i64 ptrtoint (ptr @.str.5 to i64), ptr %2, align 8
  br label %48

17:                                               ; preds = %1
  %18 = call ptr @malloc(i64 noundef 65536) #14
  store ptr %18, ptr %6, align 8
  store i64 0, ptr %7, align 8
  br label %19

19:                                               ; preds = %24, %17
  %20 = getelementptr inbounds [4096 x i8], ptr %8, i64 0, i64 0
  %21 = load ptr, ptr %5, align 8
  %22 = call ptr @fgets(ptr noundef %20, i32 noundef 4096, ptr noundef %21)
  %23 = icmp ne ptr %22, null
  br i1 %23, label %24, label %40

24:                                               ; preds = %19
  %25 = getelementptr inbounds [4096 x i8], ptr %8, i64 0, i64 0
  %26 = call i64 @strlen(ptr noundef %25) #13
  store i64 %26, ptr %9, align 8
  %27 = load ptr, ptr %6, align 8
  %28 = load i64, ptr %7, align 8
  %29 = getelementptr inbounds i8, ptr %27, i64 %28
  %30 = getelementptr inbounds [4096 x i8], ptr %8, i64 0, i64 0
  %31 = load i64, ptr %9, align 8
  %32 = load ptr, ptr %6, align 8
  %33 = load i64, ptr %7, align 8
  %34 = getelementptr inbounds i8, ptr %32, i64 %33
  %35 = call i64 @llvm.objectsize.i64.p0(ptr %34, i1 false, i1 true, i1 false)
  %36 = call ptr @__memcpy_chk(ptr noundef %29, ptr noundef %30, i64 noundef %31, i64 noundef %35) #13
  %37 = load i64, ptr %9, align 8
  %38 = load i64, ptr %7, align 8
  %39 = add i64 %38, %37
  store i64 %39, ptr %7, align 8
  br label %19, !llvm.loop !45

40:                                               ; preds = %19
  %41 = load ptr, ptr %6, align 8
  %42 = load i64, ptr %7, align 8
  %43 = getelementptr inbounds i8, ptr %41, i64 %42
  store i8 0, ptr %43, align 1
  %44 = load ptr, ptr %5, align 8
  %45 = call i32 @pclose(ptr noundef %44)
  %46 = load ptr, ptr %6, align 8
  %47 = ptrtoint ptr %46 to i64
  store i64 %47, ptr %2, align 8
  br label %48

48:                                               ; preds = %40, %16
  %49 = load i64, ptr %2, align 8
  ret i64 %49
}

declare ptr @"\01_popen"(ptr noundef, ptr noundef) #1

declare ptr @fgets(ptr noundef, i32 noundef, ptr noundef) #1

declare i32 @pclose(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_hash_string(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i32, align 4
  store i64 %0, ptr %3, align 8
  %7 = load i64, ptr %3, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %4, align 8
  %9 = load ptr, ptr %4, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %12, label %11

11:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %29

12:                                               ; preds = %1
  store i64 5381, ptr %5, align 8
  br label %13

13:                                               ; preds = %19, %12
  %14 = load ptr, ptr %4, align 8
  %15 = getelementptr inbounds i8, ptr %14, i32 1
  store ptr %15, ptr %4, align 8
  %16 = load i8, ptr %14, align 1
  %17 = sext i8 %16 to i32
  store i32 %17, ptr %6, align 4
  %18 = icmp ne i32 %17, 0
  br i1 %18, label %19, label %27

19:                                               ; preds = %13
  %20 = load i64, ptr %5, align 8
  %21 = shl i64 %20, 5
  %22 = load i64, ptr %5, align 8
  %23 = add i64 %21, %22
  %24 = load i32, ptr %6, align 4
  %25 = sext i32 %24 to i64
  %26 = add i64 %23, %25
  store i64 %26, ptr %5, align 8
  br label %13, !llvm.loop !46

27:                                               ; preds = %13
  %28 = load i64, ptr %5, align 8
  store i64 %28, ptr %2, align 8
  br label %29

29:                                               ; preds = %27, %11
  %30 = load i64, ptr %2, align 8
  ret i64 %30
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_str_equals(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %6 = load i64, ptr %4, align 8
  %7 = load i64, ptr %5, align 8
  %8 = icmp eq i64 %6, %7
  br i1 %8, label %9, label %10

9:                                                ; preds = %2
  store i64 1, ptr %3, align 8
  br label %34

10:                                               ; preds = %2
  %11 = load i64, ptr %4, align 8
  %12 = icmp ne i64 %11, 0
  br i1 %12, label %13, label %16

13:                                               ; preds = %10
  %14 = load i64, ptr %5, align 8
  %15 = icmp ne i64 %14, 0
  br i1 %15, label %17, label %16

16:                                               ; preds = %13, %10
  store i64 0, ptr %3, align 8
  br label %34

17:                                               ; preds = %13
  %18 = load i64, ptr %4, align 8
  %19 = icmp ult i64 %18, 4096
  br i1 %19, label %23, label %20

20:                                               ; preds = %17
  %21 = load i64, ptr %5, align 8
  %22 = icmp ult i64 %21, 4096
  br i1 %22, label %23, label %24

23:                                               ; preds = %20, %17
  store i64 0, ptr %3, align 8
  br label %34

24:                                               ; preds = %20
  %25 = load i64, ptr %4, align 8
  %26 = inttoptr i64 %25 to ptr
  %27 = load i64, ptr %5, align 8
  %28 = inttoptr i64 %27 to ptr
  %29 = call i32 @strcmp(ptr noundef %26, ptr noundef %28) #13
  %30 = icmp eq i32 %29, 0
  %31 = zext i1 %30 to i64
  %32 = select i1 %30, i32 1, i32 0
  %33 = sext i32 %32 to i64
  store i64 %33, ptr %3, align 8
  br label %34

34:                                               ; preds = %24, %23, %16, %9
  %35 = load i64, ptr %3, align 8
  ret i64 %35
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_mutex_create() #0 {
  %1 = alloca ptr, align 8
  %2 = call ptr @malloc(i64 noundef 64) #14
  store ptr %2, ptr %1, align 8
  %3 = load ptr, ptr %1, align 8
  %4 = call i32 @pthread_mutex_init(ptr noundef %3, ptr noundef null)
  %5 = load ptr, ptr %1, align 8
  %6 = ptrtoint ptr %5 to i64
  ret i64 %6
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_mutex_lock_fn(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @pthread_mutex_lock(ptr noundef %4)
  %6 = icmp eq i32 %5, 0
  %7 = zext i1 %6 to i64
  %8 = select i1 %6, i32 1, i32 0
  %9 = sext i32 %8 to i64
  ret i64 %9
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_mutex_unlock_fn(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @pthread_mutex_unlock(ptr noundef %4)
  %6 = icmp eq i32 %5, 0
  %7 = zext i1 %6 to i64
  %8 = select i1 %6, i32 1, i32 0
  %9 = sext i32 %8 to i64
  ret i64 %9
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_mutex_trylock(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @pthread_mutex_trylock(ptr noundef %4)
  %6 = icmp eq i32 %5, 0
  %7 = zext i1 %6 to i64
  %8 = select i1 %6, i32 1, i32 0
  %9 = sext i32 %8 to i64
  ret i64 %9
}

declare i32 @pthread_mutex_trylock(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_mutex_destroy(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @pthread_mutex_destroy(ptr noundef %4)
  %6 = load i64, ptr %2, align 8
  %7 = inttoptr i64 %6 to ptr
  call void @free(ptr noundef %7)
  ret i64 0
}

declare i32 @pthread_mutex_destroy(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_rwlock_create() #0 {
  %1 = alloca ptr, align 8
  %2 = call ptr @malloc(i64 noundef 200) #14
  store ptr %2, ptr %1, align 8
  %3 = load ptr, ptr %1, align 8
  %4 = call i32 @"\01_pthread_rwlock_init"(ptr noundef %3, ptr noundef null)
  %5 = load ptr, ptr %1, align 8
  %6 = ptrtoint ptr %5 to i64
  ret i64 %6
}

declare i32 @"\01_pthread_rwlock_init"(ptr noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_rwlock_read_lock(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @"\01_pthread_rwlock_rdlock"(ptr noundef %4)
  %6 = icmp eq i32 %5, 0
  %7 = zext i1 %6 to i64
  %8 = select i1 %6, i32 1, i32 0
  %9 = sext i32 %8 to i64
  ret i64 %9
}

declare i32 @"\01_pthread_rwlock_rdlock"(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_rwlock_write_lock(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @"\01_pthread_rwlock_wrlock"(ptr noundef %4)
  %6 = icmp eq i32 %5, 0
  %7 = zext i1 %6 to i64
  %8 = select i1 %6, i32 1, i32 0
  %9 = sext i32 %8 to i64
  ret i64 %9
}

declare i32 @"\01_pthread_rwlock_wrlock"(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_rwlock_unlock(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @"\01_pthread_rwlock_unlock"(ptr noundef %4)
  %6 = icmp eq i32 %5, 0
  %7 = zext i1 %6 to i64
  %8 = select i1 %6, i32 1, i32 0
  %9 = sext i32 %8 to i64
  ret i64 %9
}

declare i32 @"\01_pthread_rwlock_unlock"(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_rwlock_destroy(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @"\01_pthread_rwlock_destroy"(ptr noundef %4)
  %6 = load i64, ptr %2, align 8
  %7 = inttoptr i64 %6 to ptr
  call void @free(ptr noundef %7)
  ret i64 0
}

declare i32 @"\01_pthread_rwlock_destroy"(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_atomic_create(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %5 = call ptr @malloc(i64 noundef 8) #14
  store ptr %5, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = load i64, ptr %2, align 8
  store i64 %7, ptr %4, align 8
  %8 = load i64, ptr %4, align 8
  store atomic i64 %8, ptr %6 seq_cst, align 8
  %9 = load ptr, ptr %3, align 8
  %10 = ptrtoint ptr %9 to i64
  ret i64 %10
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_atomic_load(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  %6 = load atomic i64, ptr %5 seq_cst, align 8
  store i64 %6, ptr %3, align 8
  %7 = load i64, ptr %3, align 8
  ret i64 %7
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_atomic_store(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  %8 = load i64, ptr %4, align 8
  store i64 %8, ptr %5, align 8
  %9 = load i64, ptr %5, align 8
  store atomic i64 %9, ptr %7 seq_cst, align 8
  %10 = load i64, ptr %4, align 8
  ret i64 %10
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_atomic_add(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %7 = load i64, ptr %3, align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = load i64, ptr %4, align 8
  store i64 %9, ptr %5, align 8
  %10 = load i64, ptr %5, align 8
  %11 = atomicrmw add ptr %8, i64 %10 seq_cst, align 8
  store i64 %11, ptr %6, align 8
  %12 = load i64, ptr %6, align 8
  ret i64 %12
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_atomic_sub(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %7 = load i64, ptr %3, align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = load i64, ptr %4, align 8
  store i64 %9, ptr %5, align 8
  %10 = load i64, ptr %5, align 8
  %11 = atomicrmw sub ptr %8, i64 %10 seq_cst, align 8
  store i64 %11, ptr %6, align 8
  %12 = load i64, ptr %6, align 8
  ret i64 %12
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_atomic_cas(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca i8, align 1
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  %10 = load i64, ptr %5, align 8
  store i64 %10, ptr %7, align 8
  %11 = load i64, ptr %4, align 8
  %12 = inttoptr i64 %11 to ptr
  %13 = load i64, ptr %6, align 8
  store i64 %13, ptr %8, align 8
  %14 = load i64, ptr %7, align 8
  %15 = load i64, ptr %8, align 8
  %16 = cmpxchg ptr %12, i64 %14, i64 %15 seq_cst seq_cst, align 8
  %17 = extractvalue { i64, i1 } %16, 0
  %18 = extractvalue { i64, i1 } %16, 1
  br i1 %18, label %20, label %19

19:                                               ; preds = %3
  store i64 %17, ptr %7, align 8
  br label %20

20:                                               ; preds = %19, %3
  %21 = zext i1 %18 to i8
  store i8 %21, ptr %9, align 1
  %22 = load i8, ptr %9, align 1
  %23 = trunc i8 %22 to i1
  %24 = zext i1 %23 to i64
  %25 = select i1 %23, i32 1, i32 0
  %26 = sext i32 %25 to i64
  ret i64 %26
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_barrier_create(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = call ptr @malloc(i64 noundef 128) #14
  store ptr %4, ptr %3, align 8
  %5 = load ptr, ptr %3, align 8
  %6 = getelementptr inbounds %struct.EpBarrier, ptr %5, i32 0, i32 0
  %7 = call i32 @pthread_mutex_init(ptr noundef %6, ptr noundef null)
  %8 = load ptr, ptr %3, align 8
  %9 = getelementptr inbounds %struct.EpBarrier, ptr %8, i32 0, i32 1
  %10 = call i32 @"\01_pthread_cond_init"(ptr noundef %9, ptr noundef null)
  %11 = load ptr, ptr %3, align 8
  %12 = getelementptr inbounds %struct.EpBarrier, ptr %11, i32 0, i32 2
  store i32 0, ptr %12, align 8
  %13 = load i64, ptr %2, align 8
  %14 = trunc i64 %13 to i32
  %15 = load ptr, ptr %3, align 8
  %16 = getelementptr inbounds %struct.EpBarrier, ptr %15, i32 0, i32 3
  store i32 %14, ptr %16, align 4
  %17 = load ptr, ptr %3, align 8
  %18 = getelementptr inbounds %struct.EpBarrier, ptr %17, i32 0, i32 4
  store i32 0, ptr %18, align 8
  %19 = load ptr, ptr %3, align 8
  %20 = ptrtoint ptr %19 to i64
  ret i64 %20
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_barrier_wait(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i32, align 4
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = getelementptr inbounds %struct.EpBarrier, ptr %8, i32 0, i32 0
  %10 = call i32 @pthread_mutex_lock(ptr noundef %9)
  %11 = load ptr, ptr %4, align 8
  %12 = getelementptr inbounds %struct.EpBarrier, ptr %11, i32 0, i32 4
  %13 = load i32, ptr %12, align 8
  store i32 %13, ptr %5, align 4
  %14 = load ptr, ptr %4, align 8
  %15 = getelementptr inbounds %struct.EpBarrier, ptr %14, i32 0, i32 2
  %16 = load i32, ptr %15, align 8
  %17 = add i32 %16, 1
  store i32 %17, ptr %15, align 8
  %18 = load ptr, ptr %4, align 8
  %19 = getelementptr inbounds %struct.EpBarrier, ptr %18, i32 0, i32 2
  %20 = load i32, ptr %19, align 8
  %21 = load ptr, ptr %4, align 8
  %22 = getelementptr inbounds %struct.EpBarrier, ptr %21, i32 0, i32 3
  %23 = load i32, ptr %22, align 4
  %24 = icmp uge i32 %20, %23
  br i1 %24, label %25, label %38

25:                                               ; preds = %1
  %26 = load ptr, ptr %4, align 8
  %27 = getelementptr inbounds %struct.EpBarrier, ptr %26, i32 0, i32 2
  store i32 0, ptr %27, align 8
  %28 = load ptr, ptr %4, align 8
  %29 = getelementptr inbounds %struct.EpBarrier, ptr %28, i32 0, i32 4
  %30 = load i32, ptr %29, align 8
  %31 = add i32 %30, 1
  store i32 %31, ptr %29, align 8
  %32 = load ptr, ptr %4, align 8
  %33 = getelementptr inbounds %struct.EpBarrier, ptr %32, i32 0, i32 1
  %34 = call i32 @pthread_cond_broadcast(ptr noundef %33)
  %35 = load ptr, ptr %4, align 8
  %36 = getelementptr inbounds %struct.EpBarrier, ptr %35, i32 0, i32 0
  %37 = call i32 @pthread_mutex_unlock(ptr noundef %36)
  store i64 1, ptr %2, align 8
  br label %55

38:                                               ; preds = %1
  br label %39

39:                                               ; preds = %45, %38
  %40 = load i32, ptr %5, align 4
  %41 = load ptr, ptr %4, align 8
  %42 = getelementptr inbounds %struct.EpBarrier, ptr %41, i32 0, i32 4
  %43 = load i32, ptr %42, align 8
  %44 = icmp eq i32 %40, %43
  br i1 %44, label %45, label %51

45:                                               ; preds = %39
  %46 = load ptr, ptr %4, align 8
  %47 = getelementptr inbounds %struct.EpBarrier, ptr %46, i32 0, i32 1
  %48 = load ptr, ptr %4, align 8
  %49 = getelementptr inbounds %struct.EpBarrier, ptr %48, i32 0, i32 0
  %50 = call i32 @"\01_pthread_cond_wait"(ptr noundef %47, ptr noundef %49)
  br label %39, !llvm.loop !47

51:                                               ; preds = %39
  %52 = load ptr, ptr %4, align 8
  %53 = getelementptr inbounds %struct.EpBarrier, ptr %52, i32 0, i32 0
  %54 = call i32 @pthread_mutex_unlock(ptr noundef %53)
  store i64 0, ptr %2, align 8
  br label %55

55:                                               ; preds = %51, %25
  %56 = load i64, ptr %2, align 8
  ret i64 %56
}

declare i32 @pthread_cond_broadcast(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_barrier_destroy(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  store ptr %5, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = getelementptr inbounds %struct.EpBarrier, ptr %6, i32 0, i32 0
  %8 = call i32 @pthread_mutex_destroy(ptr noundef %7)
  %9 = load ptr, ptr %3, align 8
  %10 = getelementptr inbounds %struct.EpBarrier, ptr %9, i32 0, i32 1
  %11 = call i32 @pthread_cond_destroy(ptr noundef %10)
  %12 = load ptr, ptr %3, align 8
  call void @free(ptr noundef %12)
  ret i64 0
}

declare i32 @pthread_cond_destroy(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_semaphore_create(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = call ptr @malloc(i64 noundef 120) #14
  store ptr %4, ptr %3, align 8
  %5 = load ptr, ptr %3, align 8
  %6 = getelementptr inbounds %struct.EpSemaphore, ptr %5, i32 0, i32 0
  %7 = call i32 @pthread_mutex_init(ptr noundef %6, ptr noundef null)
  %8 = load ptr, ptr %3, align 8
  %9 = getelementptr inbounds %struct.EpSemaphore, ptr %8, i32 0, i32 1
  %10 = call i32 @"\01_pthread_cond_init"(ptr noundef %9, ptr noundef null)
  %11 = load i64, ptr %2, align 8
  %12 = load ptr, ptr %3, align 8
  %13 = getelementptr inbounds %struct.EpSemaphore, ptr %12, i32 0, i32 2
  store i64 %11, ptr %13, align 8
  %14 = load ptr, ptr %3, align 8
  %15 = ptrtoint ptr %14 to i64
  ret i64 %15
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_semaphore_wait(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  store ptr %5, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = getelementptr inbounds %struct.EpSemaphore, ptr %6, i32 0, i32 0
  %8 = call i32 @pthread_mutex_lock(ptr noundef %7)
  br label %9

9:                                                ; preds = %14, %1
  %10 = load ptr, ptr %3, align 8
  %11 = getelementptr inbounds %struct.EpSemaphore, ptr %10, i32 0, i32 2
  %12 = load i64, ptr %11, align 8
  %13 = icmp sle i64 %12, 0
  br i1 %13, label %14, label %20

14:                                               ; preds = %9
  %15 = load ptr, ptr %3, align 8
  %16 = getelementptr inbounds %struct.EpSemaphore, ptr %15, i32 0, i32 1
  %17 = load ptr, ptr %3, align 8
  %18 = getelementptr inbounds %struct.EpSemaphore, ptr %17, i32 0, i32 0
  %19 = call i32 @"\01_pthread_cond_wait"(ptr noundef %16, ptr noundef %18)
  br label %9, !llvm.loop !48

20:                                               ; preds = %9
  %21 = load ptr, ptr %3, align 8
  %22 = getelementptr inbounds %struct.EpSemaphore, ptr %21, i32 0, i32 2
  %23 = load i64, ptr %22, align 8
  %24 = add nsw i64 %23, -1
  store i64 %24, ptr %22, align 8
  %25 = load ptr, ptr %3, align 8
  %26 = getelementptr inbounds %struct.EpSemaphore, ptr %25, i32 0, i32 0
  %27 = call i32 @pthread_mutex_unlock(ptr noundef %26)
  ret i64 1
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_semaphore_post(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  store ptr %5, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = getelementptr inbounds %struct.EpSemaphore, ptr %6, i32 0, i32 0
  %8 = call i32 @pthread_mutex_lock(ptr noundef %7)
  %9 = load ptr, ptr %3, align 8
  %10 = getelementptr inbounds %struct.EpSemaphore, ptr %9, i32 0, i32 2
  %11 = load i64, ptr %10, align 8
  %12 = add nsw i64 %11, 1
  store i64 %12, ptr %10, align 8
  %13 = load ptr, ptr %3, align 8
  %14 = getelementptr inbounds %struct.EpSemaphore, ptr %13, i32 0, i32 1
  %15 = call i32 @pthread_cond_signal(ptr noundef %14)
  %16 = load ptr, ptr %3, align 8
  %17 = getelementptr inbounds %struct.EpSemaphore, ptr %16, i32 0, i32 0
  %18 = call i32 @pthread_mutex_unlock(ptr noundef %17)
  ret i64 1
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_semaphore_trywait(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  store ptr %6, ptr %4, align 8
  %7 = load ptr, ptr %4, align 8
  %8 = getelementptr inbounds %struct.EpSemaphore, ptr %7, i32 0, i32 0
  %9 = call i32 @pthread_mutex_lock(ptr noundef %8)
  %10 = load ptr, ptr %4, align 8
  %11 = getelementptr inbounds %struct.EpSemaphore, ptr %10, i32 0, i32 2
  %12 = load i64, ptr %11, align 8
  %13 = icmp sgt i64 %12, 0
  br i1 %13, label %14, label %22

14:                                               ; preds = %1
  %15 = load ptr, ptr %4, align 8
  %16 = getelementptr inbounds %struct.EpSemaphore, ptr %15, i32 0, i32 2
  %17 = load i64, ptr %16, align 8
  %18 = add nsw i64 %17, -1
  store i64 %18, ptr %16, align 8
  %19 = load ptr, ptr %4, align 8
  %20 = getelementptr inbounds %struct.EpSemaphore, ptr %19, i32 0, i32 0
  %21 = call i32 @pthread_mutex_unlock(ptr noundef %20)
  store i64 1, ptr %2, align 8
  br label %26

22:                                               ; preds = %1
  %23 = load ptr, ptr %4, align 8
  %24 = getelementptr inbounds %struct.EpSemaphore, ptr %23, i32 0, i32 0
  %25 = call i32 @pthread_mutex_unlock(ptr noundef %24)
  store i64 0, ptr %2, align 8
  br label %26

26:                                               ; preds = %22, %14
  %27 = load i64, ptr %2, align 8
  ret i64 %27
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_semaphore_destroy(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = inttoptr i64 %4 to ptr
  store ptr %5, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = getelementptr inbounds %struct.EpSemaphore, ptr %6, i32 0, i32 0
  %8 = call i32 @pthread_mutex_destroy(ptr noundef %7)
  %9 = load ptr, ptr %3, align 8
  %10 = getelementptr inbounds %struct.EpSemaphore, ptr %9, i32 0, i32 1
  %11 = call i32 @pthread_cond_destroy(ptr noundef %10)
  %12 = load ptr, ptr %3, align 8
  call void @free(ptr noundef %12)
  ret i64 0
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_condvar_create() #0 {
  %1 = alloca ptr, align 8
  %2 = call ptr @malloc(i64 noundef 48) #14
  store ptr %2, ptr %1, align 8
  %3 = load ptr, ptr %1, align 8
  %4 = call i32 @"\01_pthread_cond_init"(ptr noundef %3, ptr noundef null)
  %5 = load ptr, ptr %1, align 8
  %6 = ptrtoint ptr %5 to i64
  ret i64 %6
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_condvar_wait(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = call i32 @"\01_pthread_cond_wait"(ptr noundef %6, ptr noundef %8)
  %10 = icmp eq i32 %9, 0
  %11 = zext i1 %10 to i64
  %12 = select i1 %10, i32 1, i32 0
  %13 = sext i32 %12 to i64
  ret i64 %13
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_condvar_signal(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @pthread_cond_signal(ptr noundef %4)
  %6 = icmp eq i32 %5, 0
  %7 = zext i1 %6 to i64
  %8 = select i1 %6, i32 1, i32 0
  %9 = sext i32 %8 to i64
  ret i64 %9
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_condvar_broadcast(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @pthread_cond_broadcast(ptr noundef %4)
  %6 = icmp eq i32 %5, 0
  %7 = zext i1 %6 to i64
  %8 = select i1 %6, i32 1, i32 0
  %9 = sext i32 %8 to i64
  ret i64 %9
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_condvar_destroy(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  %5 = call i32 @pthread_cond_destroy(ptr noundef %4)
  %6 = load i64, ptr %2, align 8
  %7 = inttoptr i64 %6 to ptr
  call void @free(ptr noundef %7)
  ret i64 0
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_regex_match(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca %struct.regex_t, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  %9 = alloca i32, align 4
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %10 = load i64, ptr %4, align 8
  %11 = inttoptr i64 %10 to ptr
  store ptr %11, ptr %7, align 8
  %12 = load i64, ptr %5, align 8
  %13 = inttoptr i64 %12 to ptr
  store ptr %13, ptr %8, align 8
  %14 = load ptr, ptr %7, align 8
  %15 = call i32 @"\01_regcomp"(ptr noundef %6, ptr noundef %14, i32 noundef 5)
  store i32 %15, ptr %9, align 4
  %16 = load i32, ptr %9, align 4
  %17 = icmp ne i32 %16, 0
  br i1 %17, label %18, label %19

18:                                               ; preds = %2
  store i64 0, ptr %3, align 8
  br label %27

19:                                               ; preds = %2
  %20 = load ptr, ptr %8, align 8
  %21 = call i32 @regexec(ptr noundef %6, ptr noundef %20, i64 noundef 0, ptr noundef null, i32 noundef 0)
  store i32 %21, ptr %9, align 4
  call void @regfree(ptr noundef %6)
  %22 = load i32, ptr %9, align 4
  %23 = icmp eq i32 %22, 0
  %24 = zext i1 %23 to i64
  %25 = select i1 %23, i32 1, i32 0
  %26 = sext i32 %25 to i64
  store i64 %26, ptr %3, align 8
  br label %27

27:                                               ; preds = %19, %18
  %28 = load i64, ptr %3, align 8
  ret i64 %28
}

declare i32 @"\01_regcomp"(ptr noundef, ptr noundef, i32 noundef) #1

declare i32 @regexec(ptr noundef, ptr noundef, i64 noundef, ptr noundef, i32 noundef) #1

declare void @regfree(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_regex_find(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca %struct.regex_t, align 8
  %7 = alloca %struct.regmatch_t, align 8
  %8 = alloca ptr, align 8
  %9 = alloca ptr, align 8
  %10 = alloca i32, align 4
  %11 = alloca i32, align 4
  %12 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %13 = load i64, ptr %4, align 8
  %14 = inttoptr i64 %13 to ptr
  store ptr %14, ptr %8, align 8
  %15 = load i64, ptr %5, align 8
  %16 = inttoptr i64 %15 to ptr
  store ptr %16, ptr %9, align 8
  %17 = load ptr, ptr %8, align 8
  %18 = call i32 @"\01_regcomp"(ptr noundef %6, ptr noundef %17, i32 noundef 1)
  store i32 %18, ptr %10, align 4
  %19 = load i32, ptr %10, align 4
  %20 = icmp ne i32 %19, 0
  br i1 %20, label %21, label %22

21:                                               ; preds = %2
  store i64 ptrtoint (ptr @.str.5 to i64), ptr %3, align 8
  br label %55

22:                                               ; preds = %2
  %23 = load ptr, ptr %9, align 8
  %24 = call i32 @regexec(ptr noundef %6, ptr noundef %23, i64 noundef 1, ptr noundef %7, i32 noundef 0)
  store i32 %24, ptr %10, align 4
  %25 = load i32, ptr %10, align 4
  %26 = icmp ne i32 %25, 0
  br i1 %26, label %27, label %28

27:                                               ; preds = %22
  call void @regfree(ptr noundef %6)
  store i64 ptrtoint (ptr @.str.5 to i64), ptr %3, align 8
  br label %55

28:                                               ; preds = %22
  %29 = getelementptr inbounds %struct.regmatch_t, ptr %7, i32 0, i32 1
  %30 = load i64, ptr %29, align 8
  %31 = getelementptr inbounds %struct.regmatch_t, ptr %7, i32 0, i32 0
  %32 = load i64, ptr %31, align 8
  %33 = sub nsw i64 %30, %32
  %34 = trunc i64 %33 to i32
  store i32 %34, ptr %11, align 4
  %35 = load i32, ptr %11, align 4
  %36 = add nsw i32 %35, 1
  %37 = sext i32 %36 to i64
  %38 = call ptr @malloc(i64 noundef %37) #14
  store ptr %38, ptr %12, align 8
  %39 = load ptr, ptr %12, align 8
  %40 = load ptr, ptr %9, align 8
  %41 = getelementptr inbounds %struct.regmatch_t, ptr %7, i32 0, i32 0
  %42 = load i64, ptr %41, align 8
  %43 = getelementptr inbounds i8, ptr %40, i64 %42
  %44 = load i32, ptr %11, align 4
  %45 = sext i32 %44 to i64
  %46 = load ptr, ptr %12, align 8
  %47 = call i64 @llvm.objectsize.i64.p0(ptr %46, i1 false, i1 true, i1 false)
  %48 = call ptr @__memcpy_chk(ptr noundef %39, ptr noundef %43, i64 noundef %45, i64 noundef %47) #13
  %49 = load ptr, ptr %12, align 8
  %50 = load i32, ptr %11, align 4
  %51 = sext i32 %50 to i64
  %52 = getelementptr inbounds i8, ptr %49, i64 %51
  store i8 0, ptr %52, align 1
  call void @regfree(ptr noundef %6)
  %53 = load ptr, ptr %12, align 8
  %54 = ptrtoint ptr %53 to i64
  store i64 %54, ptr %3, align 8
  br label %55

55:                                               ; preds = %28, %27, %21
  %56 = load i64, ptr %3, align 8
  ret i64 %56
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_regex_find_all(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca %struct.regex_t, align 8
  %7 = alloca %struct.regmatch_t, align 8
  %8 = alloca ptr, align 8
  %9 = alloca ptr, align 8
  %10 = alloca i64, align 8
  %11 = alloca i32, align 4
  %12 = alloca ptr, align 8
  %13 = alloca i32, align 4
  %14 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %15 = load i64, ptr %4, align 8
  %16 = inttoptr i64 %15 to ptr
  store ptr %16, ptr %8, align 8
  %17 = load i64, ptr %5, align 8
  %18 = inttoptr i64 %17 to ptr
  store ptr %18, ptr %9, align 8
  %19 = call i64 @create_list()
  store i64 %19, ptr %10, align 8
  %20 = load ptr, ptr %8, align 8
  %21 = call i32 @"\01_regcomp"(ptr noundef %6, ptr noundef %20, i32 noundef 1)
  store i32 %21, ptr %11, align 4
  %22 = load i32, ptr %11, align 4
  %23 = icmp ne i32 %22, 0
  br i1 %23, label %24, label %26

24:                                               ; preds = %2
  %25 = load i64, ptr %10, align 8
  store i64 %25, ptr %3, align 8
  br label %72

26:                                               ; preds = %2
  %27 = load ptr, ptr %9, align 8
  store ptr %27, ptr %12, align 8
  br label %28

28:                                               ; preds = %69, %26
  %29 = load ptr, ptr %12, align 8
  %30 = call i32 @regexec(ptr noundef %6, ptr noundef %29, i64 noundef 1, ptr noundef %7, i32 noundef 0)
  %31 = icmp eq i32 %30, 0
  br i1 %31, label %32, label %70

32:                                               ; preds = %28
  %33 = getelementptr inbounds %struct.regmatch_t, ptr %7, i32 0, i32 1
  %34 = load i64, ptr %33, align 8
  %35 = getelementptr inbounds %struct.regmatch_t, ptr %7, i32 0, i32 0
  %36 = load i64, ptr %35, align 8
  %37 = sub nsw i64 %34, %36
  %38 = trunc i64 %37 to i32
  store i32 %38, ptr %13, align 4
  %39 = load i32, ptr %13, align 4
  %40 = add nsw i32 %39, 1
  %41 = sext i32 %40 to i64
  %42 = call ptr @malloc(i64 noundef %41) #14
  store ptr %42, ptr %14, align 8
  %43 = load ptr, ptr %14, align 8
  %44 = load ptr, ptr %12, align 8
  %45 = getelementptr inbounds %struct.regmatch_t, ptr %7, i32 0, i32 0
  %46 = load i64, ptr %45, align 8
  %47 = getelementptr inbounds i8, ptr %44, i64 %46
  %48 = load i32, ptr %13, align 4
  %49 = sext i32 %48 to i64
  %50 = load ptr, ptr %14, align 8
  %51 = call i64 @llvm.objectsize.i64.p0(ptr %50, i1 false, i1 true, i1 false)
  %52 = call ptr @__memcpy_chk(ptr noundef %43, ptr noundef %47, i64 noundef %49, i64 noundef %51) #13
  %53 = load ptr, ptr %14, align 8
  %54 = load i32, ptr %13, align 4
  %55 = sext i32 %54 to i64
  %56 = getelementptr inbounds i8, ptr %53, i64 %55
  store i8 0, ptr %56, align 1
  %57 = load i64, ptr %10, align 8
  %58 = load ptr, ptr %14, align 8
  %59 = ptrtoint ptr %58 to i64
  %60 = call i64 @append_list(i64 noundef %57, i64 noundef %59)
  %61 = getelementptr inbounds %struct.regmatch_t, ptr %7, i32 0, i32 1
  %62 = load i64, ptr %61, align 8
  %63 = load ptr, ptr %12, align 8
  %64 = getelementptr inbounds i8, ptr %63, i64 %62
  store ptr %64, ptr %12, align 8
  %65 = getelementptr inbounds %struct.regmatch_t, ptr %7, i32 0, i32 1
  %66 = load i64, ptr %65, align 8
  %67 = icmp eq i64 %66, 0
  br i1 %67, label %68, label %69

68:                                               ; preds = %32
  br label %70

69:                                               ; preds = %32
  br label %28, !llvm.loop !49

70:                                               ; preds = %68, %28
  call void @regfree(ptr noundef %6)
  %71 = load i64, ptr %10, align 8
  store i64 %71, ptr %3, align 8
  br label %72

72:                                               ; preds = %70, %24
  %73 = load i64, ptr %3, align 8
  ret i64 %73
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_regex_replace(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca %struct.regex_t, align 8
  %9 = alloca %struct.regmatch_t, align 8
  %10 = alloca ptr, align 8
  %11 = alloca ptr, align 8
  %12 = alloca ptr, align 8
  %13 = alloca i32, align 4
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca i64, align 8
  %17 = alloca ptr, align 8
  store i64 %0, ptr %5, align 8
  store i64 %1, ptr %6, align 8
  store i64 %2, ptr %7, align 8
  %18 = load i64, ptr %5, align 8
  %19 = inttoptr i64 %18 to ptr
  store ptr %19, ptr %10, align 8
  %20 = load i64, ptr %6, align 8
  %21 = inttoptr i64 %20 to ptr
  store ptr %21, ptr %11, align 8
  %22 = load i64, ptr %7, align 8
  %23 = inttoptr i64 %22 to ptr
  store ptr %23, ptr %12, align 8
  %24 = load ptr, ptr %10, align 8
  %25 = call i32 @"\01_regcomp"(ptr noundef %8, ptr noundef %24, i32 noundef 1)
  store i32 %25, ptr %13, align 4
  %26 = load i32, ptr %13, align 4
  %27 = icmp ne i32 %26, 0
  br i1 %27, label %28, label %30

28:                                               ; preds = %3
  %29 = load i64, ptr %6, align 8
  store i64 %29, ptr %4, align 8
  br label %100

30:                                               ; preds = %3
  %31 = load ptr, ptr %11, align 8
  %32 = call i32 @regexec(ptr noundef %8, ptr noundef %31, i64 noundef 1, ptr noundef %9, i32 noundef 0)
  store i32 %32, ptr %13, align 4
  %33 = load i32, ptr %13, align 4
  %34 = icmp ne i32 %33, 0
  br i1 %34, label %35, label %37

35:                                               ; preds = %30
  call void @regfree(ptr noundef %8)
  %36 = load i64, ptr %6, align 8
  store i64 %36, ptr %4, align 8
  br label %100

37:                                               ; preds = %30
  %38 = load ptr, ptr %11, align 8
  %39 = call i64 @strlen(ptr noundef %38) #13
  store i64 %39, ptr %14, align 8
  %40 = load ptr, ptr %12, align 8
  %41 = call i64 @strlen(ptr noundef %40) #13
  store i64 %41, ptr %15, align 8
  %42 = load i64, ptr %14, align 8
  %43 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 1
  %44 = load i64, ptr %43, align 8
  %45 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 0
  %46 = load i64, ptr %45, align 8
  %47 = sub nsw i64 %44, %46
  %48 = sub i64 %42, %47
  %49 = load i64, ptr %15, align 8
  %50 = add i64 %48, %49
  store i64 %50, ptr %16, align 8
  %51 = load i64, ptr %16, align 8
  %52 = add i64 %51, 1
  %53 = call ptr @malloc(i64 noundef %52) #14
  store ptr %53, ptr %17, align 8
  %54 = load ptr, ptr %17, align 8
  %55 = load ptr, ptr %11, align 8
  %56 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 0
  %57 = load i64, ptr %56, align 8
  %58 = load ptr, ptr %17, align 8
  %59 = call i64 @llvm.objectsize.i64.p0(ptr %58, i1 false, i1 true, i1 false)
  %60 = call ptr @__memcpy_chk(ptr noundef %54, ptr noundef %55, i64 noundef %57, i64 noundef %59) #13
  %61 = load ptr, ptr %17, align 8
  %62 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 0
  %63 = load i64, ptr %62, align 8
  %64 = getelementptr inbounds i8, ptr %61, i64 %63
  %65 = load ptr, ptr %12, align 8
  %66 = load i64, ptr %15, align 8
  %67 = load ptr, ptr %17, align 8
  %68 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 0
  %69 = load i64, ptr %68, align 8
  %70 = getelementptr inbounds i8, ptr %67, i64 %69
  %71 = call i64 @llvm.objectsize.i64.p0(ptr %70, i1 false, i1 true, i1 false)
  %72 = call ptr @__memcpy_chk(ptr noundef %64, ptr noundef %65, i64 noundef %66, i64 noundef %71) #13
  %73 = load ptr, ptr %17, align 8
  %74 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 0
  %75 = load i64, ptr %74, align 8
  %76 = getelementptr inbounds i8, ptr %73, i64 %75
  %77 = load i64, ptr %15, align 8
  %78 = getelementptr inbounds i8, ptr %76, i64 %77
  %79 = load ptr, ptr %11, align 8
  %80 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 1
  %81 = load i64, ptr %80, align 8
  %82 = getelementptr inbounds i8, ptr %79, i64 %81
  %83 = load i64, ptr %14, align 8
  %84 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 1
  %85 = load i64, ptr %84, align 8
  %86 = sub i64 %83, %85
  %87 = load ptr, ptr %17, align 8
  %88 = getelementptr inbounds %struct.regmatch_t, ptr %9, i32 0, i32 0
  %89 = load i64, ptr %88, align 8
  %90 = getelementptr inbounds i8, ptr %87, i64 %89
  %91 = load i64, ptr %15, align 8
  %92 = getelementptr inbounds i8, ptr %90, i64 %91
  %93 = call i64 @llvm.objectsize.i64.p0(ptr %92, i1 false, i1 true, i1 false)
  %94 = call ptr @__memcpy_chk(ptr noundef %78, ptr noundef %82, i64 noundef %86, i64 noundef %93) #13
  %95 = load ptr, ptr %17, align 8
  %96 = load i64, ptr %16, align 8
  %97 = getelementptr inbounds i8, ptr %95, i64 %96
  store i8 0, ptr %97, align 1
  call void @regfree(ptr noundef %8)
  %98 = load ptr, ptr %17, align 8
  %99 = ptrtoint ptr %98 to i64
  store i64 %99, ptr %4, align 8
  br label %100

100:                                              ; preds = %37, %35, %28
  %101 = load i64, ptr %4, align 8
  ret i64 %101
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_regex_split(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca %struct.regex_t, align 8
  %8 = alloca %struct.regmatch_t, align 8
  %9 = alloca ptr, align 8
  %10 = alloca ptr, align 8
  %11 = alloca i32, align 4
  %12 = alloca ptr, align 8
  %13 = alloca i32, align 4
  %14 = alloca ptr, align 8
  %15 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %16 = call i64 @create_list()
  store i64 %16, ptr %6, align 8
  %17 = load i64, ptr %4, align 8
  %18 = inttoptr i64 %17 to ptr
  store ptr %18, ptr %9, align 8
  %19 = load i64, ptr %5, align 8
  %20 = inttoptr i64 %19 to ptr
  store ptr %20, ptr %10, align 8
  %21 = load ptr, ptr %9, align 8
  %22 = call i32 @"\01_regcomp"(ptr noundef %7, ptr noundef %21, i32 noundef 1)
  store i32 %22, ptr %11, align 4
  %23 = load i32, ptr %11, align 4
  %24 = icmp ne i32 %23, 0
  br i1 %24, label %25, label %30

25:                                               ; preds = %2
  %26 = load i64, ptr %6, align 8
  %27 = load i64, ptr %5, align 8
  %28 = call i64 @append_list(i64 noundef %26, i64 noundef %27)
  %29 = load i64, ptr %6, align 8
  store i64 %29, ptr %3, align 8
  br label %76

30:                                               ; preds = %2
  %31 = load ptr, ptr %10, align 8
  store ptr %31, ptr %12, align 8
  br label %32

32:                                               ; preds = %67, %30
  %33 = load ptr, ptr %12, align 8
  %34 = call i32 @regexec(ptr noundef %7, ptr noundef %33, i64 noundef 1, ptr noundef %8, i32 noundef 0)
  %35 = icmp eq i32 %34, 0
  br i1 %35, label %36, label %68

36:                                               ; preds = %32
  %37 = getelementptr inbounds %struct.regmatch_t, ptr %8, i32 0, i32 0
  %38 = load i64, ptr %37, align 8
  %39 = trunc i64 %38 to i32
  store i32 %39, ptr %13, align 4
  %40 = load i32, ptr %13, align 4
  %41 = add nsw i32 %40, 1
  %42 = sext i32 %41 to i64
  %43 = call ptr @malloc(i64 noundef %42) #14
  store ptr %43, ptr %14, align 8
  %44 = load ptr, ptr %14, align 8
  %45 = load ptr, ptr %12, align 8
  %46 = load i32, ptr %13, align 4
  %47 = sext i32 %46 to i64
  %48 = load ptr, ptr %14, align 8
  %49 = call i64 @llvm.objectsize.i64.p0(ptr %48, i1 false, i1 true, i1 false)
  %50 = call ptr @__memcpy_chk(ptr noundef %44, ptr noundef %45, i64 noundef %47, i64 noundef %49) #13
  %51 = load ptr, ptr %14, align 8
  %52 = load i32, ptr %13, align 4
  %53 = sext i32 %52 to i64
  %54 = getelementptr inbounds i8, ptr %51, i64 %53
  store i8 0, ptr %54, align 1
  %55 = load i64, ptr %6, align 8
  %56 = load ptr, ptr %14, align 8
  %57 = ptrtoint ptr %56 to i64
  %58 = call i64 @append_list(i64 noundef %55, i64 noundef %57)
  %59 = getelementptr inbounds %struct.regmatch_t, ptr %8, i32 0, i32 1
  %60 = load i64, ptr %59, align 8
  %61 = load ptr, ptr %12, align 8
  %62 = getelementptr inbounds i8, ptr %61, i64 %60
  store ptr %62, ptr %12, align 8
  %63 = getelementptr inbounds %struct.regmatch_t, ptr %8, i32 0, i32 1
  %64 = load i64, ptr %63, align 8
  %65 = icmp eq i64 %64, 0
  br i1 %65, label %66, label %67

66:                                               ; preds = %36
  br label %68

67:                                               ; preds = %36
  br label %32, !llvm.loop !50

68:                                               ; preds = %66, %32
  %69 = load ptr, ptr %12, align 8
  %70 = call ptr @strdup(ptr noundef %69) #13
  store ptr %70, ptr %15, align 8
  %71 = load i64, ptr %6, align 8
  %72 = load ptr, ptr %15, align 8
  %73 = ptrtoint ptr %72 to i64
  %74 = call i64 @append_list(i64 noundef %71, i64 noundef %73)
  call void @regfree(ptr noundef %7)
  %75 = load i64, ptr %6, align 8
  store i64 %75, ptr %3, align 8
  br label %76

76:                                               ; preds = %68, %25
  %77 = load i64, ptr %3, align 8
  ret i64 %77
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_base64_encode(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca i32, align 4
  store i64 %0, ptr %2, align 8
  %10 = load i64, ptr %2, align 8
  %11 = inttoptr i64 %10 to ptr
  store ptr %11, ptr %3, align 8
  %12 = load ptr, ptr %3, align 8
  %13 = call i64 @strlen(ptr noundef %12) #13
  store i64 %13, ptr %4, align 8
  %14 = load i64, ptr %4, align 8
  %15 = add i64 %14, 2
  %16 = udiv i64 %15, 3
  %17 = mul i64 4, %16
  store i64 %17, ptr %5, align 8
  %18 = load i64, ptr %5, align 8
  %19 = add i64 %18, 1
  %20 = call ptr @malloc(i64 noundef %19) #14
  store ptr %20, ptr %6, align 8
  store i64 0, ptr %8, align 8
  store i64 0, ptr %7, align 8
  br label %21

21:                                               ; preds = %120, %1
  %22 = load i64, ptr %7, align 8
  %23 = load i64, ptr %4, align 8
  %24 = icmp ult i64 %22, %23
  br i1 %24, label %25, label %123

25:                                               ; preds = %21
  %26 = load ptr, ptr %3, align 8
  %27 = load i64, ptr %7, align 8
  %28 = getelementptr inbounds i8, ptr %26, i64 %27
  %29 = load i8, ptr %28, align 1
  %30 = zext i8 %29 to i32
  %31 = shl i32 %30, 16
  store i32 %31, ptr %9, align 4
  %32 = load i64, ptr %7, align 8
  %33 = add i64 %32, 1
  %34 = load i64, ptr %4, align 8
  %35 = icmp ult i64 %33, %34
  br i1 %35, label %36, label %46

36:                                               ; preds = %25
  %37 = load ptr, ptr %3, align 8
  %38 = load i64, ptr %7, align 8
  %39 = add i64 %38, 1
  %40 = getelementptr inbounds i8, ptr %37, i64 %39
  %41 = load i8, ptr %40, align 1
  %42 = zext i8 %41 to i32
  %43 = shl i32 %42, 8
  %44 = load i32, ptr %9, align 4
  %45 = or i32 %44, %43
  store i32 %45, ptr %9, align 4
  br label %46

46:                                               ; preds = %36, %25
  %47 = load i64, ptr %7, align 8
  %48 = add i64 %47, 2
  %49 = load i64, ptr %4, align 8
  %50 = icmp ult i64 %48, %49
  br i1 %50, label %51, label %60

51:                                               ; preds = %46
  %52 = load ptr, ptr %3, align 8
  %53 = load i64, ptr %7, align 8
  %54 = add i64 %53, 2
  %55 = getelementptr inbounds i8, ptr %52, i64 %54
  %56 = load i8, ptr %55, align 1
  %57 = zext i8 %56 to i32
  %58 = load i32, ptr %9, align 4
  %59 = or i32 %58, %57
  store i32 %59, ptr %9, align 4
  br label %60

60:                                               ; preds = %51, %46
  %61 = load i32, ptr %9, align 4
  %62 = lshr i32 %61, 18
  %63 = and i32 %62, 63
  %64 = zext i32 %63 to i64
  %65 = getelementptr inbounds [65 x i8], ptr @b64_table, i64 0, i64 %64
  %66 = load i8, ptr %65, align 1
  %67 = load ptr, ptr %6, align 8
  %68 = load i64, ptr %8, align 8
  %69 = add i64 %68, 1
  store i64 %69, ptr %8, align 8
  %70 = getelementptr inbounds i8, ptr %67, i64 %68
  store i8 %66, ptr %70, align 1
  %71 = load i32, ptr %9, align 4
  %72 = lshr i32 %71, 12
  %73 = and i32 %72, 63
  %74 = zext i32 %73 to i64
  %75 = getelementptr inbounds [65 x i8], ptr @b64_table, i64 0, i64 %74
  %76 = load i8, ptr %75, align 1
  %77 = load ptr, ptr %6, align 8
  %78 = load i64, ptr %8, align 8
  %79 = add i64 %78, 1
  store i64 %79, ptr %8, align 8
  %80 = getelementptr inbounds i8, ptr %77, i64 %78
  store i8 %76, ptr %80, align 1
  %81 = load i64, ptr %7, align 8
  %82 = add i64 %81, 1
  %83 = load i64, ptr %4, align 8
  %84 = icmp ult i64 %82, %83
  br i1 %84, label %85, label %93

85:                                               ; preds = %60
  %86 = load i32, ptr %9, align 4
  %87 = lshr i32 %86, 6
  %88 = and i32 %87, 63
  %89 = zext i32 %88 to i64
  %90 = getelementptr inbounds [65 x i8], ptr @b64_table, i64 0, i64 %89
  %91 = load i8, ptr %90, align 1
  %92 = sext i8 %91 to i32
  br label %94

93:                                               ; preds = %60
  br label %94

94:                                               ; preds = %93, %85
  %95 = phi i32 [ %92, %85 ], [ 61, %93 ]
  %96 = trunc i32 %95 to i8
  %97 = load ptr, ptr %6, align 8
  %98 = load i64, ptr %8, align 8
  %99 = add i64 %98, 1
  store i64 %99, ptr %8, align 8
  %100 = getelementptr inbounds i8, ptr %97, i64 %98
  store i8 %96, ptr %100, align 1
  %101 = load i64, ptr %7, align 8
  %102 = add i64 %101, 2
  %103 = load i64, ptr %4, align 8
  %104 = icmp ult i64 %102, %103
  br i1 %104, label %105, label %112

105:                                              ; preds = %94
  %106 = load i32, ptr %9, align 4
  %107 = and i32 %106, 63
  %108 = zext i32 %107 to i64
  %109 = getelementptr inbounds [65 x i8], ptr @b64_table, i64 0, i64 %108
  %110 = load i8, ptr %109, align 1
  %111 = sext i8 %110 to i32
  br label %113

112:                                              ; preds = %94
  br label %113

113:                                              ; preds = %112, %105
  %114 = phi i32 [ %111, %105 ], [ 61, %112 ]
  %115 = trunc i32 %114 to i8
  %116 = load ptr, ptr %6, align 8
  %117 = load i64, ptr %8, align 8
  %118 = add i64 %117, 1
  store i64 %118, ptr %8, align 8
  %119 = getelementptr inbounds i8, ptr %116, i64 %117
  store i8 %115, ptr %119, align 1
  br label %120

120:                                              ; preds = %113
  %121 = load i64, ptr %7, align 8
  %122 = add i64 %121, 3
  store i64 %122, ptr %7, align 8
  br label %21, !llvm.loop !51

123:                                              ; preds = %21
  %124 = load ptr, ptr %6, align 8
  %125 = load i64, ptr %8, align 8
  %126 = getelementptr inbounds i8, ptr %124, i64 %125
  store i8 0, ptr %126, align 1
  %127 = load ptr, ptr %6, align 8
  %128 = ptrtoint ptr %127 to i64
  ret i64 %128
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_uuid_v4() #0 {
  %1 = alloca ptr, align 8
  %2 = alloca [16 x i8], align 1
  %3 = alloca i32, align 4
  %4 = call ptr @malloc(i64 noundef 37) #14
  store ptr %4, ptr %1, align 8
  store i32 0, ptr %3, align 4
  br label %5

5:                                                ; preds = %15, %0
  %6 = load i32, ptr %3, align 4
  %7 = icmp slt i32 %6, 16
  br i1 %7, label %8, label %18

8:                                                ; preds = %5
  %9 = call i32 @rand()
  %10 = and i32 %9, 255
  %11 = trunc i32 %10 to i8
  %12 = load i32, ptr %3, align 4
  %13 = sext i32 %12 to i64
  %14 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 %13
  store i8 %11, ptr %14, align 1
  br label %15

15:                                               ; preds = %8
  %16 = load i32, ptr %3, align 4
  %17 = add nsw i32 %16, 1
  store i32 %17, ptr %3, align 4
  br label %5, !llvm.loop !52

18:                                               ; preds = %5
  %19 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 6
  %20 = load i8, ptr %19, align 1
  %21 = zext i8 %20 to i32
  %22 = and i32 %21, 15
  %23 = or i32 %22, 64
  %24 = trunc i32 %23 to i8
  %25 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 6
  store i8 %24, ptr %25, align 1
  %26 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 8
  %27 = load i8, ptr %26, align 1
  %28 = zext i8 %27 to i32
  %29 = and i32 %28, 63
  %30 = or i32 %29, 128
  %31 = trunc i32 %30 to i8
  %32 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 8
  store i8 %31, ptr %32, align 1
  %33 = load ptr, ptr %1, align 8
  %34 = load ptr, ptr %1, align 8
  %35 = call i64 @llvm.objectsize.i64.p0(ptr %34, i1 false, i1 true, i1 false)
  %36 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 0
  %37 = load i8, ptr %36, align 1
  %38 = zext i8 %37 to i32
  %39 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 1
  %40 = load i8, ptr %39, align 1
  %41 = zext i8 %40 to i32
  %42 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 2
  %43 = load i8, ptr %42, align 1
  %44 = zext i8 %43 to i32
  %45 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 3
  %46 = load i8, ptr %45, align 1
  %47 = zext i8 %46 to i32
  %48 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 4
  %49 = load i8, ptr %48, align 1
  %50 = zext i8 %49 to i32
  %51 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 5
  %52 = load i8, ptr %51, align 1
  %53 = zext i8 %52 to i32
  %54 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 6
  %55 = load i8, ptr %54, align 1
  %56 = zext i8 %55 to i32
  %57 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 7
  %58 = load i8, ptr %57, align 1
  %59 = zext i8 %58 to i32
  %60 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 8
  %61 = load i8, ptr %60, align 1
  %62 = zext i8 %61 to i32
  %63 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 9
  %64 = load i8, ptr %63, align 1
  %65 = zext i8 %64 to i32
  %66 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 10
  %67 = load i8, ptr %66, align 1
  %68 = zext i8 %67 to i32
  %69 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 11
  %70 = load i8, ptr %69, align 1
  %71 = zext i8 %70 to i32
  %72 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 12
  %73 = load i8, ptr %72, align 1
  %74 = zext i8 %73 to i32
  %75 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 13
  %76 = load i8, ptr %75, align 1
  %77 = zext i8 %76 to i32
  %78 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 14
  %79 = load i8, ptr %78, align 1
  %80 = zext i8 %79 to i32
  %81 = getelementptr inbounds [16 x i8], ptr %2, i64 0, i64 15
  %82 = load i8, ptr %81, align 1
  %83 = zext i8 %82 to i32
  %84 = call i32 (ptr, i64, i32, i64, ptr, ...) @__snprintf_chk(ptr noundef %33, i64 noundef 37, i32 noundef 0, i64 noundef %35, ptr noundef @.str.24, i32 noundef %38, i32 noundef %41, i32 noundef %44, i32 noundef %47, i32 noundef %50, i32 noundef %53, i32 noundef %56, i32 noundef %59, i32 noundef %62, i32 noundef %65, i32 noundef %68, i32 noundef %71, i32 noundef %74, i32 noundef %77, i32 noundef %80, i32 noundef %83)
  %85 = load ptr, ptr %1, align 8
  %86 = ptrtoint ptr %85 to i64
  ret i64 %86
}

declare i32 @rand() #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @file_read(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %8 = load i64, ptr %3, align 8
  %9 = inttoptr i64 %8 to ptr
  store ptr %9, ptr %4, align 8
  %10 = load ptr, ptr %4, align 8
  %11 = icmp ne ptr %10, null
  br i1 %11, label %15, label %12

12:                                               ; preds = %1
  %13 = call ptr @strdup(ptr noundef @.str.5) #13
  %14 = ptrtoint ptr %13 to i64
  store i64 %14, ptr %2, align 8
  br label %54

15:                                               ; preds = %1
  %16 = load ptr, ptr %4, align 8
  %17 = call ptr @"\01_fopen"(ptr noundef %16, ptr noundef @.str.3)
  store ptr %17, ptr %5, align 8
  %18 = load ptr, ptr %5, align 8
  %19 = icmp ne ptr %18, null
  br i1 %19, label %23, label %20

20:                                               ; preds = %15
  %21 = call ptr @strdup(ptr noundef @.str.5) #13
  %22 = ptrtoint ptr %21 to i64
  store i64 %22, ptr %2, align 8
  br label %54

23:                                               ; preds = %15
  %24 = load ptr, ptr %5, align 8
  %25 = call i32 @fseek(ptr noundef %24, i64 noundef 0, i32 noundef 2)
  %26 = load ptr, ptr %5, align 8
  %27 = call i64 @ftell(ptr noundef %26)
  store i64 %27, ptr %6, align 8
  %28 = load ptr, ptr %5, align 8
  %29 = call i32 @fseek(ptr noundef %28, i64 noundef 0, i32 noundef 0)
  %30 = load i64, ptr %6, align 8
  %31 = add nsw i64 %30, 1
  %32 = call ptr @malloc(i64 noundef %31) #14
  store ptr %32, ptr %7, align 8
  %33 = load ptr, ptr %7, align 8
  %34 = icmp ne ptr %33, null
  br i1 %34, label %40, label %35

35:                                               ; preds = %23
  %36 = load ptr, ptr %5, align 8
  %37 = call i32 @fclose(ptr noundef %36)
  %38 = call ptr @strdup(ptr noundef @.str.5) #13
  %39 = ptrtoint ptr %38 to i64
  store i64 %39, ptr %2, align 8
  br label %54

40:                                               ; preds = %23
  %41 = load ptr, ptr %7, align 8
  %42 = load i64, ptr %6, align 8
  %43 = load ptr, ptr %5, align 8
  %44 = call i64 @fread(ptr noundef %41, i64 noundef 1, i64 noundef %42, ptr noundef %43)
  %45 = load ptr, ptr %7, align 8
  %46 = load i64, ptr %6, align 8
  %47 = getelementptr inbounds i8, ptr %45, i64 %46
  store i8 0, ptr %47, align 1
  %48 = load ptr, ptr %5, align 8
  %49 = call i32 @fclose(ptr noundef %48)
  %50 = load ptr, ptr %7, align 8
  %51 = call ptr @ep_gc_register(ptr noundef %50, i32 noundef 1)
  %52 = load ptr, ptr %7, align 8
  %53 = ptrtoint ptr %52 to i64
  store i64 %53, ptr %2, align 8
  br label %54

54:                                               ; preds = %40, %35, %20, %12
  %55 = load i64, ptr %2, align 8
  ret i64 %55
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @file_write(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  %9 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %10 = load i64, ptr %4, align 8
  %11 = inttoptr i64 %10 to ptr
  store ptr %11, ptr %6, align 8
  %12 = load i64, ptr %5, align 8
  %13 = inttoptr i64 %12 to ptr
  store ptr %13, ptr %7, align 8
  %14 = load ptr, ptr %6, align 8
  %15 = icmp ne ptr %14, null
  br i1 %15, label %16, label %19

16:                                               ; preds = %2
  %17 = load ptr, ptr %7, align 8
  %18 = icmp ne ptr %17, null
  br i1 %18, label %20, label %19

19:                                               ; preds = %16, %2
  store i64 0, ptr %3, align 8
  br label %35

20:                                               ; preds = %16
  %21 = load ptr, ptr %6, align 8
  %22 = call ptr @"\01_fopen"(ptr noundef %21, ptr noundef @.str.4)
  store ptr %22, ptr %8, align 8
  %23 = load ptr, ptr %8, align 8
  %24 = icmp ne ptr %23, null
  br i1 %24, label %26, label %25

25:                                               ; preds = %20
  store i64 0, ptr %3, align 8
  br label %35

26:                                               ; preds = %20
  %27 = load ptr, ptr %7, align 8
  %28 = call i64 @strlen(ptr noundef %27) #13
  store i64 %28, ptr %9, align 8
  %29 = load ptr, ptr %7, align 8
  %30 = load i64, ptr %9, align 8
  %31 = load ptr, ptr %8, align 8
  %32 = call i64 @"\01_fwrite"(ptr noundef %29, i64 noundef 1, i64 noundef %30, ptr noundef %31)
  %33 = load ptr, ptr %8, align 8
  %34 = call i32 @fclose(ptr noundef %33)
  store i64 1, ptr %3, align 8
  br label %35

35:                                               ; preds = %26, %25, %19
  %36 = load i64, ptr %3, align 8
  ret i64 %36
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @file_append(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  %9 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %10 = load i64, ptr %4, align 8
  %11 = inttoptr i64 %10 to ptr
  store ptr %11, ptr %6, align 8
  %12 = load i64, ptr %5, align 8
  %13 = inttoptr i64 %12 to ptr
  store ptr %13, ptr %7, align 8
  %14 = load ptr, ptr %6, align 8
  %15 = icmp ne ptr %14, null
  br i1 %15, label %16, label %19

16:                                               ; preds = %2
  %17 = load ptr, ptr %7, align 8
  %18 = icmp ne ptr %17, null
  br i1 %18, label %20, label %19

19:                                               ; preds = %16, %2
  store i64 0, ptr %3, align 8
  br label %35

20:                                               ; preds = %16
  %21 = load ptr, ptr %6, align 8
  %22 = call ptr @"\01_fopen"(ptr noundef %21, ptr noundef @.str.19)
  store ptr %22, ptr %8, align 8
  %23 = load ptr, ptr %8, align 8
  %24 = icmp ne ptr %23, null
  br i1 %24, label %26, label %25

25:                                               ; preds = %20
  store i64 0, ptr %3, align 8
  br label %35

26:                                               ; preds = %20
  %27 = load ptr, ptr %7, align 8
  %28 = call i64 @strlen(ptr noundef %27) #13
  store i64 %28, ptr %9, align 8
  %29 = load ptr, ptr %7, align 8
  %30 = load i64, ptr %9, align 8
  %31 = load ptr, ptr %8, align 8
  %32 = call i64 @"\01_fwrite"(ptr noundef %29, i64 noundef 1, i64 noundef %30, ptr noundef %31)
  %33 = load ptr, ptr %8, align 8
  %34 = call i32 @fclose(ptr noundef %33)
  store i64 1, ptr %3, align 8
  br label %35

35:                                               ; preds = %26, %25, %19
  %36 = load i64, ptr %3, align 8
  ret i64 %36
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @file_exists(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %6 = load i64, ptr %3, align 8
  %7 = inttoptr i64 %6 to ptr
  store ptr %7, ptr %4, align 8
  %8 = load ptr, ptr %4, align 8
  %9 = icmp ne ptr %8, null
  br i1 %9, label %11, label %10

10:                                               ; preds = %1
  store i64 0, ptr %2, align 8
  br label %20

11:                                               ; preds = %1
  %12 = load ptr, ptr %4, align 8
  %13 = call ptr @"\01_fopen"(ptr noundef %12, ptr noundef @.str.23)
  store ptr %13, ptr %5, align 8
  %14 = load ptr, ptr %5, align 8
  %15 = icmp ne ptr %14, null
  br i1 %15, label %16, label %19

16:                                               ; preds = %11
  %17 = load ptr, ptr %5, align 8
  %18 = call i32 @fclose(ptr noundef %17)
  store i64 1, ptr %2, align 8
  br label %20

19:                                               ; preds = %11
  store i64 0, ptr %2, align 8
  br label %20

20:                                               ; preds = %19, %16, %10
  %21 = load i64, ptr %2, align 8
  ret i64 %21
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_contains(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %8 = load i64, ptr %4, align 8
  %9 = inttoptr i64 %8 to ptr
  store ptr %9, ptr %6, align 8
  %10 = load i64, ptr %5, align 8
  %11 = inttoptr i64 %10 to ptr
  store ptr %11, ptr %7, align 8
  %12 = load ptr, ptr %6, align 8
  %13 = icmp ne ptr %12, null
  br i1 %13, label %14, label %17

14:                                               ; preds = %2
  %15 = load ptr, ptr %7, align 8
  %16 = icmp ne ptr %15, null
  br i1 %16, label %18, label %17

17:                                               ; preds = %14, %2
  store i64 0, ptr %3, align 8
  br label %26

18:                                               ; preds = %14
  %19 = load ptr, ptr %6, align 8
  %20 = load ptr, ptr %7, align 8
  %21 = call ptr @strstr(ptr noundef %19, ptr noundef %20) #13
  %22 = icmp ne ptr %21, null
  %23 = zext i1 %22 to i64
  %24 = select i1 %22, i32 1, i32 0
  %25 = sext i32 %24 to i64
  store i64 %25, ptr %3, align 8
  br label %26

26:                                               ; preds = %18, %17
  %27 = load i64, ptr %3, align 8
  ret i64 %27
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_index_of(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %4, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %6, align 8
  %11 = load i64, ptr %5, align 8
  %12 = inttoptr i64 %11 to ptr
  store ptr %12, ptr %7, align 8
  %13 = load ptr, ptr %6, align 8
  %14 = icmp ne ptr %13, null
  br i1 %14, label %15, label %18

15:                                               ; preds = %2
  %16 = load ptr, ptr %7, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %19, label %18

18:                                               ; preds = %15, %2
  store i64 -1, ptr %3, align 8
  br label %32

19:                                               ; preds = %15
  %20 = load ptr, ptr %6, align 8
  %21 = load ptr, ptr %7, align 8
  %22 = call ptr @strstr(ptr noundef %20, ptr noundef %21) #13
  store ptr %22, ptr %8, align 8
  %23 = load ptr, ptr %8, align 8
  %24 = icmp ne ptr %23, null
  br i1 %24, label %26, label %25

25:                                               ; preds = %19
  store i64 -1, ptr %3, align 8
  br label %32

26:                                               ; preds = %19
  %27 = load ptr, ptr %8, align 8
  %28 = load ptr, ptr %6, align 8
  %29 = ptrtoint ptr %27 to i64
  %30 = ptrtoint ptr %28 to i64
  %31 = sub i64 %29, %30
  store i64 %31, ptr %3, align 8
  br label %32

32:                                               ; preds = %26, %25, %18
  %33 = load i64, ptr %3, align 8
  ret i64 %33
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_replace(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca ptr, align 8
  %9 = alloca ptr, align 8
  %10 = alloca ptr, align 8
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  %13 = alloca i32, align 4
  %14 = alloca ptr, align 8
  %15 = alloca i64, align 8
  %16 = alloca ptr, align 8
  %17 = alloca ptr, align 8
  store i64 %0, ptr %5, align 8
  store i64 %1, ptr %6, align 8
  store i64 %2, ptr %7, align 8
  %18 = load i64, ptr %5, align 8
  %19 = inttoptr i64 %18 to ptr
  store ptr %19, ptr %8, align 8
  %20 = load i64, ptr %6, align 8
  %21 = inttoptr i64 %20 to ptr
  store ptr %21, ptr %9, align 8
  %22 = load i64, ptr %7, align 8
  %23 = inttoptr i64 %22 to ptr
  store ptr %23, ptr %10, align 8
  %24 = load ptr, ptr %8, align 8
  %25 = icmp ne ptr %24, null
  br i1 %25, label %26, label %32

26:                                               ; preds = %3
  %27 = load ptr, ptr %9, align 8
  %28 = icmp ne ptr %27, null
  br i1 %28, label %29, label %32

29:                                               ; preds = %26
  %30 = load ptr, ptr %10, align 8
  %31 = icmp ne ptr %30, null
  br i1 %31, label %42, label %32

32:                                               ; preds = %29, %26, %3
  %33 = load ptr, ptr %8, align 8
  %34 = icmp ne ptr %33, null
  br i1 %34, label %35, label %37

35:                                               ; preds = %32
  %36 = load ptr, ptr %8, align 8
  br label %38

37:                                               ; preds = %32
  br label %38

38:                                               ; preds = %37, %35
  %39 = phi ptr [ %36, %35 ], [ @.str.5, %37 ]
  %40 = call ptr @strdup(ptr noundef %39) #13
  %41 = ptrtoint ptr %40 to i64
  store i64 %41, ptr %4, align 8
  br label %124

42:                                               ; preds = %29
  %43 = load ptr, ptr %9, align 8
  %44 = call i64 @strlen(ptr noundef %43) #13
  store i64 %44, ptr %11, align 8
  %45 = load ptr, ptr %10, align 8
  %46 = call i64 @strlen(ptr noundef %45) #13
  store i64 %46, ptr %12, align 8
  %47 = load i64, ptr %11, align 8
  %48 = icmp eq i64 %47, 0
  br i1 %48, label %49, label %53

49:                                               ; preds = %42
  %50 = load ptr, ptr %8, align 8
  %51 = call ptr @strdup(ptr noundef %50) #13
  %52 = ptrtoint ptr %51 to i64
  store i64 %52, ptr %4, align 8
  br label %124

53:                                               ; preds = %42
  store i32 0, ptr %13, align 4
  %54 = load ptr, ptr %8, align 8
  store ptr %54, ptr %14, align 8
  br label %55

55:                                               ; preds = %60, %53
  %56 = load ptr, ptr %14, align 8
  %57 = load ptr, ptr %9, align 8
  %58 = call ptr @strstr(ptr noundef %56, ptr noundef %57) #13
  store ptr %58, ptr %14, align 8
  %59 = icmp ne ptr %58, null
  br i1 %59, label %60, label %66

60:                                               ; preds = %55
  %61 = load i32, ptr %13, align 4
  %62 = add nsw i32 %61, 1
  store i32 %62, ptr %13, align 4
  %63 = load i64, ptr %11, align 8
  %64 = load ptr, ptr %14, align 8
  %65 = getelementptr inbounds i8, ptr %64, i64 %63
  store ptr %65, ptr %14, align 8
  br label %55, !llvm.loop !53

66:                                               ; preds = %55
  %67 = load ptr, ptr %8, align 8
  %68 = call i64 @strlen(ptr noundef %67) #13
  %69 = load i32, ptr %13, align 4
  %70 = sext i32 %69 to i64
  %71 = load i64, ptr %12, align 8
  %72 = load i64, ptr %11, align 8
  %73 = sub i64 %71, %72
  %74 = mul i64 %70, %73
  %75 = add i64 %68, %74
  store i64 %75, ptr %15, align 8
  %76 = load i64, ptr %15, align 8
  %77 = add i64 %76, 1
  %78 = call ptr @malloc(i64 noundef %77) #14
  store ptr %78, ptr %16, align 8
  %79 = load ptr, ptr %16, align 8
  %80 = icmp ne ptr %79, null
  br i1 %80, label %85, label %81

81:                                               ; preds = %66
  %82 = load ptr, ptr %8, align 8
  %83 = call ptr @strdup(ptr noundef %82) #13
  %84 = ptrtoint ptr %83 to i64
  store i64 %84, ptr %4, align 8
  br label %124

85:                                               ; preds = %66
  %86 = load ptr, ptr %16, align 8
  store ptr %86, ptr %17, align 8
  %87 = load ptr, ptr %8, align 8
  store ptr %87, ptr %14, align 8
  br label %88

88:                                               ; preds = %117, %85
  %89 = load ptr, ptr %14, align 8
  %90 = load i8, ptr %89, align 1
  %91 = icmp ne i8 %90, 0
  br i1 %91, label %92, label %118

92:                                               ; preds = %88
  %93 = load ptr, ptr %14, align 8
  %94 = load ptr, ptr %9, align 8
  %95 = load i64, ptr %11, align 8
  %96 = call i32 @strncmp(ptr noundef %93, ptr noundef %94, i64 noundef %95) #13
  %97 = icmp eq i32 %96, 0
  br i1 %97, label %98, label %111

98:                                               ; preds = %92
  %99 = load ptr, ptr %17, align 8
  %100 = load ptr, ptr %10, align 8
  %101 = load i64, ptr %12, align 8
  %102 = load ptr, ptr %17, align 8
  %103 = call i64 @llvm.objectsize.i64.p0(ptr %102, i1 false, i1 true, i1 false)
  %104 = call ptr @__memcpy_chk(ptr noundef %99, ptr noundef %100, i64 noundef %101, i64 noundef %103) #13
  %105 = load i64, ptr %12, align 8
  %106 = load ptr, ptr %17, align 8
  %107 = getelementptr inbounds i8, ptr %106, i64 %105
  store ptr %107, ptr %17, align 8
  %108 = load i64, ptr %11, align 8
  %109 = load ptr, ptr %14, align 8
  %110 = getelementptr inbounds i8, ptr %109, i64 %108
  store ptr %110, ptr %14, align 8
  br label %117

111:                                              ; preds = %92
  %112 = load ptr, ptr %14, align 8
  %113 = getelementptr inbounds i8, ptr %112, i32 1
  store ptr %113, ptr %14, align 8
  %114 = load i8, ptr %112, align 1
  %115 = load ptr, ptr %17, align 8
  %116 = getelementptr inbounds i8, ptr %115, i32 1
  store ptr %116, ptr %17, align 8
  store i8 %114, ptr %115, align 1
  br label %117

117:                                              ; preds = %111, %98
  br label %88, !llvm.loop !54

118:                                              ; preds = %88
  %119 = load ptr, ptr %17, align 8
  store i8 0, ptr %119, align 1
  %120 = load ptr, ptr %16, align 8
  %121 = call ptr @ep_gc_register(ptr noundef %120, i32 noundef 1)
  %122 = load ptr, ptr %16, align 8
  %123 = ptrtoint ptr %122 to i64
  store i64 %123, ptr %4, align 8
  br label %124

124:                                              ; preds = %118, %81, %49, %38
  %125 = load i64, ptr %4, align 8
  ret i64 %125
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_upper(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %8 = load i64, ptr %3, align 8
  %9 = inttoptr i64 %8 to ptr
  store ptr %9, ptr %4, align 8
  %10 = load ptr, ptr %4, align 8
  %11 = icmp ne ptr %10, null
  br i1 %11, label %15, label %12

12:                                               ; preds = %1
  %13 = call ptr @strdup(ptr noundef @.str.5) #13
  %14 = ptrtoint ptr %13 to i64
  store i64 %14, ptr %2, align 8
  br label %47

15:                                               ; preds = %1
  %16 = load ptr, ptr %4, align 8
  %17 = call i64 @strlen(ptr noundef %16) #13
  store i64 %17, ptr %5, align 8
  %18 = load i64, ptr %5, align 8
  %19 = add nsw i64 %18, 1
  %20 = call ptr @malloc(i64 noundef %19) #14
  store ptr %20, ptr %6, align 8
  store i64 0, ptr %7, align 8
  br label %21

21:                                               ; preds = %36, %15
  %22 = load i64, ptr %7, align 8
  %23 = load i64, ptr %5, align 8
  %24 = icmp slt i64 %22, %23
  br i1 %24, label %25, label %39

25:                                               ; preds = %21
  %26 = load ptr, ptr %4, align 8
  %27 = load i64, ptr %7, align 8
  %28 = getelementptr inbounds i8, ptr %26, i64 %27
  %29 = load i8, ptr %28, align 1
  %30 = zext i8 %29 to i32
  %31 = call i32 @toupper(i32 noundef %30) #17
  %32 = trunc i32 %31 to i8
  %33 = load ptr, ptr %6, align 8
  %34 = load i64, ptr %7, align 8
  %35 = getelementptr inbounds i8, ptr %33, i64 %34
  store i8 %32, ptr %35, align 1
  br label %36

36:                                               ; preds = %25
  %37 = load i64, ptr %7, align 8
  %38 = add nsw i64 %37, 1
  store i64 %38, ptr %7, align 8
  br label %21, !llvm.loop !55

39:                                               ; preds = %21
  %40 = load ptr, ptr %6, align 8
  %41 = load i64, ptr %5, align 8
  %42 = getelementptr inbounds i8, ptr %40, i64 %41
  store i8 0, ptr %42, align 1
  %43 = load ptr, ptr %6, align 8
  %44 = call ptr @ep_gc_register(ptr noundef %43, i32 noundef 1)
  %45 = load ptr, ptr %6, align 8
  %46 = ptrtoint ptr %45 to i64
  store i64 %46, ptr %2, align 8
  br label %47

47:                                               ; preds = %39, %12
  %48 = load i64, ptr %2, align 8
  ret i64 %48
}

; Function Attrs: nounwind willreturn memory(read)
declare i32 @toupper(i32 noundef) #10

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_lower(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %8 = load i64, ptr %3, align 8
  %9 = inttoptr i64 %8 to ptr
  store ptr %9, ptr %4, align 8
  %10 = load ptr, ptr %4, align 8
  %11 = icmp ne ptr %10, null
  br i1 %11, label %15, label %12

12:                                               ; preds = %1
  %13 = call ptr @strdup(ptr noundef @.str.5) #13
  %14 = ptrtoint ptr %13 to i64
  store i64 %14, ptr %2, align 8
  br label %47

15:                                               ; preds = %1
  %16 = load ptr, ptr %4, align 8
  %17 = call i64 @strlen(ptr noundef %16) #13
  store i64 %17, ptr %5, align 8
  %18 = load i64, ptr %5, align 8
  %19 = add nsw i64 %18, 1
  %20 = call ptr @malloc(i64 noundef %19) #14
  store ptr %20, ptr %6, align 8
  store i64 0, ptr %7, align 8
  br label %21

21:                                               ; preds = %36, %15
  %22 = load i64, ptr %7, align 8
  %23 = load i64, ptr %5, align 8
  %24 = icmp slt i64 %22, %23
  br i1 %24, label %25, label %39

25:                                               ; preds = %21
  %26 = load ptr, ptr %4, align 8
  %27 = load i64, ptr %7, align 8
  %28 = getelementptr inbounds i8, ptr %26, i64 %27
  %29 = load i8, ptr %28, align 1
  %30 = zext i8 %29 to i32
  %31 = call i32 @tolower(i32 noundef %30) #17
  %32 = trunc i32 %31 to i8
  %33 = load ptr, ptr %6, align 8
  %34 = load i64, ptr %7, align 8
  %35 = getelementptr inbounds i8, ptr %33, i64 %34
  store i8 %32, ptr %35, align 1
  br label %36

36:                                               ; preds = %25
  %37 = load i64, ptr %7, align 8
  %38 = add nsw i64 %37, 1
  store i64 %38, ptr %7, align 8
  br label %21, !llvm.loop !56

39:                                               ; preds = %21
  %40 = load ptr, ptr %6, align 8
  %41 = load i64, ptr %5, align 8
  %42 = getelementptr inbounds i8, ptr %40, i64 %41
  store i8 0, ptr %42, align 1
  %43 = load ptr, ptr %6, align 8
  %44 = call ptr @ep_gc_register(ptr noundef %43, i32 noundef 1)
  %45 = load ptr, ptr %6, align 8
  %46 = ptrtoint ptr %45 to i64
  store i64 %46, ptr %2, align 8
  br label %47

47:                                               ; preds = %39, %12
  %48 = load i64, ptr %2, align 8
  ret i64 %48
}

; Function Attrs: nounwind willreturn memory(read)
declare i32 @tolower(i32 noundef) #10

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_trim(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %7 = load i64, ptr %3, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %4, align 8
  %9 = load ptr, ptr %4, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %14, label %11

11:                                               ; preds = %1
  %12 = call ptr @strdup(ptr noundef @.str.5) #13
  %13 = ptrtoint ptr %12 to i64
  store i64 %13, ptr %2, align 8
  br label %68

14:                                               ; preds = %1
  br label %15

15:                                               ; preds = %28, %14
  %16 = load ptr, ptr %4, align 8
  %17 = load i8, ptr %16, align 1
  %18 = sext i8 %17 to i32
  %19 = icmp ne i32 %18, 0
  br i1 %19, label %20, label %26

20:                                               ; preds = %15
  %21 = load ptr, ptr %4, align 8
  %22 = load i8, ptr %21, align 1
  %23 = zext i8 %22 to i32
  %24 = call i32 @isspace(i32 noundef %23) #17
  %25 = icmp ne i32 %24, 0
  br label %26

26:                                               ; preds = %20, %15
  %27 = phi i1 [ false, %15 ], [ %25, %20 ]
  br i1 %27, label %28, label %31

28:                                               ; preds = %26
  %29 = load ptr, ptr %4, align 8
  %30 = getelementptr inbounds i8, ptr %29, i32 1
  store ptr %30, ptr %4, align 8
  br label %15, !llvm.loop !57

31:                                               ; preds = %26
  %32 = load ptr, ptr %4, align 8
  %33 = call i64 @strlen(ptr noundef %32) #13
  store i64 %33, ptr %5, align 8
  br label %34

34:                                               ; preds = %48, %31
  %35 = load i64, ptr %5, align 8
  %36 = icmp sgt i64 %35, 0
  br i1 %36, label %37, label %46

37:                                               ; preds = %34
  %38 = load ptr, ptr %4, align 8
  %39 = load i64, ptr %5, align 8
  %40 = sub nsw i64 %39, 1
  %41 = getelementptr inbounds i8, ptr %38, i64 %40
  %42 = load i8, ptr %41, align 1
  %43 = zext i8 %42 to i32
  %44 = call i32 @isspace(i32 noundef %43) #17
  %45 = icmp ne i32 %44, 0
  br label %46

46:                                               ; preds = %37, %34
  %47 = phi i1 [ false, %34 ], [ %45, %37 ]
  br i1 %47, label %48, label %51

48:                                               ; preds = %46
  %49 = load i64, ptr %5, align 8
  %50 = add nsw i64 %49, -1
  store i64 %50, ptr %5, align 8
  br label %34, !llvm.loop !58

51:                                               ; preds = %46
  %52 = load i64, ptr %5, align 8
  %53 = add nsw i64 %52, 1
  %54 = call ptr @malloc(i64 noundef %53) #14
  store ptr %54, ptr %6, align 8
  %55 = load ptr, ptr %6, align 8
  %56 = load ptr, ptr %4, align 8
  %57 = load i64, ptr %5, align 8
  %58 = load ptr, ptr %6, align 8
  %59 = call i64 @llvm.objectsize.i64.p0(ptr %58, i1 false, i1 true, i1 false)
  %60 = call ptr @__memcpy_chk(ptr noundef %55, ptr noundef %56, i64 noundef %57, i64 noundef %59) #13
  %61 = load ptr, ptr %6, align 8
  %62 = load i64, ptr %5, align 8
  %63 = getelementptr inbounds i8, ptr %61, i64 %62
  store i8 0, ptr %63, align 1
  %64 = load ptr, ptr %6, align 8
  %65 = call ptr @ep_gc_register(ptr noundef %64, i32 noundef 1)
  %66 = load ptr, ptr %6, align 8
  %67 = ptrtoint ptr %66 to i64
  store i64 %67, ptr %2, align 8
  br label %68

68:                                               ; preds = %51, %11
  %69 = load i64, ptr %2, align 8
  ret i64 %69
}

; Function Attrs: nounwind willreturn memory(read)
declare i32 @isspace(i32 noundef) #10

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_split(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca ptr, align 8
  %11 = alloca ptr, align 8
  %12 = alloca i64, align 8
  %13 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %14 = load i64, ptr %4, align 8
  %15 = inttoptr i64 %14 to ptr
  store ptr %15, ptr %6, align 8
  %16 = load i64, ptr %5, align 8
  %17 = inttoptr i64 %16 to ptr
  store ptr %17, ptr %7, align 8
  %18 = load ptr, ptr %6, align 8
  %19 = icmp ne ptr %18, null
  br i1 %19, label %20, label %23

20:                                               ; preds = %2
  %21 = load ptr, ptr %7, align 8
  %22 = icmp ne ptr %21, null
  br i1 %22, label %25, label %23

23:                                               ; preds = %20, %2
  %24 = call i64 @create_list()
  store i64 %24, ptr %3, align 8
  br label %82

25:                                               ; preds = %20
  %26 = call i64 @create_list()
  store i64 %26, ptr %8, align 8
  %27 = load ptr, ptr %7, align 8
  %28 = call i64 @strlen(ptr noundef %27) #13
  store i64 %28, ptr %9, align 8
  %29 = load i64, ptr %9, align 8
  %30 = icmp eq i64 %29, 0
  br i1 %30, label %31, label %36

31:                                               ; preds = %25
  %32 = load i64, ptr %8, align 8
  %33 = load i64, ptr %4, align 8
  %34 = call i64 @append_list(i64 noundef %32, i64 noundef %33)
  %35 = load i64, ptr %8, align 8
  store i64 %35, ptr %3, align 8
  br label %82

36:                                               ; preds = %25
  %37 = load ptr, ptr %6, align 8
  store ptr %37, ptr %10, align 8
  br label %38

38:                                               ; preds = %36, %76
  %39 = load ptr, ptr %10, align 8
  %40 = load ptr, ptr %7, align 8
  %41 = call ptr @strstr(ptr noundef %39, ptr noundef %40) #13
  store ptr %41, ptr %11, align 8
  %42 = load ptr, ptr %11, align 8
  %43 = icmp ne ptr %42, null
  br i1 %43, label %44, label %50

44:                                               ; preds = %38
  %45 = load ptr, ptr %11, align 8
  %46 = load ptr, ptr %10, align 8
  %47 = ptrtoint ptr %45 to i64
  %48 = ptrtoint ptr %46 to i64
  %49 = sub i64 %47, %48
  br label %53

50:                                               ; preds = %38
  %51 = load ptr, ptr %10, align 8
  %52 = call i64 @strlen(ptr noundef %51) #13
  br label %53

53:                                               ; preds = %50, %44
  %54 = phi i64 [ %49, %44 ], [ %52, %50 ]
  store i64 %54, ptr %12, align 8
  %55 = load i64, ptr %12, align 8
  %56 = add nsw i64 %55, 1
  %57 = call ptr @malloc(i64 noundef %56) #14
  store ptr %57, ptr %13, align 8
  %58 = load ptr, ptr %13, align 8
  %59 = load ptr, ptr %10, align 8
  %60 = load i64, ptr %12, align 8
  %61 = load ptr, ptr %13, align 8
  %62 = call i64 @llvm.objectsize.i64.p0(ptr %61, i1 false, i1 true, i1 false)
  %63 = call ptr @__memcpy_chk(ptr noundef %58, ptr noundef %59, i64 noundef %60, i64 noundef %62) #13
  %64 = load ptr, ptr %13, align 8
  %65 = load i64, ptr %12, align 8
  %66 = getelementptr inbounds i8, ptr %64, i64 %65
  store i8 0, ptr %66, align 1
  %67 = load ptr, ptr %13, align 8
  %68 = call ptr @ep_gc_register(ptr noundef %67, i32 noundef 1)
  %69 = load i64, ptr %8, align 8
  %70 = load ptr, ptr %13, align 8
  %71 = ptrtoint ptr %70 to i64
  %72 = call i64 @append_list(i64 noundef %69, i64 noundef %71)
  %73 = load ptr, ptr %11, align 8
  %74 = icmp ne ptr %73, null
  br i1 %74, label %76, label %75

75:                                               ; preds = %53
  br label %80

76:                                               ; preds = %53
  %77 = load ptr, ptr %11, align 8
  %78 = load i64, ptr %9, align 8
  %79 = getelementptr inbounds i8, ptr %77, i64 %78
  store ptr %79, ptr %10, align 8
  br label %38

80:                                               ; preds = %75
  %81 = load i64, ptr %8, align 8
  store i64 %81, ptr %3, align 8
  br label %82

82:                                               ; preds = %80, %31, %23
  %83 = load i64, ptr %3, align 8
  ret i64 %83
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @char_at(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %7 = load i64, ptr %4, align 8
  %8 = inttoptr i64 %7 to ptr
  store ptr %8, ptr %6, align 8
  %9 = load ptr, ptr %6, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %11, label %19

11:                                               ; preds = %2
  %12 = load i64, ptr %5, align 8
  %13 = icmp slt i64 %12, 0
  br i1 %13, label %19, label %14

14:                                               ; preds = %11
  %15 = load i64, ptr %5, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = call i64 @strlen(ptr noundef %16) #13
  %18 = icmp sge i64 %15, %17
  br i1 %18, label %19, label %20

19:                                               ; preds = %14, %11, %2
  store i64 0, ptr %3, align 8
  br label %26

20:                                               ; preds = %14
  %21 = load ptr, ptr %6, align 8
  %22 = load i64, ptr %5, align 8
  %23 = getelementptr inbounds i8, ptr %21, i64 %22
  %24 = load i8, ptr %23, align 1
  %25 = zext i8 %24 to i64
  store i64 %25, ptr %3, align 8
  br label %26

26:                                               ; preds = %20, %19
  %27 = load i64, ptr %3, align 8
  ret i64 %27
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @char_from_code(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = call ptr @malloc(i64 noundef 2) #14
  store ptr %4, ptr %3, align 8
  %5 = load i64, ptr %2, align 8
  %6 = trunc i64 %5 to i8
  %7 = load ptr, ptr %3, align 8
  %8 = getelementptr inbounds i8, ptr %7, i64 0
  store i8 %6, ptr %8, align 1
  %9 = load ptr, ptr %3, align 8
  %10 = getelementptr inbounds i8, ptr %9, i64 1
  store i8 0, ptr %10, align 1
  %11 = load ptr, ptr %3, align 8
  %12 = call ptr @ep_gc_register(ptr noundef %11, i32 noundef 1)
  %13 = load ptr, ptr %3, align 8
  %14 = ptrtoint ptr %13 to i64
  ret i64 %14
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_abs(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = icmp slt i64 %3, 0
  br i1 %4, label %5, label %8

5:                                                ; preds = %1
  %6 = load i64, ptr %2, align 8
  %7 = sub nsw i64 0, %6
  br label %10

8:                                                ; preds = %1
  %9 = load i64, ptr %2, align 8
  br label %10

10:                                               ; preds = %8, %5
  %11 = phi i64 [ %7, %5 ], [ %9, %8 ]
  ret i64 %11
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_auto_to_string(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i8, align 1
  %6 = alloca i64, align 8
  %7 = alloca i32, align 4
  %8 = alloca i8, align 1
  %9 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %10 = load i64, ptr %3, align 8
  %11 = icmp eq i64 %10, 0
  br i1 %11, label %12, label %15

12:                                               ; preds = %1
  %13 = call ptr @strdup(ptr noundef @.str.25) #13
  %14 = ptrtoint ptr %13 to i64
  store i64 %14, ptr %2, align 8
  br label %86

15:                                               ; preds = %1
  %16 = load i64, ptr %3, align 8
  %17 = inttoptr i64 %16 to ptr
  %18 = call ptr @ep_gc_find(ptr noundef %17)
  store ptr %18, ptr %4, align 8
  %19 = load ptr, ptr %4, align 8
  %20 = icmp ne ptr %19, null
  br i1 %20, label %21, label %28

21:                                               ; preds = %15
  %22 = load ptr, ptr %4, align 8
  %23 = getelementptr inbounds %struct.EpGCObject, ptr %22, i32 0, i32 0
  %24 = load i32, ptr %23, align 8
  %25 = icmp eq i32 %24, 1
  br i1 %25, label %26, label %28

26:                                               ; preds = %21
  %27 = load i64, ptr %3, align 8
  store i64 %27, ptr %2, align 8
  br label %86

28:                                               ; preds = %21, %15
  %29 = load i64, ptr %3, align 8
  %30 = icmp sgt i64 %29, 1048576
  br i1 %30, label %31, label %75

31:                                               ; preds = %28
  store i64 1, ptr %6, align 8
  %32 = load i32, ptr @mach_task_self_, align 4
  %33 = load i64, ptr %3, align 8
  %34 = ptrtoint ptr %5 to i64
  %35 = call i32 @vm_read_overwrite(i32 noundef %32, i64 noundef %33, i64 noundef 1, i64 noundef %34, ptr noundef %6)
  store i32 %35, ptr %7, align 4
  %36 = load i32, ptr %7, align 4
  %37 = icmp eq i32 %36, 0
  br i1 %37, label %38, label %74

38:                                               ; preds = %31
  %39 = load i8, ptr %5, align 1
  store i8 %39, ptr %8, align 1
  %40 = load i8, ptr %8, align 1
  %41 = zext i8 %40 to i32
  %42 = icmp sge i32 %41, 32
  br i1 %42, label %43, label %47

43:                                               ; preds = %38
  %44 = load i8, ptr %8, align 1
  %45 = zext i8 %44 to i32
  %46 = icmp sle i32 %45, 126
  br i1 %46, label %71, label %47

47:                                               ; preds = %43, %38
  %48 = load i8, ptr %8, align 1
  %49 = zext i8 %48 to i32
  %50 = icmp sge i32 %49, 192
  br i1 %50, label %51, label %55

51:                                               ; preds = %47
  %52 = load i8, ptr %8, align 1
  %53 = zext i8 %52 to i32
  %54 = icmp sle i32 %53, 253
  br i1 %54, label %71, label %55

55:                                               ; preds = %51, %47
  %56 = load i8, ptr %8, align 1
  %57 = zext i8 %56 to i32
  %58 = icmp eq i32 %57, 10
  br i1 %58, label %71, label %59

59:                                               ; preds = %55
  %60 = load i8, ptr %8, align 1
  %61 = zext i8 %60 to i32
  %62 = icmp eq i32 %61, 9
  br i1 %62, label %71, label %63

63:                                               ; preds = %59
  %64 = load i8, ptr %8, align 1
  %65 = zext i8 %64 to i32
  %66 = icmp eq i32 %65, 13
  br i1 %66, label %71, label %67

67:                                               ; preds = %63
  %68 = load i8, ptr %8, align 1
  %69 = zext i8 %68 to i32
  %70 = icmp eq i32 %69, 0
  br i1 %70, label %71, label %73

71:                                               ; preds = %67, %63, %59, %55, %51, %43
  %72 = load i64, ptr %3, align 8
  store i64 %72, ptr %2, align 8
  br label %86

73:                                               ; preds = %67
  br label %74

74:                                               ; preds = %73, %31
  br label %75

75:                                               ; preds = %74, %28
  %76 = call ptr @malloc(i64 noundef 32) #14
  store ptr %76, ptr %9, align 8
  %77 = load ptr, ptr %9, align 8
  %78 = load ptr, ptr %9, align 8
  %79 = call i64 @llvm.objectsize.i64.p0(ptr %78, i1 false, i1 true, i1 false)
  %80 = load i64, ptr %3, align 8
  %81 = call i32 (ptr, i64, i32, i64, ptr, ...) @__snprintf_chk(ptr noundef %77, i64 noundef 32, i32 noundef 0, i64 noundef %79, ptr noundef @.str.26, i64 noundef %80)
  %82 = load ptr, ptr %9, align 8
  %83 = call ptr @ep_gc_register(ptr noundef %82, i32 noundef 1)
  %84 = load ptr, ptr %9, align 8
  %85 = ptrtoint ptr %84 to i64
  store i64 %85, ptr %2, align 8
  br label %86

86:                                               ; preds = %75, %71, %26, %12
  %87 = load i64, ptr %2, align 8
  ret i64 %87
}

declare i32 @vm_read_overwrite(i32 noundef, i64 noundef, i64 noundef, i64 noundef, ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_random_int(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %6 = load i64, ptr %5, align 8
  %7 = load i64, ptr %4, align 8
  %8 = icmp sle i64 %6, %7
  br i1 %8, label %9, label %11

9:                                                ; preds = %2
  %10 = load i64, ptr %4, align 8
  store i64 %10, ptr %3, align 8
  br label %21

11:                                               ; preds = %2
  %12 = load i64, ptr %4, align 8
  %13 = call i32 @rand()
  %14 = sext i32 %13 to i64
  %15 = load i64, ptr %5, align 8
  %16 = load i64, ptr %4, align 8
  %17 = sub nsw i64 %15, %16
  %18 = add nsw i64 %17, 1
  %19 = srem i64 %14, %18
  %20 = add nsw i64 %12, %19
  store i64 %20, ptr %3, align 8
  br label %21

21:                                               ; preds = %11, %9
  %22 = load i64, ptr %3, align 8
  ret i64 %22
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @json_get_string(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  %9 = alloca ptr, align 8
  %10 = alloca i64, align 8
  %11 = alloca ptr, align 8
  %12 = alloca i64, align 8
  %13 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %14 = load i64, ptr %4, align 8
  %15 = inttoptr i64 %14 to ptr
  store ptr %15, ptr %6, align 8
  %16 = load i64, ptr %5, align 8
  %17 = inttoptr i64 %16 to ptr
  store ptr %17, ptr %7, align 8
  %18 = load ptr, ptr %6, align 8
  %19 = icmp ne ptr %18, null
  br i1 %19, label %20, label %23

20:                                               ; preds = %2
  %21 = load ptr, ptr %7, align 8
  %22 = icmp ne ptr %21, null
  br i1 %22, label %26, label %23

23:                                               ; preds = %20, %2
  %24 = call ptr @strdup(ptr noundef @.str.5) #13
  %25 = ptrtoint ptr %24 to i64
  store i64 %25, ptr %3, align 8
  br label %148

26:                                               ; preds = %20
  %27 = load ptr, ptr %6, align 8
  %28 = load ptr, ptr %7, align 8
  %29 = call ptr @json_find_key(ptr noundef %27, ptr noundef %28)
  store ptr %29, ptr %8, align 8
  %30 = load ptr, ptr %8, align 8
  %31 = icmp ne ptr %30, null
  br i1 %31, label %32, label %37

32:                                               ; preds = %26
  %33 = load ptr, ptr %8, align 8
  %34 = load i8, ptr %33, align 1
  %35 = sext i8 %34 to i32
  %36 = icmp ne i32 %35, 34
  br i1 %36, label %37, label %40

37:                                               ; preds = %32, %26
  %38 = call ptr @strdup(ptr noundef @.str.5) #13
  %39 = ptrtoint ptr %38 to i64
  store i64 %39, ptr %3, align 8
  br label %148

40:                                               ; preds = %32
  %41 = load ptr, ptr %8, align 8
  %42 = getelementptr inbounds i8, ptr %41, i32 1
  store ptr %42, ptr %8, align 8
  %43 = load ptr, ptr %8, align 8
  store ptr %43, ptr %9, align 8
  br label %44

44:                                               ; preds = %64, %40
  %45 = load ptr, ptr %9, align 8
  %46 = load i8, ptr %45, align 1
  %47 = sext i8 %46 to i32
  %48 = icmp ne i32 %47, 0
  br i1 %48, label %49, label %54

49:                                               ; preds = %44
  %50 = load ptr, ptr %9, align 8
  %51 = load i8, ptr %50, align 1
  %52 = sext i8 %51 to i32
  %53 = icmp ne i32 %52, 34
  br label %54

54:                                               ; preds = %49, %44
  %55 = phi i1 [ false, %44 ], [ %53, %49 ]
  br i1 %55, label %56, label %67

56:                                               ; preds = %54
  %57 = load ptr, ptr %9, align 8
  %58 = load i8, ptr %57, align 1
  %59 = sext i8 %58 to i32
  %60 = icmp eq i32 %59, 92
  br i1 %60, label %61, label %64

61:                                               ; preds = %56
  %62 = load ptr, ptr %9, align 8
  %63 = getelementptr inbounds i8, ptr %62, i32 1
  store ptr %63, ptr %9, align 8
  br label %64

64:                                               ; preds = %61, %56
  %65 = load ptr, ptr %9, align 8
  %66 = getelementptr inbounds i8, ptr %65, i32 1
  store ptr %66, ptr %9, align 8
  br label %44, !llvm.loop !59

67:                                               ; preds = %54
  %68 = load ptr, ptr %9, align 8
  %69 = load ptr, ptr %8, align 8
  %70 = ptrtoint ptr %68 to i64
  %71 = ptrtoint ptr %69 to i64
  %72 = sub i64 %70, %71
  store i64 %72, ptr %10, align 8
  %73 = load i64, ptr %10, align 8
  %74 = add i64 %73, 1
  %75 = call ptr @malloc(i64 noundef %74) #14
  store ptr %75, ptr %11, align 8
  store i64 0, ptr %12, align 8
  %76 = load ptr, ptr %8, align 8
  store ptr %76, ptr %13, align 8
  br label %77

77:                                               ; preds = %137, %67
  %78 = load ptr, ptr %13, align 8
  %79 = load ptr, ptr %9, align 8
  %80 = icmp ult ptr %78, %79
  br i1 %80, label %81, label %140

81:                                               ; preds = %77
  %82 = load ptr, ptr %13, align 8
  %83 = load i8, ptr %82, align 1
  %84 = sext i8 %83 to i32
  %85 = icmp eq i32 %84, 92
  br i1 %85, label %86, label %130

86:                                               ; preds = %81
  %87 = load ptr, ptr %13, align 8
  %88 = getelementptr inbounds i8, ptr %87, i64 1
  %89 = load ptr, ptr %9, align 8
  %90 = icmp ult ptr %88, %89
  br i1 %90, label %91, label %130

91:                                               ; preds = %86
  %92 = load ptr, ptr %13, align 8
  %93 = getelementptr inbounds i8, ptr %92, i32 1
  store ptr %93, ptr %13, align 8
  %94 = load ptr, ptr %13, align 8
  %95 = load i8, ptr %94, align 1
  %96 = sext i8 %95 to i32
  switch i32 %96, label %122 [
    i32 110, label %97
    i32 116, label %102
    i32 114, label %107
    i32 34, label %112
    i32 92, label %117
  ]

97:                                               ; preds = %91
  %98 = load ptr, ptr %11, align 8
  %99 = load i64, ptr %12, align 8
  %100 = add i64 %99, 1
  store i64 %100, ptr %12, align 8
  %101 = getelementptr inbounds i8, ptr %98, i64 %99
  store i8 10, ptr %101, align 1
  br label %129

102:                                              ; preds = %91
  %103 = load ptr, ptr %11, align 8
  %104 = load i64, ptr %12, align 8
  %105 = add i64 %104, 1
  store i64 %105, ptr %12, align 8
  %106 = getelementptr inbounds i8, ptr %103, i64 %104
  store i8 9, ptr %106, align 1
  br label %129

107:                                              ; preds = %91
  %108 = load ptr, ptr %11, align 8
  %109 = load i64, ptr %12, align 8
  %110 = add i64 %109, 1
  store i64 %110, ptr %12, align 8
  %111 = getelementptr inbounds i8, ptr %108, i64 %109
  store i8 13, ptr %111, align 1
  br label %129

112:                                              ; preds = %91
  %113 = load ptr, ptr %11, align 8
  %114 = load i64, ptr %12, align 8
  %115 = add i64 %114, 1
  store i64 %115, ptr %12, align 8
  %116 = getelementptr inbounds i8, ptr %113, i64 %114
  store i8 34, ptr %116, align 1
  br label %129

117:                                              ; preds = %91
  %118 = load ptr, ptr %11, align 8
  %119 = load i64, ptr %12, align 8
  %120 = add i64 %119, 1
  store i64 %120, ptr %12, align 8
  %121 = getelementptr inbounds i8, ptr %118, i64 %119
  store i8 92, ptr %121, align 1
  br label %129

122:                                              ; preds = %91
  %123 = load ptr, ptr %13, align 8
  %124 = load i8, ptr %123, align 1
  %125 = load ptr, ptr %11, align 8
  %126 = load i64, ptr %12, align 8
  %127 = add i64 %126, 1
  store i64 %127, ptr %12, align 8
  %128 = getelementptr inbounds i8, ptr %125, i64 %126
  store i8 %124, ptr %128, align 1
  br label %129

129:                                              ; preds = %122, %117, %112, %107, %102, %97
  br label %137

130:                                              ; preds = %86, %81
  %131 = load ptr, ptr %13, align 8
  %132 = load i8, ptr %131, align 1
  %133 = load ptr, ptr %11, align 8
  %134 = load i64, ptr %12, align 8
  %135 = add i64 %134, 1
  store i64 %135, ptr %12, align 8
  %136 = getelementptr inbounds i8, ptr %133, i64 %134
  store i8 %132, ptr %136, align 1
  br label %137

137:                                              ; preds = %130, %129
  %138 = load ptr, ptr %13, align 8
  %139 = getelementptr inbounds i8, ptr %138, i32 1
  store ptr %139, ptr %13, align 8
  br label %77, !llvm.loop !60

140:                                              ; preds = %77
  %141 = load ptr, ptr %11, align 8
  %142 = load i64, ptr %12, align 8
  %143 = getelementptr inbounds i8, ptr %141, i64 %142
  store i8 0, ptr %143, align 1
  %144 = load ptr, ptr %11, align 8
  %145 = call ptr @ep_gc_register(ptr noundef %144, i32 noundef 1)
  %146 = load ptr, ptr %11, align 8
  %147 = ptrtoint ptr %146 to i64
  store i64 %147, ptr %3, align 8
  br label %148

148:                                              ; preds = %140, %37, %23
  %149 = load i64, ptr %3, align 8
  ret i64 %149
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal ptr @json_find_key(ptr noundef %0, ptr noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  store ptr %0, ptr %4, align 8
  store ptr %1, ptr %5, align 8
  %9 = load ptr, ptr %4, align 8
  %10 = call ptr @json_skip_ws(ptr noundef %9)
  store ptr %10, ptr %6, align 8
  %11 = load ptr, ptr %6, align 8
  %12 = load i8, ptr %11, align 1
  %13 = sext i8 %12 to i32
  %14 = icmp ne i32 %13, 123
  br i1 %14, label %15, label %16

15:                                               ; preds = %2
  store ptr null, ptr %3, align 8
  br label %116

16:                                               ; preds = %2
  %17 = load ptr, ptr %6, align 8
  %18 = getelementptr inbounds i8, ptr %17, i32 1
  store ptr %18, ptr %6, align 8
  br label %19

19:                                               ; preds = %114, %16
  %20 = load ptr, ptr %6, align 8
  %21 = load i8, ptr %20, align 1
  %22 = icmp ne i8 %21, 0
  br i1 %22, label %23, label %115

23:                                               ; preds = %19
  %24 = load ptr, ptr %6, align 8
  %25 = call ptr @json_skip_ws(ptr noundef %24)
  store ptr %25, ptr %6, align 8
  %26 = load ptr, ptr %6, align 8
  %27 = load i8, ptr %26, align 1
  %28 = sext i8 %27 to i32
  %29 = icmp eq i32 %28, 125
  br i1 %29, label %30, label %31

30:                                               ; preds = %23
  store ptr null, ptr %3, align 8
  br label %116

31:                                               ; preds = %23
  %32 = load ptr, ptr %6, align 8
  %33 = load i8, ptr %32, align 1
  %34 = sext i8 %33 to i32
  %35 = icmp ne i32 %34, 34
  br i1 %35, label %36, label %37

36:                                               ; preds = %31
  store ptr null, ptr %3, align 8
  br label %116

37:                                               ; preds = %31
  %38 = load ptr, ptr %6, align 8
  %39 = getelementptr inbounds i8, ptr %38, i32 1
  store ptr %39, ptr %6, align 8
  %40 = load ptr, ptr %6, align 8
  store ptr %40, ptr %7, align 8
  br label %41

41:                                               ; preds = %61, %37
  %42 = load ptr, ptr %6, align 8
  %43 = load i8, ptr %42, align 1
  %44 = sext i8 %43 to i32
  %45 = icmp ne i32 %44, 0
  br i1 %45, label %46, label %51

46:                                               ; preds = %41
  %47 = load ptr, ptr %6, align 8
  %48 = load i8, ptr %47, align 1
  %49 = sext i8 %48 to i32
  %50 = icmp ne i32 %49, 34
  br label %51

51:                                               ; preds = %46, %41
  %52 = phi i1 [ false, %41 ], [ %50, %46 ]
  br i1 %52, label %53, label %64

53:                                               ; preds = %51
  %54 = load ptr, ptr %6, align 8
  %55 = load i8, ptr %54, align 1
  %56 = sext i8 %55 to i32
  %57 = icmp eq i32 %56, 92
  br i1 %57, label %58, label %61

58:                                               ; preds = %53
  %59 = load ptr, ptr %6, align 8
  %60 = getelementptr inbounds i8, ptr %59, i32 1
  store ptr %60, ptr %6, align 8
  br label %61

61:                                               ; preds = %58, %53
  %62 = load ptr, ptr %6, align 8
  %63 = getelementptr inbounds i8, ptr %62, i32 1
  store ptr %63, ptr %6, align 8
  br label %41, !llvm.loop !61

64:                                               ; preds = %51
  %65 = load ptr, ptr %6, align 8
  %66 = load ptr, ptr %7, align 8
  %67 = ptrtoint ptr %65 to i64
  %68 = ptrtoint ptr %66 to i64
  %69 = sub i64 %67, %68
  store i64 %69, ptr %8, align 8
  %70 = load ptr, ptr %6, align 8
  %71 = load i8, ptr %70, align 1
  %72 = sext i8 %71 to i32
  %73 = icmp eq i32 %72, 34
  br i1 %73, label %74, label %77

74:                                               ; preds = %64
  %75 = load ptr, ptr %6, align 8
  %76 = getelementptr inbounds i8, ptr %75, i32 1
  store ptr %76, ptr %6, align 8
  br label %77

77:                                               ; preds = %74, %64
  %78 = load ptr, ptr %6, align 8
  %79 = call ptr @json_skip_ws(ptr noundef %78)
  store ptr %79, ptr %6, align 8
  %80 = load ptr, ptr %6, align 8
  %81 = load i8, ptr %80, align 1
  %82 = sext i8 %81 to i32
  %83 = icmp eq i32 %82, 58
  br i1 %83, label %84, label %87

84:                                               ; preds = %77
  %85 = load ptr, ptr %6, align 8
  %86 = getelementptr inbounds i8, ptr %85, i32 1
  store ptr %86, ptr %6, align 8
  br label %87

87:                                               ; preds = %84, %77
  %88 = load ptr, ptr %6, align 8
  %89 = call ptr @json_skip_ws(ptr noundef %88)
  store ptr %89, ptr %6, align 8
  %90 = load i64, ptr %8, align 8
  %91 = load ptr, ptr %5, align 8
  %92 = call i64 @strlen(ptr noundef %91) #13
  %93 = icmp eq i64 %90, %92
  br i1 %93, label %94, label %102

94:                                               ; preds = %87
  %95 = load ptr, ptr %7, align 8
  %96 = load ptr, ptr %5, align 8
  %97 = load i64, ptr %8, align 8
  %98 = call i32 @strncmp(ptr noundef %95, ptr noundef %96, i64 noundef %97) #13
  %99 = icmp eq i32 %98, 0
  br i1 %99, label %100, label %102

100:                                              ; preds = %94
  %101 = load ptr, ptr %6, align 8
  store ptr %101, ptr %3, align 8
  br label %116

102:                                              ; preds = %94, %87
  %103 = load ptr, ptr %6, align 8
  %104 = call ptr @json_skip_value(ptr noundef %103)
  store ptr %104, ptr %6, align 8
  %105 = load ptr, ptr %6, align 8
  %106 = call ptr @json_skip_ws(ptr noundef %105)
  store ptr %106, ptr %6, align 8
  %107 = load ptr, ptr %6, align 8
  %108 = load i8, ptr %107, align 1
  %109 = sext i8 %108 to i32
  %110 = icmp eq i32 %109, 44
  br i1 %110, label %111, label %114

111:                                              ; preds = %102
  %112 = load ptr, ptr %6, align 8
  %113 = getelementptr inbounds i8, ptr %112, i32 1
  store ptr %113, ptr %6, align 8
  br label %114

114:                                              ; preds = %111, %102
  br label %19, !llvm.loop !62

115:                                              ; preds = %19
  store ptr null, ptr %3, align 8
  br label %116

116:                                              ; preds = %115, %100, %36, %30, %15
  %117 = load ptr, ptr %3, align 8
  ret ptr %117
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @json_get_int(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %4, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %6, align 8
  %11 = load i64, ptr %5, align 8
  %12 = inttoptr i64 %11 to ptr
  store ptr %12, ptr %7, align 8
  %13 = load ptr, ptr %6, align 8
  %14 = icmp ne ptr %13, null
  br i1 %14, label %15, label %18

15:                                               ; preds = %2
  %16 = load ptr, ptr %7, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %19, label %18

18:                                               ; preds = %15, %2
  store i64 0, ptr %3, align 8
  br label %29

19:                                               ; preds = %15
  %20 = load ptr, ptr %6, align 8
  %21 = load ptr, ptr %7, align 8
  %22 = call ptr @json_find_key(ptr noundef %20, ptr noundef %21)
  store ptr %22, ptr %8, align 8
  %23 = load ptr, ptr %8, align 8
  %24 = icmp ne ptr %23, null
  br i1 %24, label %26, label %25

25:                                               ; preds = %19
  store i64 0, ptr %3, align 8
  br label %29

26:                                               ; preds = %19
  %27 = load ptr, ptr %8, align 8
  %28 = call i64 @atoll(ptr noundef %27)
  store i64 %28, ptr %3, align 8
  br label %29

29:                                               ; preds = %26, %25, %18
  %30 = load i64, ptr %3, align 8
  ret i64 %30
}

declare i64 @atoll(ptr noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @json_get_bool(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca ptr, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %4, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %6, align 8
  %11 = load i64, ptr %5, align 8
  %12 = inttoptr i64 %11 to ptr
  store ptr %12, ptr %7, align 8
  %13 = load ptr, ptr %6, align 8
  %14 = icmp ne ptr %13, null
  br i1 %14, label %15, label %18

15:                                               ; preds = %2
  %16 = load ptr, ptr %7, align 8
  %17 = icmp ne ptr %16, null
  br i1 %17, label %19, label %18

18:                                               ; preds = %15, %2
  store i64 0, ptr %3, align 8
  br label %32

19:                                               ; preds = %15
  %20 = load ptr, ptr %6, align 8
  %21 = load ptr, ptr %7, align 8
  %22 = call ptr @json_find_key(ptr noundef %20, ptr noundef %21)
  store ptr %22, ptr %8, align 8
  %23 = load ptr, ptr %8, align 8
  %24 = icmp ne ptr %23, null
  br i1 %24, label %26, label %25

25:                                               ; preds = %19
  store i64 0, ptr %3, align 8
  br label %32

26:                                               ; preds = %19
  %27 = load ptr, ptr %8, align 8
  %28 = call i32 @strncmp(ptr noundef %27, ptr noundef @.str.27, i64 noundef 4) #13
  %29 = icmp eq i32 %28, 0
  br i1 %29, label %30, label %31

30:                                               ; preds = %26
  store i64 1, ptr %3, align 8
  br label %32

31:                                               ; preds = %26
  store i64 0, ptr %3, align 8
  br label %32

32:                                               ; preds = %31, %30, %25, %18
  %33 = load i64, ptr %3, align 8
  ret i64 %33
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_sha1(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i32, align 4
  %7 = alloca i32, align 4
  %8 = alloca i32, align 4
  %9 = alloca i32, align 4
  %10 = alloca i32, align 4
  %11 = alloca i64, align 8
  %12 = alloca ptr, align 8
  %13 = alloca i64, align 8
  %14 = alloca i32, align 4
  %15 = alloca i64, align 8
  %16 = alloca [80 x i32], align 4
  %17 = alloca i32, align 4
  %18 = alloca i32, align 4
  %19 = alloca i32, align 4
  %20 = alloca i32, align 4
  %21 = alloca i32, align 4
  %22 = alloca i32, align 4
  %23 = alloca i32, align 4
  %24 = alloca i32, align 4
  %25 = alloca i32, align 4
  %26 = alloca i32, align 4
  %27 = alloca i32, align 4
  %28 = alloca [20 x i8], align 1
  %29 = alloca i64, align 8
  %30 = alloca ptr, align 8
  %31 = alloca i64, align 8
  %32 = alloca i64, align 8
  %33 = alloca i32, align 4
  store i64 %0, ptr %3, align 8
  %34 = load i64, ptr %3, align 8
  %35 = inttoptr i64 %34 to ptr
  store ptr %35, ptr %4, align 8
  %36 = load ptr, ptr %4, align 8
  %37 = icmp ne ptr %36, null
  br i1 %37, label %41, label %38

38:                                               ; preds = %1
  %39 = call ptr @strdup(ptr noundef @.str.5) #13
  %40 = ptrtoint ptr %39 to i64
  store i64 %40, ptr %2, align 8
  br label %482

41:                                               ; preds = %1
  %42 = load ptr, ptr %4, align 8
  %43 = call i64 @strlen(ptr noundef %42) #13
  store i64 %43, ptr %5, align 8
  store i32 1732584193, ptr %6, align 4
  store i32 -271733879, ptr %7, align 4
  store i32 -1732584194, ptr %8, align 4
  store i32 271733878, ptr %9, align 4
  store i32 -1009589776, ptr %10, align 4
  %44 = load i64, ptr %5, align 8
  %45 = add i64 %44, 1
  store i64 %45, ptr %11, align 8
  br label %46

46:                                               ; preds = %50, %41
  %47 = load i64, ptr %11, align 8
  %48 = urem i64 %47, 64
  %49 = icmp ne i64 %48, 56
  br i1 %49, label %50, label %53

50:                                               ; preds = %46
  %51 = load i64, ptr %11, align 8
  %52 = add i64 %51, 1
  store i64 %52, ptr %11, align 8
  br label %46, !llvm.loop !63

53:                                               ; preds = %46
  %54 = load i64, ptr %11, align 8
  %55 = add i64 %54, 8
  %56 = call ptr @calloc(i64 noundef %55, i64 noundef 1) #15
  store ptr %56, ptr %12, align 8
  %57 = load ptr, ptr %12, align 8
  %58 = load ptr, ptr %4, align 8
  %59 = load i64, ptr %5, align 8
  %60 = load ptr, ptr %12, align 8
  %61 = call i64 @llvm.objectsize.i64.p0(ptr %60, i1 false, i1 true, i1 false)
  %62 = call ptr @__memcpy_chk(ptr noundef %57, ptr noundef %58, i64 noundef %59, i64 noundef %61) #13
  %63 = load ptr, ptr %12, align 8
  %64 = load i64, ptr %5, align 8
  %65 = getelementptr inbounds i8, ptr %63, i64 %64
  store i8 -128, ptr %65, align 1
  %66 = load i64, ptr %5, align 8
  %67 = mul i64 %66, 8
  store i64 %67, ptr %13, align 8
  store i32 0, ptr %14, align 4
  br label %68

68:                                               ; preds = %85, %53
  %69 = load i32, ptr %14, align 4
  %70 = icmp slt i32 %69, 8
  br i1 %70, label %71, label %88

71:                                               ; preds = %68
  %72 = load i64, ptr %13, align 8
  %73 = load i32, ptr %14, align 4
  %74 = mul nsw i32 %73, 8
  %75 = zext i32 %74 to i64
  %76 = lshr i64 %72, %75
  %77 = trunc i64 %76 to i8
  %78 = load ptr, ptr %12, align 8
  %79 = load i64, ptr %11, align 8
  %80 = add i64 %79, 7
  %81 = load i32, ptr %14, align 4
  %82 = sext i32 %81 to i64
  %83 = sub i64 %80, %82
  %84 = getelementptr inbounds i8, ptr %78, i64 %83
  store i8 %77, ptr %84, align 1
  br label %85

85:                                               ; preds = %71
  %86 = load i32, ptr %14, align 4
  %87 = add nsw i32 %86, 1
  store i32 %87, ptr %14, align 4
  br label %68, !llvm.loop !64

88:                                               ; preds = %68
  store i64 0, ptr %15, align 8
  br label %89

89:                                               ; preds = %277, %88
  %90 = load i64, ptr %15, align 8
  %91 = load i64, ptr %11, align 8
  %92 = add i64 %91, 8
  %93 = icmp ult i64 %90, %92
  br i1 %93, label %94, label %280

94:                                               ; preds = %89
  store i32 0, ptr %17, align 4
  br label %95

95:                                               ; preds = %147, %94
  %96 = load i32, ptr %17, align 4
  %97 = icmp slt i32 %96, 16
  br i1 %97, label %98, label %150

98:                                               ; preds = %95
  %99 = load ptr, ptr %12, align 8
  %100 = load i64, ptr %15, align 8
  %101 = load i32, ptr %17, align 4
  %102 = mul nsw i32 %101, 4
  %103 = sext i32 %102 to i64
  %104 = add i64 %100, %103
  %105 = getelementptr inbounds i8, ptr %99, i64 %104
  %106 = load i8, ptr %105, align 1
  %107 = zext i8 %106 to i32
  %108 = shl i32 %107, 24
  %109 = load ptr, ptr %12, align 8
  %110 = load i64, ptr %15, align 8
  %111 = load i32, ptr %17, align 4
  %112 = mul nsw i32 %111, 4
  %113 = sext i32 %112 to i64
  %114 = add i64 %110, %113
  %115 = add i64 %114, 1
  %116 = getelementptr inbounds i8, ptr %109, i64 %115
  %117 = load i8, ptr %116, align 1
  %118 = zext i8 %117 to i32
  %119 = shl i32 %118, 16
  %120 = or i32 %108, %119
  %121 = load ptr, ptr %12, align 8
  %122 = load i64, ptr %15, align 8
  %123 = load i32, ptr %17, align 4
  %124 = mul nsw i32 %123, 4
  %125 = sext i32 %124 to i64
  %126 = add i64 %122, %125
  %127 = add i64 %126, 2
  %128 = getelementptr inbounds i8, ptr %121, i64 %127
  %129 = load i8, ptr %128, align 1
  %130 = zext i8 %129 to i32
  %131 = shl i32 %130, 8
  %132 = or i32 %120, %131
  %133 = load ptr, ptr %12, align 8
  %134 = load i64, ptr %15, align 8
  %135 = load i32, ptr %17, align 4
  %136 = mul nsw i32 %135, 4
  %137 = sext i32 %136 to i64
  %138 = add i64 %134, %137
  %139 = add i64 %138, 3
  %140 = getelementptr inbounds i8, ptr %133, i64 %139
  %141 = load i8, ptr %140, align 1
  %142 = zext i8 %141 to i32
  %143 = or i32 %132, %142
  %144 = load i32, ptr %17, align 4
  %145 = sext i32 %144 to i64
  %146 = getelementptr inbounds [80 x i32], ptr %16, i64 0, i64 %145
  store i32 %143, ptr %146, align 4
  br label %147

147:                                              ; preds = %98
  %148 = load i32, ptr %17, align 4
  %149 = add nsw i32 %148, 1
  store i32 %149, ptr %17, align 4
  br label %95, !llvm.loop !65

150:                                              ; preds = %95
  store i32 16, ptr %18, align 4
  br label %151

151:                                              ; preds = %182, %150
  %152 = load i32, ptr %18, align 4
  %153 = icmp slt i32 %152, 80
  br i1 %153, label %154, label %185

154:                                              ; preds = %151
  %155 = load i32, ptr %18, align 4
  %156 = sub nsw i32 %155, 3
  %157 = sext i32 %156 to i64
  %158 = getelementptr inbounds [80 x i32], ptr %16, i64 0, i64 %157
  %159 = load i32, ptr %158, align 4
  %160 = load i32, ptr %18, align 4
  %161 = sub nsw i32 %160, 8
  %162 = sext i32 %161 to i64
  %163 = getelementptr inbounds [80 x i32], ptr %16, i64 0, i64 %162
  %164 = load i32, ptr %163, align 4
  %165 = xor i32 %159, %164
  %166 = load i32, ptr %18, align 4
  %167 = sub nsw i32 %166, 14
  %168 = sext i32 %167 to i64
  %169 = getelementptr inbounds [80 x i32], ptr %16, i64 0, i64 %168
  %170 = load i32, ptr %169, align 4
  %171 = xor i32 %165, %170
  %172 = load i32, ptr %18, align 4
  %173 = sub nsw i32 %172, 16
  %174 = sext i32 %173 to i64
  %175 = getelementptr inbounds [80 x i32], ptr %16, i64 0, i64 %174
  %176 = load i32, ptr %175, align 4
  %177 = xor i32 %171, %176
  %178 = call i32 @sha1_left_rotate(i32 noundef %177, i32 noundef 1)
  %179 = load i32, ptr %18, align 4
  %180 = sext i32 %179 to i64
  %181 = getelementptr inbounds [80 x i32], ptr %16, i64 0, i64 %180
  store i32 %178, ptr %181, align 4
  br label %182

182:                                              ; preds = %154
  %183 = load i32, ptr %18, align 4
  %184 = add nsw i32 %183, 1
  store i32 %184, ptr %18, align 4
  br label %151, !llvm.loop !66

185:                                              ; preds = %151
  %186 = load i32, ptr %6, align 4
  store i32 %186, ptr %19, align 4
  %187 = load i32, ptr %7, align 4
  store i32 %187, ptr %20, align 4
  %188 = load i32, ptr %8, align 4
  store i32 %188, ptr %21, align 4
  %189 = load i32, ptr %9, align 4
  store i32 %189, ptr %22, align 4
  %190 = load i32, ptr %10, align 4
  store i32 %190, ptr %23, align 4
  store i32 0, ptr %24, align 4
  br label %191

191:                                              ; preds = %258, %185
  %192 = load i32, ptr %24, align 4
  %193 = icmp slt i32 %192, 80
  br i1 %193, label %194, label %261

194:                                              ; preds = %191
  %195 = load i32, ptr %24, align 4
  %196 = icmp slt i32 %195, 20
  br i1 %196, label %197, label %206

197:                                              ; preds = %194
  %198 = load i32, ptr %20, align 4
  %199 = load i32, ptr %21, align 4
  %200 = and i32 %198, %199
  %201 = load i32, ptr %20, align 4
  %202 = xor i32 %201, -1
  %203 = load i32, ptr %22, align 4
  %204 = and i32 %202, %203
  %205 = or i32 %200, %204
  store i32 %205, ptr %25, align 4
  store i32 1518500249, ptr %26, align 4
  br label %238

206:                                              ; preds = %194
  %207 = load i32, ptr %24, align 4
  %208 = icmp slt i32 %207, 40
  br i1 %208, label %209, label %215

209:                                              ; preds = %206
  %210 = load i32, ptr %20, align 4
  %211 = load i32, ptr %21, align 4
  %212 = xor i32 %210, %211
  %213 = load i32, ptr %22, align 4
  %214 = xor i32 %212, %213
  store i32 %214, ptr %25, align 4
  store i32 1859775393, ptr %26, align 4
  br label %237

215:                                              ; preds = %206
  %216 = load i32, ptr %24, align 4
  %217 = icmp slt i32 %216, 60
  br i1 %217, label %218, label %230

218:                                              ; preds = %215
  %219 = load i32, ptr %20, align 4
  %220 = load i32, ptr %21, align 4
  %221 = and i32 %219, %220
  %222 = load i32, ptr %20, align 4
  %223 = load i32, ptr %22, align 4
  %224 = and i32 %222, %223
  %225 = or i32 %221, %224
  %226 = load i32, ptr %21, align 4
  %227 = load i32, ptr %22, align 4
  %228 = and i32 %226, %227
  %229 = or i32 %225, %228
  store i32 %229, ptr %25, align 4
  store i32 -1894007588, ptr %26, align 4
  br label %236

230:                                              ; preds = %215
  %231 = load i32, ptr %20, align 4
  %232 = load i32, ptr %21, align 4
  %233 = xor i32 %231, %232
  %234 = load i32, ptr %22, align 4
  %235 = xor i32 %233, %234
  store i32 %235, ptr %25, align 4
  store i32 -899497514, ptr %26, align 4
  br label %236

236:                                              ; preds = %230, %218
  br label %237

237:                                              ; preds = %236, %209
  br label %238

238:                                              ; preds = %237, %197
  %239 = load i32, ptr %19, align 4
  %240 = call i32 @sha1_left_rotate(i32 noundef %239, i32 noundef 5)
  %241 = load i32, ptr %25, align 4
  %242 = add i32 %240, %241
  %243 = load i32, ptr %23, align 4
  %244 = add i32 %242, %243
  %245 = load i32, ptr %26, align 4
  %246 = add i32 %244, %245
  %247 = load i32, ptr %24, align 4
  %248 = sext i32 %247 to i64
  %249 = getelementptr inbounds [80 x i32], ptr %16, i64 0, i64 %248
  %250 = load i32, ptr %249, align 4
  %251 = add i32 %246, %250
  store i32 %251, ptr %27, align 4
  %252 = load i32, ptr %22, align 4
  store i32 %252, ptr %23, align 4
  %253 = load i32, ptr %21, align 4
  store i32 %253, ptr %22, align 4
  %254 = load i32, ptr %20, align 4
  %255 = call i32 @sha1_left_rotate(i32 noundef %254, i32 noundef 30)
  store i32 %255, ptr %21, align 4
  %256 = load i32, ptr %19, align 4
  store i32 %256, ptr %20, align 4
  %257 = load i32, ptr %27, align 4
  store i32 %257, ptr %19, align 4
  br label %258

258:                                              ; preds = %238
  %259 = load i32, ptr %24, align 4
  %260 = add nsw i32 %259, 1
  store i32 %260, ptr %24, align 4
  br label %191, !llvm.loop !67

261:                                              ; preds = %191
  %262 = load i32, ptr %19, align 4
  %263 = load i32, ptr %6, align 4
  %264 = add i32 %263, %262
  store i32 %264, ptr %6, align 4
  %265 = load i32, ptr %20, align 4
  %266 = load i32, ptr %7, align 4
  %267 = add i32 %266, %265
  store i32 %267, ptr %7, align 4
  %268 = load i32, ptr %21, align 4
  %269 = load i32, ptr %8, align 4
  %270 = add i32 %269, %268
  store i32 %270, ptr %8, align 4
  %271 = load i32, ptr %22, align 4
  %272 = load i32, ptr %9, align 4
  %273 = add i32 %272, %271
  store i32 %273, ptr %9, align 4
  %274 = load i32, ptr %23, align 4
  %275 = load i32, ptr %10, align 4
  %276 = add i32 %275, %274
  store i32 %276, ptr %10, align 4
  br label %277

277:                                              ; preds = %261
  %278 = load i64, ptr %15, align 8
  %279 = add i64 %278, 64
  store i64 %279, ptr %15, align 8
  br label %89, !llvm.loop !68

280:                                              ; preds = %89
  %281 = load ptr, ptr %12, align 8
  call void @free(ptr noundef %281)
  %282 = load i32, ptr %6, align 4
  %283 = lshr i32 %282, 24
  %284 = and i32 %283, 255
  %285 = trunc i32 %284 to i8
  %286 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 0
  store i8 %285, ptr %286, align 1
  %287 = load i32, ptr %6, align 4
  %288 = lshr i32 %287, 16
  %289 = and i32 %288, 255
  %290 = trunc i32 %289 to i8
  %291 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 1
  store i8 %290, ptr %291, align 1
  %292 = load i32, ptr %6, align 4
  %293 = lshr i32 %292, 8
  %294 = and i32 %293, 255
  %295 = trunc i32 %294 to i8
  %296 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 2
  store i8 %295, ptr %296, align 1
  %297 = load i32, ptr %6, align 4
  %298 = and i32 %297, 255
  %299 = trunc i32 %298 to i8
  %300 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 3
  store i8 %299, ptr %300, align 1
  %301 = load i32, ptr %7, align 4
  %302 = lshr i32 %301, 24
  %303 = and i32 %302, 255
  %304 = trunc i32 %303 to i8
  %305 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 4
  store i8 %304, ptr %305, align 1
  %306 = load i32, ptr %7, align 4
  %307 = lshr i32 %306, 16
  %308 = and i32 %307, 255
  %309 = trunc i32 %308 to i8
  %310 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 5
  store i8 %309, ptr %310, align 1
  %311 = load i32, ptr %7, align 4
  %312 = lshr i32 %311, 8
  %313 = and i32 %312, 255
  %314 = trunc i32 %313 to i8
  %315 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 6
  store i8 %314, ptr %315, align 1
  %316 = load i32, ptr %7, align 4
  %317 = and i32 %316, 255
  %318 = trunc i32 %317 to i8
  %319 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 7
  store i8 %318, ptr %319, align 1
  %320 = load i32, ptr %8, align 4
  %321 = lshr i32 %320, 24
  %322 = and i32 %321, 255
  %323 = trunc i32 %322 to i8
  %324 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 8
  store i8 %323, ptr %324, align 1
  %325 = load i32, ptr %8, align 4
  %326 = lshr i32 %325, 16
  %327 = and i32 %326, 255
  %328 = trunc i32 %327 to i8
  %329 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 9
  store i8 %328, ptr %329, align 1
  %330 = load i32, ptr %8, align 4
  %331 = lshr i32 %330, 8
  %332 = and i32 %331, 255
  %333 = trunc i32 %332 to i8
  %334 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 10
  store i8 %333, ptr %334, align 1
  %335 = load i32, ptr %8, align 4
  %336 = and i32 %335, 255
  %337 = trunc i32 %336 to i8
  %338 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 11
  store i8 %337, ptr %338, align 1
  %339 = load i32, ptr %9, align 4
  %340 = lshr i32 %339, 24
  %341 = and i32 %340, 255
  %342 = trunc i32 %341 to i8
  %343 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 12
  store i8 %342, ptr %343, align 1
  %344 = load i32, ptr %9, align 4
  %345 = lshr i32 %344, 16
  %346 = and i32 %345, 255
  %347 = trunc i32 %346 to i8
  %348 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 13
  store i8 %347, ptr %348, align 1
  %349 = load i32, ptr %9, align 4
  %350 = lshr i32 %349, 8
  %351 = and i32 %350, 255
  %352 = trunc i32 %351 to i8
  %353 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 14
  store i8 %352, ptr %353, align 1
  %354 = load i32, ptr %9, align 4
  %355 = and i32 %354, 255
  %356 = trunc i32 %355 to i8
  %357 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 15
  store i8 %356, ptr %357, align 1
  %358 = load i32, ptr %10, align 4
  %359 = lshr i32 %358, 24
  %360 = and i32 %359, 255
  %361 = trunc i32 %360 to i8
  %362 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 16
  store i8 %361, ptr %362, align 1
  %363 = load i32, ptr %10, align 4
  %364 = lshr i32 %363, 16
  %365 = and i32 %364, 255
  %366 = trunc i32 %365 to i8
  %367 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 17
  store i8 %366, ptr %367, align 1
  %368 = load i32, ptr %10, align 4
  %369 = lshr i32 %368, 8
  %370 = and i32 %369, 255
  %371 = trunc i32 %370 to i8
  %372 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 18
  store i8 %371, ptr %372, align 1
  %373 = load i32, ptr %10, align 4
  %374 = and i32 %373, 255
  %375 = trunc i32 %374 to i8
  %376 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 19
  store i8 %375, ptr %376, align 1
  store i64 28, ptr %29, align 8
  %377 = load i64, ptr %29, align 8
  %378 = add i64 %377, 1
  %379 = call ptr @malloc(i64 noundef %378) #14
  store ptr %379, ptr %30, align 8
  store i64 0, ptr %31, align 8
  store i64 0, ptr %32, align 8
  br label %380

380:                                              ; preds = %471, %280
  %381 = load i64, ptr %32, align 8
  %382 = icmp ult i64 %381, 20
  br i1 %382, label %383, label %474

383:                                              ; preds = %380
  %384 = load i64, ptr %32, align 8
  %385 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 %384
  %386 = load i8, ptr %385, align 1
  %387 = zext i8 %386 to i32
  %388 = shl i32 %387, 16
  store i32 %388, ptr %33, align 4
  %389 = load i64, ptr %32, align 8
  %390 = add i64 %389, 1
  %391 = icmp ult i64 %390, 20
  br i1 %391, label %392, label %401

392:                                              ; preds = %383
  %393 = load i64, ptr %32, align 8
  %394 = add i64 %393, 1
  %395 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 %394
  %396 = load i8, ptr %395, align 1
  %397 = zext i8 %396 to i32
  %398 = shl i32 %397, 8
  %399 = load i32, ptr %33, align 4
  %400 = or i32 %399, %398
  store i32 %400, ptr %33, align 4
  br label %401

401:                                              ; preds = %392, %383
  %402 = load i64, ptr %32, align 8
  %403 = add i64 %402, 2
  %404 = icmp ult i64 %403, 20
  br i1 %404, label %405, label %413

405:                                              ; preds = %401
  %406 = load i64, ptr %32, align 8
  %407 = add i64 %406, 2
  %408 = getelementptr inbounds [20 x i8], ptr %28, i64 0, i64 %407
  %409 = load i8, ptr %408, align 1
  %410 = zext i8 %409 to i32
  %411 = load i32, ptr %33, align 4
  %412 = or i32 %411, %410
  store i32 %412, ptr %33, align 4
  br label %413

413:                                              ; preds = %405, %401
  %414 = load i32, ptr %33, align 4
  %415 = lshr i32 %414, 18
  %416 = and i32 %415, 63
  %417 = zext i32 %416 to i64
  %418 = getelementptr inbounds [65 x i8], ptr @b64_table, i64 0, i64 %417
  %419 = load i8, ptr %418, align 1
  %420 = load ptr, ptr %30, align 8
  %421 = load i64, ptr %31, align 8
  %422 = add i64 %421, 1
  store i64 %422, ptr %31, align 8
  %423 = getelementptr inbounds i8, ptr %420, i64 %421
  store i8 %419, ptr %423, align 1
  %424 = load i32, ptr %33, align 4
  %425 = lshr i32 %424, 12
  %426 = and i32 %425, 63
  %427 = zext i32 %426 to i64
  %428 = getelementptr inbounds [65 x i8], ptr @b64_table, i64 0, i64 %427
  %429 = load i8, ptr %428, align 1
  %430 = load ptr, ptr %30, align 8
  %431 = load i64, ptr %31, align 8
  %432 = add i64 %431, 1
  store i64 %432, ptr %31, align 8
  %433 = getelementptr inbounds i8, ptr %430, i64 %431
  store i8 %429, ptr %433, align 1
  %434 = load i64, ptr %32, align 8
  %435 = add i64 %434, 1
  %436 = icmp ult i64 %435, 20
  br i1 %436, label %437, label %445

437:                                              ; preds = %413
  %438 = load i32, ptr %33, align 4
  %439 = lshr i32 %438, 6
  %440 = and i32 %439, 63
  %441 = zext i32 %440 to i64
  %442 = getelementptr inbounds [65 x i8], ptr @b64_table, i64 0, i64 %441
  %443 = load i8, ptr %442, align 1
  %444 = sext i8 %443 to i32
  br label %446

445:                                              ; preds = %413
  br label %446

446:                                              ; preds = %445, %437
  %447 = phi i32 [ %444, %437 ], [ 61, %445 ]
  %448 = trunc i32 %447 to i8
  %449 = load ptr, ptr %30, align 8
  %450 = load i64, ptr %31, align 8
  %451 = add i64 %450, 1
  store i64 %451, ptr %31, align 8
  %452 = getelementptr inbounds i8, ptr %449, i64 %450
  store i8 %448, ptr %452, align 1
  %453 = load i64, ptr %32, align 8
  %454 = add i64 %453, 2
  %455 = icmp ult i64 %454, 20
  br i1 %455, label %456, label %463

456:                                              ; preds = %446
  %457 = load i32, ptr %33, align 4
  %458 = and i32 %457, 63
  %459 = zext i32 %458 to i64
  %460 = getelementptr inbounds [65 x i8], ptr @b64_table, i64 0, i64 %459
  %461 = load i8, ptr %460, align 1
  %462 = sext i8 %461 to i32
  br label %464

463:                                              ; preds = %446
  br label %464

464:                                              ; preds = %463, %456
  %465 = phi i32 [ %462, %456 ], [ 61, %463 ]
  %466 = trunc i32 %465 to i8
  %467 = load ptr, ptr %30, align 8
  %468 = load i64, ptr %31, align 8
  %469 = add i64 %468, 1
  store i64 %469, ptr %31, align 8
  %470 = getelementptr inbounds i8, ptr %467, i64 %468
  store i8 %466, ptr %470, align 1
  br label %471

471:                                              ; preds = %464
  %472 = load i64, ptr %32, align 8
  %473 = add i64 %472, 3
  store i64 %473, ptr %32, align 8
  br label %380, !llvm.loop !69

474:                                              ; preds = %380
  %475 = load ptr, ptr %30, align 8
  %476 = load i64, ptr %31, align 8
  %477 = getelementptr inbounds i8, ptr %475, i64 %476
  store i8 0, ptr %477, align 1
  %478 = load ptr, ptr %30, align 8
  %479 = call ptr @ep_gc_register(ptr noundef %478, i32 noundef 1)
  %480 = load ptr, ptr %30, align 8
  %481 = ptrtoint ptr %480 to i64
  store i64 %481, ptr %2, align 8
  br label %482

482:                                              ; preds = %474, %38
  %483 = load i64, ptr %2, align 8
  ret i64 %483
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal i32 @sha1_left_rotate(i32 noundef %0, i32 noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 %0, ptr %3, align 4
  store i32 %1, ptr %4, align 4
  %5 = load i32, ptr %3, align 4
  %6 = load i32, ptr %4, align 4
  %7 = shl i32 %5, %6
  %8 = load i32, ptr %3, align 4
  %9 = load i32, ptr %4, align 4
  %10 = sub nsw i32 32, %9
  %11 = lshr i32 %8, %10
  %12 = or i32 %7, %11
  ret i32 %12
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_net_recv_bytes(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  %9 = load i64, ptr %5, align 8
  %10 = icmp sle i64 %9, 0
  br i1 %10, label %11, label %14

11:                                               ; preds = %2
  %12 = call ptr @strdup(ptr noundef @.str.5) #13
  %13 = ptrtoint ptr %12 to i64
  store i64 %13, ptr %3, align 8
  br label %47

14:                                               ; preds = %2
  %15 = load i64, ptr %5, align 8
  %16 = add nsw i64 %15, 1
  %17 = call ptr @malloc(i64 noundef %16) #14
  store ptr %17, ptr %6, align 8
  store i64 0, ptr %7, align 8
  br label %18

18:                                               ; preds = %35, %14
  %19 = load i64, ptr %7, align 8
  %20 = load i64, ptr %5, align 8
  %21 = icmp slt i64 %19, %20
  br i1 %21, label %22, label %39

22:                                               ; preds = %18
  %23 = load i64, ptr %4, align 8
  %24 = trunc i64 %23 to i32
  %25 = load ptr, ptr %6, align 8
  %26 = load i64, ptr %7, align 8
  %27 = getelementptr inbounds i8, ptr %25, i64 %26
  %28 = load i64, ptr %5, align 8
  %29 = load i64, ptr %7, align 8
  %30 = sub nsw i64 %28, %29
  %31 = call i64 @"\01_recv"(i32 noundef %24, ptr noundef %27, i64 noundef %30, i32 noundef 0)
  store i64 %31, ptr %8, align 8
  %32 = load i64, ptr %8, align 8
  %33 = icmp sle i64 %32, 0
  br i1 %33, label %34, label %35

34:                                               ; preds = %22
  br label %39

35:                                               ; preds = %22
  %36 = load i64, ptr %8, align 8
  %37 = load i64, ptr %7, align 8
  %38 = add nsw i64 %37, %36
  store i64 %38, ptr %7, align 8
  br label %18, !llvm.loop !70

39:                                               ; preds = %34, %18
  %40 = load ptr, ptr %6, align 8
  %41 = load i64, ptr %7, align 8
  %42 = getelementptr inbounds i8, ptr %40, i64 %41
  store i8 0, ptr %42, align 1
  %43 = load ptr, ptr %6, align 8
  %44 = call ptr @ep_gc_register(ptr noundef %43, i32 noundef 1)
  %45 = load ptr, ptr %6, align 8
  %46 = ptrtoint ptr %45 to i64
  store i64 %46, ptr %3, align 8
  br label %47

47:                                               ; preds = %39, %11
  %48 = load i64, ptr %3, align 8
  ret i64 %48
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_get_args() #0 {
  %1 = alloca i64, align 8
  %2 = alloca i32, align 4
  %3 = alloca ptr, align 8
  %4 = call i64 @create_list()
  store i64 %4, ptr %1, align 8
  store i32 0, ptr %2, align 4
  br label %5

5:                                                ; preds = %22, %0
  %6 = load i32, ptr %2, align 4
  %7 = load i32, ptr @ep_argc, align 4
  %8 = icmp slt i32 %6, %7
  br i1 %8, label %9, label %25

9:                                                ; preds = %5
  %10 = load ptr, ptr @ep_argv, align 8
  %11 = load i32, ptr %2, align 4
  %12 = sext i32 %11 to i64
  %13 = getelementptr inbounds ptr, ptr %10, i64 %12
  %14 = load ptr, ptr %13, align 8
  %15 = call ptr @strdup(ptr noundef %14) #13
  store ptr %15, ptr %3, align 8
  %16 = load ptr, ptr %3, align 8
  %17 = call ptr @ep_gc_register(ptr noundef %16, i32 noundef 1)
  %18 = load i64, ptr %1, align 8
  %19 = load ptr, ptr %3, align 8
  %20 = ptrtoint ptr %19 to i64
  %21 = call i64 @append_list(i64 noundef %18, i64 noundef %20)
  br label %22

22:                                               ; preds = %9
  %23 = load i32, ptr %2, align 4
  %24 = add nsw i32 %23, 1
  store i32 %24, ptr %2, align 4
  br label %5, !llvm.loop !71

25:                                               ; preds = %5
  %26 = load i64, ptr %1, align 8
  ret i64 %26
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @free_struct_Node(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = load i64, ptr %2, align 8
  %5 = icmp eq i64 %4, 0
  br i1 %5, label %6, label %7

6:                                                ; preds = %1
  br label %21

7:                                                ; preds = %1
  %8 = load i64, ptr %2, align 8
  %9 = inttoptr i64 %8 to ptr
  %10 = call ptr @ep_gc_find(ptr noundef %9)
  %11 = icmp ne ptr %10, null
  br i1 %11, label %13, label %12

12:                                               ; preds = %7
  br label %21

13:                                               ; preds = %7
  %14 = load i64, ptr %2, align 8
  %15 = inttoptr i64 %14 to ptr
  store ptr %15, ptr %3, align 8
  %16 = load ptr, ptr %3, align 8
  %17 = getelementptr inbounds %struct.EpStruct_Node, ptr %16, i32 0, i32 1
  %18 = load i64, ptr %17, align 8
  call void @free_struct_Node(i64 noundef %18)
  %19 = load ptr, ptr %3, align 8
  call void @ep_gc_unregister(ptr noundef %19)
  %20 = load ptr, ptr %3, align 8
  call void @free(ptr noundef %20)
  br label %21

21:                                               ; preds = %13, %12, %6
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @concat(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %10 = load i64, ptr %3, align 8
  %11 = inttoptr i64 %10 to ptr
  store ptr %11, ptr %5, align 8
  %12 = load i64, ptr %4, align 8
  %13 = inttoptr i64 %12 to ptr
  store ptr %13, ptr %6, align 8
  %14 = load ptr, ptr %5, align 8
  %15 = call i64 @strlen(ptr noundef %14) #13
  store i64 %15, ptr %7, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = call i64 @strlen(ptr noundef %16) #13
  store i64 %17, ptr %8, align 8
  %18 = load i64, ptr %7, align 8
  %19 = load i64, ptr %8, align 8
  %20 = add nsw i64 %18, %19
  %21 = add nsw i64 %20, 1
  %22 = call ptr @malloc(i64 noundef %21) #14
  store ptr %22, ptr %9, align 8
  %23 = load ptr, ptr %9, align 8
  %24 = load ptr, ptr %5, align 8
  %25 = load i64, ptr %7, align 8
  %26 = load ptr, ptr %9, align 8
  %27 = call i64 @llvm.objectsize.i64.p0(ptr %26, i1 false, i1 true, i1 false)
  %28 = call ptr @__memcpy_chk(ptr noundef %23, ptr noundef %24, i64 noundef %25, i64 noundef %27) #13
  %29 = load ptr, ptr %9, align 8
  %30 = load i64, ptr %7, align 8
  %31 = getelementptr inbounds i8, ptr %29, i64 %30
  %32 = load ptr, ptr %6, align 8
  %33 = load i64, ptr %8, align 8
  %34 = load ptr, ptr %9, align 8
  %35 = load i64, ptr %7, align 8
  %36 = getelementptr inbounds i8, ptr %34, i64 %35
  %37 = call i64 @llvm.objectsize.i64.p0(ptr %36, i1 false, i1 true, i1 false)
  %38 = call ptr @__memcpy_chk(ptr noundef %31, ptr noundef %32, i64 noundef %33, i64 noundef %37) #13
  %39 = load ptr, ptr %9, align 8
  %40 = load i64, ptr %7, align 8
  %41 = load i64, ptr %8, align 8
  %42 = add nsw i64 %40, %41
  %43 = getelementptr inbounds i8, ptr %39, i64 %42
  store i8 0, ptr %43, align 1
  %44 = load ptr, ptr %9, align 8
  %45 = call ptr @ep_gc_register(ptr noundef %44, i32 noundef 1)
  %46 = load ptr, ptr %9, align 8
  %47 = ptrtoint ptr %46 to i64
  ret i64 %47
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @int_to_string(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  store i64 %0, ptr %2, align 8
  %4 = call ptr @malloc(i64 noundef 32) #14
  store ptr %4, ptr %3, align 8
  %5 = load ptr, ptr %3, align 8
  %6 = load ptr, ptr %3, align 8
  %7 = call i64 @llvm.objectsize.i64.p0(ptr %6, i1 false, i1 true, i1 false)
  %8 = load i64, ptr %2, align 8
  %9 = call i32 (ptr, i64, i32, i64, ptr, ...) @__snprintf_chk(ptr noundef %5, i64 noundef 32, i32 noundef 0, i64 noundef %7, ptr noundef @.str.26, i64 noundef %8)
  %10 = load ptr, ptr %3, align 8
  %11 = call ptr @ep_gc_register(ptr noundef %10, i32 noundef 1)
  %12 = load ptr, ptr %3, align 8
  %13 = ptrtoint ptr %12 to i64
  ret i64 %13
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_int_to_str(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = call i64 @int_to_string(i64 noundef %3)
  ret i64 %4
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @str_to_ptr(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  ret i64 %3
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ptr_to_str(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  store i64 %0, ptr %3, align 8
  %5 = load i64, ptr %3, align 8
  %6 = icmp eq i64 %5, 0
  br i1 %6, label %7, label %10

7:                                                ; preds = %1
  %8 = call ptr @strdup(ptr noundef @.str.5) #13
  %9 = ptrtoint ptr %8 to i64
  store i64 %9, ptr %2, align 8
  br label %18

10:                                               ; preds = %1
  %11 = load i64, ptr %3, align 8
  %12 = inttoptr i64 %11 to ptr
  %13 = call ptr @strdup(ptr noundef %12) #13
  store ptr %13, ptr %4, align 8
  %14 = load ptr, ptr %4, align 8
  %15 = call ptr @ep_gc_register(ptr noundef %14, i32 noundef 1)
  %16 = load ptr, ptr %4, align 8
  %17 = ptrtoint ptr %16 to i64
  store i64 %17, ptr %2, align 8
  br label %18

18:                                               ; preds = %10, %7
  %19 = load i64, ptr %2, align 8
  ret i64 %19
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @peek_byte(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %5 = load i64, ptr %3, align 8
  %6 = inttoptr i64 %5 to ptr
  %7 = load i64, ptr %4, align 8
  %8 = getelementptr inbounds i8, ptr %6, i64 %7
  %9 = load i8, ptr %8, align 1
  %10 = zext i8 %9 to i64
  ret i64 %10
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @poke_byte(i64 noundef %0, i64 noundef %1, i64 noundef %2) #0 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  store i64 %0, ptr %4, align 8
  store i64 %1, ptr %5, align 8
  store i64 %2, ptr %6, align 8
  %7 = load i64, ptr %6, align 8
  %8 = trunc i64 %7 to i8
  %9 = load i64, ptr %4, align 8
  %10 = inttoptr i64 %9 to ptr
  %11 = load i64, ptr %5, align 8
  %12 = getelementptr inbounds i8, ptr %10, i64 %11
  store i8 %8, ptr %12, align 1
  ret i64 0
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @alloc_bytes(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = call ptr @calloc(i64 noundef %3, i64 noundef 1) #15
  %5 = ptrtoint ptr %4 to i64
  ret i64 %5
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @free_bytes(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = inttoptr i64 %3 to ptr
  call void @free(ptr noundef %4)
  ret i64 0
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @list_to_bytes(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %6 = load i64, ptr %2, align 8
  %7 = call i64 @length_list(i64 noundef %6)
  store i64 %7, ptr %3, align 8
  %8 = load i64, ptr %3, align 8
  %9 = call ptr @malloc(i64 noundef %8) #14
  store ptr %9, ptr %4, align 8
  store i64 0, ptr %5, align 8
  br label %10

10:                                               ; preds = %22, %1
  %11 = load i64, ptr %5, align 8
  %12 = load i64, ptr %3, align 8
  %13 = icmp slt i64 %11, %12
  br i1 %13, label %14, label %25

14:                                               ; preds = %10
  %15 = load i64, ptr %2, align 8
  %16 = load i64, ptr %5, align 8
  %17 = call i64 @get_list(i64 noundef %15, i64 noundef %16)
  %18 = trunc i64 %17 to i8
  %19 = load ptr, ptr %4, align 8
  %20 = load i64, ptr %5, align 8
  %21 = getelementptr inbounds i8, ptr %19, i64 %20
  store i8 %18, ptr %21, align 1
  br label %22

22:                                               ; preds = %14
  %23 = load i64, ptr %5, align 8
  %24 = add nsw i64 %23, 1
  store i64 %24, ptr %5, align 8
  br label %10, !llvm.loop !72

25:                                               ; preds = %10
  %26 = load ptr, ptr %4, align 8
  %27 = ptrtoint ptr %26 to i64
  ret i64 %27
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @bytes_to_list(i64 noundef %0, i64 noundef %1) #0 {
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  store i64 %1, ptr %4, align 8
  %8 = call i64 @create_list()
  store i64 %8, ptr %5, align 8
  %9 = load i64, ptr %3, align 8
  %10 = inttoptr i64 %9 to ptr
  store ptr %10, ptr %6, align 8
  store i64 0, ptr %7, align 8
  br label %11

11:                                               ; preds = %23, %2
  %12 = load i64, ptr %7, align 8
  %13 = load i64, ptr %4, align 8
  %14 = icmp slt i64 %12, %13
  br i1 %14, label %15, label %26

15:                                               ; preds = %11
  %16 = load i64, ptr %5, align 8
  %17 = load ptr, ptr %6, align 8
  %18 = load i64, ptr %7, align 8
  %19 = getelementptr inbounds i8, ptr %17, i64 %18
  %20 = load i8, ptr %19, align 1
  %21 = zext i8 %20 to i64
  %22 = call i64 @append_list(i64 noundef %16, i64 noundef %21)
  br label %23

23:                                               ; preds = %15
  %24 = load i64, ptr %7, align 8
  %25 = add nsw i64 %24, 1
  store i64 %25, ptr %7, align 8
  br label %11, !llvm.loop !73

26:                                               ; preds = %11
  %27 = load i64, ptr %5, align 8
  ret i64 %27
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_gc_get_minor_count() #0 {
  %1 = load i32, ptr @ep_gc_minor_count, align 4
  %2 = sext i32 %1 to i64
  ret i64 %2
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_gc_get_major_count() #0 {
  %1 = load i32, ptr @ep_gc_major_count, align 4
  %2 = sext i32 %1 to i64
  ret i64 %2
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @ep_gc_get_nursery_count() #0 {
  %1 = load i64, ptr @ep_gc_nursery_count, align 8
  ret i64 %1
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @string_to_int(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  store i64 %0, ptr %3, align 8
  %4 = load i64, ptr %3, align 8
  %5 = icmp eq i64 %4, 0
  br i1 %5, label %6, label %7

6:                                                ; preds = %1
  store i64 0, ptr %2, align 8
  br label %11

7:                                                ; preds = %1
  %8 = load i64, ptr %3, align 8
  %9 = inttoptr i64 %8 to ptr
  %10 = call i64 @atoll(ptr noundef %9)
  store i64 %10, ptr %2, align 8
  br label %11

11:                                               ; preds = %7, %6
  %12 = load i64, ptr %2, align 8
  ret i64 %12
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @read_line() #0 {
  %1 = alloca [4096 x i8], align 1
  %2 = alloca i64, align 8
  %3 = alloca ptr, align 8
  %4 = getelementptr inbounds [4096 x i8], ptr %1, i64 0, i64 0
  %5 = load ptr, ptr @__stdinp, align 8
  %6 = call ptr @fgets(ptr noundef %4, i32 noundef 4096, ptr noundef %5)
  %7 = icmp eq ptr %6, null
  br i1 %7, label %8, label %10

8:                                                ; preds = %0
  %9 = getelementptr inbounds [4096 x i8], ptr %1, i64 0, i64 0
  store i8 0, ptr %9, align 1
  br label %10

10:                                               ; preds = %8, %0
  %11 = getelementptr inbounds [4096 x i8], ptr %1, i64 0, i64 0
  %12 = call i64 @strlen(ptr noundef %11) #13
  store i64 %12, ptr %2, align 8
  %13 = load i64, ptr %2, align 8
  %14 = icmp ugt i64 %13, 0
  br i1 %14, label %15, label %26

15:                                               ; preds = %10
  %16 = load i64, ptr %2, align 8
  %17 = sub i64 %16, 1
  %18 = getelementptr inbounds [4096 x i8], ptr %1, i64 0, i64 %17
  %19 = load i8, ptr %18, align 1
  %20 = sext i8 %19 to i32
  %21 = icmp eq i32 %20, 10
  br i1 %21, label %22, label %26

22:                                               ; preds = %15
  %23 = load i64, ptr %2, align 8
  %24 = sub i64 %23, 1
  %25 = getelementptr inbounds [4096 x i8], ptr %1, i64 0, i64 %24
  store i8 0, ptr %25, align 1
  br label %26

26:                                               ; preds = %22, %15, %10
  %27 = getelementptr inbounds [4096 x i8], ptr %1, i64 0, i64 0
  %28 = call ptr @strdup(ptr noundef %27) #13
  store ptr %28, ptr %3, align 8
  %29 = load ptr, ptr %3, align 8
  %30 = call ptr @ep_gc_register(ptr noundef %29, i32 noundef 1)
  %31 = load ptr, ptr %3, align 8
  %32 = ptrtoint ptr %31 to i64
  ret i64 %32
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @read_int() #0 {
  %1 = alloca i64, align 8
  store i64 0, ptr %1, align 8
  %2 = call i32 (ptr, ...) @scanf(ptr noundef @.str.26, ptr noundef %1)
  br label %3

3:                                                ; preds = %6, %0
  %4 = call i32 @getchar()
  %5 = icmp ne i32 %4, 10
  br i1 %5, label %6, label %7

6:                                                ; preds = %3
  br label %3, !llvm.loop !74

7:                                                ; preds = %3
  %8 = load i64, ptr %1, align 8
  ret i64 %8
}

declare i32 @scanf(ptr noundef, ...) #1

declare i32 @getchar() #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @read_float() #0 {
  %1 = alloca double, align 8
  %2 = alloca i64, align 8
  store double 0.000000e+00, ptr %1, align 8
  %3 = call i32 (ptr, ...) @scanf(ptr noundef @.str.28, ptr noundef %1)
  br label %4

4:                                                ; preds = %7, %0
  %5 = call i32 @getchar()
  %6 = icmp ne i32 %5, 10
  br i1 %6, label %7, label %8

7:                                                ; preds = %4
  br label %4, !llvm.loop !75

8:                                                ; preds = %4
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %2, ptr align 8 %1, i64 8, i1 false)
  %9 = load i64, ptr %2, align 8
  ret i64 %9
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #11

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @int_to_float(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca double, align 8
  %4 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %5 = load i64, ptr %2, align 8
  %6 = sitofp i64 %5 to double
  store double %6, ptr %3, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %4, ptr align 8 %3, i64 8, i1 false)
  %7 = load i64, ptr %4, align 8
  ret i64 %7
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @float_to_int(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca double, align 8
  store i64 %0, ptr %2, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %3, ptr align 8 %2, i64 8, i1 false)
  %4 = load double, ptr %3, align 8
  %5 = fptosi double %4 to i64
  ret i64 %5
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @allocate_garbage(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  store i64 0, ptr %3, align 8
  store i64 0, ptr %4, align 8
  store i64 0, ptr %5, align 8
  call void @ep_gc_push_root(ptr noundef %3)
  call void @ep_gc_push_root(ptr noundef %4)
  call void @ep_gc_push_root(ptr noundef %2)
  call void @ep_gc_maybe_collect()
  store i64 0, ptr %4, align 8
  br label %9

9:                                                ; preds = %27, %1
  %10 = load i64, ptr %4, align 8
  %11 = load i64, ptr %2, align 8
  %12 = icmp slt i64 %10, %11
  br i1 %12, label %13, label %33

13:                                               ; preds = %9
  %14 = call ptr @malloc(i64 noundef 16) #14
  store ptr %14, ptr %6, align 8
  %15 = load i64, ptr %4, align 8
  %16 = load ptr, ptr %6, align 8
  %17 = getelementptr inbounds %struct.EpStruct_Node, ptr %16, i32 0, i32 0
  store i64 %15, ptr %17, align 8
  %18 = load ptr, ptr %6, align 8
  %19 = getelementptr inbounds %struct.EpStruct_Node, ptr %18, i32 0, i32 1
  store i64 0, ptr %19, align 8
  %20 = load ptr, ptr %6, align 8
  %21 = call ptr @ep_gc_register(ptr noundef %20, i32 noundef 2)
  store ptr %21, ptr %7, align 8
  %22 = load ptr, ptr %7, align 8
  %23 = icmp ne ptr %22, null
  br i1 %23, label %24, label %27

24:                                               ; preds = %13
  %25 = load ptr, ptr %7, align 8
  %26 = getelementptr inbounds %struct.EpGCObject, ptr %25, i32 0, i32 4
  store i64 2, ptr %26, align 8
  br label %27

27:                                               ; preds = %24, %13
  %28 = load ptr, ptr %6, align 8
  %29 = ptrtoint ptr %28 to i64
  store i64 %29, ptr %8, align 8
  %30 = load i64, ptr %8, align 8
  store i64 %30, ptr %3, align 8
  %31 = load i64, ptr %4, align 8
  %32 = add nsw i64 %31, 1
  store i64 %32, ptr %4, align 8
  br label %9, !llvm.loop !76

33:                                               ; preds = %9
  store i64 0, ptr %5, align 8
  br label %34

34:                                               ; preds = %33
  call void @ep_gc_pop_roots(i64 noundef 3)
  %35 = load i64, ptr %3, align 8
  call void @free_struct_Node(i64 noundef %35)
  store i64 0, ptr %3, align 8
  %36 = load i64, ptr %5, align 8
  ret i64 %36
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_push_root(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %4 = load i32, ptr %3, align 4
  %5 = icmp slt i32 %4, 4096
  br i1 %5, label %6, label %48

6:                                                ; preds = %1
  %7 = load ptr, ptr %2, align 8
  %8 = call align 8 ptr @llvm.threadlocal.address.p0(ptr align 8 @ep_gc_root_stack)
  %9 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %10 = load i32, ptr %9, align 4
  %11 = sext i32 %10 to i64
  %12 = getelementptr inbounds [4096 x ptr], ptr %8, i64 0, i64 %11
  store ptr %7, ptr %12, align 8
  %13 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %14 = load i32, ptr %13, align 4
  %15 = add nsw i32 %14, 1
  store i32 %15, ptr %13, align 4
  %16 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_thread_slot)
  %17 = load i32, ptr %16, align 4
  %18 = icmp sge i32 %17, 0
  br i1 %18, label %19, label %47

19:                                               ; preds = %6
  %20 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_thread_slot)
  %21 = load i32, ptr %20, align 4
  %22 = sext i32 %21 to i64
  %23 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %22
  %24 = load ptr, ptr %23, align 8
  %25 = icmp ne ptr %24, null
  br i1 %25, label %26, label %47

26:                                               ; preds = %19
  %27 = load ptr, ptr %2, align 8
  %28 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_thread_slot)
  %29 = load i32, ptr %28, align 4
  %30 = sext i32 %29 to i64
  %31 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %30
  %32 = load ptr, ptr %31, align 8
  %33 = getelementptr inbounds %struct.EpThreadGCState, ptr %32, i32 0, i32 0
  %34 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %35 = load i32, ptr %34, align 4
  %36 = sub nsw i32 %35, 1
  %37 = sext i32 %36 to i64
  %38 = getelementptr inbounds [4096 x ptr], ptr %33, i64 0, i64 %37
  store ptr %27, ptr %38, align 8
  %39 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %40 = load i32, ptr %39, align 4
  %41 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_thread_slot)
  %42 = load i32, ptr %41, align 4
  %43 = sext i32 %42 to i64
  %44 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %43
  %45 = load ptr, ptr %44, align 8
  %46 = getelementptr inbounds %struct.EpThreadGCState, ptr %45, i32 0, i32 1
  store volatile i32 %40, ptr %46, align 8
  br label %47

47:                                               ; preds = %26, %19, %6
  br label %48

48:                                               ; preds = %47, %1
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_maybe_collect() #0 {
  %1 = alloca i32, align 4
  %2 = load i32, ptr @ep_gc_enabled, align 4
  %3 = icmp ne i32 %2, 0
  br i1 %3, label %5, label %4

4:                                                ; preds = %0
  br label %19

5:                                                ; preds = %0
  %6 = call align 8 ptr @llvm.threadlocal.address.p0(ptr align 8 @ep_thread_local_top)
  store volatile ptr %1, ptr %6, align 8
  %7 = call i32 @pthread_mutex_lock(ptr noundef @ep_gc_mutex)
  %8 = load i64, ptr @ep_gc_nursery_count, align 8
  %9 = load i64, ptr @ep_gc_nursery_threshold, align 8
  %10 = icmp sge i64 %8, %9
  br i1 %10, label %11, label %12

11:                                               ; preds = %5
  call void @ep_gc_collect_minor()
  br label %12

12:                                               ; preds = %11, %5
  %13 = load i64, ptr @ep_gc_count, align 8
  %14 = load i64, ptr @ep_gc_threshold, align 8
  %15 = icmp sge i64 %13, %14
  br i1 %15, label %16, label %17

16:                                               ; preds = %12
  call void @ep_gc_collect_major()
  br label %17

17:                                               ; preds = %16, %12
  %18 = call i32 @pthread_mutex_unlock(ptr noundef @ep_gc_mutex)
  br label %19

19:                                               ; preds = %17, %4
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_pop_roots(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  %3 = load i64, ptr %2, align 8
  %4 = trunc i64 %3 to i32
  %5 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %6 = load i32, ptr %5, align 4
  %7 = sub nsw i32 %6, %4
  store i32 %7, ptr %5, align 4
  %8 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %9 = load i32, ptr %8, align 4
  %10 = icmp slt i32 %9, 0
  br i1 %10, label %11, label %13

11:                                               ; preds = %1
  %12 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  store i32 0, ptr %12, align 4
  br label %13

13:                                               ; preds = %11, %1
  %14 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_thread_slot)
  %15 = load i32, ptr %14, align 4
  %16 = icmp sge i32 %15, 0
  br i1 %16, label %17, label %33

17:                                               ; preds = %13
  %18 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_thread_slot)
  %19 = load i32, ptr %18, align 4
  %20 = sext i32 %19 to i64
  %21 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %20
  %22 = load ptr, ptr %21, align 8
  %23 = icmp ne ptr %22, null
  br i1 %23, label %24, label %33

24:                                               ; preds = %17
  %25 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %26 = load i32, ptr %25, align 4
  %27 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_thread_slot)
  %28 = load i32, ptr %27, align 4
  %29 = sext i32 %28 to i64
  %30 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %29
  %31 = load ptr, ptr %30, align 8
  %32 = getelementptr inbounds %struct.EpThreadGCState, ptr %31, i32 0, i32 1
  store volatile i32 %26, ptr %32, align 8
  br label %33

33:                                               ; preds = %24, %17, %13
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @trigger_gc(i64 noundef %0) #0 {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  store i64 %0, ptr %2, align 8
  store i64 0, ptr %3, align 8
  call void @ep_gc_push_root(ptr noundef %2)
  call void @ep_gc_maybe_collect()
  %4 = load i64, ptr %2, align 8
  %5 = icmp sgt i64 %4, 0
  br i1 %5, label %6, label %10

6:                                                ; preds = %1
  %7 = load i64, ptr %2, align 8
  %8 = sub nsw i64 %7, 1
  %9 = call i64 @trigger_gc(i64 noundef %8)
  store i64 %9, ptr %3, align 8
  br label %11

10:                                               ; preds = %1
  store i64 0, ptr %3, align 8
  br label %11

11:                                               ; preds = %10, %6
  call void @ep_gc_pop_roots(i64 noundef 1)
  %12 = load i64, ptr %3, align 8
  ret i64 %12
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i64 @_main() #0 {
  %1 = alloca i64, align 8
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca ptr, align 8
  %10 = alloca ptr, align 8
  %11 = alloca i64, align 8
  %12 = alloca ptr, align 8
  %13 = alloca ptr, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca i64, align 8
  %17 = alloca i64, align 8
  %18 = alloca i64, align 8
  store i64 0, ptr %1, align 8
  store i64 0, ptr %2, align 8
  store i64 0, ptr %3, align 8
  store i64 0, ptr %4, align 8
  store i64 0, ptr %5, align 8
  store i64 0, ptr %6, align 8
  store i64 0, ptr %7, align 8
  store i64 0, ptr %8, align 8
  call void @ep_gc_push_root(ptr noundef %1)
  call void @ep_gc_push_root(ptr noundef %2)
  call void @ep_gc_push_root(ptr noundef %3)
  call void @ep_gc_push_root(ptr noundef %4)
  call void @ep_gc_push_root(ptr noundef %5)
  call void @ep_gc_push_root(ptr noundef %6)
  call void @ep_gc_push_root(ptr noundef %7)
  call void @ep_gc_maybe_collect()
  %19 = call i32 (ptr, ...) @printf(ptr noundef @.str.29, ptr noundef @.str.30)
  %20 = call i64 @ep_gc_get_minor_count()
  store i64 %20, ptr %5, align 8
  %21 = load i64, ptr %5, align 8
  %22 = call i64 @ep_auto_to_string(i64 noundef %21)
  %23 = call i64 @concat(i64 noundef ptrtoint (ptr @.str.31 to i64), i64 noundef %22)
  %24 = inttoptr i64 %23 to ptr
  %25 = call i32 (ptr, ...) @printf(ptr noundef @.str.29, ptr noundef %24)
  %26 = call i64 @allocate_garbage(i64 noundef 1000)
  store i64 %26, ptr %4, align 8
  %27 = call i64 @trigger_gc(i64 noundef 1)
  store i64 %27, ptr %4, align 8
  %28 = call i64 @ep_gc_get_minor_count()
  store i64 %28, ptr %3, align 8
  %29 = load i64, ptr %3, align 8
  %30 = call i64 @ep_auto_to_string(i64 noundef %29)
  %31 = call i64 @concat(i64 noundef ptrtoint (ptr @.str.32 to i64), i64 noundef %30)
  %32 = inttoptr i64 %31 to ptr
  %33 = call i32 (ptr, ...) @printf(ptr noundef @.str.29, ptr noundef %32)
  %34 = call ptr @malloc(i64 noundef 16) #14
  store ptr %34, ptr %9, align 8
  %35 = load ptr, ptr %9, align 8
  %36 = getelementptr inbounds %struct.EpStruct_Node, ptr %35, i32 0, i32 0
  store i64 42, ptr %36, align 8
  %37 = load ptr, ptr %9, align 8
  %38 = getelementptr inbounds %struct.EpStruct_Node, ptr %37, i32 0, i32 1
  store i64 0, ptr %38, align 8
  %39 = load ptr, ptr %9, align 8
  %40 = call ptr @ep_gc_register(ptr noundef %39, i32 noundef 2)
  store ptr %40, ptr %10, align 8
  %41 = load ptr, ptr %10, align 8
  %42 = icmp ne ptr %41, null
  br i1 %42, label %43, label %46

43:                                               ; preds = %0
  %44 = load ptr, ptr %10, align 8
  %45 = getelementptr inbounds %struct.EpGCObject, ptr %44, i32 0, i32 4
  store i64 2, ptr %45, align 8
  br label %46

46:                                               ; preds = %43, %0
  %47 = load ptr, ptr %9, align 8
  %48 = ptrtoint ptr %47 to i64
  store i64 %48, ptr %11, align 8
  %49 = load i64, ptr %11, align 8
  store i64 %49, ptr %6, align 8
  %50 = call i64 @allocate_garbage(i64 noundef 1000)
  store i64 %50, ptr %4, align 8
  %51 = call i64 @trigger_gc(i64 noundef 1)
  store i64 %51, ptr %4, align 8
  %52 = call ptr @malloc(i64 noundef 16) #14
  store ptr %52, ptr %12, align 8
  %53 = load ptr, ptr %12, align 8
  %54 = getelementptr inbounds %struct.EpStruct_Node, ptr %53, i32 0, i32 0
  store i64 99, ptr %54, align 8
  %55 = load ptr, ptr %12, align 8
  %56 = getelementptr inbounds %struct.EpStruct_Node, ptr %55, i32 0, i32 1
  store i64 0, ptr %56, align 8
  %57 = load ptr, ptr %12, align 8
  %58 = call ptr @ep_gc_register(ptr noundef %57, i32 noundef 2)
  store ptr %58, ptr %13, align 8
  %59 = load ptr, ptr %13, align 8
  %60 = icmp ne ptr %59, null
  br i1 %60, label %61, label %64

61:                                               ; preds = %46
  %62 = load ptr, ptr %13, align 8
  %63 = getelementptr inbounds %struct.EpGCObject, ptr %62, i32 0, i32 4
  store i64 2, ptr %63, align 8
  br label %64

64:                                               ; preds = %61, %46
  %65 = load ptr, ptr %12, align 8
  %66 = ptrtoint ptr %65 to i64
  store i64 %66, ptr %14, align 8
  %67 = load i64, ptr %14, align 8
  store i64 %67, ptr %7, align 8
  %68 = load i64, ptr %7, align 8
  %69 = load i64, ptr %6, align 8
  %70 = inttoptr i64 %69 to ptr
  %71 = getelementptr inbounds %struct.EpStruct_Node, ptr %70, i32 0, i32 1
  store i64 %68, ptr %71, align 8
  %72 = load i64, ptr %6, align 8
  %73 = inttoptr i64 %72 to ptr
  %74 = load i64, ptr %7, align 8
  call void @ep_gc_write_barrier(ptr noundef %73, i64 noundef %74)
  %75 = call i64 @allocate_garbage(i64 noundef 1000)
  store i64 %75, ptr %4, align 8
  %76 = call i64 @trigger_gc(i64 noundef 1)
  store i64 %76, ptr %4, align 8
  %77 = load i64, ptr %6, align 8
  store i64 %77, ptr %15, align 8
  %78 = load i64, ptr %15, align 8
  %79 = icmp eq i64 %78, 0
  br i1 %79, label %80, label %83

80:                                               ; preds = %64
  %81 = load ptr, ptr @__stderrp, align 8
  %82 = call i32 (ptr, ptr, ...) @fprintf(ptr noundef %81, ptr noundef @.str.33) #13
  call void @exit(i32 noundef 1) #12
  unreachable

83:                                               ; preds = %64
  %84 = load i64, ptr %15, align 8
  %85 = inttoptr i64 %84 to ptr
  %86 = getelementptr inbounds %struct.EpStruct_Node, ptr %85, i32 0, i32 1
  %87 = load i64, ptr %86, align 8
  store i64 %87, ptr %16, align 8
  %88 = load i64, ptr %16, align 8
  store i64 %88, ptr %1, align 8
  %89 = load i64, ptr %1, align 8
  store i64 %89, ptr %17, align 8
  %90 = load i64, ptr %17, align 8
  %91 = icmp eq i64 %90, 0
  br i1 %91, label %92, label %95

92:                                               ; preds = %83
  %93 = load ptr, ptr @__stderrp, align 8
  %94 = call i32 (ptr, ptr, ...) @fprintf(ptr noundef %93, ptr noundef @.str.34) #13
  call void @exit(i32 noundef 1) #12
  unreachable

95:                                               ; preds = %83
  %96 = load i64, ptr %17, align 8
  %97 = inttoptr i64 %96 to ptr
  %98 = getelementptr inbounds %struct.EpStruct_Node, ptr %97, i32 0, i32 0
  %99 = load i64, ptr %98, align 8
  store i64 %99, ptr %18, align 8
  %100 = load i64, ptr %18, align 8
  store i64 %100, ptr %2, align 8
  %101 = load i64, ptr %2, align 8
  %102 = call i64 @ep_auto_to_string(i64 noundef %101)
  %103 = call i64 @concat(i64 noundef ptrtoint (ptr @.str.35 to i64), i64 noundef %102)
  %104 = inttoptr i64 %103 to ptr
  %105 = call i32 (ptr, ...) @printf(ptr noundef @.str.29, ptr noundef %104)
  %106 = call i32 (ptr, ...) @printf(ptr noundef @.str.29, ptr noundef @.str.36)
  store i64 0, ptr %8, align 8
  br label %107

107:                                              ; preds = %95
  call void @ep_gc_pop_roots(i64 noundef 7)
  %108 = load i64, ptr %6, align 8
  call void @free_struct_Node(i64 noundef %108)
  store i64 0, ptr %6, align 8
  %109 = load i64, ptr %7, align 8
  call void @free_struct_Node(i64 noundef %109)
  store i64 0, ptr %7, align 8
  %110 = load i64, ptr %8, align 8
  ret i64 %110
}

declare i32 @printf(ptr noundef, ...) #1

; Function Attrs: nounwind
declare i32 @fprintf(ptr noundef, ptr noundef, ...) #4

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @main(i32 noundef %0, ptr noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca ptr, align 8
  %6 = alloca i32, align 4
  %7 = alloca ptr, align 8
  %8 = alloca i32, align 4
  store i32 0, ptr %3, align 4
  store i32 %0, ptr %4, align 4
  store ptr %1, ptr %5, align 8
  %9 = call ptr @"\01_fopen"(ptr noundef @.str.37, ptr noundef @.str.3)
  store ptr %9, ptr %7, align 8
  %10 = load ptr, ptr %7, align 8
  %11 = icmp ne ptr %10, null
  br i1 %11, label %12, label %19

12:                                               ; preds = %2
  %13 = load ptr, ptr %7, align 8
  %14 = call i64 @fread(ptr noundef %6, i64 noundef 4, i64 noundef 1, ptr noundef %13)
  %15 = icmp eq i64 %14, 1
  br i1 %15, label %16, label %19

16:                                               ; preds = %12
  %17 = load ptr, ptr %7, align 8
  %18 = call i32 @fclose(ptr noundef %17)
  br label %30

19:                                               ; preds = %12, %2
  %20 = load ptr, ptr %7, align 8
  %21 = icmp ne ptr %20, null
  br i1 %21, label %22, label %25

22:                                               ; preds = %19
  %23 = load ptr, ptr %7, align 8
  %24 = call i32 @fclose(ptr noundef %23)
  br label %25

25:                                               ; preds = %22, %19
  %26 = call i64 @time(ptr noundef null)
  %27 = trunc i64 %26 to i32
  %28 = call i32 @getpid()
  %29 = xor i32 %27, %28
  store i32 %29, ptr %6, align 4
  br label %30

30:                                               ; preds = %25, %16
  %31 = load i32, ptr %6, align 4
  call void @srand(i32 noundef %31)
  %32 = load i32, ptr %4, align 4
  %33 = load ptr, ptr %5, align 8
  call void @init_ep_args(i32 noundef %32, ptr noundef %33)
  %34 = call i64 @_main()
  %35 = trunc i64 %34 to i32
  store i32 %35, ptr %8, align 4
  call void @ep_gc_shutdown()
  %36 = load i32, ptr %8, align 4
  ret i32 %36
}

declare void @srand(i32 noundef) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_shutdown() #0 {
  %1 = alloca ptr, align 8
  %2 = alloca ptr, align 8
  store i32 0, ptr @ep_gc_enabled, align 4
  %3 = load ptr, ptr @ep_gc_head, align 8
  store ptr %3, ptr %1, align 8
  br label %4

4:                                                ; preds = %7, %0
  %5 = load ptr, ptr %1, align 8
  %6 = icmp ne ptr %5, null
  br i1 %6, label %7, label %13

7:                                                ; preds = %4
  %8 = load ptr, ptr %1, align 8
  %9 = getelementptr inbounds %struct.EpGCObject, ptr %8, i32 0, i32 6
  %10 = load ptr, ptr %9, align 8
  store ptr %10, ptr %2, align 8
  %11 = load ptr, ptr %1, align 8
  call void @free(ptr noundef %11)
  %12 = load ptr, ptr %2, align 8
  store ptr %12, ptr %1, align 8
  br label %4, !llvm.loop !77

13:                                               ; preds = %4
  store ptr null, ptr @ep_gc_head, align 8
  store i64 0, ptr @ep_gc_count, align 8
  %14 = load ptr, ptr @ep_gc_table, align 8
  %15 = icmp ne ptr %14, null
  br i1 %15, label %16, label %18

16:                                               ; preds = %13
  %17 = load ptr, ptr @ep_gc_table, align 8
  call void @free(ptr noundef %17)
  store ptr null, ptr @ep_gc_table, align 8
  br label %18

18:                                               ; preds = %16, %13
  store i64 0, ptr @ep_gc_table_cap, align 8
  store i64 0, ptr @ep_gc_table_size, align 8
  ret void
}

; Function Attrs: noreturn
declare void @longjmp(ptr noundef, i32 noundef) #9

; Function Attrs: noreturn
declare void @_exit(i32 noundef) #9

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal ptr @ep_gc_table_get(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca ptr, align 8
  %4 = alloca i64, align 8
  store ptr %0, ptr %3, align 8
  %5 = load i64, ptr @ep_gc_table_cap, align 8
  %6 = icmp eq i64 %5, 0
  br i1 %6, label %7, label %8

7:                                                ; preds = %1
  store ptr null, ptr %2, align 8
  br label %40

8:                                                ; preds = %1
  %9 = load ptr, ptr %3, align 8
  %10 = ptrtoint ptr %9 to i64
  %11 = load i64, ptr @ep_gc_table_cap, align 8
  %12 = urem i64 %10, %11
  store i64 %12, ptr %4, align 8
  br label %13

13:                                               ; preds = %34, %8
  %14 = load ptr, ptr @ep_gc_table, align 8
  %15 = load i64, ptr %4, align 8
  %16 = getelementptr inbounds %struct.EpGCEntry, ptr %14, i64 %15
  %17 = getelementptr inbounds %struct.EpGCEntry, ptr %16, i32 0, i32 0
  %18 = load ptr, ptr %17, align 8
  %19 = icmp ne ptr %18, null
  br i1 %19, label %20, label %39

20:                                               ; preds = %13
  %21 = load ptr, ptr @ep_gc_table, align 8
  %22 = load i64, ptr %4, align 8
  %23 = getelementptr inbounds %struct.EpGCEntry, ptr %21, i64 %22
  %24 = getelementptr inbounds %struct.EpGCEntry, ptr %23, i32 0, i32 0
  %25 = load ptr, ptr %24, align 8
  %26 = load ptr, ptr %3, align 8
  %27 = icmp eq ptr %25, %26
  br i1 %27, label %28, label %34

28:                                               ; preds = %20
  %29 = load ptr, ptr @ep_gc_table, align 8
  %30 = load i64, ptr %4, align 8
  %31 = getelementptr inbounds %struct.EpGCEntry, ptr %29, i64 %30
  %32 = getelementptr inbounds %struct.EpGCEntry, ptr %31, i32 0, i32 1
  %33 = load ptr, ptr %32, align 8
  store ptr %33, ptr %2, align 8
  br label %40

34:                                               ; preds = %20
  %35 = load i64, ptr %4, align 8
  %36 = add nsw i64 %35, 1
  %37 = load i64, ptr @ep_gc_table_cap, align 8
  %38 = srem i64 %36, %37
  store i64 %38, ptr %4, align 8
  br label %13, !llvm.loop !78

39:                                               ; preds = %13
  store ptr null, ptr %2, align 8
  br label %40

40:                                               ; preds = %39, %28, %7
  %41 = load ptr, ptr %2, align 8
  ret ptr %41
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_table_remove(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %7 = load i64, ptr @ep_gc_table_cap, align 8
  %8 = icmp eq i64 %7, 0
  br i1 %8, label %9, label %10

9:                                                ; preds = %1
  br label %85

10:                                               ; preds = %1
  %11 = load ptr, ptr %2, align 8
  %12 = ptrtoint ptr %11 to i64
  %13 = load i64, ptr @ep_gc_table_cap, align 8
  %14 = urem i64 %12, %13
  store i64 %14, ptr %3, align 8
  br label %15

15:                                               ; preds = %80, %10
  %16 = load ptr, ptr @ep_gc_table, align 8
  %17 = load i64, ptr %3, align 8
  %18 = getelementptr inbounds %struct.EpGCEntry, ptr %16, i64 %17
  %19 = getelementptr inbounds %struct.EpGCEntry, ptr %18, i32 0, i32 0
  %20 = load ptr, ptr %19, align 8
  %21 = icmp ne ptr %20, null
  br i1 %21, label %22, label %85

22:                                               ; preds = %15
  %23 = load ptr, ptr @ep_gc_table, align 8
  %24 = load i64, ptr %3, align 8
  %25 = getelementptr inbounds %struct.EpGCEntry, ptr %23, i64 %24
  %26 = getelementptr inbounds %struct.EpGCEntry, ptr %25, i32 0, i32 0
  %27 = load ptr, ptr %26, align 8
  %28 = load ptr, ptr %2, align 8
  %29 = icmp eq ptr %27, %28
  br i1 %29, label %30, label %80

30:                                               ; preds = %22
  %31 = load ptr, ptr @ep_gc_table, align 8
  %32 = load i64, ptr %3, align 8
  %33 = getelementptr inbounds %struct.EpGCEntry, ptr %31, i64 %32
  %34 = getelementptr inbounds %struct.EpGCEntry, ptr %33, i32 0, i32 0
  store ptr null, ptr %34, align 8
  %35 = load ptr, ptr @ep_gc_table, align 8
  %36 = load i64, ptr %3, align 8
  %37 = getelementptr inbounds %struct.EpGCEntry, ptr %35, i64 %36
  %38 = getelementptr inbounds %struct.EpGCEntry, ptr %37, i32 0, i32 1
  store ptr null, ptr %38, align 8
  %39 = load i64, ptr @ep_gc_table_size, align 8
  %40 = add nsw i64 %39, -1
  store i64 %40, ptr @ep_gc_table_size, align 8
  %41 = load i64, ptr %3, align 8
  %42 = add nsw i64 %41, 1
  %43 = load i64, ptr @ep_gc_table_cap, align 8
  %44 = srem i64 %42, %43
  store i64 %44, ptr %4, align 8
  br label %45

45:                                               ; preds = %52, %30
  %46 = load ptr, ptr @ep_gc_table, align 8
  %47 = load i64, ptr %4, align 8
  %48 = getelementptr inbounds %struct.EpGCEntry, ptr %46, i64 %47
  %49 = getelementptr inbounds %struct.EpGCEntry, ptr %48, i32 0, i32 0
  %50 = load ptr, ptr %49, align 8
  %51 = icmp ne ptr %50, null
  br i1 %51, label %52, label %79

52:                                               ; preds = %45
  %53 = load ptr, ptr @ep_gc_table, align 8
  %54 = load i64, ptr %4, align 8
  %55 = getelementptr inbounds %struct.EpGCEntry, ptr %53, i64 %54
  %56 = getelementptr inbounds %struct.EpGCEntry, ptr %55, i32 0, i32 0
  %57 = load ptr, ptr %56, align 8
  store ptr %57, ptr %5, align 8
  %58 = load ptr, ptr @ep_gc_table, align 8
  %59 = load i64, ptr %4, align 8
  %60 = getelementptr inbounds %struct.EpGCEntry, ptr %58, i64 %59
  %61 = getelementptr inbounds %struct.EpGCEntry, ptr %60, i32 0, i32 1
  %62 = load ptr, ptr %61, align 8
  store ptr %62, ptr %6, align 8
  %63 = load ptr, ptr @ep_gc_table, align 8
  %64 = load i64, ptr %4, align 8
  %65 = getelementptr inbounds %struct.EpGCEntry, ptr %63, i64 %64
  %66 = getelementptr inbounds %struct.EpGCEntry, ptr %65, i32 0, i32 0
  store ptr null, ptr %66, align 8
  %67 = load ptr, ptr @ep_gc_table, align 8
  %68 = load i64, ptr %4, align 8
  %69 = getelementptr inbounds %struct.EpGCEntry, ptr %67, i64 %68
  %70 = getelementptr inbounds %struct.EpGCEntry, ptr %69, i32 0, i32 1
  store ptr null, ptr %70, align 8
  %71 = load i64, ptr @ep_gc_table_size, align 8
  %72 = add nsw i64 %71, -1
  store i64 %72, ptr @ep_gc_table_size, align 8
  %73 = load ptr, ptr %5, align 8
  %74 = load ptr, ptr %6, align 8
  call void @ep_gc_table_insert(ptr noundef %73, ptr noundef %74)
  %75 = load i64, ptr %4, align 8
  %76 = add nsw i64 %75, 1
  %77 = load i64, ptr @ep_gc_table_cap, align 8
  %78 = srem i64 %76, %77
  store i64 %78, ptr %4, align 8
  br label %45, !llvm.loop !79

79:                                               ; preds = %45
  br label %85

80:                                               ; preds = %22
  %81 = load i64, ptr %3, align 8
  %82 = add nsw i64 %81, 1
  %83 = load i64, ptr @ep_gc_table_cap, align 8
  %84 = srem i64 %82, %83
  store i64 %84, ptr %3, align 8
  br label %15, !llvm.loop !80

85:                                               ; preds = %9, %79, %15
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_table_insert(ptr noundef %0, ptr noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  store ptr %0, ptr %3, align 8
  store ptr %1, ptr %4, align 8
  %11 = load i64, ptr @ep_gc_table_size, align 8
  %12 = mul nsw i64 %11, 2
  %13 = load i64, ptr @ep_gc_table_cap, align 8
  %14 = icmp sge i64 %12, %13
  br i1 %14, label %15, label %74

15:                                               ; preds = %2
  %16 = load i64, ptr @ep_gc_table_cap, align 8
  store i64 %16, ptr %5, align 8
  %17 = load i64, ptr %5, align 8
  %18 = icmp eq i64 %17, 0
  br i1 %18, label %19, label %20

19:                                               ; preds = %15
  br label %23

20:                                               ; preds = %15
  %21 = load i64, ptr %5, align 8
  %22 = mul nsw i64 %21, 2
  br label %23

23:                                               ; preds = %20, %19
  %24 = phi i64 [ 512, %19 ], [ %22, %20 ]
  store i64 %24, ptr %6, align 8
  %25 = load i64, ptr %6, align 8
  %26 = call ptr @calloc(i64 noundef %25, i64 noundef 16) #15
  store ptr %26, ptr %7, align 8
  store i64 0, ptr %8, align 8
  br label %27

27:                                               ; preds = %67, %23
  %28 = load i64, ptr %8, align 8
  %29 = load i64, ptr %5, align 8
  %30 = icmp slt i64 %28, %29
  br i1 %30, label %31, label %70

31:                                               ; preds = %27
  %32 = load ptr, ptr @ep_gc_table, align 8
  %33 = load i64, ptr %8, align 8
  %34 = getelementptr inbounds %struct.EpGCEntry, ptr %32, i64 %33
  %35 = getelementptr inbounds %struct.EpGCEntry, ptr %34, i32 0, i32 0
  %36 = load ptr, ptr %35, align 8
  %37 = icmp ne ptr %36, null
  br i1 %37, label %38, label %66

38:                                               ; preds = %31
  %39 = load ptr, ptr @ep_gc_table, align 8
  %40 = load i64, ptr %8, align 8
  %41 = getelementptr inbounds %struct.EpGCEntry, ptr %39, i64 %40
  %42 = getelementptr inbounds %struct.EpGCEntry, ptr %41, i32 0, i32 0
  %43 = load ptr, ptr %42, align 8
  %44 = ptrtoint ptr %43 to i64
  %45 = load i64, ptr %6, align 8
  %46 = urem i64 %44, %45
  store i64 %46, ptr %9, align 8
  br label %47

47:                                               ; preds = %54, %38
  %48 = load ptr, ptr %7, align 8
  %49 = load i64, ptr %9, align 8
  %50 = getelementptr inbounds %struct.EpGCEntry, ptr %48, i64 %49
  %51 = getelementptr inbounds %struct.EpGCEntry, ptr %50, i32 0, i32 0
  %52 = load ptr, ptr %51, align 8
  %53 = icmp ne ptr %52, null
  br i1 %53, label %54, label %59

54:                                               ; preds = %47
  %55 = load i64, ptr %9, align 8
  %56 = add nsw i64 %55, 1
  %57 = load i64, ptr %6, align 8
  %58 = srem i64 %56, %57
  store i64 %58, ptr %9, align 8
  br label %47, !llvm.loop !81

59:                                               ; preds = %47
  %60 = load ptr, ptr %7, align 8
  %61 = load i64, ptr %9, align 8
  %62 = getelementptr inbounds %struct.EpGCEntry, ptr %60, i64 %61
  %63 = load ptr, ptr @ep_gc_table, align 8
  %64 = load i64, ptr %8, align 8
  %65 = getelementptr inbounds %struct.EpGCEntry, ptr %63, i64 %64
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %62, ptr align 8 %65, i64 16, i1 false)
  br label %66

66:                                               ; preds = %59, %31
  br label %67

67:                                               ; preds = %66
  %68 = load i64, ptr %8, align 8
  %69 = add nsw i64 %68, 1
  store i64 %69, ptr %8, align 8
  br label %27, !llvm.loop !82

70:                                               ; preds = %27
  %71 = load ptr, ptr @ep_gc_table, align 8
  call void @free(ptr noundef %71)
  %72 = load ptr, ptr %7, align 8
  store ptr %72, ptr @ep_gc_table, align 8
  %73 = load i64, ptr %6, align 8
  store i64 %73, ptr @ep_gc_table_cap, align 8
  br label %74

74:                                               ; preds = %70, %2
  %75 = load ptr, ptr %3, align 8
  %76 = ptrtoint ptr %75 to i64
  %77 = load i64, ptr @ep_gc_table_cap, align 8
  %78 = urem i64 %76, %77
  store i64 %78, ptr %10, align 8
  br label %79

79:                                               ; preds = %100, %74
  %80 = load ptr, ptr @ep_gc_table, align 8
  %81 = load i64, ptr %10, align 8
  %82 = getelementptr inbounds %struct.EpGCEntry, ptr %80, i64 %81
  %83 = getelementptr inbounds %struct.EpGCEntry, ptr %82, i32 0, i32 0
  %84 = load ptr, ptr %83, align 8
  %85 = icmp ne ptr %84, null
  br i1 %85, label %86, label %105

86:                                               ; preds = %79
  %87 = load ptr, ptr @ep_gc_table, align 8
  %88 = load i64, ptr %10, align 8
  %89 = getelementptr inbounds %struct.EpGCEntry, ptr %87, i64 %88
  %90 = getelementptr inbounds %struct.EpGCEntry, ptr %89, i32 0, i32 0
  %91 = load ptr, ptr %90, align 8
  %92 = load ptr, ptr %3, align 8
  %93 = icmp eq ptr %91, %92
  br i1 %93, label %94, label %100

94:                                               ; preds = %86
  %95 = load ptr, ptr %4, align 8
  %96 = load ptr, ptr @ep_gc_table, align 8
  %97 = load i64, ptr %10, align 8
  %98 = getelementptr inbounds %struct.EpGCEntry, ptr %96, i64 %97
  %99 = getelementptr inbounds %struct.EpGCEntry, ptr %98, i32 0, i32 1
  store ptr %95, ptr %99, align 8
  br label %118

100:                                              ; preds = %86
  %101 = load i64, ptr %10, align 8
  %102 = add nsw i64 %101, 1
  %103 = load i64, ptr @ep_gc_table_cap, align 8
  %104 = srem i64 %102, %103
  store i64 %104, ptr %10, align 8
  br label %79, !llvm.loop !83

105:                                              ; preds = %79
  %106 = load ptr, ptr %3, align 8
  %107 = load ptr, ptr @ep_gc_table, align 8
  %108 = load i64, ptr %10, align 8
  %109 = getelementptr inbounds %struct.EpGCEntry, ptr %107, i64 %108
  %110 = getelementptr inbounds %struct.EpGCEntry, ptr %109, i32 0, i32 0
  store ptr %106, ptr %110, align 8
  %111 = load ptr, ptr %4, align 8
  %112 = load ptr, ptr @ep_gc_table, align 8
  %113 = load i64, ptr %10, align 8
  %114 = getelementptr inbounds %struct.EpGCEntry, ptr %112, i64 %113
  %115 = getelementptr inbounds %struct.EpGCEntry, ptr %114, i32 0, i32 1
  store ptr %111, ptr %115, align 8
  %116 = load i64, ptr @ep_gc_table_size, align 8
  %117 = add nsw i64 %116, 1
  store i64 %117, ptr @ep_gc_table_size, align 8
  br label %118

118:                                              ; preds = %105, %94
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare nonnull ptr @llvm.threadlocal.address.p0(ptr nonnull) #8

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal ptr @json_skip_ws(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  br label %3

3:                                                ; preds = %25, %1
  %4 = load ptr, ptr %2, align 8
  %5 = load i8, ptr %4, align 1
  %6 = sext i8 %5 to i32
  %7 = icmp eq i32 %6, 32
  br i1 %7, label %23, label %8

8:                                                ; preds = %3
  %9 = load ptr, ptr %2, align 8
  %10 = load i8, ptr %9, align 1
  %11 = sext i8 %10 to i32
  %12 = icmp eq i32 %11, 9
  br i1 %12, label %23, label %13

13:                                               ; preds = %8
  %14 = load ptr, ptr %2, align 8
  %15 = load i8, ptr %14, align 1
  %16 = sext i8 %15 to i32
  %17 = icmp eq i32 %16, 10
  br i1 %17, label %23, label %18

18:                                               ; preds = %13
  %19 = load ptr, ptr %2, align 8
  %20 = load i8, ptr %19, align 1
  %21 = sext i8 %20 to i32
  %22 = icmp eq i32 %21, 13
  br label %23

23:                                               ; preds = %18, %13, %8, %3
  %24 = phi i1 [ true, %13 ], [ true, %8 ], [ true, %3 ], [ %22, %18 ]
  br i1 %24, label %25, label %28

25:                                               ; preds = %23
  %26 = load ptr, ptr %2, align 8
  %27 = getelementptr inbounds i8, ptr %26, i32 1
  store ptr %27, ptr %2, align 8
  br label %3, !llvm.loop !84

28:                                               ; preds = %23
  %29 = load ptr, ptr %2, align 8
  ret ptr %29
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal ptr @json_skip_value(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store ptr %0, ptr %2, align 8
  %5 = load ptr, ptr %2, align 8
  %6 = call ptr @json_skip_ws(ptr noundef %5)
  store ptr %6, ptr %2, align 8
  %7 = load ptr, ptr %2, align 8
  %8 = load i8, ptr %7, align 1
  %9 = sext i8 %8 to i32
  %10 = icmp eq i32 %9, 34
  br i1 %10, label %11, label %46

11:                                               ; preds = %1
  %12 = load ptr, ptr %2, align 8
  %13 = getelementptr inbounds i8, ptr %12, i32 1
  store ptr %13, ptr %2, align 8
  br label %14

14:                                               ; preds = %34, %11
  %15 = load ptr, ptr %2, align 8
  %16 = load i8, ptr %15, align 1
  %17 = sext i8 %16 to i32
  %18 = icmp ne i32 %17, 0
  br i1 %18, label %19, label %24

19:                                               ; preds = %14
  %20 = load ptr, ptr %2, align 8
  %21 = load i8, ptr %20, align 1
  %22 = sext i8 %21 to i32
  %23 = icmp ne i32 %22, 34
  br label %24

24:                                               ; preds = %19, %14
  %25 = phi i1 [ false, %14 ], [ %23, %19 ]
  br i1 %25, label %26, label %37

26:                                               ; preds = %24
  %27 = load ptr, ptr %2, align 8
  %28 = load i8, ptr %27, align 1
  %29 = sext i8 %28 to i32
  %30 = icmp eq i32 %29, 92
  br i1 %30, label %31, label %34

31:                                               ; preds = %26
  %32 = load ptr, ptr %2, align 8
  %33 = getelementptr inbounds i8, ptr %32, i32 1
  store ptr %33, ptr %2, align 8
  br label %34

34:                                               ; preds = %31, %26
  %35 = load ptr, ptr %2, align 8
  %36 = getelementptr inbounds i8, ptr %35, i32 1
  store ptr %36, ptr %2, align 8
  br label %14, !llvm.loop !85

37:                                               ; preds = %24
  %38 = load ptr, ptr %2, align 8
  %39 = load i8, ptr %38, align 1
  %40 = sext i8 %39 to i32
  %41 = icmp eq i32 %40, 34
  br i1 %41, label %42, label %45

42:                                               ; preds = %37
  %43 = load ptr, ptr %2, align 8
  %44 = getelementptr inbounds i8, ptr %43, i32 1
  store ptr %44, ptr %2, align 8
  br label %45

45:                                               ; preds = %42, %37
  br label %253

46:                                               ; preds = %1
  %47 = load ptr, ptr %2, align 8
  %48 = load i8, ptr %47, align 1
  %49 = sext i8 %48 to i32
  %50 = icmp eq i32 %49, 123
  br i1 %50, label %51, label %130

51:                                               ; preds = %46
  store i32 1, ptr %3, align 4
  %52 = load ptr, ptr %2, align 8
  %53 = getelementptr inbounds i8, ptr %52, i32 1
  store ptr %53, ptr %2, align 8
  br label %54

54:                                               ; preds = %128, %51
  %55 = load ptr, ptr %2, align 8
  %56 = load i8, ptr %55, align 1
  %57 = sext i8 %56 to i32
  %58 = icmp ne i32 %57, 0
  br i1 %58, label %59, label %62

59:                                               ; preds = %54
  %60 = load i32, ptr %3, align 4
  %61 = icmp sgt i32 %60, 0
  br label %62

62:                                               ; preds = %59, %54
  %63 = phi i1 [ false, %54 ], [ %61, %59 ]
  br i1 %63, label %64, label %129

64:                                               ; preds = %62
  %65 = load ptr, ptr %2, align 8
  %66 = load i8, ptr %65, align 1
  %67 = sext i8 %66 to i32
  %68 = icmp eq i32 %67, 34
  br i1 %68, label %69, label %103

69:                                               ; preds = %64
  %70 = load ptr, ptr %2, align 8
  %71 = getelementptr inbounds i8, ptr %70, i32 1
  store ptr %71, ptr %2, align 8
  br label %72

72:                                               ; preds = %92, %69
  %73 = load ptr, ptr %2, align 8
  %74 = load i8, ptr %73, align 1
  %75 = sext i8 %74 to i32
  %76 = icmp ne i32 %75, 0
  br i1 %76, label %77, label %82

77:                                               ; preds = %72
  %78 = load ptr, ptr %2, align 8
  %79 = load i8, ptr %78, align 1
  %80 = sext i8 %79 to i32
  %81 = icmp ne i32 %80, 34
  br label %82

82:                                               ; preds = %77, %72
  %83 = phi i1 [ false, %72 ], [ %81, %77 ]
  br i1 %83, label %84, label %95

84:                                               ; preds = %82
  %85 = load ptr, ptr %2, align 8
  %86 = load i8, ptr %85, align 1
  %87 = sext i8 %86 to i32
  %88 = icmp eq i32 %87, 92
  br i1 %88, label %89, label %92

89:                                               ; preds = %84
  %90 = load ptr, ptr %2, align 8
  %91 = getelementptr inbounds i8, ptr %90, i32 1
  store ptr %91, ptr %2, align 8
  br label %92

92:                                               ; preds = %89, %84
  %93 = load ptr, ptr %2, align 8
  %94 = getelementptr inbounds i8, ptr %93, i32 1
  store ptr %94, ptr %2, align 8
  br label %72, !llvm.loop !86

95:                                               ; preds = %82
  %96 = load ptr, ptr %2, align 8
  %97 = load i8, ptr %96, align 1
  %98 = icmp ne i8 %97, 0
  br i1 %98, label %99, label %102

99:                                               ; preds = %95
  %100 = load ptr, ptr %2, align 8
  %101 = getelementptr inbounds i8, ptr %100, i32 1
  store ptr %101, ptr %2, align 8
  br label %102

102:                                              ; preds = %99, %95
  br label %128

103:                                              ; preds = %64
  %104 = load ptr, ptr %2, align 8
  %105 = load i8, ptr %104, align 1
  %106 = sext i8 %105 to i32
  %107 = icmp eq i32 %106, 123
  br i1 %107, label %108, label %113

108:                                              ; preds = %103
  %109 = load i32, ptr %3, align 4
  %110 = add nsw i32 %109, 1
  store i32 %110, ptr %3, align 4
  %111 = load ptr, ptr %2, align 8
  %112 = getelementptr inbounds i8, ptr %111, i32 1
  store ptr %112, ptr %2, align 8
  br label %127

113:                                              ; preds = %103
  %114 = load ptr, ptr %2, align 8
  %115 = load i8, ptr %114, align 1
  %116 = sext i8 %115 to i32
  %117 = icmp eq i32 %116, 125
  br i1 %117, label %118, label %123

118:                                              ; preds = %113
  %119 = load i32, ptr %3, align 4
  %120 = add nsw i32 %119, -1
  store i32 %120, ptr %3, align 4
  %121 = load ptr, ptr %2, align 8
  %122 = getelementptr inbounds i8, ptr %121, i32 1
  store ptr %122, ptr %2, align 8
  br label %126

123:                                              ; preds = %113
  %124 = load ptr, ptr %2, align 8
  %125 = getelementptr inbounds i8, ptr %124, i32 1
  store ptr %125, ptr %2, align 8
  br label %126

126:                                              ; preds = %123, %118
  br label %127

127:                                              ; preds = %126, %108
  br label %128

128:                                              ; preds = %127, %102
  br label %54, !llvm.loop !87

129:                                              ; preds = %62
  br label %252

130:                                              ; preds = %46
  %131 = load ptr, ptr %2, align 8
  %132 = load i8, ptr %131, align 1
  %133 = sext i8 %132 to i32
  %134 = icmp eq i32 %133, 91
  br i1 %134, label %135, label %214

135:                                              ; preds = %130
  store i32 1, ptr %4, align 4
  %136 = load ptr, ptr %2, align 8
  %137 = getelementptr inbounds i8, ptr %136, i32 1
  store ptr %137, ptr %2, align 8
  br label %138

138:                                              ; preds = %212, %135
  %139 = load ptr, ptr %2, align 8
  %140 = load i8, ptr %139, align 1
  %141 = sext i8 %140 to i32
  %142 = icmp ne i32 %141, 0
  br i1 %142, label %143, label %146

143:                                              ; preds = %138
  %144 = load i32, ptr %4, align 4
  %145 = icmp sgt i32 %144, 0
  br label %146

146:                                              ; preds = %143, %138
  %147 = phi i1 [ false, %138 ], [ %145, %143 ]
  br i1 %147, label %148, label %213

148:                                              ; preds = %146
  %149 = load ptr, ptr %2, align 8
  %150 = load i8, ptr %149, align 1
  %151 = sext i8 %150 to i32
  %152 = icmp eq i32 %151, 34
  br i1 %152, label %153, label %187

153:                                              ; preds = %148
  %154 = load ptr, ptr %2, align 8
  %155 = getelementptr inbounds i8, ptr %154, i32 1
  store ptr %155, ptr %2, align 8
  br label %156

156:                                              ; preds = %176, %153
  %157 = load ptr, ptr %2, align 8
  %158 = load i8, ptr %157, align 1
  %159 = sext i8 %158 to i32
  %160 = icmp ne i32 %159, 0
  br i1 %160, label %161, label %166

161:                                              ; preds = %156
  %162 = load ptr, ptr %2, align 8
  %163 = load i8, ptr %162, align 1
  %164 = sext i8 %163 to i32
  %165 = icmp ne i32 %164, 34
  br label %166

166:                                              ; preds = %161, %156
  %167 = phi i1 [ false, %156 ], [ %165, %161 ]
  br i1 %167, label %168, label %179

168:                                              ; preds = %166
  %169 = load ptr, ptr %2, align 8
  %170 = load i8, ptr %169, align 1
  %171 = sext i8 %170 to i32
  %172 = icmp eq i32 %171, 92
  br i1 %172, label %173, label %176

173:                                              ; preds = %168
  %174 = load ptr, ptr %2, align 8
  %175 = getelementptr inbounds i8, ptr %174, i32 1
  store ptr %175, ptr %2, align 8
  br label %176

176:                                              ; preds = %173, %168
  %177 = load ptr, ptr %2, align 8
  %178 = getelementptr inbounds i8, ptr %177, i32 1
  store ptr %178, ptr %2, align 8
  br label %156, !llvm.loop !88

179:                                              ; preds = %166
  %180 = load ptr, ptr %2, align 8
  %181 = load i8, ptr %180, align 1
  %182 = icmp ne i8 %181, 0
  br i1 %182, label %183, label %186

183:                                              ; preds = %179
  %184 = load ptr, ptr %2, align 8
  %185 = getelementptr inbounds i8, ptr %184, i32 1
  store ptr %185, ptr %2, align 8
  br label %186

186:                                              ; preds = %183, %179
  br label %212

187:                                              ; preds = %148
  %188 = load ptr, ptr %2, align 8
  %189 = load i8, ptr %188, align 1
  %190 = sext i8 %189 to i32
  %191 = icmp eq i32 %190, 91
  br i1 %191, label %192, label %197

192:                                              ; preds = %187
  %193 = load i32, ptr %4, align 4
  %194 = add nsw i32 %193, 1
  store i32 %194, ptr %4, align 4
  %195 = load ptr, ptr %2, align 8
  %196 = getelementptr inbounds i8, ptr %195, i32 1
  store ptr %196, ptr %2, align 8
  br label %211

197:                                              ; preds = %187
  %198 = load ptr, ptr %2, align 8
  %199 = load i8, ptr %198, align 1
  %200 = sext i8 %199 to i32
  %201 = icmp eq i32 %200, 93
  br i1 %201, label %202, label %207

202:                                              ; preds = %197
  %203 = load i32, ptr %4, align 4
  %204 = add nsw i32 %203, -1
  store i32 %204, ptr %4, align 4
  %205 = load ptr, ptr %2, align 8
  %206 = getelementptr inbounds i8, ptr %205, i32 1
  store ptr %206, ptr %2, align 8
  br label %210

207:                                              ; preds = %197
  %208 = load ptr, ptr %2, align 8
  %209 = getelementptr inbounds i8, ptr %208, i32 1
  store ptr %209, ptr %2, align 8
  br label %210

210:                                              ; preds = %207, %202
  br label %211

211:                                              ; preds = %210, %192
  br label %212

212:                                              ; preds = %211, %186
  br label %138, !llvm.loop !89

213:                                              ; preds = %146
  br label %251

214:                                              ; preds = %130
  br label %215

215:                                              ; preds = %247, %214
  %216 = load ptr, ptr %2, align 8
  %217 = load i8, ptr %216, align 1
  %218 = sext i8 %217 to i32
  %219 = icmp ne i32 %218, 0
  br i1 %219, label %220, label %245

220:                                              ; preds = %215
  %221 = load ptr, ptr %2, align 8
  %222 = load i8, ptr %221, align 1
  %223 = sext i8 %222 to i32
  %224 = icmp ne i32 %223, 44
  br i1 %224, label %225, label %245

225:                                              ; preds = %220
  %226 = load ptr, ptr %2, align 8
  %227 = load i8, ptr %226, align 1
  %228 = sext i8 %227 to i32
  %229 = icmp ne i32 %228, 125
  br i1 %229, label %230, label %245

230:                                              ; preds = %225
  %231 = load ptr, ptr %2, align 8
  %232 = load i8, ptr %231, align 1
  %233 = sext i8 %232 to i32
  %234 = icmp ne i32 %233, 93
  br i1 %234, label %235, label %245

235:                                              ; preds = %230
  %236 = load ptr, ptr %2, align 8
  %237 = load i8, ptr %236, align 1
  %238 = sext i8 %237 to i32
  %239 = icmp ne i32 %238, 32
  br i1 %239, label %240, label %245

240:                                              ; preds = %235
  %241 = load ptr, ptr %2, align 8
  %242 = load i8, ptr %241, align 1
  %243 = sext i8 %242 to i32
  %244 = icmp ne i32 %243, 10
  br label %245

245:                                              ; preds = %240, %235, %230, %225, %220, %215
  %246 = phi i1 [ false, %235 ], [ false, %230 ], [ false, %225 ], [ false, %220 ], [ false, %215 ], [ %244, %240 ]
  br i1 %246, label %247, label %250

247:                                              ; preds = %245
  %248 = load ptr, ptr %2, align 8
  %249 = getelementptr inbounds i8, ptr %248, i32 1
  store ptr %249, ptr %2, align 8
  br label %215, !llvm.loop !90

250:                                              ; preds = %245
  br label %251

251:                                              ; preds = %250, %213
  br label %252

252:                                              ; preds = %251, %129
  br label %253

253:                                              ; preds = %252, %45
  %254 = load ptr, ptr %2, align 8
  ret ptr %254
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_collect_minor() #0 {
  %1 = load i32, ptr @ep_gc_enabled, align 4
  %2 = icmp ne i32 %1, 0
  br i1 %2, label %4, label %3

3:                                                ; preds = %0
  br label %7

4:                                                ; preds = %0
  %5 = load i32, ptr @ep_gc_minor_count, align 4
  %6 = add nsw i32 %5, 1
  store i32 %6, ptr @ep_gc_minor_count, align 4
  call void @ep_gc_mark_minor()
  call void @ep_gc_sweep_minor()
  br label %7

7:                                                ; preds = %4, %3
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_collect_major() #0 {
  %1 = load i32, ptr @ep_gc_enabled, align 4
  %2 = icmp ne i32 %1, 0
  br i1 %2, label %4, label %3

3:                                                ; preds = %0
  br label %12

4:                                                ; preds = %0
  %5 = load i32, ptr @ep_gc_major_count, align 4
  %6 = add nsw i32 %5, 1
  store i32 %6, ptr @ep_gc_major_count, align 4
  call void @ep_gc_mark()
  call void @ep_gc_sweep_major()
  %7 = load i64, ptr @ep_gc_count, align 8
  %8 = mul nsw i64 %7, 2
  store i64 %8, ptr @ep_gc_threshold, align 8
  %9 = load i64, ptr @ep_gc_threshold, align 8
  %10 = icmp slt i64 %9, 4096
  br i1 %10, label %11, label %12

11:                                               ; preds = %4
  store i64 4096, ptr @ep_gc_threshold, align 8
  br label %12

12:                                               ; preds = %3, %11, %4
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_mark_minor() #0 {
  %1 = alloca i32, align 4
  %2 = alloca ptr, align 8
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca i32, align 4
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  store i32 0, ptr %1, align 4
  br label %10

10:                                               ; preds = %67, %0
  %11 = load i32, ptr %1, align 4
  %12 = load i32, ptr @ep_num_threads, align 4
  %13 = icmp slt i32 %11, %12
  br i1 %13, label %14, label %70

14:                                               ; preds = %10
  %15 = load i32, ptr %1, align 4
  %16 = sext i32 %15 to i64
  %17 = getelementptr inbounds [256 x i32], ptr @ep_thread_active, i64 0, i64 %16
  %18 = load volatile i32, ptr %17, align 4
  %19 = icmp ne i32 %18, 0
  br i1 %19, label %21, label %20

20:                                               ; preds = %14
  br label %67

21:                                               ; preds = %14
  %22 = load i32, ptr %1, align 4
  %23 = sext i32 %22 to i64
  %24 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %23
  %25 = load ptr, ptr %24, align 8
  store ptr %25, ptr %2, align 8
  %26 = load ptr, ptr %2, align 8
  %27 = icmp ne ptr %26, null
  br i1 %27, label %29, label %28

28:                                               ; preds = %21
  br label %67

29:                                               ; preds = %21
  %30 = load ptr, ptr %2, align 8
  %31 = getelementptr inbounds %struct.EpThreadGCState, ptr %30, i32 0, i32 1
  %32 = load volatile i32, ptr %31, align 8
  store i32 %32, ptr %3, align 4
  %33 = load i32, ptr %3, align 4
  %34 = icmp sle i32 %33, 0
  br i1 %34, label %38, label %35

35:                                               ; preds = %29
  %36 = load i32, ptr %3, align 4
  %37 = icmp sgt i32 %36, 4096
  br i1 %37, label %38, label %39

38:                                               ; preds = %35, %29
  br label %67

39:                                               ; preds = %35
  store i32 0, ptr %4, align 4
  br label %40

40:                                               ; preds = %63, %39
  %41 = load i32, ptr %4, align 4
  %42 = load i32, ptr %3, align 4
  %43 = icmp slt i32 %41, %42
  br i1 %43, label %44, label %66

44:                                               ; preds = %40
  %45 = load ptr, ptr %2, align 8
  %46 = getelementptr inbounds %struct.EpThreadGCState, ptr %45, i32 0, i32 0
  %47 = load i32, ptr %4, align 4
  %48 = sext i32 %47 to i64
  %49 = getelementptr inbounds [4096 x ptr], ptr %46, i64 0, i64 %48
  %50 = load ptr, ptr %49, align 8
  store ptr %50, ptr %5, align 8
  %51 = load ptr, ptr %5, align 8
  %52 = icmp ne ptr %51, null
  br i1 %52, label %54, label %53

53:                                               ; preds = %44
  br label %63

54:                                               ; preds = %44
  %55 = load ptr, ptr %5, align 8
  %56 = load i64, ptr %55, align 8
  store i64 %56, ptr %6, align 8
  %57 = load i64, ptr %6, align 8
  %58 = icmp ne i64 %57, 0
  br i1 %58, label %59, label %62

59:                                               ; preds = %54
  %60 = load i64, ptr %6, align 8
  %61 = inttoptr i64 %60 to ptr
  call void @ep_gc_mark_object_minor(ptr noundef %61)
  br label %62

62:                                               ; preds = %59, %54
  br label %63

63:                                               ; preds = %62, %53
  %64 = load i32, ptr %4, align 4
  %65 = add nsw i32 %64, 1
  store i32 %65, ptr %4, align 4
  br label %40, !llvm.loop !91

66:                                               ; preds = %40
  br label %67

67:                                               ; preds = %66, %38, %28, %20
  %68 = load i32, ptr %1, align 4
  %69 = add nsw i32 %68, 1
  store i32 %69, ptr %1, align 4
  br label %10, !llvm.loop !92

70:                                               ; preds = %10
  store i32 0, ptr %7, align 4
  br label %71

71:                                               ; preds = %89, %70
  %72 = load i32, ptr %7, align 4
  %73 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %74 = load i32, ptr %73, align 4
  %75 = icmp slt i32 %72, %74
  br i1 %75, label %76, label %92

76:                                               ; preds = %71
  %77 = call align 8 ptr @llvm.threadlocal.address.p0(ptr align 8 @ep_gc_root_stack)
  %78 = load i32, ptr %7, align 4
  %79 = sext i32 %78 to i64
  %80 = getelementptr inbounds [4096 x ptr], ptr %77, i64 0, i64 %79
  %81 = load ptr, ptr %80, align 8
  %82 = load i64, ptr %81, align 8
  store i64 %82, ptr %8, align 8
  %83 = load i64, ptr %8, align 8
  %84 = icmp ne i64 %83, 0
  br i1 %84, label %85, label %88

85:                                               ; preds = %76
  %86 = load i64, ptr %8, align 8
  %87 = inttoptr i64 %86 to ptr
  call void @ep_gc_mark_object_minor(ptr noundef %87)
  br label %88

88:                                               ; preds = %85, %76
  br label %89

89:                                               ; preds = %88
  %90 = load i32, ptr %7, align 4
  %91 = add nsw i32 %90, 1
  store i32 %91, ptr %7, align 4
  br label %71, !llvm.loop !93

92:                                               ; preds = %71
  store i64 0, ptr %9, align 8
  br label %93

93:                                               ; preds = %102, %92
  %94 = load i64, ptr %9, align 8
  %95 = load i64, ptr @ep_gc_remembered_size, align 8
  %96 = icmp slt i64 %94, %95
  br i1 %96, label %97, label %105

97:                                               ; preds = %93
  %98 = load ptr, ptr @ep_gc_remembered_set, align 8
  %99 = load i64, ptr %9, align 8
  %100 = getelementptr inbounds ptr, ptr %98, i64 %99
  %101 = load ptr, ptr %100, align 8
  call void @ep_gc_mark_object_minor(ptr noundef %101)
  br label %102

102:                                              ; preds = %97
  %103 = load i64, ptr %9, align 8
  %104 = add nsw i64 %103, 1
  store i64 %104, ptr %9, align 8
  br label %93, !llvm.loop !94

105:                                              ; preds = %93
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_sweep_minor() #0 {
  %1 = alloca ptr, align 8
  %2 = alloca ptr, align 8
  %3 = alloca ptr, align 8
  store ptr @ep_gc_head, ptr %1, align 8
  br label %4

4:                                                ; preds = %99, %0
  %5 = load ptr, ptr %1, align 8
  %6 = load ptr, ptr %5, align 8
  %7 = icmp ne ptr %6, null
  br i1 %7, label %8, label %100

8:                                                ; preds = %4
  %9 = load ptr, ptr %1, align 8
  %10 = load ptr, ptr %9, align 8
  %11 = getelementptr inbounds %struct.EpGCObject, ptr %10, i32 0, i32 5
  %12 = load i32, ptr %11, align 8
  %13 = icmp eq i32 %12, 0
  br i1 %13, label %14, label %95

14:                                               ; preds = %8
  %15 = load ptr, ptr %1, align 8
  %16 = load ptr, ptr %15, align 8
  %17 = getelementptr inbounds %struct.EpGCObject, ptr %16, i32 0, i32 1
  %18 = load i32, ptr %17, align 4
  %19 = icmp ne i32 %18, 0
  br i1 %19, label %82, label %20

20:                                               ; preds = %14
  %21 = load ptr, ptr %1, align 8
  %22 = load ptr, ptr %21, align 8
  store ptr %22, ptr %2, align 8
  %23 = load ptr, ptr %2, align 8
  %24 = getelementptr inbounds %struct.EpGCObject, ptr %23, i32 0, i32 6
  %25 = load ptr, ptr %24, align 8
  %26 = load ptr, ptr %1, align 8
  store ptr %25, ptr %26, align 8
  %27 = load ptr, ptr %2, align 8
  %28 = getelementptr inbounds %struct.EpGCObject, ptr %27, i32 0, i32 2
  %29 = load ptr, ptr %28, align 8
  call void @ep_gc_table_remove(ptr noundef %29)
  %30 = load ptr, ptr %2, align 8
  %31 = getelementptr inbounds %struct.EpGCObject, ptr %30, i32 0, i32 0
  %32 = load i32, ptr %31, align 8
  %33 = icmp eq i32 %32, 0
  br i1 %33, label %34, label %46

34:                                               ; preds = %20
  %35 = load ptr, ptr %2, align 8
  %36 = getelementptr inbounds %struct.EpGCObject, ptr %35, i32 0, i32 2
  %37 = load ptr, ptr %36, align 8
  store ptr %37, ptr %3, align 8
  %38 = load ptr, ptr %3, align 8
  %39 = icmp ne ptr %38, null
  br i1 %39, label %40, label %45

40:                                               ; preds = %34
  %41 = load ptr, ptr %3, align 8
  %42 = getelementptr inbounds %struct.EpList, ptr %41, i32 0, i32 0
  %43 = load ptr, ptr %42, align 8
  call void @free(ptr noundef %43)
  %44 = load ptr, ptr %3, align 8
  call void @free(ptr noundef %44)
  br label %45

45:                                               ; preds = %40, %34
  br label %76

46:                                               ; preds = %20
  %47 = load ptr, ptr %2, align 8
  %48 = getelementptr inbounds %struct.EpGCObject, ptr %47, i32 0, i32 0
  %49 = load i32, ptr %48, align 8
  %50 = icmp eq i32 %49, 1
  br i1 %50, label %51, label %55

51:                                               ; preds = %46
  %52 = load ptr, ptr %2, align 8
  %53 = getelementptr inbounds %struct.EpGCObject, ptr %52, i32 0, i32 2
  %54 = load ptr, ptr %53, align 8
  call void @free(ptr noundef %54)
  br label %75

55:                                               ; preds = %46
  %56 = load ptr, ptr %2, align 8
  %57 = getelementptr inbounds %struct.EpGCObject, ptr %56, i32 0, i32 0
  %58 = load i32, ptr %57, align 8
  %59 = icmp eq i32 %58, 2
  br i1 %59, label %60, label %64

60:                                               ; preds = %55
  %61 = load ptr, ptr %2, align 8
  %62 = getelementptr inbounds %struct.EpGCObject, ptr %61, i32 0, i32 2
  %63 = load ptr, ptr %62, align 8
  call void @free(ptr noundef %63)
  br label %74

64:                                               ; preds = %55
  %65 = load ptr, ptr %2, align 8
  %66 = getelementptr inbounds %struct.EpGCObject, ptr %65, i32 0, i32 0
  %67 = load i32, ptr %66, align 8
  %68 = icmp eq i32 %67, 3
  br i1 %68, label %69, label %73

69:                                               ; preds = %64
  %70 = load ptr, ptr %2, align 8
  %71 = getelementptr inbounds %struct.EpGCObject, ptr %70, i32 0, i32 2
  %72 = load ptr, ptr %71, align 8
  call void @free(ptr noundef %72)
  br label %73

73:                                               ; preds = %69, %64
  br label %74

74:                                               ; preds = %73, %60
  br label %75

75:                                               ; preds = %74, %51
  br label %76

76:                                               ; preds = %75, %45
  %77 = load ptr, ptr %2, align 8
  call void @free(ptr noundef %77)
  %78 = load i64, ptr @ep_gc_count, align 8
  %79 = add nsw i64 %78, -1
  store i64 %79, ptr @ep_gc_count, align 8
  %80 = load i64, ptr @ep_gc_nursery_count, align 8
  %81 = add nsw i64 %80, -1
  store i64 %81, ptr @ep_gc_nursery_count, align 8
  br label %94

82:                                               ; preds = %14
  %83 = load ptr, ptr %1, align 8
  %84 = load ptr, ptr %83, align 8
  %85 = getelementptr inbounds %struct.EpGCObject, ptr %84, i32 0, i32 1
  store i32 0, ptr %85, align 4
  %86 = load ptr, ptr %1, align 8
  %87 = load ptr, ptr %86, align 8
  %88 = getelementptr inbounds %struct.EpGCObject, ptr %87, i32 0, i32 5
  store i32 1, ptr %88, align 8
  %89 = load i64, ptr @ep_gc_nursery_count, align 8
  %90 = add nsw i64 %89, -1
  store i64 %90, ptr @ep_gc_nursery_count, align 8
  %91 = load ptr, ptr %1, align 8
  %92 = load ptr, ptr %91, align 8
  %93 = getelementptr inbounds %struct.EpGCObject, ptr %92, i32 0, i32 6
  store ptr %93, ptr %1, align 8
  br label %94

94:                                               ; preds = %82, %76
  br label %99

95:                                               ; preds = %8
  %96 = load ptr, ptr %1, align 8
  %97 = load ptr, ptr %96, align 8
  %98 = getelementptr inbounds %struct.EpGCObject, ptr %97, i32 0, i32 6
  store ptr %98, ptr %1, align 8
  br label %99

99:                                               ; preds = %95, %94
  br label %4, !llvm.loop !95

100:                                              ; preds = %4
  store i64 0, ptr @ep_gc_remembered_size, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_mark_object_minor(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  store ptr %0, ptr %2, align 8
  %9 = load ptr, ptr %2, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %12, label %11

11:                                               ; preds = %1
  br label %91

12:                                               ; preds = %1
  %13 = load ptr, ptr %2, align 8
  %14 = call ptr @ep_gc_find(ptr noundef %13)
  store ptr %14, ptr %3, align 8
  %15 = load ptr, ptr %3, align 8
  %16 = icmp ne ptr %15, null
  br i1 %16, label %17, label %27

17:                                               ; preds = %12
  %18 = load ptr, ptr %3, align 8
  %19 = getelementptr inbounds %struct.EpGCObject, ptr %18, i32 0, i32 5
  %20 = load i32, ptr %19, align 8
  %21 = icmp ne i32 %20, 0
  br i1 %21, label %27, label %22

22:                                               ; preds = %17
  %23 = load ptr, ptr %3, align 8
  %24 = getelementptr inbounds %struct.EpGCObject, ptr %23, i32 0, i32 1
  %25 = load i32, ptr %24, align 4
  %26 = icmp ne i32 %25, 0
  br i1 %26, label %27, label %28

27:                                               ; preds = %22, %17, %12
  br label %91

28:                                               ; preds = %22
  %29 = load ptr, ptr %3, align 8
  %30 = getelementptr inbounds %struct.EpGCObject, ptr %29, i32 0, i32 1
  store i32 1, ptr %30, align 4
  %31 = load ptr, ptr %3, align 8
  %32 = getelementptr inbounds %struct.EpGCObject, ptr %31, i32 0, i32 0
  %33 = load i32, ptr %32, align 8
  %34 = icmp eq i32 %33, 0
  br i1 %34, label %35, label %60

35:                                               ; preds = %28
  %36 = load ptr, ptr %2, align 8
  store ptr %36, ptr %4, align 8
  store i64 0, ptr %5, align 8
  br label %37

37:                                               ; preds = %56, %35
  %38 = load i64, ptr %5, align 8
  %39 = load ptr, ptr %4, align 8
  %40 = getelementptr inbounds %struct.EpList, ptr %39, i32 0, i32 1
  %41 = load i64, ptr %40, align 8
  %42 = icmp slt i64 %38, %41
  br i1 %42, label %43, label %59

43:                                               ; preds = %37
  %44 = load ptr, ptr %4, align 8
  %45 = getelementptr inbounds %struct.EpList, ptr %44, i32 0, i32 0
  %46 = load ptr, ptr %45, align 8
  %47 = load i64, ptr %5, align 8
  %48 = getelementptr inbounds i64, ptr %46, i64 %47
  %49 = load i64, ptr %48, align 8
  store i64 %49, ptr %6, align 8
  %50 = load i64, ptr %6, align 8
  %51 = icmp ne i64 %50, 0
  br i1 %51, label %52, label %55

52:                                               ; preds = %43
  %53 = load i64, ptr %6, align 8
  %54 = inttoptr i64 %53 to ptr
  call void @ep_gc_mark_object_minor(ptr noundef %54)
  br label %55

55:                                               ; preds = %52, %43
  br label %56

56:                                               ; preds = %55
  %57 = load i64, ptr %5, align 8
  %58 = add nsw i64 %57, 1
  store i64 %58, ptr %5, align 8
  br label %37, !llvm.loop !96

59:                                               ; preds = %37
  br label %91

60:                                               ; preds = %28
  %61 = load ptr, ptr %3, align 8
  %62 = getelementptr inbounds %struct.EpGCObject, ptr %61, i32 0, i32 0
  %63 = load i32, ptr %62, align 8
  %64 = icmp eq i32 %63, 2
  br i1 %64, label %65, label %90

65:                                               ; preds = %60
  %66 = load ptr, ptr %2, align 8
  store ptr %66, ptr %7, align 8
  store i64 0, ptr %8, align 8
  br label %67

67:                                               ; preds = %86, %65
  %68 = load i64, ptr %8, align 8
  %69 = load ptr, ptr %3, align 8
  %70 = getelementptr inbounds %struct.EpGCObject, ptr %69, i32 0, i32 4
  %71 = load i64, ptr %70, align 8
  %72 = icmp slt i64 %68, %71
  br i1 %72, label %73, label %89

73:                                               ; preds = %67
  %74 = load ptr, ptr %7, align 8
  %75 = load i64, ptr %8, align 8
  %76 = getelementptr inbounds i64, ptr %74, i64 %75
  %77 = load i64, ptr %76, align 8
  %78 = icmp ne i64 %77, 0
  br i1 %78, label %79, label %85

79:                                               ; preds = %73
  %80 = load ptr, ptr %7, align 8
  %81 = load i64, ptr %8, align 8
  %82 = getelementptr inbounds i64, ptr %80, i64 %81
  %83 = load i64, ptr %82, align 8
  %84 = inttoptr i64 %83 to ptr
  call void @ep_gc_mark_object_minor(ptr noundef %84)
  br label %85

85:                                               ; preds = %79, %73
  br label %86

86:                                               ; preds = %85
  %87 = load i64, ptr %8, align 8
  %88 = add nsw i64 %87, 1
  store i64 %88, ptr %8, align 8
  br label %67, !llvm.loop !97

89:                                               ; preds = %67
  br label %90

90:                                               ; preds = %89, %60
  br label %91

91:                                               ; preds = %11, %27, %90, %59
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_mark() #0 {
  %1 = alloca i32, align 4
  %2 = alloca ptr, align 8
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca ptr, align 8
  %6 = alloca i64, align 8
  %7 = alloca i32, align 4
  %8 = alloca i64, align 8
  store i32 0, ptr %1, align 4
  br label %9

9:                                                ; preds = %66, %0
  %10 = load i32, ptr %1, align 4
  %11 = load i32, ptr @ep_num_threads, align 4
  %12 = icmp slt i32 %10, %11
  br i1 %12, label %13, label %69

13:                                               ; preds = %9
  %14 = load i32, ptr %1, align 4
  %15 = sext i32 %14 to i64
  %16 = getelementptr inbounds [256 x i32], ptr @ep_thread_active, i64 0, i64 %15
  %17 = load volatile i32, ptr %16, align 4
  %18 = icmp ne i32 %17, 0
  br i1 %18, label %20, label %19

19:                                               ; preds = %13
  br label %66

20:                                               ; preds = %13
  %21 = load i32, ptr %1, align 4
  %22 = sext i32 %21 to i64
  %23 = getelementptr inbounds [256 x ptr], ptr @ep_thread_gc_states, i64 0, i64 %22
  %24 = load ptr, ptr %23, align 8
  store ptr %24, ptr %2, align 8
  %25 = load ptr, ptr %2, align 8
  %26 = icmp ne ptr %25, null
  br i1 %26, label %28, label %27

27:                                               ; preds = %20
  br label %66

28:                                               ; preds = %20
  %29 = load ptr, ptr %2, align 8
  %30 = getelementptr inbounds %struct.EpThreadGCState, ptr %29, i32 0, i32 1
  %31 = load volatile i32, ptr %30, align 8
  store i32 %31, ptr %3, align 4
  %32 = load i32, ptr %3, align 4
  %33 = icmp sle i32 %32, 0
  br i1 %33, label %37, label %34

34:                                               ; preds = %28
  %35 = load i32, ptr %3, align 4
  %36 = icmp sgt i32 %35, 4096
  br i1 %36, label %37, label %38

37:                                               ; preds = %34, %28
  br label %66

38:                                               ; preds = %34
  store i32 0, ptr %4, align 4
  br label %39

39:                                               ; preds = %62, %38
  %40 = load i32, ptr %4, align 4
  %41 = load i32, ptr %3, align 4
  %42 = icmp slt i32 %40, %41
  br i1 %42, label %43, label %65

43:                                               ; preds = %39
  %44 = load ptr, ptr %2, align 8
  %45 = getelementptr inbounds %struct.EpThreadGCState, ptr %44, i32 0, i32 0
  %46 = load i32, ptr %4, align 4
  %47 = sext i32 %46 to i64
  %48 = getelementptr inbounds [4096 x ptr], ptr %45, i64 0, i64 %47
  %49 = load ptr, ptr %48, align 8
  store ptr %49, ptr %5, align 8
  %50 = load ptr, ptr %5, align 8
  %51 = icmp ne ptr %50, null
  br i1 %51, label %53, label %52

52:                                               ; preds = %43
  br label %62

53:                                               ; preds = %43
  %54 = load ptr, ptr %5, align 8
  %55 = load i64, ptr %54, align 8
  store i64 %55, ptr %6, align 8
  %56 = load i64, ptr %6, align 8
  %57 = icmp ne i64 %56, 0
  br i1 %57, label %58, label %61

58:                                               ; preds = %53
  %59 = load i64, ptr %6, align 8
  %60 = inttoptr i64 %59 to ptr
  call void @ep_gc_mark_object(ptr noundef %60)
  br label %61

61:                                               ; preds = %58, %53
  br label %62

62:                                               ; preds = %61, %52
  %63 = load i32, ptr %4, align 4
  %64 = add nsw i32 %63, 1
  store i32 %64, ptr %4, align 4
  br label %39, !llvm.loop !98

65:                                               ; preds = %39
  br label %66

66:                                               ; preds = %65, %37, %27, %19
  %67 = load i32, ptr %1, align 4
  %68 = add nsw i32 %67, 1
  store i32 %68, ptr %1, align 4
  br label %9, !llvm.loop !99

69:                                               ; preds = %9
  store i32 0, ptr %7, align 4
  br label %70

70:                                               ; preds = %88, %69
  %71 = load i32, ptr %7, align 4
  %72 = call align 4 ptr @llvm.threadlocal.address.p0(ptr align 4 @ep_gc_root_sp)
  %73 = load i32, ptr %72, align 4
  %74 = icmp slt i32 %71, %73
  br i1 %74, label %75, label %91

75:                                               ; preds = %70
  %76 = call align 8 ptr @llvm.threadlocal.address.p0(ptr align 8 @ep_gc_root_stack)
  %77 = load i32, ptr %7, align 4
  %78 = sext i32 %77 to i64
  %79 = getelementptr inbounds [4096 x ptr], ptr %76, i64 0, i64 %78
  %80 = load ptr, ptr %79, align 8
  %81 = load i64, ptr %80, align 8
  store i64 %81, ptr %8, align 8
  %82 = load i64, ptr %8, align 8
  %83 = icmp ne i64 %82, 0
  br i1 %83, label %84, label %87

84:                                               ; preds = %75
  %85 = load i64, ptr %8, align 8
  %86 = inttoptr i64 %85 to ptr
  call void @ep_gc_mark_object(ptr noundef %86)
  br label %87

87:                                               ; preds = %84, %75
  br label %88

88:                                               ; preds = %87
  %89 = load i32, ptr %7, align 4
  %90 = add nsw i32 %89, 1
  store i32 %90, ptr %7, align 4
  br label %70, !llvm.loop !100

91:                                               ; preds = %70
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_sweep_major() #0 {
  %1 = alloca ptr, align 8
  %2 = alloca ptr, align 8
  %3 = alloca ptr, align 8
  store ptr @ep_gc_head, ptr %1, align 8
  br label %4

4:                                                ; preds = %101, %0
  %5 = load ptr, ptr %1, align 8
  %6 = load ptr, ptr %5, align 8
  %7 = icmp ne ptr %6, null
  br i1 %7, label %8, label %102

8:                                                ; preds = %4
  %9 = load ptr, ptr %1, align 8
  %10 = load ptr, ptr %9, align 8
  %11 = getelementptr inbounds %struct.EpGCObject, ptr %10, i32 0, i32 1
  %12 = load i32, ptr %11, align 4
  %13 = icmp ne i32 %12, 0
  br i1 %13, label %82, label %14

14:                                               ; preds = %8
  %15 = load ptr, ptr %1, align 8
  %16 = load ptr, ptr %15, align 8
  store ptr %16, ptr %2, align 8
  %17 = load ptr, ptr %2, align 8
  %18 = getelementptr inbounds %struct.EpGCObject, ptr %17, i32 0, i32 6
  %19 = load ptr, ptr %18, align 8
  %20 = load ptr, ptr %1, align 8
  store ptr %19, ptr %20, align 8
  %21 = load ptr, ptr %2, align 8
  %22 = getelementptr inbounds %struct.EpGCObject, ptr %21, i32 0, i32 2
  %23 = load ptr, ptr %22, align 8
  call void @ep_gc_table_remove(ptr noundef %23)
  %24 = load ptr, ptr %2, align 8
  %25 = getelementptr inbounds %struct.EpGCObject, ptr %24, i32 0, i32 5
  %26 = load i32, ptr %25, align 8
  %27 = icmp eq i32 %26, 0
  br i1 %27, label %28, label %31

28:                                               ; preds = %14
  %29 = load i64, ptr @ep_gc_nursery_count, align 8
  %30 = add nsw i64 %29, -1
  store i64 %30, ptr @ep_gc_nursery_count, align 8
  br label %31

31:                                               ; preds = %28, %14
  %32 = load ptr, ptr %2, align 8
  %33 = getelementptr inbounds %struct.EpGCObject, ptr %32, i32 0, i32 0
  %34 = load i32, ptr %33, align 8
  %35 = icmp eq i32 %34, 0
  br i1 %35, label %36, label %48

36:                                               ; preds = %31
  %37 = load ptr, ptr %2, align 8
  %38 = getelementptr inbounds %struct.EpGCObject, ptr %37, i32 0, i32 2
  %39 = load ptr, ptr %38, align 8
  store ptr %39, ptr %3, align 8
  %40 = load ptr, ptr %3, align 8
  %41 = icmp ne ptr %40, null
  br i1 %41, label %42, label %47

42:                                               ; preds = %36
  %43 = load ptr, ptr %3, align 8
  %44 = getelementptr inbounds %struct.EpList, ptr %43, i32 0, i32 0
  %45 = load ptr, ptr %44, align 8
  call void @free(ptr noundef %45)
  %46 = load ptr, ptr %3, align 8
  call void @free(ptr noundef %46)
  br label %47

47:                                               ; preds = %42, %36
  br label %78

48:                                               ; preds = %31
  %49 = load ptr, ptr %2, align 8
  %50 = getelementptr inbounds %struct.EpGCObject, ptr %49, i32 0, i32 0
  %51 = load i32, ptr %50, align 8
  %52 = icmp eq i32 %51, 1
  br i1 %52, label %53, label %57

53:                                               ; preds = %48
  %54 = load ptr, ptr %2, align 8
  %55 = getelementptr inbounds %struct.EpGCObject, ptr %54, i32 0, i32 2
  %56 = load ptr, ptr %55, align 8
  call void @free(ptr noundef %56)
  br label %77

57:                                               ; preds = %48
  %58 = load ptr, ptr %2, align 8
  %59 = getelementptr inbounds %struct.EpGCObject, ptr %58, i32 0, i32 0
  %60 = load i32, ptr %59, align 8
  %61 = icmp eq i32 %60, 2
  br i1 %61, label %62, label %66

62:                                               ; preds = %57
  %63 = load ptr, ptr %2, align 8
  %64 = getelementptr inbounds %struct.EpGCObject, ptr %63, i32 0, i32 2
  %65 = load ptr, ptr %64, align 8
  call void @free(ptr noundef %65)
  br label %76

66:                                               ; preds = %57
  %67 = load ptr, ptr %2, align 8
  %68 = getelementptr inbounds %struct.EpGCObject, ptr %67, i32 0, i32 0
  %69 = load i32, ptr %68, align 8
  %70 = icmp eq i32 %69, 3
  br i1 %70, label %71, label %75

71:                                               ; preds = %66
  %72 = load ptr, ptr %2, align 8
  %73 = getelementptr inbounds %struct.EpGCObject, ptr %72, i32 0, i32 2
  %74 = load ptr, ptr %73, align 8
  call void @free(ptr noundef %74)
  br label %75

75:                                               ; preds = %71, %66
  br label %76

76:                                               ; preds = %75, %62
  br label %77

77:                                               ; preds = %76, %53
  br label %78

78:                                               ; preds = %77, %47
  %79 = load ptr, ptr %2, align 8
  call void @free(ptr noundef %79)
  %80 = load i64, ptr @ep_gc_count, align 8
  %81 = add nsw i64 %80, -1
  store i64 %81, ptr @ep_gc_count, align 8
  br label %101

82:                                               ; preds = %8
  %83 = load ptr, ptr %1, align 8
  %84 = load ptr, ptr %83, align 8
  %85 = getelementptr inbounds %struct.EpGCObject, ptr %84, i32 0, i32 1
  store i32 0, ptr %85, align 4
  %86 = load ptr, ptr %1, align 8
  %87 = load ptr, ptr %86, align 8
  %88 = getelementptr inbounds %struct.EpGCObject, ptr %87, i32 0, i32 5
  %89 = load i32, ptr %88, align 8
  %90 = icmp eq i32 %89, 0
  br i1 %90, label %91, label %97

91:                                               ; preds = %82
  %92 = load ptr, ptr %1, align 8
  %93 = load ptr, ptr %92, align 8
  %94 = getelementptr inbounds %struct.EpGCObject, ptr %93, i32 0, i32 5
  store i32 1, ptr %94, align 8
  %95 = load i64, ptr @ep_gc_nursery_count, align 8
  %96 = add nsw i64 %95, -1
  store i64 %96, ptr @ep_gc_nursery_count, align 8
  br label %97

97:                                               ; preds = %91, %82
  %98 = load ptr, ptr %1, align 8
  %99 = load ptr, ptr %98, align 8
  %100 = getelementptr inbounds %struct.EpGCObject, ptr %99, i32 0, i32 6
  store ptr %100, ptr %1, align 8
  br label %101

101:                                              ; preds = %97, %78
  br label %4, !llvm.loop !101

102:                                              ; preds = %4
  store i64 0, ptr @ep_gc_remembered_size, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define internal void @ep_gc_mark_object(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca ptr, align 8
  %4 = alloca ptr, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i64, align 8
  store ptr %0, ptr %2, align 8
  %9 = load ptr, ptr %2, align 8
  %10 = icmp ne ptr %9, null
  br i1 %10, label %12, label %11

11:                                               ; preds = %1
  br label %86

12:                                               ; preds = %1
  %13 = load ptr, ptr %2, align 8
  %14 = call ptr @ep_gc_find(ptr noundef %13)
  store ptr %14, ptr %3, align 8
  %15 = load ptr, ptr %3, align 8
  %16 = icmp ne ptr %15, null
  br i1 %16, label %17, label %22

17:                                               ; preds = %12
  %18 = load ptr, ptr %3, align 8
  %19 = getelementptr inbounds %struct.EpGCObject, ptr %18, i32 0, i32 1
  %20 = load i32, ptr %19, align 4
  %21 = icmp ne i32 %20, 0
  br i1 %21, label %22, label %23

22:                                               ; preds = %17, %12
  br label %86

23:                                               ; preds = %17
  %24 = load ptr, ptr %3, align 8
  %25 = getelementptr inbounds %struct.EpGCObject, ptr %24, i32 0, i32 1
  store i32 1, ptr %25, align 4
  %26 = load ptr, ptr %3, align 8
  %27 = getelementptr inbounds %struct.EpGCObject, ptr %26, i32 0, i32 0
  %28 = load i32, ptr %27, align 8
  %29 = icmp eq i32 %28, 0
  br i1 %29, label %30, label %55

30:                                               ; preds = %23
  %31 = load ptr, ptr %2, align 8
  store ptr %31, ptr %4, align 8
  store i64 0, ptr %5, align 8
  br label %32

32:                                               ; preds = %51, %30
  %33 = load i64, ptr %5, align 8
  %34 = load ptr, ptr %4, align 8
  %35 = getelementptr inbounds %struct.EpList, ptr %34, i32 0, i32 1
  %36 = load i64, ptr %35, align 8
  %37 = icmp slt i64 %33, %36
  br i1 %37, label %38, label %54

38:                                               ; preds = %32
  %39 = load ptr, ptr %4, align 8
  %40 = getelementptr inbounds %struct.EpList, ptr %39, i32 0, i32 0
  %41 = load ptr, ptr %40, align 8
  %42 = load i64, ptr %5, align 8
  %43 = getelementptr inbounds i64, ptr %41, i64 %42
  %44 = load i64, ptr %43, align 8
  store i64 %44, ptr %6, align 8
  %45 = load i64, ptr %6, align 8
  %46 = icmp ne i64 %45, 0
  br i1 %46, label %47, label %50

47:                                               ; preds = %38
  %48 = load i64, ptr %6, align 8
  %49 = inttoptr i64 %48 to ptr
  call void @ep_gc_mark_object(ptr noundef %49)
  br label %50

50:                                               ; preds = %47, %38
  br label %51

51:                                               ; preds = %50
  %52 = load i64, ptr %5, align 8
  %53 = add nsw i64 %52, 1
  store i64 %53, ptr %5, align 8
  br label %32, !llvm.loop !102

54:                                               ; preds = %32
  br label %86

55:                                               ; preds = %23
  %56 = load ptr, ptr %3, align 8
  %57 = getelementptr inbounds %struct.EpGCObject, ptr %56, i32 0, i32 0
  %58 = load i32, ptr %57, align 8
  %59 = icmp eq i32 %58, 2
  br i1 %59, label %60, label %85

60:                                               ; preds = %55
  %61 = load ptr, ptr %2, align 8
  store ptr %61, ptr %7, align 8
  store i64 0, ptr %8, align 8
  br label %62

62:                                               ; preds = %81, %60
  %63 = load i64, ptr %8, align 8
  %64 = load ptr, ptr %3, align 8
  %65 = getelementptr inbounds %struct.EpGCObject, ptr %64, i32 0, i32 4
  %66 = load i64, ptr %65, align 8
  %67 = icmp slt i64 %63, %66
  br i1 %67, label %68, label %84

68:                                               ; preds = %62
  %69 = load ptr, ptr %7, align 8
  %70 = load i64, ptr %8, align 8
  %71 = getelementptr inbounds i64, ptr %69, i64 %70
  %72 = load i64, ptr %71, align 8
  %73 = icmp ne i64 %72, 0
  br i1 %73, label %74, label %80

74:                                               ; preds = %68
  %75 = load ptr, ptr %7, align 8
  %76 = load i64, ptr %8, align 8
  %77 = getelementptr inbounds i64, ptr %75, i64 %76
  %78 = load i64, ptr %77, align 8
  %79 = inttoptr i64 %78 to ptr
  call void @ep_gc_mark_object(ptr noundef %79)
  br label %80

80:                                               ; preds = %74, %68
  br label %81

81:                                               ; preds = %80
  %82 = load i64, ptr %8, align 8
  %83 = add nsw i64 %82, 1
  store i64 %83, ptr %8, align 8
  br label %62, !llvm.loop !103

84:                                               ; preds = %62
  br label %85

85:                                               ; preds = %84, %55
  br label %86

86:                                               ; preds = %11, %22, %85, %54
  ret void
}

attributes #0 = { noinline nounwind optnone ssp uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+bti,+ccdp,+ccidx,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #1 = { "frame-pointer"="non-leaf" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+bti,+ccdp,+ccidx,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #2 = { allocsize(0) "frame-pointer"="non-leaf" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+bti,+ccdp,+ccidx,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #3 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #4 = { nounwind "frame-pointer"="non-leaf" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+bti,+ccdp,+ccidx,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #5 = { convergent nocallback nofree nosync nounwind willreturn memory(none) }
attributes #6 = { allocsize(0,1) "frame-pointer"="non-leaf" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+bti,+ccdp,+ccidx,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #7 = { allocsize(1) "frame-pointer"="non-leaf" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+bti,+ccdp,+ccidx,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #8 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #9 = { noreturn "frame-pointer"="non-leaf" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+bti,+ccdp,+ccidx,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #10 = { nounwind willreturn memory(read) "frame-pointer"="non-leaf" "no-trapping-math"="true" "probe-stack"="__chkstk_darwin" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+bti,+ccdp,+ccidx,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #11 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #12 = { noreturn }
attributes #13 = { nounwind }
attributes #14 = { allocsize(0) }
attributes #15 = { allocsize(0,1) }
attributes #16 = { allocsize(1) }
attributes #17 = { nounwind willreturn memory(read) }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 2, !"SDK Version", [2 x i32] [i32 15, i32 5]}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 8, !"PIC Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 1}
!4 = !{i32 7, !"frame-pointer", i32 1}
!5 = !{!"Apple clang version 17.0.0 (clang-1700.0.13.5)"}
!6 = distinct !{!6, !7}
!7 = !{!"llvm.loop.mustprogress"}
!8 = distinct !{!8, !7}
!9 = distinct !{!9, !7}
!10 = distinct !{!10, !7}
!11 = distinct !{!11, !7}
!12 = distinct !{!12, !7}
!13 = distinct !{!13, !7}
!14 = distinct !{!14, !7}
!15 = distinct !{!15, !7}
!16 = distinct !{!16, !7}
!17 = distinct !{!17, !7}
!18 = distinct !{!18, !7}
!19 = distinct !{!19, !7}
!20 = distinct !{!20, !7}
!21 = distinct !{!21, !7}
!22 = distinct !{!22, !7}
!23 = distinct !{!23, !7}
!24 = distinct !{!24, !7}
!25 = distinct !{!25, !7}
!26 = distinct !{!26, !7}
!27 = distinct !{!27, !7}
!28 = distinct !{!28, !7}
!29 = distinct !{!29, !7}
!30 = distinct !{!30, !7}
!31 = distinct !{!31, !7}
!32 = distinct !{!32, !7}
!33 = distinct !{!33, !7}
!34 = distinct !{!34, !7}
!35 = distinct !{!35, !7}
!36 = distinct !{!36, !7}
!37 = distinct !{!37, !7}
!38 = distinct !{!38, !7}
!39 = distinct !{!39, !7}
!40 = distinct !{!40, !7}
!41 = distinct !{!41, !7}
!42 = distinct !{!42, !7}
!43 = distinct !{!43, !7}
!44 = distinct !{!44, !7}
!45 = distinct !{!45, !7}
!46 = distinct !{!46, !7}
!47 = distinct !{!47, !7}
!48 = distinct !{!48, !7}
!49 = distinct !{!49, !7}
!50 = distinct !{!50, !7}
!51 = distinct !{!51, !7}
!52 = distinct !{!52, !7}
!53 = distinct !{!53, !7}
!54 = distinct !{!54, !7}
!55 = distinct !{!55, !7}
!56 = distinct !{!56, !7}
!57 = distinct !{!57, !7}
!58 = distinct !{!58, !7}
!59 = distinct !{!59, !7}
!60 = distinct !{!60, !7}
!61 = distinct !{!61, !7}
!62 = distinct !{!62, !7}
!63 = distinct !{!63, !7}
!64 = distinct !{!64, !7}
!65 = distinct !{!65, !7}
!66 = distinct !{!66, !7}
!67 = distinct !{!67, !7}
!68 = distinct !{!68, !7}
!69 = distinct !{!69, !7}
!70 = distinct !{!70, !7}
!71 = distinct !{!71, !7}
!72 = distinct !{!72, !7}
!73 = distinct !{!73, !7}
!74 = distinct !{!74, !7}
!75 = distinct !{!75, !7}
!76 = distinct !{!76, !7}
!77 = distinct !{!77, !7}
!78 = distinct !{!78, !7}
!79 = distinct !{!79, !7}
!80 = distinct !{!80, !7}
!81 = distinct !{!81, !7}
!82 = distinct !{!82, !7}
!83 = distinct !{!83, !7}
!84 = distinct !{!84, !7}
!85 = distinct !{!85, !7}
!86 = distinct !{!86, !7}
!87 = distinct !{!87, !7}
!88 = distinct !{!88, !7}
!89 = distinct !{!89, !7}
!90 = distinct !{!90, !7}
!91 = distinct !{!91, !7}
!92 = distinct !{!92, !7}
!93 = distinct !{!93, !7}
!94 = distinct !{!94, !7}
!95 = distinct !{!95, !7}
!96 = distinct !{!96, !7}
!97 = distinct !{!97, !7}
!98 = distinct !{!98, !7}
!99 = distinct !{!99, !7}
!100 = distinct !{!100, !7}
!101 = distinct !{!101, !7}
!102 = distinct !{!102, !7}
!103 = distinct !{!103, !7}
