# OmniLang Specification: Concurrency Model

**Status**: Stable (Distributed Fabric)
**Version**: v2.1.0

## 1. Goroutines (Lightweight Threads)

OmniLang uses lightweight threads called *mesh-routines* (or just constraints), spawned with the `go` keyword.

```omni
go processInput(data);
```

## 2. Channels

Communication between routines is done via channels, which are strictly typed.

```omni
let ch = make(chan int);

go fn() {
    ch <- 42; // Send
};

let val = <-ch; // Receive
```

The `@mesh` annotation automatically distributes execution across the xAetherOS fabric via TCP RPC.

```omni
@mesh(target: "127.0.0.1:8081")
fn heavyCompute(data: [f64]) -> [f64] {
    // Forwarded to 127.0.0.1:8081
}
```

### Mesh Channels
Channels can seamlessly cross network boundaries.

```omni
@mesh
fn worker(input: chan Job, output: chan Result) {
    for job in input {
        output <- process(job);
    }
}
```

## 4. Select Statement

Wait on multiple channel operations.

```omni
select {
    case msg = <-ch1:
        print("Received from ch1:", msg);
    case ch2 <- valid:
        print("Sent to ch2");
    default:
        print("No activity");
}
```
