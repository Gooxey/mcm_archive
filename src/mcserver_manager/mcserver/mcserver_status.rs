//! This module provides the [`MCServerStatus enum`](MCServerStatus), which gets used for the representation of the [`MCServer's`](super::MCServer) status.


/// This enum represents the [`MCServer's`](super::MCServer) status.
/// 
/// # Status
/// 
/// | Status                                     | Description                                                                                                                             |
/// |--------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
/// | [`Stopped`](MCServerStatus::Stopped)       | The MCServer is currently stopped. Therefore, the assigned Minecraft server is not running at this moment.                              |
/// | [`Started`](MCServerStatus::Started)       | The MCServer has been started. Therefore, the assigned Minecraft server is running at this moment.                                      |
/// | [`Starting`](MCServerStatus::Starting)     | The MCServer is currently starting. It will be fully functional as soon as the status switches to [`Started`](MCServerStatus::Started). |
/// | [`Stopping`](MCServerStatus::Stopping)     | The MCServer is currently stopping. Before doing anything, wait for the status to change to [`Stopped`](MCServerStatus::Stopped).       |
/// | [`Restarting`](MCServerStatus::Restarting) | The MCServer is currently restarting. Wait for the status to change to [`Started`](MCServerStatus::Started) for full functionality.     |
#[derive(PartialEq)]
pub enum MCServerStatus {
    /// The MCServer is currently stopped. Therefore, the assigned Minecraft server is not running at this moment.
    Stopped,
    /// The MCServer has been started. Therefore, the assigned Minecraft server is running at this moment.
    Started,
    /// The MCServer is currently starting. It will be fully functional as soon as the status switches to [`Started`](MCServerStatus::Started).
    Starting,
    /// The MCServer is currently stopping. Before doing anything, wait for the status to change to [`Stopped`](MCServerStatus::Stopped).
    Stopping,
    /// The MCServer is currently restarting. Wait for the status to change to [`Started`](MCServerStatus::Started) for full functionality.
    Restarting
}
impl Clone for MCServerStatus {
    fn clone(&self) -> Self {
        match self {
            MCServerStatus::Stopped => { MCServerStatus::Stopped }
            MCServerStatus::Started => { MCServerStatus::Started }
            MCServerStatus::Starting => { MCServerStatus::Starting }
            MCServerStatus::Stopping => { MCServerStatus::Stopping }
            MCServerStatus::Restarting => { MCServerStatus::Restarting }
        }
    }
}