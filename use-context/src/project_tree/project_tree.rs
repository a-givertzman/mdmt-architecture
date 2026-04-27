use std::{sync::Arc, time::Duration};
use sal_core::{dbg::Dbg, error::{self, Error}};
use sal_sync::{kernel::state::ExitNotify, services::{Service, entity::{Name, Object}}, sync::{Handles, Owner}, thread_pool::Scheduler};

use crate::{kernel::types::channel::{self, Receiver, RecvTimeoutError, Sender}, project_tree::ProjectTreeConf, snapshot::Event};

///
/// ### Service | ProjectTree
/// 
/// Это навигационный граф, связывающий воедино исходные данные,
/// результаты расчетов,  математику, тэги зависимостей расчетовб 3D модель,
/// отчеты, и актуальные статусов для бэкенда и пользователя.
///
/// Работает в самостоятельном потоке
pub struct ProjectTree {
    name: Name,
    conf: ProjectTreeConf,
    /// Канал для отправки событий слиенту
    client_link: Owner<Sender<Event>>,
    /// Внешний кончик канала, в который расветы будут отправлять статусы нод
    link_tx: Sender<Event>,
    /// Тут получаем статусы нод от расчетов
    link_rx: Owner<Receiver<Event>>,
    scheduler: Scheduler,
    handles: Handles<()>,
    exit: Arc<ExitNotify>,
    dbg: Dbg,
}
//
//
impl ProjectTree {
    //
    /// Crteates new instance of the [ProjectTree] 
    pub fn new(conf: ProjectTreeConf, client: Sender<Event>, scheduler: Scheduler) -> Self {
        let dbg = Dbg::new(conf.name.parent(), conf.name.me());
        let (link_tx, rx) = channel::channel::unbounded();
        Self {
            name: conf.name.clone(),
            conf,
            client_link: Owner::new(client),
            link_tx,
            link_rx: Owner::new(rx),
            scheduler,
            handles: Handles::new(&dbg),
            exit: Arc::new(ExitNotify::new(&dbg,None, None)),
            dbg,
        }
    }
}
//
//
impl Object for ProjectTree {
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl std::fmt::Debug for ProjectTree {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProjectTree")
            .field("name", &self.name)
            .finish()
    }
}
//
// 
impl Service for ProjectTree {
    //
    fn run(&self) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        let error = Error::new(&dbg, "run");
        log::info!("{dbg}.run | Starting...");
        let handle = self.scheduler.spawn({
            let dbg = dbg.clone();
            let exit = self.exit.clone();
            let link_rx = self.link_rx.take().ok_or(error.err("Can't take link_rx from self"))?;
            let client_link = self.client_link.take().ok_or(error.err("Can't take client_link from self"))?;
            let recv_timeout = Duration::from_millis(100);
            move || {
                log::info!("{dbg}.run | Ready");
                while !exit.get() {
                    let mut is_empty = false;
                    let mut events = vec![];
                    while !is_empty {
                        match link_rx.recv_timeout(recv_timeout) {
                            Ok(event) => {
                                events.push(event);
                            }
                            Err(err) => match err {
                                RecvTimeoutError::Timeout => is_empty = true,
                                _ => exit.exit(),
                            },
                        }
                    }
                    for e in events {
                        if let Err(err) = client_link.send(e) {
                            log::warn!("{dbg}.run | Can't send event to the clint: {err}");
                        }
                    }
                }
                Ok(())
            }
        }).map_err(|err| error.pass_with("Start failed", err))?;
        self.handles.push(handle);
        log::info!("{dbg}.run | Starting - Ok");
        Ok(())
    }
    //
    fn is_finished(&self) -> bool {
        self.handles.is_finished()
    }
    //
    fn wait(&self) -> Result<(), Error> {
        self.handles.wait()
    }
    //
    fn exit(&self) {
        self.exit.exit();
    }    
}