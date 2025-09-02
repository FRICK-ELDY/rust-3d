graph TD

  %% ========= Title =========
  %% FRICK-ELDY/rust-3d — Workspace Dependency Map (draft)

  %% ========= Examples =========
  subgraph Examples
    ex_desk_min["examples/desktop/integration_min"]
    ex_desk_clear["examples/desktop/hello_clear"]
    ex_web_min["examples/web/integration_min"]
    ex_web_clear["examples/web/hello_clear"]
  end

  %% ========= Engine Wrapper =========
  subgraph Engine["game_engine (wrapper)"]
    ge["game_engine"]
  end

  %% ========= Core Libraries =========
  subgraph Libraries
    game["game"]
    render["render"]
  end

  %% ========= Platforms / Runners =========
  subgraph Platforms
    app_desktop["platform/desktop (app_desktop)"]
    app_web["platform/web (app_web)"]
  end

  %% ========= Tooling =========
  subgraph Tooling
    xtask["xtask"]
  end

  %% ========= External crates (conceptual) =========
  wgpu[(wgpu)]
  winit[(winit)]
  wasm_bindgen[(wasm-bindgen)]

  %% ========= Edges: usage / deps (high-level) =========
  %% examples run via the unified engine wrapper
  ex_desk_min --> ge
  ex_desk_clear --> ge
  ex_web_min  --> ge
  ex_web_clear --> ge

  %% engine wrapper ties libs + platform runners
  ge --> game
  ge --> render
  ge --> app_desktop
  ge --> app_web

  %% platforms use render
  app_desktop --> render
  app_web --> render

  %% conceptual externals
  render --> wgpu
  app_desktop --> winit
  app_web --> wasm_bindgen

  %% xtask generates WorkspaceLayout.md (dotted = meta relation)
  xtask -. generates .-> ws_layout[("WorkspaceLayout.md")]
