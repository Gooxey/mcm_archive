//! This module provides the [`ConcurrentClass trait`](ConcurrentClass) which provides standard functions used by every concurrent struct in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).


use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::marker;
use std::time::Instant;

use crate::config::Config;
use crate::mcmanage_error::MCManageError;
use crate::log;


/// This trait provides standard functions used by every concurrent struct in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). \
/// 
/// ## Required Methods
/// 
/// | Method                                                                           | Description                                                                                                                      |
/// |----------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
/// | [`get_config_unlocked(...) -> Arc<...>`](ConcurrentClass::get_config_unlocked)   | Return the config of a given struct.                                                                                             |
/// | [`get_name_unlocked(...) -> String`](ConcurrentClass::get_name_unlocked)         | Return the name a given struct is identified with.                                                                               |
/// | [`get_name_poison_error(...) -> String`](ConcurrentClass::get_name_poison_error) | Return the name a given struct is identified with.                                                                               |
/// | [`get_default_state(...) -> T`](ConcurrentClass::get_default_state)              | The purpose of this function is to create a new struct of type T based on the data that can be recovered from the corrupted one. |
/// | [`start(...) -> Result<...>`](ConcurrentClass::start)                            | Start a given struct.                                                                                                            |
/// | [`stop(...) -> Result<...>`](ConcurrentClass::stop)                              | Stop a given struct.                                                                                                             |
/// 
/// 
/// ## Provided Methods
/// 
/// | Method                                                                              | Description                                                                                                                                  |
/// |-------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`wait_for_start_confirm(...)`](ConcurrentClass::wait_for_start_confirm)            | This function is optional and only required to be defined if it is required for the restart function to wait until a specific event happens. |
/// | [`reset(...)`](ConcurrentClass::reset)                                              | Reset the provided struct to its default state.                                                                                              |
/// | [`reset_unlocked(...)`](ConcurrentClass::reset_unlocked)                            | Reset the provided struct to its default state.                                                                                              |
/// | [`get_lock_pure(...) -> Option<...>`](ConcurrentClass::get_lock_pure)               | Get the lock of a given struct.                                                                                                              |
/// | [`get_lock_nonblocking(...) -> Result<...>`](ConcurrentClass::get_lock_nonblocking) | Get the lock of a given struct.                                                                                                              |
/// | [`get_lock(...) -> MutexGuard<...>`](ConcurrentClass::get_lock)                     | Get the lock of a given struct.                                                                                                              |
/// | [`restart(...) -> Result<...>`](ConcurrentClass::restart)                           | Restart the given struct.                                                                                                                    |
/// | [`self_restart(...)`](ConcurrentClass::self_restart)                                | Restart the given struct.                                                                                                                    |
pub trait ConcurrentClass<T, C>
where
    T: marker::Send + marker::Sync + 'static,
    C: Config
{
    /// Return the config of a given struct. \
    /// The struct provided needs to be unlocked.
    fn get_config_unlocked(class_lock: &MutexGuard<T>) -> Arc<C>;
    /// Return the name a given struct is identified with. \
    /// The struct provided needs to be unlocked.
    fn get_name_unlocked(class_lock: &MutexGuard<T>) -> String;
    /// Return the name a given struct is identified with. \
    /// The struct provided needs to be contained inside a [`PoisonError`].
    fn get_name_poison_error(class_lock: &MutexGuard<T>) -> String;
    /// The purpose of this function is to create a new struct of type T based on the data that can be recovered from the corrupted one.
    fn get_default_state(class_lock: &MutexGuard<T>) -> T;

    /// Start a given struct.
    fn start(class: &Arc<Mutex<T>>, log_messages: bool) -> Result<(), MCManageError>;
    /// Stop a given struct.
    fn stop(class: &Arc<Mutex<T>>, log_messages: bool) -> Result<(), MCManageError>;


    /// This function is optional and only required to be defined if it is required for the restart function to wait until a specific event happens. \
    /// \
    /// It has to be ensured that this function does not hold the lock of a struct for the entire duration of this function's execution.
    fn wait_for_start_confirm(_class: &Arc<Mutex<T>>) {
        // implementation only optional
    }

    /// Reset the provided struct to its default state. \
    /// Use the [`reset_unlocked function`](ConcurrentClass::reset_unlocked) if you want to unlock the struct yourself.
    fn reset(class: &Arc<Mutex<T>>) {
        match class.lock() {
            Ok(mut class) => {
                Self::reset_unlocked(&mut class);
            }
            Err(class) => {
                Self::reset_unlocked(&mut class.into_inner());
            }
        }
    }
    /// Reset the provided struct to its default state. \
    /// Use the [`reset function`](ConcurrentClass::reset) if you don't want to unlock the struct yourself.
    fn reset_unlocked(class: &mut MutexGuard<T>) {
        **class = Self::get_default_state(class);
    }
    
    /// Get the lock of a given struct. \
    /// This function will block the thread calling until the lock is claimed. If an error occurs, this function will return None. \
    /// This function will not handle the poison error.
    /// 
    /// ## Alternatives
    /// 
    /// Any error handling done will include the struct restarting in the event of an error.
    /// 
    /// 1. [`get_lock`](ConcurrentClass::get_lock)
    ///     - This function will handle the poison error, blocking the thread calling. Because this function waits on the end of the error handling process, it can be
    ///       guaranteed that the lock will be returned.
    /// 2. [`get_lock_nonblocking`](ConcurrentClass::get_lock_nonblocking):
    ///     - This function will handle the poison error in a separate thread.
    fn get_lock_pure(class: &Arc<Mutex<T>>, error_message: bool) -> Option<MutexGuard<T>> {
        match class.lock() {
            Ok(lock) => {
                return Some(lock);
            }
            Err(erro) => { 
                let class_lock = erro.into_inner();
                if error_message { log!("erro", Self::get_name_poison_error(&class_lock), "This struct got corrupted! A restart will be attempted."); }
                return None;
            }
        }
    }
    /// Get the lock of a given struct. \
    /// This function will block the thread calling until the lock is claimed. If an error occurs, this function will handle the poison error in a separate thread and
    /// return an error.
    /// 
    /// ## Alternatives
    /// 
    /// Any error handling done will include the struct restarting in the event of an error.
    /// 
    /// 1. [`get_lock`](ConcurrentClass::get_lock)
    ///     - This function will handle the poison error, blocking the thread calling. Because this function waits on the end of the error handling process, it can be
    ///       guaranteed that the lock will be returned.
    /// 2. [`get_lock_pure`](ConcurrentClass::get_lock_pure):
    ///     - This function will not handle the poison error and will only return None if it fails to receive the lock.
    fn get_lock_nonblocking(class: &Arc<Mutex<T>>) -> Result<MutexGuard<T>, MCManageError> {
        let class_clone = class.clone();
        if let Some(lock) = Self::get_lock_pure(class, true) {
            return Ok(lock);
        }
        thread::spawn(move || {
            if let Err(_) = Self::restart(&class_clone) {
                Self::reset(&class_clone);
            }
        });

        return Err(MCManageError::CriticalError);
    }
    /// Get the lock of a given struct. \
    /// This function will block the thread calling until the lock is claimed. If an error occurs, this function will handle it and try again to acquire the lock. \
    /// Therefore, it is guaranteed that this function will return the lock.
    /// 
    /// ## Alternatives
    /// 
    /// Any error handling done will include the struct restarting in the event of an error.
    /// 
    /// 1. [`get_lock_pure`](ConcurrentClass::get_lock_pure):
    ///     - This function will not handle the poison error and will only return None if it fails to receive the lock.
    /// 2. [`get_lock_nonblocking`](ConcurrentClass::get_lock_nonblocking):
    ///     - This function will handle the poison error in a separate thread.
    fn get_lock(class: &Arc<Mutex<T>>) -> MutexGuard<T> {
        if let Some(lock) = Self::get_lock_pure(class, true) {
            return lock;
        }
        if let Err(_) = Self::restart(&class) {
            Self::reset(class);
        }
        
        return Self::get_lock(class);
    }

    /// Restart the given struct. \
    /// A returned error indicates that this function was unable to start the struct provided.\
    /// \
    /// If you want to restart a given struct without blocking the thread calling the function, use the [`self_restart function`](ConcurrentClass::self_restart).
    fn restart(class: &Arc<Mutex<T>>) -> Result<(), MCManageError> {
        let restart_time = Instant::now();

        let name;
        let config;
        match class.lock() {
            Ok(lock) => {
                name = Self::get_name_unlocked(&lock);
                config = Self::get_config_unlocked(&lock);
            }
            Err(erro) => {
                let class_lock = erro.into_inner();

                name = Self::get_name_unlocked(&class_lock);
                config = Self::get_config_unlocked(&class_lock);
            }
        }

        log!("", name, "Restarting...");


        // ### STOPPING ###
        loop {
            match Self::stop(&class, false) {
                Ok(_) => {
                    break;
                }
                Err(erro) => {
                    match erro {
                        MCManageError::FatalError => {
                            break;
                        }
                        _ => {
                            thread::sleep(*config.refresh_rate());
                        }
                    }
                }
            }
        }
        Self::reset(&class);


        // ### STARTING ###

        // Try to start the class until it succeeds or the fail limit is reached
        let failcounter = 0;
        loop {
            if let Err(_) = Self::start(&class, false) {
                if failcounter == *config.max_tries() {
                    log!("erro", &name, "The maximum number of start attempts has been reached. This struct will no longer attempt to start.");
                    Self::reset(&class);
                    return Err(MCManageError::FatalError);
                } else {
                    log!("warn", &name, "This was attempt number {} out of {}", failcounter, config.max_tries());
                }
                thread::sleep(*config.refresh_rate());
            } else {
                break;
            }
        }

        // wait for an optional start confirmation
        Self::wait_for_start_confirm(&class);

        log!("", &name, "Restarted in {:.3} secs!", restart_time.elapsed().as_secs_f64());
        return Ok(());
    }
    /// Restart the given struct. \
    /// A returned error indicates that this function was unable to start the struct provided.\
    /// \
    /// If you want to restart a given struct and block the thread calling the function, use the [`restart function`](ConcurrentClass::restart).
    fn self_restart(class: &Arc<Mutex<T>>) {
        let class_clone = class.clone();
        thread::spawn(move || 
            // Try to restart the class
            // if it returns an error the thread will panic
            if let Err(_) = Self::restart(&class_clone) {
                panic!("The restart function was unable to start the struct.")
            } 
        );
    }
}