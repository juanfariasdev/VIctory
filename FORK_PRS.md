# Custom Fork — Integrated Upstream PRs

This fork keeps `main` tracking clean `upstream/main` (`Marvisak/VIctory`) and integrates tested upstream pull requests onto `custom`.

## Branches

| Branch | Purpose |
|---|---|
| `main` | Clean mirror of `upstream/main` (`Marvisak/VIctory`) |
| `custom` | Active build integrating the PRs listed below |
| `pr/<N>` | Local tracking branches for individual fetched PR heads |

## Integrated PRs on `custom`

| PR | Author | Title | Purpose |
|---|---|---|---|
| [#1832](https://github.com/Marvisak/VIctory/pull/1832) | `fiestorra` | GPU: drop the unordered-access permission a format cannot carry, before any call names it | Fixes `0x887A0001` (`DXGI_ERROR_INVALID_CALL`) when creating UAVs on formats without host typed-UAV support (e.g. `DXGI_FORMAT_B5G5R5A1_UNORM` = 86 on RTX 3070) |
| [#1778](https://github.com/Marvisak/VIctory/pull/1778) | `fiestorra` | GPU: bind a sampler heap of our own when the title binds none | Ensures directly-indexed root signatures always have a valid sampler heap bound |
| [#2315](https://github.com/Marvisak/VIctory/pull/2315) | `xiaotu030202` | GPU: the heap clamp bound must fit what D3D12 gives | Adapts shader heap-index clamping and `NULL_BUFFER_VIEW` to the host D3D12 descriptor heap limit (`1,000,000` on NVIDIA vs `2^21` on AMD) |
| [#2155](https://github.com/Marvisak/VIctory/pull/2155) | `fuzeofficlal` | GPU: rebind compute view-mark root descriptors after depth-twin conversion | Rebinds `view_marks` root SRV/UAV after `crate::twin` depth conversion runs its compute shader |
| [#1803](https://github.com/Marvisak/VIctory/pull/1803) | `mgunes1311-lab` | GPU: copy depth twins from pre-list states in prelude and route depth UAV barriers to twin planes | Preserves pre-list texture states during `HostList::prelude` depth-twin copies so barriers do not emit invalid transitions |
| [#2367](https://github.com/Marvisak/VIctory/pull/2367) | `PanS4ge` | GPU: select host DXGI adapter via VICTORY_ADAPTER and log GPU | Logs the selected GPU adapter at startup and allows explicit adapter selection via `VICTORY_ADAPTER` |

## Managing This Fork

### Sync `main` with upstream
```powershell
git fetch upstream main
git checkout main
git merge --ff-only upstream/main
```

### Fetch and merge another upstream PR into `custom`
```powershell
$pr = 1234
git fetch upstream "pull/$pr/head:pr/$pr"
git checkout custom
git merge --no-ff "pr/$pr" -m "Merge upstream PR #$pr"
```

### Push updates to your fork (`origin`)
```powershell
git push origin main custom
```
