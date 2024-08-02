
## 添加日志

```bash
 RUST_LOG=track cargo run
```

```powershell
$env:RUST_LOG = "trace"; cargo run
```


```bash
  RUST_LOG=tower_http=debug,axumapp=debug cargo run
```

```powershell
$env:RUST_LOG = "tower_http=debug,axumapp=debug"; cargo run
```