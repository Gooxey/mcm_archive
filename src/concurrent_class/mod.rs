//! This module provides the [`ConcurrentClass trait`](ConcurrentClass) which provides standard functions used by every concurrent struct in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).


use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::oneshot::{Sender, Receiver};
use tokio::time::sleep;

use crate::config::Config;
use crate::mcmanage_error::MCManageError;
use crate::log;

use self::status::Status;
use self::qol_functions::check_allowed_restart;


pub mod status;
pub mod qol_functions;


/// This trait provides standard functions used by every concurrent struct in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). \
/// 
/// # Important
/// 
/// To implement this trait the async_trait proc_macro needs to be used:
/// ```
/// use mcm_misc::concurrent_class::ConcurrentClass;
/// use async_trait::async_trait;
/// 
/// struct MyConcurrentStruct {}
/// #[async_trait]
/// impl ConcurrentClass for MyConcurrentStruct {
///     fn name(self: &Arc<Self>) -> String {
///         self.name.clone()
///     }
///     fn config(self: &Arc<Self>) -> Arc<Config> {
///         self.config.clone()
///     }
///     async fn status(self: &Arc<Self>) -> Status {
///         *self.status.lock().await
///     }
///     async fn set_status(self: &Arc<Self>, new_status: Status) {
///         *self.status.lock().await = new_status
///     }
///     async fn reset(self: &Arc<Self>) {
///         todo!()
///     }
///     async fn impl_start(self: Arc<Self>, restart: bool) -> Result<(), MCManageError> {
///         check_allowed_start(&self, restart).await?;
///         todo!()
///     }
///     async fn impl_stop(self: Arc<Self>, restart: bool, forced: bool) -> Result<(), MCManageError> {
///         check_allowed_stop(&self, restart, forced).await?;
///         todo!()
///     }
///     async fn main(self: Arc<Self>, mut bootup_result: Option<Sender<()>>) -> Result<(), MCManageError> {
///         todo!()
///     }
/// }
/// ```
/// 
/// # Required Methods
/// 
/// | Method                                                          | Description                                                  |
/// |-----------------------------------------------------------------|--------------------------------------------------------------|
/// | [`name(...) -> String`](ConcurrentClass::name)                  | Return the name a given struct is identified with.           |
/// | [`config(...) -> Arc<Config>`](ConcurrentClass::config)         | Return the config of a given struct.                         |
/// | [`status(...) -> Status`](ConcurrentClass::status)              | Return the status a given struct.                            |
/// | [`set_status(...)`](ConcurrentClass::set_status)                | Set the status a given struct.                               |
/// |                                                                 |                                                              |
/// | [`reset(...)`](ConcurrentClass::reset)                          | Reset a given struct to its starting values.                 |
/// | [`impl_start(...) -> Result<...>`](ConcurrentClass::impl_start) | This is the blocking implementation to start a given struct. |
/// | [`impl_stop(...) -> Result<...>`](ConcurrentClass::impl_stop)   | This is the blocking implementation to stop a given struct.  |
/// | [`main(...)`](ConcurrentClass::main)                            | This represents the main loop of a given struct.             |
/// 
/// 
/// # Provided Methods
/// 
/// | Method                                                                        | Description                       	                         |
/// |-------------------------------------------------------------------------------|----------------------------------------------------------------|
/// | [`impl_restart(...) -> Result<...>`](ConcurrentClass::impl_restart)           | This is the blocking implementation to restart a given struct. |
/// | [`start(...)`](ConcurrentClass::start)                                        | Start a given struct without blocking the calling thread.      |
/// | [`stop(...)`](ConcurrentClass::stop)                                          | Stop a given struct without blocking the calling thread.       |
/// | [`restart(...)`](ConcurrentClass::restart)                                    | Restart a given struct without blocking the calling thread.    |
/// |                                                                               |   	                                                         |
/// | [`recv_start_result(...) -> Result<...>`](ConcurrentClass::recv_start_result) | Wait for the started signal.                                   |
/// | [`send_start_result(...) -> Result<...>`](ConcurrentClass::send_start_result) | Send the started signal.                                       |
#[async_trait]
pub trait ConcurrentClass
where
    Self: Sized + Send + Sync + 'static
{   
    /// Return the name a given struct is identified with.
    fn name(self: &Arc<Self>) -> String;
    /// Return the config of a given struct.
    fn config(self: &Arc<Self>) -> Arc<Config>;
    /// Return the status a given struct.
    async fn status(self: &Arc<Self>) -> Status;
    /// Set the status a given struct.
    async fn set_status(self: &Arc<Self>, new_status: Status);

    /// Reset a given struct to its starting values.
    async fn reset(self: &Arc<Self>);
    /// This is the blocking implementation to start a given struct. \
    /// For a non-blocking mode use the [`start method`](ConcurrentClass::start). \
    /// \
    /// The `restart` parameter will be used by the [`restart method`](ConcurrentClass::impl_restart) to deactivate all non-fatal error messages of this method and to enable
    /// this method to be executed during a restart. \
    /// \
    /// It is advisable to use the [`check_allowed_start`](qol_functions::check_allowed_stop) function at the start of this method. \
    /// Use the [`recv_start_result function`](ConcurrentClass::recv_start_result) to wait till the main thread has started.
    async fn impl_start(self: Arc<Self>, restart: bool) -> Result<(), MCManageError>;
    /// This is the blocking implementation to stop a given struct. \
    /// For a non-blocking mode use the [`stop method`](ConcurrentClass::stop). \
    /// \
    /// The `restart` parameter will be used by the [`restart method`](ConcurrentClass::impl_restart) to deactivate all non-fatal error messages of this method and to enable
    /// this method to be executed during a restart. \
    /// \
    /// The `forced` parameter is used to wait for a given struct to start / stop to ensure a stop attempt. \
    /// \
    /// It is advisable to use the [`check_allowed_stop`](qol_functions::check_allowed_stop) function at the start of this method.
    async fn impl_stop(self: Arc<Self>, restart: bool, forced: bool) -> Result<(), MCManageError>;
    /// This represents the main loop of a given struct.
    async fn main(self: Arc<Self>, mut bootup_result: Option<Sender<()>>) -> Result<(), MCManageError>;
    

    /// This is the blocking implementation to restart a given struct. \
    /// For a non-blocking mode use the [`restart method`](Self::restart).
    async fn impl_restart(self: Arc<Self>) -> Result<(), MCManageError> {
        check_allowed_restart(&self).await?;
        
        let restart_time = Instant::now();

        log!("", self.name(), "Restarting...");


        // ### STOPPING ###
        loop {
            match self.clone().impl_stop(true, true).await {
                Ok(_) => {
                    break;
                }
                Err(erro) => {
                    match erro {
                        MCManageError::FatalError => {
                            break;
                        }
                        _ => {
                            sleep(*self.config().refresh_rate()).await;
                        }
                    }
                }
            }
        }
        self.reset().await;
        self.set_status(Status::Restarting).await;


        // ### STARTING ###

        // Try to start the class until it succeeds or the fail limit is reached
        let mut failcounter = 0;
        loop {
            if let Err(_) = self.clone().impl_start(true).await {
                if failcounter == *self.config().max_tries() {
                    log!("erro", self.name(), "The maximum number of start attempts has been reached. This struct will no longer attempt to start.");
                    self.reset().await;
                    return Err(MCManageError::FatalError);
                } else {
                    self.set_status(Status::Restarting).await;
                    failcounter += 1;
                    log!("warn", self.name(), "This was attempt number {} out of {}", failcounter, self.config().max_tries());
                }
                sleep(*self.config().refresh_rate()).await;
            } else {
                break;
            }
        }
        self.set_status(Status::Started).await;

        log!("", self.name(), "Restarted in {:.3} secs!", restart_time.elapsed().as_secs_f64());
        return Ok(());
    }
    /// Start a given struct without blocking the calling thread. \
    /// For a blocking mode use the [`impl_start method`](Self::impl_start).
    fn start(self: &Arc<Self>) {
        spawn(self.clone().impl_start(false));
    }
    /// Stop a given struct without blocking the calling thread. \
    /// For a blocking mode use the [`impl_stop method`](Self::impl_stop).
    fn stop(self: &Arc<Self>) {
        spawn(self.clone().impl_stop(false, true));
    }
    /// Restart a given struct without blocking the calling thread. \
    /// For a blocking mode use the [`impl_restart method`](Self::impl_restart).
    fn restart(self: &Arc<Self>) {
        spawn(self.clone().impl_restart());
    }

    /// Wait for the started signal.
    async fn recv_start_result(self: &Arc<Self>, bootup_result: Receiver<()>) -> Result<(), MCManageError> {
        if let Err(_) = bootup_result.await {
            if let Status::Stopping = self.status().await {
            } else {
                log!("erro", self.name(), "The main thread crashed. This struct could not be started.");
                self.reset().await;
                return Err(MCManageError::FatalError);
            }
        }
        Ok(())
    }
    /// Send the started signal.
    async fn send_start_result(self: &Arc<Self>, bootup_result: Sender<()>) -> Result<(), MCManageError> {
        if let Err(_) = bootup_result.send(()) {            
            log!("erro", self.name(), "The thread starting the struct got stopped! This struct will now shut down.");
            self.stop();
            return Err(MCManageError::FatalError);
        }
        Ok(())
    }
}