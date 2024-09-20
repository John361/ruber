use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::CreateKind;

use crate::config::RuberConfig;
use crate::driving::Driver;
use crate::routing::Route;

pub async fn start(config: &RuberConfig) -> anyhow::Result<()> {
    for route in &config.routes.routes {
        listen_for_new_passenger(route).await?; // TODO: spawn in new thread
    }

    Ok(())
}

async fn listen_for_new_passenger(route: &Route) -> anyhow::Result<()> { // TODO: should not return something
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    let folder = &route.source.folder();

    watcher.watch(folder, RecursiveMode::Recursive)?;
    log::debug!("Agent is watching {}", folder.display());

    loop {
        match rx.recv() {
            Ok(event) => {
                if let Ok(event) = event {
                    if event.kind == EventKind::Create(CreateKind::File) {
                        for path in event.paths {
                            if let Some(path) = path.file_name() {
                                let file_name = path.to_str().unwrap();
                                let mut driver = Driver::new(&route);
                                driver.take_passenger(file_name).await?;
                                driver.drive().await?;
                            }
                        }
                    } else {
                        log::debug!("Unmanaged event: {:?}", event.kind);
                    }
                }
            }

            Err(error) => {
                log::error!("Cannot receive passenger from: {}. Error: {}", folder.display(), error);
            }
        }
    }
}
