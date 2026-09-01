//! karu.tokyo — Rust + Poem 版TOPページ。
//! aruaru-tokyo-server / aon-tokyo と同じ技術スタック・実装方針を踏襲する:
//! DB非依存・1バイナリ完結。
//!
//! テーマ: 軽井沢・あきる野市・東京を含む日本の観光とリモートワークを
//! メインに、IT・AI・AUDIO・貿易産業を紹介するサイト。
//!
//! HTML/CSSは`static/`配下の実ファイルとしてディスクから配信する
//! (`poem::endpoint::StaticFileEndpoint`、aruaru.tokyo/open-gitea と同じ
//! 配信方式)。全ページが実行時パラメータを持たない完全な静的コンテンツ
//! のため、サーバー側での文字列組み立ては行わない。

use poem::endpoint::StaticFileEndpoint;
use poem::{get, handler, Route, Server};
use poem::listener::TcpListener;

#[handler]
fn healthz() -> &'static str {
    healthz_impl()
}

fn healthz_impl() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn healthz_returns_ok() {
        assert_eq!(healthz_impl(), "ok");
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    let app = Route::new()
        .at("/", StaticFileEndpoint::new("static/index.html"))
        .at("/healthz", get(healthz))
        .at("/tourism", StaticFileEndpoint::new("static/tourism.html"))
        .at("/lifestyle", StaticFileEndpoint::new("static/lifestyle.html"))
        .at("/industry", StaticFileEndpoint::new("static/industry.html"))
        .at("/help", StaticFileEndpoint::new("static/help.html"))
        .at("/style.css", StaticFileEndpoint::new("static/style.css"));

    tracing::info!("karu-tokyo-server listening on 127.0.0.1:4300");
    Server::new(TcpListener::bind("127.0.0.1:4300")).run(app).await
}
