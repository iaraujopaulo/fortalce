use tauri::{LogicalPosition, LogicalSize, Manager};

// Sites antigos que devem abrir no modo Internet Explorer (somente Windows).
// Adicione aqui os enderecos, sem "https://". Exemplo: "sistema-antigo.gov.br"
#[allow(dead_code)]
const SITES_LEGADOS: &[&str] = &[];

#[cfg(windows)]
fn modo_ie(u: &tauri::Url) -> bool {
    let h = u.host_str().unwrap_or("");
    SITES_LEGADOS.iter().any(|s| h == *s || h.ends_with(&format!(".{s}")))
}

// O modo IE usa o motor MSHTML do Windows, que so e acessivel pelo Edge.
// Requer que a politica "Configurar integracao com o Internet Explorer" esteja ativa.
#[cfg(windows)]
fn abrir_no_edge_ie(url: &str) {
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", "msedge", "--ie-mode-test", url])
        .spawn();
}

#[cfg(desktop)]
const BARRA: f64 = 56.0;

#[cfg(desktop)]
#[tauri::command]
fn navegar(app: tauri::AppHandle, url: String) -> Result<(), String> {
    let t = url.trim().to_string();
    let u = if t.contains("://") {
        t
    } else if t.contains('.') && !t.contains(' ') {
        format!("https://{t}")
    } else {
        format!("https://duckduckgo.com/?q={}", t.replace(' ', "+"))
    };
    let parsed: tauri::Url = u.parse().map_err(|e| format!("{e}"))?;
    #[cfg(windows)]
    if modo_ie(&parsed) {
        abrir_no_edge_ie(&u);
        return Ok(());
    }
    app.get_webview("conteudo").ok_or("sem pagina")?.navigate(parsed).map_err(|e| e.to_string())
}

#[cfg(desktop)]
#[tauri::command]
fn acao(app: tauri::AppHandle, nome: String) -> Result<(), String> {
    let js = match nome.as_str() {
        "voltar" => "history.back()",
        "avancar" => "history.forward()",
        _ => "location.reload()",
    };
    app.get_webview("conteudo").ok_or("sem pagina")?.eval(js).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let b = tauri::Builder::default();
    #[cfg(desktop)]
    let b = b.invoke_handler(tauri::generate_handler![navegar, acao]);
    b.setup(|app| {
        #[cfg(desktop)]
        {
            use tauri::{webview::WebviewBuilder, window::WindowBuilder, WebviewUrl};
            let w = WindowBuilder::new(app, "main").title("FortalCE").inner_size(1200.0, 800.0).build()?;
            w.add_child(
                WebviewBuilder::new("barra", WebviewUrl::App("index.html".into())),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(1200.0, BARRA),
            )?;
            w.add_child(
                WebviewBuilder::new("conteudo", WebviewUrl::External("https://duckduckgo.com".parse().unwrap())),
                LogicalPosition::new(0.0, BARRA),
                LogicalSize::new(1200.0, 800.0 - BARRA),
            )?;
            let w2 = w.clone();
            w.on_window_event(move |e| {
                if let tauri::WindowEvent::Resized(s) = e {
                    let l = s.to_logical::<f64>(w2.scale_factor().unwrap_or(1.0));
                    if let Some(x) = w2.get_webview("barra") { let _ = x.set_size(LogicalSize::new(l.width, BARRA)); }
                    if let Some(x) = w2.get_webview("conteudo") {
                        let _ = x.set_position(LogicalPosition::new(0.0, BARRA));
                        let _ = x.set_size(LogicalSize::new(l.width, (l.height - BARRA).max(0.0)));
                    }
                }
            });
        }
        #[cfg(mobile)]
        {
            use tauri::{WebviewUrl, WebviewWindowBuilder};
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External("https://duckduckgo.com".parse().unwrap())).build()?;
        }
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("erro ao iniciar o FortalCE");
}
